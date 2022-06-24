/// Unifying equality.
pub trait UnifyEq<C>
where
    Self: Eq,
{
    fn eq(&self, other: &Self, _ctx: C) -> bool;
}
