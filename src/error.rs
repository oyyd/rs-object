#[derive(Debug)]
pub struct Error {
  inner: anyhow::Error,
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.inner.fmt(f)
  }
}

impl std::error::Error for Error {}

impl std::convert::From<Error> for napi::Error {
  fn from(e: Error) -> Self {
    napi::Error {
      // TODO need a custom error status here
      status: napi::Status::Unknown,
      reason: format!("{}", e),
    }
  }
}

impl std::convert::From<object::Error> for Error {
  fn from(e: object::Error) -> Self {
    Error {
      inner: anyhow::Error::new(e),
    }
  }
}

pub fn object_err(e: object::Error) -> napi::Error {
  let e: Error = e.into();
  e.into()
}
