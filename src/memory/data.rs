extern crate serde;

use std::ops::{Add, Sub, Mul, Div, Rem};
use std::fmt::{Display, self};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum InnerData {
    INT(i8),
    INT16(i16),
    INT32(i32),
    STR(String),
}

impl PartialEq for InnerData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => a == b,
            (InnerData::INT16(a), InnerData::INT16(b)) => a == b,
            (InnerData::INT32(a), InnerData::INT32(b)) => a == b,
            (InnerData::STR(a), InnerData::STR(b)) => a == b,
            _ => false,
        }
    }
}

impl InnerData {
    pub fn from(data: &str, variant: &str) -> InnerData {
        match variant {
            "INT" => InnerData::INT(data.parse::<i8>().unwrap()),
            "INT16" => InnerData::INT16(data.parse::<i16>().unwrap()),
            "INT32" => InnerData::INT32(data.parse::<i32>().unwrap()),
            "STR" => InnerData::STR(data.to_string()),
            _ => panic!("Value out of bounds!"),
        }
    }

    pub fn get_u8(&self) -> u8 {
        match self {
            InnerData::INT(a) => *a as u8,
            _ => panic!("Illegal cast"),
        }
    }

    pub fn get_i8(&self) -> i8 {
        match self {
            InnerData::INT(a) => *a,
            _ => panic!("Illegal cast"),
        }
    }

    pub fn get_u16(&self) -> u16 {
        match self {
            InnerData::INT16(a) => *a as u16,
            _ => panic!("Illegal cast"),
        }
    }

    pub fn get_i16(&self) -> i16 {
        match self {
            InnerData::INT16(a) => *a,
            _ => panic!("Illegal cast"),
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            InnerData::INT(_) => "i8",
            InnerData::INT16(_) => "i16",
            InnerData::INT32(_) => "i32",
            InnerData::STR(_) => "str",
        }
    }

    pub fn clone(&self) -> InnerData {
        match self {
            InnerData::INT(a) => InnerData::INT(*a),
            InnerData::INT16(a) => InnerData::INT16(*a),
            InnerData::INT32(a) => InnerData::INT32(*a),
            InnerData::STR(a) => InnerData::STR(a.clone()),
        }
    }

    fn promote_type(self, other: Self) -> (InnerData, InnerData) {
        let mut promoted_self = self;
        let mut promoted_other = other;

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT16(b)) => {
                promoted_self = InnerData::INT16(a as i16);
                promoted_other = InnerData::INT16(b);
            }
            (InnerData::INT(a), InnerData::INT32(b)) => {
                promoted_self = InnerData::INT32(a as i32);
                promoted_other = InnerData::INT32(b);
            }
            (InnerData::INT16(a), InnerData::INT(b)) => {
                promoted_self = InnerData::INT16(a);
                promoted_other = InnerData::INT16(b as i16);
            }
            (InnerData::INT16(a), InnerData::INT32(b)) => {
                promoted_self = InnerData::INT32(a as i32);
                promoted_other = InnerData::INT32(b);
            }
            (InnerData::INT32(a), InnerData::INT(b)) => {
                promoted_self = InnerData::INT32(a);
                promoted_other = InnerData::INT32(b as i32);
            }
            (InnerData::INT32(a), InnerData::INT16(b)) => {
                promoted_self = InnerData::INT32(a);
                promoted_other = InnerData::INT32(b as i32);
            }
            _ => panic!("Illegal types for promotion"),
        }

        (promoted_self, promoted_other)
    }

    pub fn variant_eq<T>(a: &T, b: &T) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }

    fn promote_or_not(self, other: Self) -> (InnerData, InnerData) {
        if self.get_type() == "str" || other.get_type() == "str" {
            return (self, other);
        }

        let mut promoted_self = self;
        let mut promoted_other = other;

        if !InnerData::variant_eq(&promoted_self, &promoted_other) {
            (promoted_self, promoted_other) =
                InnerData::promote_type(promoted_self, promoted_other);
        }

        (promoted_self, promoted_other)
    }

    fn compute_or_promote_i8(a: i8, b: i8, checked_fn: &dyn Fn(i8, i8) -> Option<i8>,
                             op_fn: &dyn Fn(Self, Self) -> Self) -> InnerData {
        let res = checked_fn(a, b);

        match res {
            Some(res) => InnerData::INT(res),
            None => {
                let promoted_self = InnerData::INT16(a as i16);
                let promoted_other = InnerData::INT16(b as i16);

                op_fn(promoted_self, promoted_other)
            }
        }
    }

    fn compute_or_promote_i16(a: i16, b: i16, checked_fn: &dyn Fn(i16, i16) -> Option<i16>,
                             op_fn: &dyn Fn(Self, Self) -> Self) -> InnerData {
        let res = checked_fn(a, b);

        match res {
            Some(res) => InnerData::INT16(res),
            None => {
                let promoted_self = InnerData::INT32(a as i32);
                let promoted_other = InnerData::INT32(b as i32);

                op_fn(promoted_self, promoted_other)
            }
        }
    }

    fn compute_or_promote_i32(a: i32, b: i32, checked_fn: &dyn Fn(i32, i32) -> Option<i32>,
                             _op_fn: &dyn Fn(Self, Self) -> Self) -> InnerData {
        let res = checked_fn(a, b);

        match res {
            Some(res) => InnerData::INT32(res),
            None => {
                panic!("Value out of bounds!");
            }
        }
    }
}

