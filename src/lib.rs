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

#[macro_export]
macro_rules! combine {
    ($A:expr, $B:expr) => {
        unsafe {
            std::mem::transmute::<&[u8], &str>(
                $crate::combine::<{ $A.len() + $B.len() }>($A, $B).as_slice(),
            )
        }
    };
}

/// useful when above doesn't work due to const expression limitations
#[macro_export]
macro_rules! combine_buf_sized {
    ($A:expr, $B:expr, $max_size:expr) => {
        unsafe {
            std::mem::transmute::<&[u8], &str>($crate::take_n(
                &$crate::combine::<$max_size>($A, $B),
                $A.len() + $B.len(),
            ))
        }
    };
}

/// does not work for strings larger than 1024 bytes
#[macro_export]
macro_rules! combine_buf {
    ($A:expr, $B:expr) => {
        $crate::combine_buf_sized!($A, $B, 1024)
    };
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

    #[test]
    fn concat_strings_sized() {
        const STR: &str = "abc";

        assert_eq!(combine_buf!(STR, "def"), "abcdef");
        assert_eq!(combine_buf!("def", STR), "defabc");
        assert_eq!(combine_buf!(STR, STR), "abcabc");
    }
}
