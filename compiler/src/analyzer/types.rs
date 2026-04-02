#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // Signed Integers
    Tiny,    // 8-bit
    Short,   // 16-bit
    Ant,     // 32-bit
    Long,    // 64-bit
    Vast,    // 128-bit

    // Unsigned Integers
    Utiny,   // 8-bit unsigned
    Ushort,  // 16-bit unsigned
    Uant,    // 32-bit unsigned
    Ulong,   // 64-bit unsigned

    // Floats
    Half,    // 32-bit float
    Dbl,     // 64-bit float
    Precise, // decimal exact

    // Text
    Chr,     // single char
    Str,     // string
    Txt,     // large text

    // Boolean & Nil
    Bool,
    Nil,

    // Collections
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Set(Box<Type>),

    // Special
    Tensor,
    Blob,

    // Function type
    Fn {
        params: Vec<Type>,
        ret: Box<Type>,
    },

    // Unknown/Any (for inference)
    Any,
    Unknown,
}

impl Type {
    // String से Type बनाओ
    pub fn from_str(s: &str) -> Type {
        match s {
            "tiny"    => Type::Tiny,
            "short"   => Type::Short,
            "ant"     => Type::Ant,
            "long"    => Type::Long,
            "vast"    => Type::Vast,
            "utiny"   => Type::Utiny,
            "ushort"  => Type::Ushort,
            "uant"    => Type::Uant,
            "ulong"   => Type::Ulong,
            "half"    => Type::Half,
            "dbl"     => Type::Dbl,
            "precise" => Type::Precise,
            "chr"     => Type::Chr,
            "str"     => Type::Str,
            "txt"     => Type::Txt,
            "bool"    => Type::Bool,
            "nil"     => Type::Nil,
            "tensor"  => Type::Tensor,
            "blob"    => Type::Blob,
            "any"     => Type::Any,
            _         => Type::Unknown,
        }
    }

    // Type को readable string में convert करो
    pub fn to_str(&self) -> &str {
        match self {
            Type::Tiny    => "tiny",
            Type::Short   => "short",
            Type::Ant     => "ant",
            Type::Long    => "long",
            Type::Vast    => "vast",
            Type::Utiny   => "utiny",
            Type::Ushort  => "ushort",
            Type::Uant    => "uant",
            Type::Ulong   => "ulong",
            Type::Half    => "half",
            Type::Dbl     => "dbl",
            Type::Precise => "precise",
            Type::Chr     => "chr",
            Type::Str     => "str",
            Type::Txt     => "txt",
            Type::Bool    => "bool",
            Type::Nil     => "nil",
            Type::Tensor  => "tensor",
            Type::Blob    => "blob",
            Type::List(_) => "list",
            Type::Map(..) => "map",
            Type::Set(_)  => "set",
            Type::Fn{..}  => "fn",
            Type::Any     => "any",
            Type::Unknown => "unknown",
        }
    }

    // क्या यह integer type है?
    pub fn is_int(&self) -> bool {
        matches!(self,
            Type::Tiny | Type::Short | Type::Ant |
            Type::Long | Type::Vast | Type::Utiny |
            Type::Ushort | Type::Uant | Type::Ulong
        )
    }

    // क्या यह float type है?
    pub fn is_float(&self) -> bool {
        matches!(self, Type::Half | Type::Dbl | Type::Precise)
    }

    // क्या यह numeric type है?
    pub fn is_numeric(&self) -> bool {
        self.is_int() || self.is_float()
    }

    // क्या यह text type है?
    pub fn is_text(&self) -> bool {
        matches!(self, Type::Chr | Type::Str | Type::Txt)
    }

    // Type compatible है?
    pub fn is_compatible(&self, other: &Type) -> bool {
        if self == other { return true; }
        if *self == Type::Any || *other == Type::Any { return true; }
        // Numeric types compatible हैं
        if self.is_numeric() && other.is_numeric() { return true; }
        // Text types compatible हैं
        if self.is_text() && other.is_text() { return true; }
        false
    }
}
