use super::*;
use parsel::{ast::Brace, syn::token::Semi};

pub mod block;
pub use block::Block;
pub mod branching;
pub use branching::If;
pub mod control_flow;
pub use control_flow::Return;
pub mod variables;
pub use variables::VariableDeclaration;

#[derive(Parse, ToTokens)]
pub enum Statement {
    Block(Block),
    If(Box<If>),
    Return(Return),
    VariableDeclaration(Box<VariableDeclaration>),
    Expression(Expression, Semi),
    Empty(Semi),
}

impl Statement {
    pub fn build(
        &self,
        ctx: &mut orco::TypeInferenceContext,
        expressions: &mut Vec<orco::Expression>,
    ) {
        match self {
            Statement::Block(block) => block.build(ctx, expressions),
            Statement::If(statement) => statement.build(ctx, expressions),
            Statement::Return(r#return) => r#return.build(ctx, expressions),
            Statement::VariableDeclaration(decl) => {
                let Some(scope) = ctx.scopes.last_mut() else {
                    todo!("Error")
                };
                let r#type = decl.ty.as_orco();
                for var in &decl.variables {
                    let name = var.name.to_string();
                    scope.insert(
                        name.clone(),
                        std::sync::Arc::new(std::sync::RwLock::new(orco::Variable::new(
                            Some(name),
                            r#type.clone(),
                        ))),
                    );
                }
            }
            Statement::Expression(expression, _) => {
                let expr = expression.build(ctx, expressions);
                expressions.push(expr)
            }
            Statement::Empty(_) => (),
        }
    }
}
