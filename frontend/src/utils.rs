pub trait NeqAssign {
    fn neq_assign(&mut self, other: Self) -> bool;
}

impl<T> NeqAssign for T
where
    T: PartialEq,
{
    fn neq_assign(&mut self, other: Self) -> bool {
        if *self == other {
            false
        } else {
            *self = other;
            true
        }
    }
}
