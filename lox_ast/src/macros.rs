#[macro_export]
macro_rules! ast_enum {
    (pub enum $enum_name: ident {$($walker: ident: $name: ident($ty: ty)),+ $(,)?}) => {
        #[derive(Debug, Clone)]
        pub enum $enum_name {
            $($name($ty)),+
        }

        impl $enum_name {
            pub fn walk<V: Visitor>(&self, visitor: &mut V) -> V::Result {
                match self {
                    $($enum_name::$name(v) => visitor.$walker(v)),+
                }
            }

            pub fn walk_mut<V: VisitorMut>(&mut self, visitor: &mut V) -> V::Result {
                match self {
                    $($enum_name::$name(v) => visitor.$walker(v)),+
                }
            }
        }
    };
}
