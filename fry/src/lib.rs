#![no_std]

use core::fmt::Debug;
use tracing::{instrument, debug, error};
use core::mem::size_of;
use include_data::include_data;

/// The BYTE_SIZE of the files as reported by `ls -l`
/// All consts defined here will be exactly this value divided by two `u16`s long.
const BYTE_SIZE: usize = 20810;

type PcmSample = i16;

/// The sample size in bytes of the .wav files.
const SAMPLE_SIZE: usize = size_of::<PcmSample>();

/// The maximum length of the output buffer in chunks of BYTE_SIZE
const MAX_LETTERS: usize = 32;

/// Single letter constant length sample (in samples, not bytes)
const LETTER_SAMPLES: usize = BYTE_SIZE/SAMPLE_SIZE;

/// The maximum length of the output buffer in bytes.
const MAX_BUFFER_SIZE: usize = BYTE_SIZE * MAX_LETTERS;

/// Include the bytes of a raw file, and place them into a constant sized buffer of u16s, size: BYTE_SIZE/2
/// Since we want half the number of bytes, but all of them to be stored in u16s.
/// This could fail dramatically if endianess is swapped for some reason.
/// By default, `espeak` will use little-endian on x86_64.
macro_rules! import_raw {
  ($var_name:ident, $file_name:literal) => {
    const $var_name: [PcmSample; BYTE_SIZE/2] = include_data!($file_name);
  }
}

import_raw!(A, "../../data/a.raw");
import_raw!(B, "../../data/b.raw");
import_raw!(C, "../../data/c.raw");
import_raw!(D, "../../data/d.raw");
import_raw!(E, "../../data/e.raw");
import_raw!(F, "../../data/f.raw");
import_raw!(G, "../../data/g.raw");
import_raw!(H, "../../data/h.raw");
import_raw!(I, "../../data/i.raw");
import_raw!(J, "../../data/j.raw");
import_raw!(K, "../../data/k.raw");
import_raw!(L, "../../data/l.raw");
import_raw!(M, "../../data/m.raw");
import_raw!(N, "../../data/n.raw");
import_raw!(O, "../../data/o.raw");
import_raw!(P, "../../data/p.raw");
import_raw!(Q, "../../data/q.raw");
import_raw!(R, "../../data/r.raw");
import_raw!(S, "../../data/s.raw");
import_raw!(T, "../../data/t.raw");
import_raw!(U, "../../data/u.raw");
import_raw!(V, "../../data/v.raw");
import_raw!(W, "../../data/w.raw");
import_raw!(X, "../../data/x.raw");
import_raw!(Y, "../../data/y.raw");
import_raw!(Z, "../../data/z.raw");
import_raw!(SPACE, "../../data/space.raw");

#[instrument]
fn letter_to_pcm(c: char) -> Option<[PcmSample; BYTE_SIZE/2]> {
  match c {
    'a' => Some(A),
    'b' => Some(B),
    'c' => Some(C),
    'd' => Some(D),
    'e' => Some(E),
    'f' => Some(F),
    'g' => Some(G),
    'h' => Some(H),
    'i' => Some(I),
    'j' => Some(J),
    'k' => Some(K),
    'l' => Some(L),
    'm' => Some(M),
    'n' => Some(N),
    'o' => Some(O),
    'p' => Some(P),
    'q' => Some(Q),
    'r' => Some(R),
    's' => Some(S),
    't' => Some(T),
    'u' => Some(U),
    'v' => Some(V),
    'w' => Some(W),
    'x' => Some(X),
    'y' => Some(Y),
    'z' => Some(Z),
    ' ' => Some(SPACE),
    _ => {
			error!("Character '{}' does not correspond to a pre-recorded sound", c);
			None
		}
  }
}

/// Fill a buffer with TTS data.
/// This is done by character.
/// It can be done for `s` where s is less less than or equal to `MAX_LETTERS`.
/// It returns Option<usize>:
///
/// * None if `s` is too large.
/// * Some(usize) if successful, contained value is number of *letters*, not bytes that have been copied to the buffer.
/// 
/// If you want the number of bytes, multiply the v in Some(v) by `BYTE_SIZE`.
#[instrument(ret, skip(buf))]
pub fn tts<S: AsRef<str> + Debug>(s: S, buf: &mut [PcmSample; MAX_BUFFER_SIZE]) -> Option<usize> {
  if s.as_ref().len() > MAX_LETTERS {
		error!("The length of the string {} ({} letters) is greater than the maximum amount of letters permitted: {}", s.as_ref(), s.as_ref().len(), MAX_LETTERS);
    return None;
  }
  Some(
    s
    .as_ref()
    .chars()
    .fold(0, |offset: usize, ch| {
      letter_to_pcm(ch)
        .unwrap()
        .iter()
        .enumerate()
        .for_each(|(i, pcm)| buf[(offset*BYTE_SIZE)+i] = *pcm);
      offset+1
    })
  )
}

#[cfg(test)]
mod tests {
	extern crate alloc;

	use test_log::test;
  use super::MAX_BUFFER_SIZE;
  use super::tts;
  use super::BYTE_SIZE;
  use super::A;
  use super::PcmSample;
  use super::LETTER_SAMPLES;
	use alloc::string::String;

  #[test]
  fn check_one_letter_str() {
    let mut buf: [PcmSample; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
    let conv = String::from("a");
    let bytes = tts(conv, &mut buf);
    assert_eq!(bytes.unwrap(), 1);
    let created_slice = &buf[0..LETTER_SAMPLES];
    assert_eq!(created_slice.len(), A.len());
    assert_eq!(created_slice, A);
  }
  #[test]
  fn check_one_word_str() {
    let mut buf: [PcmSample; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
    let conv = String::from("hello");
    let bytes = tts(conv, &mut buf);
    assert_eq!(bytes.unwrap(), 5);
  }
  #[test]
  fn check_multi_word() {
    let mut buf: [PcmSample; MAX_BUFFER_SIZE] = [0; MAX_BUFFER_SIZE];
    let conv = String::from("hello world");
    let bytes = tts(conv, &mut buf);
    assert_eq!(bytes.unwrap(), 11);
  }
}
