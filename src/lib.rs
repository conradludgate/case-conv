#![doc = include_str!("../README.md")]
#![feature(unicode_internals)]
use core::unicode::conversions;

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
pub fn to_lowercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
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
                break
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

    const UNROLL: usize = 32;
    while out.len() + UNROLL <= b.len() {
        let n = out.len() + UNROLL;
        // Safety:
        // we have checked the length of b and out ahead of time
        unsafe {
            if !b.get_unchecked(out.len()..n).iter().all(u8::is_ascii) {
                break
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
    fn uppercase() {
        assert_eq!(to_uppercase(""), "");
        assert_eq!(to_uppercase("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
    }
}
