use crate::type_inference::TypeVariableID;
use crate::{Span, Spanned};
use std::num::NonZeroU16;

/// A type enum consists of all builtin types and a custom variant
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// Integer, f.e. i32, i64, etc. Stores the size in bytes
    Int(NonZeroU16),
    /// Unsigned integer, f.e. u8, u32, u64, etc. Stores the size in bytes
    Unsigned(NonZeroU16),
    /// Floating point number, f.e. f32, f64. Stores the size in bytes
    Float(NonZeroU16),
    /// Boolean (true or false)
    Bool,
    /// Character
    Char,

    /// Pointer type
    Pointer(Box<Type>),
    /// Function pointer
    FunctionPointer(Vec<Spanned<Type>>, Box<Spanned<Type>>),
    /// Custom type, f.e. a struct or a type alias
    Custom(Span),

    /// Never type, can't hold any value
    Never,

    /// Unit type, can only hold one value
    Unit,

    /// A wildcard type (aka non-inferred)
    Wildcard,
    /// Integer wildcard (number literal, that automatically infers type)
    IntegerWildcard,
    /// Type variable (used only during type inference)
    TypeVariable(TypeVariableID),
    /// Error type
    Error,
}

impl Type {
    /// Create a new unit type
    pub fn unit() -> Self {
        Self::Unit
    }

    /// Does this type morphs to the target type
    pub fn morphs(&self, target_type: &Type) -> bool {
        match (self, target_type) {
            (Self::Never, _) => true,
            (Self::Wildcard, _) => true,
            (Self::Error, _) => true,
            (_, Self::Never) => true,
            (_, Self::Wildcard) => true,
            (_, Self::Error) => true,
            (Self::IntegerWildcard, Self::Int(_)) => true,
            (Self::IntegerWildcard, Self::Unsigned(_)) => true,
            (Self::Int(_), Self::IntegerWildcard) => true,
            (Self::Unsigned(_), Self::IntegerWildcard) => true,
            _ => self == target_type,
        }
    }

    /// Is this type complete (nothing to infer)
    /// TypeVariables are considered complete
    pub fn complete(&self) -> bool {
        !matches!(self, Self::Wildcard | Self::IntegerWildcard | Self::Error)
    }

    /// Complete this type to be equal to the other type
    pub fn equate(&mut self, other: Type) -> bool {
        if self == &other {
            return true;
        }
        match (self as &Self, &other) {
            (Self::Never, _) => true,
            (Self::Wildcard | Self::Error, _) => {
                *self = other;
                true
            }
            (_, Self::Never | Self::Wildcard | Self::Error) => true,
            (Self::IntegerWildcard, Self::Int(_) | Self::Unsigned(_)) => {
                *self = other;
                true
            }
            (Self::Int(_) | Self::Unsigned(_), Self::IntegerWildcard) => true,
            _ => false,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(size) => write!(f, "i{}", size.get() * 8),
            Self::Unsigned(size) => write!(f, "u{}", size.get() * 8),
            Self::Float(size) => write!(f, "f{}", size.get() * 8),
            Self::Bool => write!(f, "bool"),
            Self::Char => write!(f, "char"),

            Self::Pointer(r#type) => write!(f, "{}*", r#type),
            Self::FunctionPointer(args, r#return) => {
                write!(f, "fn(")?;
                for (index, arg) in args.iter().enumerate() {
                    if index > 0 {
                        write!(f, ", ")?;
                    }
                    arg.fmt(f)?;
                }
                write!(f, ") -> {}", r#return.inner)?;
                Ok(())
            }
            Self::Custom(name) => write!(f, "{}", name),

            Self::Never => write!(f, "!"),
            Self::Unit => write!(f, "()"),

            Self::Wildcard => write!(f, "_"),
            Self::IntegerWildcard => write!(f, "integer"),
            Self::TypeVariable(id) => write!(f, "{}", id),
            Self::Error => write!(f, "<ERROR>"),
        }
    }
}

impl std::ops::BitOr for Type {
    type Output = Type;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.equate(rhs);
        self
    }
}
