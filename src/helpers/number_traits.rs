use std::ops::*;
use num::{Bounded, FromPrimitive, One, ToPrimitive};

pub trait U8Cast {
    fn from(value: u8) -> Self;
    
    fn into(self) -> u8;
}

impl<T: FromPrimitive + ToPrimitive> U8Cast for T {
    fn from(value: u8) -> Self {
        <Self as FromPrimitive>::from_u8(value).unwrap()
    }
    
    fn into(self) -> u8 {
        self.to_u8().unwrap()
    }
}

pub trait Number:
Add<Output = Self> + AddAssign +
Sub<Output = Self> + SubAssign +
Mul<Output = Self> + MulAssign +
Div<Output = Self> + DivAssign +
U8Cast + PartialOrd + Copy + Bounded + One {}

impl<T:
Add<Output = Self> + AddAssign +
Sub<Output = Self> + SubAssign +
Mul<Output = Self> + MulAssign +
Div<Output = Self> + DivAssign +
U8Cast + PartialOrd + Copy + Bounded + One
> Number for T {}