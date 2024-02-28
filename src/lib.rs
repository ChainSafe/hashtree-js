#![deny(clippy::all)]

use std::{ffi::c_ulonglong, os::raw::c_uchar};
use napi::{bindgen_prelude::Uint8Array, Error};

#[macro_use]
extern crate napi_derive;

#[link(name = "hashtree", kind = "static")]
extern "C" {
  pub fn hashtree_hash(output: *mut c_uchar, input: *const c_uchar, length: c_ulonglong);
}

#[napi]
pub fn hash(input: Uint8Array) -> Result<Uint8Array, Error> {
  let input_len = input.len();
  if (input_len % 64) != 0 {
    return Err(Error::from_reason("Input must be a multiple of 64 bytes"));
  }
  let output_len = input_len / 2;
  let mut output = vec![0u8; output_len];
  unsafe {
    hashtree_hash(output.as_mut_ptr(), input.as_ptr(), (output_len / 32) as u64);
  }
  Ok(Uint8Array::from(output))
}

#[napi]
pub fn hash_into(input: Uint8Array, mut output: Uint8Array) -> Result<(), Error> {
  let input_len = input.len();
  if (input_len % 64) != 0 {
    return Err(Error::from_reason("Input length must be a multiple of 64 bytes"));
  }
  let output_len = output.len();
  if output_len != input_len / 2 {
    return Err(Error::from_reason("Output length must be half input"));
  }
  unsafe {
    hashtree_hash(output.as_mut_ptr(), input.as_ptr(), (output_len / 32) as u64);
  }
  Ok(())
}