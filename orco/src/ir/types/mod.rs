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

    /// Never type, can't hold any value
    Never,

    /// Pointer type
    Pointer(Box<Type>),

    /// Unit type, can only hold one value
    Unit,

    /// Custom type, f.e. a struct or a type alias
    Custom(String),
    /// Error type
    Error,
}

impl Type {
    /// Create a new unit type
    pub fn unit() -> Self {
        Self::Unit
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
            Self::Never => write!(f, "!"),
            Self::Pointer(r#type) => write!(f, "{}*", r#type),
            Self::Unit => write!(f, "()"),
            Self::Custom(name) => write!(f, "{}", name),
            Self::Error => write!(f, "<ERROR>"),
        }
    }
}

impl std::ops::BitOr for Type {
    type Output = Type;

    fn bitor(self, rhs: Type) -> Self::Output {
        match self {
            Self::Never | Self::Error => rhs,
            r#type => r#type,
        }
    }
}
