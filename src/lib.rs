#![doc = include_str!("../README.md")]
#![feature(unicode_internals)]
use core::unicode::conversions;

mod conversions2;

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
pub fn to_lowercase(s: &str) -> String {
    // Over-allocate by 3 bytes so that we can do a 4-byte write for every (non-ASCII)
    // character.
    let mut out = Vec::<u8>::with_capacity(s.len() + 3);
    let b = s.as_bytes();

    // Fast path for ASCII.
    // This first attempts to process the start of the string as ascii only.
    // For performance, it processes the string in chunks of 32 bytes
    const UNROLL: usize = 32;
    while out.len() + UNROLL <= b.len() {
        let n = out.len() + UNROLL;
        // Safety:
        // we have checked the length of b and out ahead of time
        unsafe {
            if !b.get_unchecked(out.len()..n).iter().all(u8::is_ascii) {
                break;
            }
            for j in out.len()..n {
                let out = out.as_mut_ptr().add(j);
                // Safety: we know that our bytes are ascii from the check above
                core::ptr::write(out, b.get_unchecked(j).to_ascii_lowercase());
            }
            out.set_len(n);
        }
    }

    // Safety: we know this is a valid char boundary since
    // 1. Our iterator guarantees that this is a valid byte
    // 2. From our loop we know this is the start of a utf8 scalar point
    let rest = unsafe { s.get_unchecked(out.len()..) };

    unsafe {
        let mut src = rest.as_ptr();
        let src_end = rest.as_ptr().add(rest.len());
        // let mut spare_capacity: *mut u8 = out.spare_capacity_mut().as_mut_ptr().cast();
        // This code layout will lowercase runs of ascii characters in a tight loop, but unlike
        // str::to_ascii_lowercase the structure of this codes is not autovectorized, so it
        // is ~2x slower.
        while src < src_end {
            if (*src).is_ascii() {
                // for ascii, we know our string capacity is already over-allocated
                debug_assert!(out.capacity() > out.len());

                let ptr = out.as_mut_ptr().add(out.len());
                *ptr = (*src).to_ascii_lowercase();
                src = src.add(1);
                out.set_len(out.len() + 1);
                continue;
            }

            let idx = src.offset_from(rest.as_ptr()) as usize;
            let c = rest[idx..].chars().next().unwrap_unchecked();
            if c == 'Σ' {
                // Σ maps to σ, except at the end of a word where it maps to ς.
                // This is the only conditional (contextual) but language-independent mapping
                // in `SpecialCasing.txt`,
                // so hard-code it rather than have a generic "condition" mechanism.
                // See https://github.com/rust-lang/rust/issues/26035
                map_uppercase_sigma(rest, idx, &mut out);
            } else {
                out.reserve(4);
                let ptr = out.as_mut_ptr().add(out.len());

                let (encoded_len, encoded) = conversions2::to_lower_encoded(c);
                // Copying a fixed number of bytes shrinks code size and prevents LLVM from
                // emitting a call to memcpy
                core::ptr::copy_nonoverlapping(encoded.as_ptr(), ptr, 4);
                // Only increment the pointer by the number of meaningful bytes we copied
                out.set_len(out.len() + encoded_len as usize);
            }
            src = src.add(c.len_utf8());
        }
    }

    #[cold]
    fn map_uppercase_sigma(from: &str, i: usize, out: &mut Vec<u8>) {
        // See http://www.unicode.org/versions/Unicode7.0.0/ch03.pdf#G33992
        // for the definition of `Final_Sigma`.
        debug_assert!('Σ'.len_utf8() == 2);
        let is_word_final = case_ignoreable_then_cased(from[..i].chars().rev())
            && !case_ignoreable_then_cased(from[i + 2..].chars());
        out.extend_from_slice(if is_word_final { "ς" } else { "σ" }.as_bytes());
    }

    fn case_ignoreable_then_cased<I: Iterator<Item = char>>(mut iter: I) -> bool {
        use core::unicode::{Case_Ignorable, Cased};
        match iter.find(|&c| !Case_Ignorable(c)) {
            Some(c) => Cased(c),
            None => false,
        }
    }

    // Safety: We have been careful to write only UTF-8
    unsafe { String::from_utf8_unchecked(out) }
}

/// Returns the uppercase equivalent of this string slice, as a new [`String`].
pub fn to_uppercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
    let b = s.as_bytes();

    const UNROLL: usize = 32;
    while out.len() + UNROLL <= b.len() {
        let n = out.len() + UNROLL;
        // Safety:
        // we have checked the length of b and out ahead of time
        unsafe {
            if !b.get_unchecked(out.len()..n).iter().all(u8::is_ascii) {
                break;
            }
            for j in out.len()..n {
                let out = out.as_mut_ptr().add(j);
                // Safety: we know that our bytes are ascii from the check above
                core::ptr::write(out, b.get_unchecked(j).to_ascii_uppercase());
            }
            out.set_len(n);
        }
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
    fn lowercase_longer() {
        let upper = "İİİİİİİİİİİİİİİİİİİİİİİ";
        let lower = "i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇i̇";

        assert_eq!(to_lowercase(upper), lower);
    }

    #[test]
    fn uppercase() {
        assert_eq!(to_uppercase(""), "");
        assert_eq!(to_uppercase("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
    }
}
