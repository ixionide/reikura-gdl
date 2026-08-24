/// const_iota! {
///     u8 = iota,
///     A,
///     B,
///     C
/// }
///
/// assert_eq!(A, 0);
/// assert_eq!(B, 1);
/// assert_eq!(C, 2);
#[macro_export]
macro_rules! const_iota {
    ($ty:ty = $expr:expr, $($id:ident),+ $(,)?) => {
        $crate::const_iota!(@internal $ty, $expr, 0, $($id),+);
    };

    (@internal $ty:ty, $expr:expr, $iota:expr, $head:ident $(, $tail:ident)*) => {
        const $head: $ty = {
            #[allow(unused_variables, non_upper_case_globals)]
            const iota: $ty = $iota;
            $expr
        };
        $crate::const_iota!(@internal $ty, $expr, $iota + 1, $($tail),*);
    };

    (@internal $ty:ty, $expr:expr, $iota:expr $(,)?) => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn iota() {
        const_iota! {
            u8 = iota,
            A,
            B,
            C
        }

        assert_eq!(A, 0);
        assert_eq!(B, 1);
        assert_eq!(C, 2);
    }

    #[test]
    fn iota_offset() {
        const_iota! {
            u8 = 1 + iota,
            D,
            E,
            F
        }
        assert_eq!(D, 1);
        assert_eq!(E, 2);
        assert_eq!(F, 3);

        const_iota! {
            u8 = 4 + iota,
            G,
            H,
            I
        }
        assert_eq!(G, 4);
        assert_eq!(H, 5);
        assert_eq!(I, 6);
    }

    #[test]
    fn iota_bitwise() {
        const_iota! {
            u8 = 1 << iota,
            FLAG_X,
            FLAG_Y,
            FLAG_Z
        }

        assert_eq!(FLAG_X, 1);
        assert_eq!(FLAG_Y, 2);
        assert_eq!(FLAG_Z, 4);
    }
}
