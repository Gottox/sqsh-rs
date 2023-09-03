mod error;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn to_result<T>(ok: T, rv: i32) -> Result<T> {
    if rv < 0 {
        Err(Error::from_c_err(rv))
    } else {
        Ok(ok)
    }
}
