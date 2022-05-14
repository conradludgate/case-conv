#![feature(unicode_internals)]
use core::unicode::conversions;

fn push_char(c: char, out: &mut Vec<u8>) {
    match c.len_utf8() {
        1 => out.push(c as u8),
        _ => out.extend_from_slice(c.encode_utf8(&mut [0; 4]).as_bytes()),
    }
}

/// Returns the lowercase equivalent of this string slice, as a new [`String`].
///
/// 'Lowercase' is defined according to the terms of the Unicode Derived Core Property
/// `Lowercase`.
///
/// Since some characters can expand into multiple characters when changing
/// the case, this function returns a [`String`] instead of modifying the
/// parameter in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let s = "HELLO";
///
/// assert_eq!("hello", s.to_lowercase());
/// ```
///
/// A tricky example, with sigma:
///
/// ```
/// let sigma = "Σ";
///
/// assert_eq!("σ", sigma.to_lowercase());
///
/// // but at the end of a word, it's ς, not σ:
/// let odysseus = "ὈΔΥΣΣΕΎΣ";
///
/// assert_eq!("ὀδυσσεύς", odysseus.to_lowercase());
/// ```
///
/// Languages without case are not changed:
///
/// ```
/// let new_year = "农历新年";
///
/// assert_eq!(new_year, new_year.to_lowercase());
/// ```
pub fn to_lowercase(s: &str) -> String {
    fn to_lowercase_cold(s: &str, out: &mut Vec<u8>) {
        for (i, c) in s.char_indices() {
            if c == 'Σ' {
                // Σ maps to σ, except at the end of a word where it maps to ς.
                // This is the only conditional (contextual) but language-independent mapping
                // in `SpecialCasing.txt`,
                // so hard-code it rather than have a generic "condition" mechanism.
                // See https://github.com/rust-lang/rust/issues/26035
                map_uppercase_sigma(s, i, out)
            } else {
                match conversions::to_lower(c) {
                    [a, '\0', _] => push_char(a, out),
                    [a, b, '\0'] => {
                        push_char(a, out);
                        push_char(b, out);
                    }
                    [a, b, c] => {
                        push_char(a, out);
                        push_char(b, out);
                        push_char(c, out);
                    }
                }
            }
        }

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
    }

    let mut out = Vec::with_capacity(s.len());
    for (i, b) in s.as_bytes().iter().enumerate() {
        if b & 0b1000_0000 == 0 {
            out.push(b.to_ascii_lowercase());
            continue;
        }

        // UTF-8 Found. Slow case

        // Safety: we know this is a valid char boundary since
        // 1. Our iterator guarantees that this is a valid byte
        // 2. From our loop we know this is the start of a utf8 scalar point
        let s = unsafe { s.get_unchecked(i..) };
        to_lowercase_cold(s, &mut out);
        break;
    }
    // Safety: We have written only valid UTF-8 to our vec
    unsafe { String::from_utf8_unchecked(out) }
}

/// Returns the uppercase equivalent of this string slice, as a new [`String`].
///
/// 'Uppercase' is defined according to the terms of the Unicode Derived Core Property
/// `Uppercase`.
///
/// Since some characters can expand into multiple characters when changing
/// the case, this function returns a [`String`] instead of modifying the
/// parameter in-place.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let s = "hello";
///
/// assert_eq!("HELLO", s.to_uppercase());
/// ```
///
/// Scripts without case are not changed:
///
/// ```
/// let new_year = "农历新年";
///
/// assert_eq!(new_year, new_year.to_uppercase());
/// ```
///
/// One character can become multiple:
/// ```
/// let s = "tschüß";
///
/// assert_eq!("TSCHÜSS", s.to_uppercase());
/// ```
pub fn to_uppercase(s: &str) -> String {
    fn to_uppercase_cold(s: &str, out: &mut Vec<u8>) {
        for c in s[..].chars() {
            match conversions::to_upper(c) {
                [a, '\0', _] => push_char(a, out),
                [a, b, '\0'] => {
                    push_char(a, out);
                    push_char(b, out);
                }
                [a, b, c] => {
                    push_char(a, out);
                    push_char(b, out);
                    push_char(c, out);
                }
            }
        }
    }

    let mut out = Vec::with_capacity(s.len());
    for (i, b) in s.as_bytes().iter().enumerate() {
        if b & 0b1000_0000 == 0 {
            out.push(b.to_ascii_uppercase());
            continue;
        }
        // UTF-8 Found. Slow case

        // Safety: we know this is a valid char boundary since
        // 1. Our iterator guarantees that this is a valid byte
        // 2. From our loop we know this is the start of a utf8 scalar point
        let s = unsafe { s.get_unchecked(i..) };
        to_uppercase_cold(s, &mut out);
        break;
    }
    // Safety: We have written only valid UTF-8 to our vec
    unsafe { String::from_utf8_unchecked(out) }
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
}
