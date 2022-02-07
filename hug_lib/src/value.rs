pub type HugExternalFunction = fn(std::vec::IntoIter<HugValue>) -> Option<HugValue>;

macro_rules! gen_impls_for_HugValue {
    ($hug_name:ident, $rust_type:ty) => {
        impl FromHugValue for $rust_type {
            fn from_hug_value(value: HugValue) -> Option<$rust_type> {
                if let HugValue::$hug_name(v) = value {
                    Some(v)
                } else {
                    None
                }
            }
        }

        impl From<$rust_type> for HugValue {
            fn from(input: $rust_type) -> HugValue {
                HugValue::$hug_name(input)
            }
        }
    };
}

pub trait FromHugValue: Sized {
    fn from_hug_value(value: HugValue) -> Option<Self>;
}
gen_impls_for_HugValue!(Int8, i8);
gen_impls_for_HugValue!(Int16, i16);
gen_impls_for_HugValue!(Int32, i32);
gen_impls_for_HugValue!(Int64, i64);
gen_impls_for_HugValue!(Int128, i128);
gen_impls_for_HugValue!(UInt8, u8);
gen_impls_for_HugValue!(UInt16, u16);
gen_impls_for_HugValue!(UInt32, u32);
gen_impls_for_HugValue!(UInt64, u64);
gen_impls_for_HugValue!(UInt128, u128);
gen_impls_for_HugValue!(Float32, f32);
gen_impls_for_HugValue!(Float64, f64);
gen_impls_for_HugValue!(String, String);
gen_impls_for_HugValue!(Function, usize);
gen_impls_for_HugValue!(ExternalFunction, HugExternalFunction);

#[derive(Debug, Clone)]
pub enum HugValue {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    UInt128(u128),
    Float32(f32),
    Float64(f64),
    String(String),
    Function(usize), // usize = pointer to instruction
    ExternalFunction(fn(std::vec::IntoIter<HugValue>) -> Option<HugValue>),
}

impl HugValue {
    pub fn assert<T: FromHugValue>(&self) -> Option<T> {
        T::from_hug_value(self.clone())
    }
}
