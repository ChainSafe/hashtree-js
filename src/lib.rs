#![deny(clippy::all)]

use hashtree_rs::{hash as HASH, init};
use napi::{bindgen_prelude::{AsyncTask, Uint8Array}, Env, Error, Task};
use std::sync::Once;

#[macro_use]
extern crate napi_derive;

static INIT: Once = Once::new();

#[napi]
pub fn hash(input: Uint8Array) -> Result<Uint8Array, Error> {
  INIT.call_once(|| {
    init();
  });

  let input_len = input.len();
  if (input_len % 64) != 0 {
    return Err(Error::from_reason("Input must be a multiple of 64 bytes"));
  }
  let output_len = input_len / 2;
  let mut output = vec![0u8; output_len];
  HASH(output.as_mut_slice(), input.as_ref(), output_len / 32);
  Ok(Uint8Array::from(output))
}

#[napi]
pub fn hash_into(input: Uint8Array, mut output: Uint8Array) -> Result<(), Error> {
  INIT.call_once(|| {
    init();
  });

  let input_len = input.len();
  if (input_len % 64) != 0 {
    return Err(Error::from_reason(
      "Input length must be a multiple of 64 bytes",
    ));
  }
  let output_len = output.len();
  if output_len != input_len / 2 {
    return Err(Error::from_reason("Output length must be half input"));
  }

  HASH(output.as_mut(), input.as_ref(), output_len / 32);
  Ok(())
}

pub struct AsyncHash {
  input: Uint8Array
}

#[napi]
impl Task for AsyncHash {
  type Output = Uint8Array;
  type JsValue = Uint8Array;
  fn compute(&mut self) -> Result<Self::Output, Error> {
    hash(self.input.clone())
  }

  fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue, Error> {
    Ok(output)
  }
}

#[napi]
pub fn hash_async(input: Uint8Array) -> AsyncTask<AsyncHash> {
  INIT.call_once(|| {
    init();
  });
  AsyncTask::new(AsyncHash { input })
}
