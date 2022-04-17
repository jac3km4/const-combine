#[macro_export]
macro_rules! const_combine {
    ($A:expr, $B:expr) => {
        unsafe {
            std::mem::transmute::<&[u8], &str>(
                $crate::internal::combine::<{ $A.len() + $B.len() }>($A, $B).as_slice(),
            )
        }
    };
}

/// useful when above doesn't work due to const expression limitations
#[macro_export]
macro_rules! const_combine_bounded_with {
    ($A:expr, $B:expr, $max_size:expr) => {
        unsafe {
            std::mem::transmute::<&[u8], &str>($crate::internal::take_n(
                &$crate::internal::combine::<$max_size>($A, $B),
                $A.len() + $B.len(),
            ))
        }
    };
}

/// does not work for strings larger than 1024 bytes
#[macro_export]
macro_rules! const_combine_bounded {
    ($A:expr, $B:expr) => {
        $crate::const_combine_bounded_with!($A, $B, 1024)
    };
}

pub mod bounded {
    pub use super::{
        const_combine_bounded as const_combine,
        const_combine_bounded_with as const_combine_with_len,
    };
}

pub mod internal {
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

    pub const fn take_n(input: &[u8], n: usize) -> &[u8] {
        let mut index = input.len();
        let mut slice = input;
        loop {
            match slice.split_last() {
                None => return slice,
                Some(_) if index == n => return slice,
                Some((_, tail)) => {
                    slice = tail;
                    index -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn concat_strings() {
        const STR: &str = "abc";

        assert_eq!(const_combine!(STR, "def"), "abcdef");
        assert_eq!(const_combine!("def", STR), "defabc");
        assert_eq!(const_combine!(STR, STR), "abcabc");
    }

    #[test]
    fn concat_strings_sized() {
        const STR: &str = "abc";

        assert_eq!(const_combine_bounded!(STR, "def"), "abcdef");
        assert_eq!(const_combine_bounded!("def", STR), "defabc");
        assert_eq!(const_combine_bounded!(STR, STR), "abcabc");
    }
}
