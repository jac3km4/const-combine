pub use const_combine_macros::str_len;

// based on https://users.rust-lang.org/t/concatenate-const-strings/51712/5
pub const fn combine<const L: usize>(a: &'static str, b: &'static str) -> [u8; L] {
    let mut out = [0u8; L];
    out = copy_slice(a.as_bytes(), out, 0);
    out = copy_slice(b.as_bytes(), out, a.len());
    out
}

pub const fn copy_slice<const L: usize>(
    input: &[u8],
    mut output: [u8; L],
    offset: usize,
) -> [u8; L] {
    let mut index = 0;
    loop {
        output[offset + index] = input[index];
        index += 1;
        if index == input.len() {
            break;
        }
    }
    output
}

#[macro_export]
macro_rules! combine {
    ($A:expr, $B:expr) => {{
        unsafe {
            std::mem::transmute::<&[u8], &str>(
                $crate::combine::<{ $crate::str_len!($A) + $crate::str_len!($B) }>($A, $B)
                    .as_slice(),
            )
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn concat_strings() {
        const STR: &str = "abc";

        assert_eq!(combine!(STR, "def"), "abcdef");
        assert_eq!(combine!("def", STR), "defabc");
        assert_eq!(combine!(STR, STR), "abcabc");
    }
}
