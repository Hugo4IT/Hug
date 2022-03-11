pub mod colors;

#[macro_export]
macro_rules! class {
    ($tyname:ident $name:ident : $value:expr) => {
        pub struct $name;
        impl $tyname::StyleSheet for $name {
            fn style(&self) -> $tyname::Style {
                $value
            }
        }
    };
}
