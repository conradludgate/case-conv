//! Fast and correct case conversion.
//!
//! [`to_lowercase`] offers good average case performance if your string starts mostly with ascii.
//! It gets this speed up by assuming each byte is ascii, but then falling back to a slow correct
//! path seamlessly if a non ascii byte is found. This can provide 5x speed improvements in common ascii cases
//!
//! [`to_lowercase2`] offers extremely good performance for ascii only strings, and slightly better performance
//! (but worse than [`to_lowercase`]) in the event that unicode is found.
//! This is achieved by checking whether the string is all ascii upfront and dispatching to a different mode depending.
//! This auto-vectorizes a lot better and can achieve 50x speed ups in extreme cases
//!
//! There are also uppercase equivalents.

#![feature(unicode_internals)]
use core::unicode::conversions;

#[cold]
fn to_lowercase_cold(s: &str, mut out: String) -> String {
    for (i, c) in s[..].char_indices() {
        if c == 'Σ' {
            // Σ maps to σ, except at the end of a word where it maps to ς.
            // This is the only conditional (contextual) but language-independent mapping
            // in `SpecialCasing.txt`,
            // so hard-code it rather than have a generic "condition" mechanism.
            // See https://github.com/rust-lang/rust/issues/26035
            map_uppercase_sigma(s, i, &mut out)
        } else {
            match conversions::to_lower(c) {
                [a, '\0', _] => out.push(a),
                [a, b, '\0'] => {
                    out.push(a);
                    out.push(b);
                }
                [a, b, c] => {
                    out.push(a);
                    out.push(b);
                    out.push(c);
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

    out
}

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
pub fn to_lowercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
    for (i, b) in s.as_bytes().iter().enumerate() {
        if b & 0b1000_0000 == 0 {
            unsafe {
                let end = out.as_mut_ptr().add(i);
                core::ptr::write(end, b.to_ascii_lowercase());
            }
            continue;
        }

        // Safety: so far we have only written i ascii bytes
        // so the length is known
        unsafe { out.set_len(i) };

        // Safety: we know this is a valid char boundary since
        // 1. Our iterator guarantees that this is a valid byte
        // 2. From our loop we know this is the start of a utf8 scalar point
        let from = unsafe { s.get_unchecked(i..) };

        // Safety: We have written only valid UTF-8 to our vec
        let to = unsafe { String::from_utf8_unchecked(out) };

        return to_lowercase_cold(from, to);
    }
    // Safety: We have written only valid ASCII to our vec
    unsafe {
        out.set_len(s.len());
        String::from_utf8_unchecked(out)
    }
}

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
///
/// Functionally the same as [`to_lowercase`] but has much better performance in the
/// ascii case, but much worse performance in the unicode case
pub fn to_lowercase2(s: &str) -> String {
    let mut out = Vec::with_capacity(s.len());
    if s.is_ascii() {
        out.extend(s.bytes().map(|b| b.to_ascii_lowercase()));
        // Safety: We have written only valid UTF-8 (ascii) to our vec
        unsafe { String::from_utf8_unchecked(out) }
    } else {
        let out = unsafe { String::from_utf8_unchecked(out) };
        to_lowercase_cold(s, out)
    }
}

#[cold]
fn to_uppercase_cold(s: &str, mut out: String) -> String {
    for c in s[..].chars() {
        match conversions::to_upper(c) {
            [a, '\0', _] => out.push(a),
            [a, b, '\0'] => {
                out.push(a);
                out.push(b);
            }
            [a, b, c] => {
                out.push(a);
                out.push(b);
                out.push(c);
            }
        }
    }
    out
}

/// Returns the uppercase equivalent of this string slice, as a new [`String`].
pub fn to_uppercase(s: &str) -> String {
    let mut out = Vec::<u8>::with_capacity(s.len());
    for (i, b) in s.as_bytes().iter().enumerate() {
        if b & 0b1000_0000 == 0 {
            // Safety:
            // we are working with ascii only at the moment
            // so we know the exact size of the buffer
            unsafe {
                let end = out.as_mut_ptr().add(i);
                core::ptr::write(end, b.to_ascii_uppercase());
            }
            continue;
        }

        // Safety: so far we have only written i ascii bytes
        // so the length is known
        unsafe { out.set_len(i) };

        // Safety: we know this is a valid char boundary since
        // 1. Our iterator guarantees that this is a valid byte
        // 2. From our loop we know this is the start of a utf8 scalar point
        let from = unsafe { s.get_unchecked(i..) };

        // Safety: We have written only valid UTF-8 to our vec
        let to = unsafe { String::from_utf8_unchecked(out) };

        return to_uppercase_cold(from, to);
    }
    // Safety: We have written only valid ASCII to our vec
    unsafe {
        out.set_len(s.len());
        String::from_utf8_unchecked(out)
    }
}

/// Returns the uppercase equivalent of this string slice, as a new [`String`].
///
/// Functionally the same as [`to_uppercase`] but has much better performance in the
/// ascii case, but much worse performance in the unicode case
pub fn to_uppercase2(s: &str) -> String {
    let mut out = Vec::with_capacity(s.len());
    if s.is_ascii() {
        out.extend(s.bytes().map(|b| b.to_ascii_uppercase()));
        // Safety: We have written only valid UTF-8 (ascii) to our vec
        unsafe { String::from_utf8_unchecked(out) }
    } else {
        let out = unsafe { String::from_utf8_unchecked(out) };
        to_uppercase_cold(s, out)
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
    fn uppercase() {
        assert_eq!(to_uppercase(""), "");
        assert_eq!(to_uppercase("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
    }

    #[test]
    fn lowercase2() {
        assert_eq!(to_lowercase2(""), "");
        assert_eq!(to_lowercase2("AÉǅaé "), "aéǆaé ");

        // https://github.com/rust-lang/rust/issues/26035
        assert_eq!(to_lowercase2("ΑΣ"), "ας");
        assert_eq!(to_lowercase2("Α'Σ"), "α'ς");
        assert_eq!(to_lowercase2("Α''Σ"), "α''ς");

        assert_eq!(to_lowercase2("ΑΣ Α"), "ας α");
        assert_eq!(to_lowercase2("Α'Σ Α"), "α'ς α");
        assert_eq!(to_lowercase2("Α''Σ Α"), "α''ς α");

        assert_eq!(to_lowercase2("ΑΣ' Α"), "ας' α");
        assert_eq!(to_lowercase2("ΑΣ'' Α"), "ας'' α");

        assert_eq!(to_lowercase2("Α'Σ' Α"), "α'ς' α");
        assert_eq!(to_lowercase2("Α''Σ'' Α"), "α''ς'' α");

        assert_eq!(to_lowercase2("Α Σ"), "α σ");
        assert_eq!(to_lowercase2("Α 'Σ"), "α 'σ");
        assert_eq!(to_lowercase2("Α ''Σ"), "α ''σ");

        assert_eq!(to_lowercase2("Σ"), "σ");
        assert_eq!(to_lowercase2("'Σ"), "'σ");
        assert_eq!(to_lowercase2("''Σ"), "''σ");

        assert_eq!(to_lowercase2("ΑΣΑ"), "ασα");
        assert_eq!(to_lowercase2("ΑΣ'Α"), "ασ'α");
        assert_eq!(to_lowercase2("ΑΣ''Α"), "ασ''α");
    }

    #[test]
    fn uppercase2() {
        assert_eq!(to_uppercase2(""), "");
        assert_eq!(to_uppercase2("aéǅßﬁᾀ"), "AÉǄSSFIἈΙ");
    }
}
