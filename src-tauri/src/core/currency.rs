use std::ops::{Add, AddAssign, Mul, Sub};

use serde::{Deserialize, Serialize};

const DOLLAR_TO_MICRODOLLAR: i128 = 1000000;
const CENTS_TO_MICRODOLLAR: i128 = 10000;

#[derive(Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
pub struct Currency(i128);

pub trait FromDollar {
    fn to_microdollar(self) -> i128;
}

impl FromDollar for i128 {
    fn to_microdollar(self) -> i128 {
        self * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for i64 {
    fn to_microdollar(self) -> i128 {
        self as i128 * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for i32 {
    fn to_microdollar(self) -> i128 {
        self as i128 * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for u128 {
    fn to_microdollar(self) -> i128 {
        self as i128 * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for u64 {
    fn to_microdollar(self) -> i128 {
        self as i128 * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for u32 {
    fn to_microdollar(self) -> i128 {
        self as i128 * DOLLAR_TO_MICRODOLLAR
    }
}

impl FromDollar for f64 {
    fn to_microdollar(self) -> i128 {
        (self * DOLLAR_TO_MICRODOLLAR as f64).round() as i128
    }
}

impl FromDollar for f32 {
    fn to_microdollar(self) -> i128 {
        (self as f64 * DOLLAR_TO_MICRODOLLAR as f64).round() as i128
    }
}

pub trait FromCent {
    fn to_microdollar(self) -> i128;
}

impl FromCent for i128 {
    fn to_microdollar(self) -> i128 {
        self * CENTS_TO_MICRODOLLAR
    }
}

impl FromCent for i64 {
    fn to_microdollar(self) -> i128 {
        self as i128 * CENTS_TO_MICRODOLLAR
    }
}

impl FromCent for i32 {
    fn to_microdollar(self) -> i128 {
        self as i128 * CENTS_TO_MICRODOLLAR
    }
}

impl FromCent for f64 {
    fn to_microdollar(self) -> i128 {
        (self * CENTS_TO_MICRODOLLAR as f64).round() as i128
    }
}

impl FromCent for f32 {
    fn to_microdollar(self) -> i128 {
        (self as f64 * CENTS_TO_MICRODOLLAR as f64).round() as i128
    }
}

pub trait FromMicrodollar {
    fn to_microdollar(self) -> i128;
}
impl FromMicrodollar for i128 {
    fn to_microdollar(self) -> i128 {
        self
    }
}

impl FromMicrodollar for i64 {
    fn to_microdollar(self) -> i128 {
        self as i128
    }
}

impl FromMicrodollar for i32 {
    fn to_microdollar(self) -> i128 {
        self as i128
    }
}

impl FromMicrodollar for u128 {
    fn to_microdollar(self) -> i128 {
        self as i128
    }
}

impl FromMicrodollar for u64 {
    fn to_microdollar(self) -> i128 {
        self as i128
    }
}

impl FromMicrodollar for u32 {
    fn to_microdollar(self) -> i128 {
        self as i128
    }
}

impl FromMicrodollar for f64 {
    fn to_microdollar(self) -> i128 {
        self.round() as i128
    }
}

impl FromMicrodollar for f32 {
    fn to_microdollar(self) -> i128 {
        self.round() as i128
    }
}


impl Currency {
    pub fn from_dollars<T: FromDollar>(amount: T) -> Currency {
        Currency(amount.to_microdollar())
    }

    pub fn from_cents<T: FromCent>(amount: T) -> Currency {
        Currency(amount.to_microdollar())
    }

    pub fn from_microdollars<T: FromMicrodollar>(amount: T) -> Currency {
        Currency(amount.to_microdollar())
    }

    pub fn as_microdollars(self) -> i128 {
        self.0
    }

    pub fn as_dollars(self) -> f32 {
        let dollars = self.0 as f32 / DOLLAR_TO_MICRODOLLAR as f32;

        (dollars * 100.0).round() / 100.0
    }

    pub fn as_string_dollars(self) -> String {
        let dollars = self.0 as f32 / DOLLAR_TO_MICRODOLLAR as f32;

        let money = (dollars * 100.0).round() / 100.0;

        format!("{:.2}", money)
    }

}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dollars = self.0 as f32 / DOLLAR_TO_MICRODOLLAR as f32;

        let money = (dollars * 100.0).round() / 100.0;

        write!(f, "${:.2}", money)
    }
}

impl Add for Currency {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Currency(self.0 + rhs.0)
    }
}

impl Sub for Currency {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Currency(self.0 - rhs.0)
    }
}

impl Mul<f32> for Currency {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Currency((self.0 as f64 * rhs as f64).round() as i128)
    }
}

impl Mul<i32> for Currency {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Currency((self.0 * rhs as i128) as i128)
    }
}

impl AddAssign for Currency {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

