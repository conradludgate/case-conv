#![doc = include_str!("../README.md")]
#![feature(unicode_internals)]
use core::unicode::conversions;
use std::mem;

#[inline]
fn contains_nonascii(v: usize) -> bool {
    const NONASCII_MASK: usize = 0x8080808080808080; // usize::repeat_u8(0x80);
    (NONASCII_MASK & v) != 0
}

#[inline]
/// SAFETY: N*size_of::<usize>() bytes must be valid of b
unsafe fn is_ascii_funsafe<const N: usize>(b: *const u8) -> bool {
    // check that the bytes are not ascii (going by chunks of usize)
    let mut count = 0;
    for j in 0..N {
        let chunk = b.cast::<usize>().add(j).read_unaligned();
        count += contains_nonascii(chunk) as usize;
    }
    count == 0
}

#[inline]
unsafe fn convert_while_ascii(
    b: &[u8],
    out: &mut [mem::MaybeUninit<u8>],
    f: fn(&u8) -> u8,
) -> usize {
    debug_assert!(out.len() >= b.len());

    const USIZE_SIZE: usize = mem::size_of::<usize>();
    const MAGIC_UNROLL: usize = 16;

    let mut i = 0;
    while i + USIZE_SIZE * MAGIC_UNROLL <= b.len() {
        let c = b.get_unchecked(i..);
        let o = out.get_unchecked_mut(i..);

        if !is_ascii_funsafe::<MAGIC_UNROLL>(c.as_ptr()) {
            return i;
        }

        // perform the case conversions on USIZE_SIZE * MAGIC_UNROLL bytes (gets heavily autovec'd)
        for j in 0..USIZE_SIZE * MAGIC_UNROLL {
            let out = o.get_unchecked_mut(j);
            out.write(f(c.get_unchecked(j)));
        }

        i += USIZE_SIZE * MAGIC_UNROLL;
    }
    i
}

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
pub fn to_lowercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
    let b = s.as_bytes();

    unsafe {
        let n = convert_while_ascii(b, out.spare_capacity_mut(), u8::to_ascii_lowercase);
        out.set_len(n);
    }

    // Safety: we know this is a valid char boundary since
    // 1. Our iterator guarantees that this is a valid byte
    // 2. From our loop we know this is the start of a utf8 scalar point
    let rest = unsafe { s.get_unchecked(out.len()..) };

    // Safety: We have written only valid ASCII to our vec
    let mut to = unsafe { String::from_utf8_unchecked(out) };

    for (i, c) in rest.char_indices() {
        if c == 'Σ' {
            // Σ maps to σ, except at the end of a word where it maps to ς.
            // This is the only conditional (contextual) but language-independent mapping
            // in `SpecialCasing.txt`,
            // so hard-code it rather than have a generic "condition" mechanism.
            // See https://github.com/rust-lang/rust/issues/26035
            map_uppercase_sigma(s, i, &mut to)
        } else {
            match conversions::to_lower(c) {
                [a, '\0', _] => to.push(a),
                [a, b, '\0'] => {
                    to.push(a);
                    to.push(b);
                }
                [a, b, c] => {
                    to.push(a);
                    to.push(b);
                    to.push(c);
                }
            }
        }
    }

    fn map_uppercase_sigma(from: &str, i: usize, out: &mut String) {
        // See http://www.unicode.org/versions/Unicode7.0.0/ch03.pdf#G33992
        // for the definition of `Final_Sigma`.
        debug_assert!('Σ'.len_utf8() == 2);
        let is_word_final = case_ignoreable_then_cased(from[..i].chars().rev())
            && !case_ignoreable_then_cased(from[i + 2..].chars());
        out.push_str(if is_word_final { "ς" } else { "σ" });
    }

    fn case_ignoreable_then_cased<I: Iterator<Item = char>>(mut iter: I) -> bool {
        use core::unicode::{Case_Ignorable, Cased};
        match iter.find(|&c| !Case_Ignorable(c)) {
            Some(c) => Cased(c),
            None => false,
        }
    }

    to
}

