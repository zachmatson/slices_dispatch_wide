#![doc = include_str!("../README.md")]

pub use itertools;
pub use paste;
pub use wide;

/// Macro to help dispatch math over slices using the `wide` crate for SIMD operations
///
/// Check out the [crate] documentation for details
#[macro_export]
macro_rules! slices_dispatch_wide {
    (
        $width: expr,
        |$($src:expr => $name:ident $($mut_ident:ident)?: $type:ty),+|
        $work:block
    ) => {
        // Assert all slices are the same length
        {
            let mut slice_length = None;
            $(
                if let Some(other_length) = slice_length {
                    assert_eq!($src.len(), other_length);
                }
                slice_length = Some($src.len());
            )+
        }

        $(
            // Matching against an ident is the most convenient way to actually tell
            // the mut keyword was used for the outer macro, but we need to make sure
            // you can't just use any random keyword in that spot
            $($crate::slices_dispatch_wide!(@enforce_mut_ident_value $mut_ident);)?
        )+

        $crate::paste::paste! {
            for (
                $([< original_ $name >]),+
            ) in $crate::itertools::izip!(
                $($crate::slices_dispatch_wide!(@get_chunks $($mut_ident)? $src, $width)),+
            ) {
                $(
                    // This try_into and unwrap should be optimized out
                    let $($mut_ident)? $name =
                        $crate::wide::[< $type x $width >]::new([< original_ $name >].try_into().unwrap());
                )+

                { $work }

                $($(
                    { let $mut_ident _ignore: (); }
                    [< original_ $name >].copy_from_slice($name.as_array_ref());
                )?)+
            }

            for (
                $([< original_ $name >]),+
            ) in $crate::itertools::izip!(
                $($crate::slices_dispatch_wide!(@get_remainder $($mut_ident)? $src, $width)),+
            ) {
               $(
                   let $($mut_ident)? $name = *[< original_ $name >];
               )+

               { $work }

                $($(
                    { let $mut_ident _ignore: (); }
                    *[< original_ $name >] = $name;
                )?)+
            }
        }
    };

    (@enforce_mut_ident_value mut) => {};

    (@get_chunks mut $src:expr, $width:literal) => {
        ($src).chunks_exact_mut($width)
    };
    (@get_chunks $src:expr, $width:literal) => {
        ($src).chunks_exact($width)
    };

    (@get_remainder mut $src:expr, $width:literal) => {
        $crate::slices_dispatch_wide!(@get_chunks mut $src, $width).into_remainder()
    };
    (@get_remainder $src:expr, $width:literal) => {
        $crate::slices_dispatch_wide!(@get_chunks $src, $width).remainder()
    };
}
