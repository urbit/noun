pub trait Jam
where
    Self: Sized,
{
    type Error;

    fn jam(self) -> Result<Vec<u8>, Self::Error>;
}

pub trait Cue
where
    Self: Sized,
{
    type Error;

    fn cue(jammed_val: Vec<u8>) -> Result<Self, Self::Error>;
}
