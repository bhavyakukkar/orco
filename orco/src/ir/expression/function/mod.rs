use super::*;

/// Function signature (Arguments and return type)
pub mod signature;
pub use signature::Signature;

/// A function
#[derive(Debug)]
pub struct Function {
    /// Function signature
    pub signature: Signature,
    /// Function body
    pub body: std::sync::Mutex<Expression>,
    /// Span of the function
    pub span: Span,
}

impl Function {
    /// Create a new function
    pub fn new(signature: Signature, body: Expression, span: Span) -> Self {
        Self {
            signature,
            body: body.into(),
            span,
        }
    }

    /// Infer & check types
    pub fn infer_and_check_types(&self, type_inference: &mut TypeInference) {
        let old_return_type = type_inference
            .return_type
            .replace(self.signature.return_type.clone());
        let old_scopes = std::mem::replace(&mut type_inference.scopes, Vec::new());

        type_inference.push_scope();
        for arg in self.signature.args.iter() {
            arg.as_ref().infer_types(type_inference);
        }

        let mut body = self.body.try_lock().unwrap();
        body.infer_types(type_inference);
        type_inference.pop_scope();

        for arg in self.signature.args.iter() {
            arg.finish_and_check_types(type_inference);
        }
        let return_type = body.finish_and_check_types(type_inference);
        if !return_type.morphs(&self.signature.return_type) {
            type_inference.reporter.report_type_error(
                format!(
                    "Return type mismatch: expected '{}', got '{}'",
                    self.signature.return_type.inner, return_type
                ),
                body.span(),
                vec![(
                    "Expected because of this",
                    self.signature.return_type.span.clone(),
                )],
            );
            type_inference.abort_compilation = true;
        }

        type_inference.scopes = old_scopes;
        type_inference.return_type = old_return_type;
    }
}

impl Clone for Function {
    fn clone(&self) -> Self {
        Self {
            signature: self.signature.clone(),
            body: std::sync::Mutex::new(self.body.try_lock().unwrap().clone()),
            span: self.span.clone(),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {} {}", self.signature, self.body.try_lock().unwrap())
    }
}

/// An extern function
#[derive(Clone, Debug)]
pub struct ExternFunction {
    /// Extern function name
    pub name: Name,
    /// Function signature
    pub signature: Signature,
    /// Span of the extern function declaration
    pub span: Span,
}

impl std::fmt::Display for ExternFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "extern fn {}{}", self.name, self.signature)
    }
}
