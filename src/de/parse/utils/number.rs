use num_traits::{Bounded, FromPrimitive, Num, NumAssignOps, ToPrimitive};

pub trait Number:
    Clone + PartialOrd + Num + NumAssignOps + FromPrimitive + ToPrimitive + Bounded
where
    Self: Sized,
{
}

impl<T> Number for T where
    T: Clone + PartialOrd + Num + NumAssignOps + FromPrimitive + ToPrimitive + Bounded
{
}
