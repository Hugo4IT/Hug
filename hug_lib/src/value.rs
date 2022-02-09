use std::{str::FromStr, ops::{Add, AddAssign, Sub, SubAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign}};

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

impl ToString for HugValue {
    fn to_string(&self) -> String {
        match self {
            HugValue::Int8(v) => v.to_string(),
            HugValue::Int16(v) => v.to_string(),
            HugValue::Int32(v) => v.to_string(),
            HugValue::Int64(v) => v.to_string(),
            HugValue::Int128(v) => v.to_string(),
            HugValue::UInt8(v) => v.to_string(),
            HugValue::UInt16(v) => v.to_string(),
            HugValue::UInt32(v) => v.to_string(),
            HugValue::UInt64(v) => v.to_string(),
            HugValue::UInt128(v) => v.to_string(),
            HugValue::Float32(v) => v.to_string(),
            HugValue::Float64(v) => v.to_string(),
            HugValue::String(v) => v.clone(),
            HugValue::Function(v) => format!("<Function [0x{:08x}]>", *v),
            HugValue::ExternalFunction(v) => format!("<ExternalFunction [{:?}]>", v),
        }
    }
}

macro_rules! impl_op {
    ($typ:ident, $ownvalue:ident, $rhs:ident, $operator:tt) => {
        if let HugValue::$typ(v) = $rhs {
            HugValue::from($ownvalue $operator v)
        } else {
            panic!("Can't add a value of type {} to another type!", stringify!($typ))
        }
    };
}

impl Add for HugValue {
    type Output = HugValue;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            HugValue::Int8(v) => impl_op!(Int8, v, rhs, +),
            HugValue::Int16(v) => impl_op!(Int16, v, rhs, +),
            HugValue::Int32(v) => impl_op!(Int32, v, rhs, +),
            HugValue::Int64(v) =>impl_op!(Int64, v, rhs, +),
            HugValue::Int128(v) => impl_op!(Int128, v, rhs, +),
            HugValue::UInt8(v) => impl_op!(UInt8, v, rhs, +),
            HugValue::UInt16(v) => impl_op!(UInt16, v, rhs, +),
            HugValue::UInt32(v) => impl_op!(UInt32, v, rhs, +),
            HugValue::UInt64(v) => impl_op!(UInt64, v, rhs, +),
            HugValue::UInt128(v) => impl_op!(UInt128, v, rhs, +),
            HugValue::Float32(v) => impl_op!(Float32, v, rhs, +),
            HugValue::Float64(v) => impl_op!(Float64, v, rhs, +),
            HugValue::String(v) => todo!(),
            _ => panic!("Cannot add values of these types!")
        }
    }
    
}