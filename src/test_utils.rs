#[macro_export]
macro_rules! assert_float_eq {
    ($a:expr, $b:expr) => {
        assert!(
            ($a - $b).abs() < 1e-10,
            "assertion failed: `{:?}` != `{:?}`",
            $a,
            $b
        );
    };
    ($a:expr, $b:expr, $tol:expr) => {
        assert!(
            ($a - $b).abs() < $tol,
            "assertion failed: `{:?}` != `{:?}` (tolerance: {:?})",
            $a,
            $b,
            $tol
        );
    };
}
