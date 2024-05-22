use crate::util::Create;

pub trait Control<T>
where
    T: Create + Clone + Copy,
{
}
