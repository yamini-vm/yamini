use std::ops::{Add, Sub, Mul, Div, Rem};
use std::fmt::{Display, self};

#[derive(Debug)]
pub enum InnerData {
    INT(i8),
    STR(String),
}

impl PartialEq for InnerData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => a == b,
            (InnerData::STR(a), InnerData::STR(b)) => a == b,
            _ => false,
        }
    }
}

impl InnerData {
    pub fn get_u8(&self) -> u8 {
        match self {
            InnerData::INT(a) => *a as u8,
            InnerData::STR(_) => panic!("InnerData::get_u8: not implemented for STR"),
        }
    }

    pub fn get_i8(&self) -> i8 {
        match self {
            InnerData::INT(a) => *a,
            InnerData::STR(_) => panic!("InnerData::get_i8: not implemented for STR"),
        }
    }

    pub fn clone(&self) -> InnerData {
        match self {
            InnerData::INT(a) => InnerData::INT(*a),
            InnerData::STR(a) => InnerData::STR(a.clone()),
        }
    }
}

impl Add for InnerData {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => InnerData::INT(a + b),
            (InnerData::STR(a), InnerData::STR(b)) => InnerData::STR(a + &b),
            _ => panic!("Illegal add operation"),
        }
    }
}

impl Sub for InnerData {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => InnerData::INT(a - b),
            _ => panic!("Illegal sub operation"),
        }
    }
}

impl Mul for InnerData {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => InnerData::INT(a * b),
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
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => InnerData::INT(a / b),
            _ => panic!("Illegal div operation"),
        }
    }
}

impl Rem for InnerData {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (InnerData::INT(a), InnerData::INT(b)) => InnerData::INT(a % b),
            _ => panic!("Illegal rem operation"),
        }
    }
}

impl Display for InnerData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InnerData::INT(a) => write!(f, "{}", a),
            InnerData::STR(a) => write!(f, "{}", a),
        }
    }
}