pub fn assert_approx_eq(a: f32, b: f32, tolerance: f32) {
    assert!(
        (a - b).abs() <= tolerance,
        "Expected approximately {} but got {}",
        a,
        b
    );
}
