use include_data::include_data;

/// The BYTE_SIZE of the files as reported by `ls -l`
/// All consts defined here will be exactly this value divided by two `u16`s long.
const BYTE_SIZE: usize = 20810;

/// The maximum length of the output buffer in chunks of BYTE_SIZE
const MAX_LETTERS: usize = 32;

/// The maximum length of the output buffer in bytes.
const MAX_BUFFER_SIZE: usize = BYTE_SIZE * MAX_LETTERS;

/// Include the bytes of a raw file, and place them into a constant sized buffer of u16s, size: BYTE_SIZE/2
/// Since we want half the number of bytes, but all of them to be stored in u16s.
/// This could fail dramatically if endianess is swapped for some reason.
/// By default, `espeak` will use little-endian on x86_64.
macro_rules! import_raw {
  ($var_name:ident, $file_name:literal) => {
    const $var_name: [u16; BYTE_SIZE/2] = include_data!($file_name);
  }
}

import_raw!(A, "../data/a.raw");
import_raw!(B, "../data/b.raw");
import_raw!(C, "../data/c.raw");
import_raw!(D, "../data/d.raw");
import_raw!(E, "../data/e.raw");
import_raw!(F, "../data/f.raw");
import_raw!(G, "../data/g.raw");
import_raw!(H, "../data/h.raw");
import_raw!(I, "../data/i.raw");
import_raw!(J, "../data/j.raw");
import_raw!(K, "../data/k.raw");
import_raw!(L, "../data/l.raw");
import_raw!(M, "../data/m.raw");
import_raw!(N, "../data/n.raw");
import_raw!(O, "../data/o.raw");
import_raw!(P, "../data/p.raw");
import_raw!(Q, "../data/q.raw");
import_raw!(R, "../data/r.raw");
import_raw!(S, "../data/s.raw");
import_raw!(T, "../data/t.raw");
import_raw!(U, "../data/u.raw");
import_raw!(V, "../data/v.raw");
import_raw!(W, "../data/w.raw");
import_raw!(X, "../data/x.raw");
import_raw!(Y, "../data/y.raw");
import_raw!(Z, "../data/z.raw");

const fn letter_to_pcm(c: char) -> [u16; BYTE_SIZE/2] {
  match c {
    'a' => A,
    'b' => B,
    'c' => C,
    'D' => D,
    'e' => E,
    'f' => F,
    'g' => G,
    'h' => H,
    'i' => I,
    'j' => J,
    'k' => K,
    'l' => L,
    'm' => M,
    'n' => N,
    'o' => O,
    'p' => P,
    'q' => Q,
    'r' => R,
    's' => S,
    't' => T,
    'u' => U,
    'v' => V,
    'w' => W,
    'x' => X,
    'y' => Y,
    'z' => Z,
    _ => todo!(),
  }
}

pub fn tts<S: AsRef<str> + IntoIterator<Item = char>>(s: S, buf: &mut [u16; MAX_BUFFER_SIZE]) -> Option<usize> {
  if s.as_ref().len() > 32 {
    return None;
  }
  Some(
    s
    .as_ref()
    .chars()
    .fold(0, |offset: usize, ch| {
      letter_to_pcm(ch)
        .iter()
        .enumerate()
        .for_each(|(i, pcm)| buf[(offset*BYTE_SIZE)+i] = *pcm);
      offset+1
    })
  )
}
