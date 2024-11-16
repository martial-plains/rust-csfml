#[macro_export]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $tolerance:expr) => {
        assert!(
            ($a - $b).abs() <= $tolerance,
            "Expected approximately {} but got {}",
            $a,
            $b
        );
    };
}
