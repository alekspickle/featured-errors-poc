
use thiserror::Error;

#[derive(Debug, Error, PartialEq, PartialOrd)]
pub enum SomeError {
    #[error("GET")]
    Get,
    #[error("GET")]
    Put,
    #[cfg(feature = "lol")]
    #[error(transparent)]
    Other(#[from] OtherError)
}

#[cfg(feature = "lol")]
#[derive(Debug, Error, PartialEq, PartialOrd)]
pub enum OtherError {
    #[error("LOL")]
    Lol,
    #[error("KEK")]
    Kek,
}

type Result<T> = std::result::Result<T, SomeError>;

mod all {
    use super::*;
    pub fn get() -> Result<()>{
        Err(SomeError::Get)
    }

    pub fn put() -> Result<()>{
        Err(SomeError::Put)
    }
}

#[cfg(feature = "lol")]
mod lol {
    use super::*;

    pub fn lol() -> Result<()>{
        Err(OtherError::Lol.into())
    }
    pub fn kek() -> Result<()>{
        Err(OtherError::Kek.into())
    }
}


fn main() {
    assert_eq!(all::get(), Err(SomeError::Get));
    assert_eq!(all::put(), Err(SomeError::Put));
    #[cfg(feature = "lol")] 
    {
        assert_eq!(lol::lol(), Err(SomeError::Other(OtherError::Lol)));
        assert_eq!(lol::kek(), Err(SomeError::Other(OtherError::Kek)));

    }
}
