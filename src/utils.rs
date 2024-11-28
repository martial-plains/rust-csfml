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

pub trait HasCsfmlPointer {
    type Output;
    fn mut_ptr(&self) -> *mut Self::Output;
}