impl Add for InnerData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let (promoted_self, promoted_other) = InnerData::promote_or_not(self, other);

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT(b)) => {
                InnerData::compute_or_promote_i8(a, b, &i8::checked_add, &Add::add)
            },
            (InnerData::INT16(a), InnerData::INT16(b)) => {
                InnerData::compute_or_promote_i16(a, b, &i16::checked_add, &Add::add)
            },
            (InnerData::INT32(a), InnerData::INT32(b)) => {
                InnerData::compute_or_promote_i32(a, b, &i32::checked_add, &Add::add)
            },
            (InnerData::STR(a), InnerData::STR(b)) => InnerData::STR(a + &b),
            _ => panic!("Illegal add operation"),
        }
    }
}

impl Sub for InnerData {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let (promoted_self, promoted_other) = InnerData::promote_or_not(self, other);

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT(b)) => {
                InnerData::compute_or_promote_i8(a, b, &i8::checked_sub, &Sub::sub)
            },
            (InnerData::INT16(a), InnerData::INT16(b)) => {
                InnerData::compute_or_promote_i16(a, b, &i16::checked_sub, &Sub::sub)
            },
            (InnerData::INT32(a), InnerData::INT32(b)) => {
                InnerData::compute_or_promote_i32(a, b, &i32::checked_sub, &Sub::sub)
            },
            _ => panic!("Illegal add operation"),
        }
    }
}

impl Mul for InnerData {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let (promoted_self, promoted_other) = InnerData::promote_or_not(self, other);

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT(b)) => {
                InnerData::compute_or_promote_i8(a, b, &i8::checked_mul, &Mul::mul)
            },
            (InnerData::INT16(a), InnerData::INT16(b)) => {
                InnerData::compute_or_promote_i16(a, b, &i16::checked_mul, &Mul::mul)
            },
            (InnerData::INT32(a), InnerData::INT32(b)) => {
                InnerData::compute_or_promote_i32(a, b, &i32::checked_mul, &Mul::mul)
            },
            (InnerData::STR(a), InnerData::INT(b)) => {
                let mut result = String::new();
                for _ in 0..b {
                    result.push_str(&a);
                }
                InnerData::STR(result)
            },
            (InnerData::INT(a), InnerData::STR(b)) => {
                let mut result = String::new();
                for _ in 0..a {
                    result.push_str(&b);
                }
                InnerData::STR(result)
            },
            _ => panic!("Illegal mul operation"),
        }
    }
}

impl Div for InnerData {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let (promoted_self, promoted_other) = InnerData::promote_or_not(self, other);

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT(b)) => {
                InnerData::compute_or_promote_i8(a, b, &i8::checked_div, &Div::div)
            },
            (InnerData::INT16(a), InnerData::INT16(b)) => {
                InnerData::compute_or_promote_i16(a, b, &i16::checked_div, &Div::div)
            },
            (InnerData::INT32(a), InnerData::INT32(b)) => {
                InnerData::compute_or_promote_i32(a, b, &i32::checked_div, &Div::div)
            },
            _ => panic!("Illegal div operation"),
        }
    }
}

impl Rem for InnerData {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let (promoted_self, promoted_other) = InnerData::promote_or_not(self, other);

        match (promoted_self, promoted_other) {
            (InnerData::INT(a), InnerData::INT(b)) => {
                InnerData::compute_or_promote_i8(a, b, &i8::checked_rem, &Rem::rem)
            },
            (InnerData::INT16(a), InnerData::INT16(b)) => {
                InnerData::compute_or_promote_i16(a, b, &i16::checked_rem, &Rem::rem)
            },
            (InnerData::INT32(a), InnerData::INT32(b)) => {
                InnerData::compute_or_promote_i32(a, b, &i32::checked_rem, &Rem::rem)
            },
            _ => panic!("Illegal rem operation"),
        }
    }
}

impl Display for InnerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerData::INT(a) => write!(f, "{}", a),
            InnerData::INT16(a) => write!(f, "{}", a),
            InnerData::INT32(a) => write!(f, "{}", a),
            InnerData::STR(a) => write!(f, "{}", a),
        }
    }
}