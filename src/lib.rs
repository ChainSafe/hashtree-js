#![deny(clippy::all)]

use hashtree_rs::{hash as HASH, init};
use napi::{bindgen_prelude::Uint8Array, Error};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

#[macro_use]
extern crate napi_derive;

/// Feature detection logic pulled from:
/// - https://github.com/OffchainLabs/hashtree/blob/main/bindings_amd64.go
/// - https://github.com/OffchainLabs/hashtree/blob/main/bindings_arm64.go
fn has_cpu_features() -> bool {
  #[cfg(target_arch = "x86_64")]
  return (is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512vl")) ||
    (is_x86_feature_detected!("avx2") && is_x86_feature_detected!("bmi2")) ||
    (is_x86_feature_detected!("sha") && is_x86_feature_detected!("avx"));

  #[cfg(target_arch = "aarch64")]
  return is_aarch64_feature_detected!("sha2");

  return false;
}


type HashFn = fn(&mut [u8], &[u8], usize);
static HASH_IMPL: OnceLock<HashFn> = OnceLock::new();

/// Portable SHA‑256: 64‑byte blocks → 32‑byte digests.
///
/// * `output.len()` asserted to be `count * 32`
/// * `input.len()`   asserted to be `count * 64`
pub fn hash_into_portable(output: &mut [u8], input: &[u8], count: usize) {
  assert_eq!(output.len(), count * 32, "output slice is the wrong size");
  assert_eq!(input.len(),  count * 64, "input  slice is the wrong size");

  // Fast path for nothing to do
  if count == 0 {
    return;
  }

  for i in 0..count {
    // Select the i‑th 64‑byte message
    let msg = &input[i * 64 .. (i + 1) * 64];

    // Hash it
    let digest: [u8; 32] = Sha256::digest(msg).into();

    // Copy into the i‑th 32‑byte slot of the output buffer
    output[i * 32 .. (i + 1) * 32].copy_from_slice(&digest);
  }
}

fn get_hash_impl() -> HashFn {
  if has_cpu_features() {
    init();
    HASH
  } else {
    hash_into_portable
  }
}

#[napi]
pub fn hash(input: Uint8Array) -> Result<Uint8Array, Error> {
  let h= HASH_IMPL.get_or_init(get_hash_impl);

  let input_len = input.len();
  if (input_len % 64) != 0 {
    return Err(Error::from_reason("Input must be a multiple of 64 bytes"));
  }
  let output_len = input_len / 2;
  let mut output = vec![0u8; output_len];
  h(output.as_mut_slice(), input.as_ref(), output_len / 32);
  Ok(Uint8Array::from(output))
}

#[napi]
pub fn hash_into(input: Uint8Array, mut output: Uint8Array) -> Result<(), Error> {
  let h = HASH_IMPL.get_or_init(get_hash_impl);

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

  h(output.as_mut(), input.as_ref(), output_len / 32);
  Ok(())
}