/// Returns the uppercase equivalent of this string slice, as a new [`String`].
pub fn to_uppercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
    let b = s.as_bytes();

    unsafe {
        let n = convert_while_ascii(b, out.spare_capacity_mut(), u8::to_ascii_uppercase);
        out.set_len(n);
    }

    // Safety: we know this is a valid char boundary since
    // 1. Our iterator guarantees that this is a valid byte
    // 2. From our loop we know this is the start of a utf8 scalar point
    let rest = unsafe { s.get_unchecked(out.len()..) };

    // Safety: We have written only valid ASCII to our vec
    let mut to = unsafe { String::from_utf8_unchecked(out) };

    for c in rest.chars() {
        match conversions::to_upper(c) {
            [a, '\0', _] => to.push(a),
            [a, b, '\0'] => {
                to.push(a);
                to.push(b);
            }
            [a, b, c] => {
                to.push(a);
                to.push(b);
                to.push(c);
            }
        }
    }
    to
}

pub fn is_ascii(b: &[u8]) -> bool {
    const USIZE_SIZE: usize = mem::size_of::<usize>();
    const MAGIC_UNROLL: usize = 16;

    if b.len() < USIZE_SIZE {
        return b.iter().all(u8::is_ascii);
    }
    unsafe {
        let mut i = 0;

        // on 16 usize chunks
        while i + USIZE_SIZE * MAGIC_UNROLL <= b.len() {
            if !is_ascii_funsafe::<MAGIC_UNROLL>(b.as_ptr().add(i)) {
                return false;
            }
            i += USIZE_SIZE * MAGIC_UNROLL;
        }

        // on usize chunks
        while i + USIZE_SIZE < b.len() {
            if !is_ascii_funsafe::<1>(b.as_ptr().add(i)) {
                return false;
            }
            i += USIZE_SIZE;
        }

        // final chunk
        let i = b.len() - USIZE_SIZE;
        is_ascii_funsafe::<1>(b.as_ptr().add(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowercase() {
        assert_eq!(to_lowercase(""), "");
        assert_eq!(to_lowercase("AÉǅaé "), "aéǆaé ");

        // https://github.com/rust-lang/rust/issues/26035
        assert_eq!(to_lowercase("ΑΣ"), "ας");
        assert_eq!(to_lowercase("Α'Σ"), "α'ς");
        assert_eq!(to_lowercase("Α''Σ"), "α''ς");

        assert_eq!(to_lowercase("ΑΣ Α"), "ας α");
        assert_eq!(to_lowercase("Α'Σ Α"), "α'ς α");
        assert_eq!(to_lowercase("Α''Σ Α"), "α''ς α");

        assert_eq!(to_lowercase("ΑΣ' Α"), "ας' α");
        assert_eq!(to_lowercase("ΑΣ'' Α"), "ας'' α");

        assert_eq!(to_lowercase("Α'Σ' Α"), "α'ς' α");
        assert_eq!(to_lowercase("Α''Σ'' Α"), "α''ς'' α");

        assert_eq!(to_lowercase("Α Σ"), "α σ");
        assert_eq!(to_lowercase("Α 'Σ"), "α 'σ");
        assert_eq!(to_lowercase("Α ''Σ"), "α ''σ");

        assert_eq!(to_lowercase("Σ"), "σ");
        assert_eq!(to_lowercase("'Σ"), "'σ");
        assert_eq!(to_lowercase("''Σ"), "''σ");

        assert_eq!(to_lowercase("ΑΣΑ"), "ασα");
        assert_eq!(to_lowercase("ΑΣ'Α"), "ασ'α");
        assert_eq!(to_lowercase("ΑΣ''Α"), "ασ''α");
    }

    #[test]
    fn long() {
        let mut upper = str::repeat("A", 128);
        let mut lower = str::repeat("a", 128);

        assert_eq!(to_lowercase(&upper), lower);
        assert_eq!(to_uppercase(&lower), upper);

        upper.push('Σ');
        lower.push('σ');

        assert_eq!(to_lowercase(&upper), lower);
        assert_eq!(to_uppercase(&lower), upper);
    }

    #[test]
    fn case_conv_long() {
        let upper = str::repeat("A", 512);
        let lower = str::repeat("a", 512);

        assert_eq!(to_lowercase(&upper), lower);
        assert_eq!(to_uppercase(&lower), upper);
    }

    #[test]
    fn case_conv_long_unicode() {
        let upper = str::repeat("É", 512);
        let lower = str::repeat("é", 512);

        assert_eq!(to_lowercase(&upper), lower);
        assert_eq!(to_uppercase(&lower), upper);
    }

    #[test]
    fn uppercase() {
        assert_eq!(to_uppercase(""), "");
        assert_eq!(to_uppercase("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
    }
}
