pub trait Jam
where
    Self: Sized,
{
    fn jam(self) -> Result<Vec<u8>, Self>;
}

pub trait Cue
where
    Self: Sized,
{
    fn cue(jammed_val: Vec<u8>) -> Result<Self, Vec<u8>>;
}
