#![expect(
    clippy::suboptimal_flops,
    reason = "the check mirrors the plain arithmetic the code uses"
)]

use super::square_root;

#[test]
fn the_root_of_zero_is_zero() {
    assert_eq!(square_root(0.0).to_bits(), 0.0_f64.to_bits());
}

#[test]
fn the_roots_of_small_perfect_squares_are_exact() {
    for whole in 1..=2000_u32 {
        let root = f64::from(whole);
        assert_eq!(
            square_root(root * root).to_bits(),
            root.to_bits(),
            "root {whole}"
        );
    }
}

#[test]
fn the_root_of_two_is_within_one_part_in_ten_to_the_fifteen() {
    assert!((square_root(2.0) - core::f64::consts::SQRT_2).abs() < 1e-15);
}

#[test]
fn the_root_of_the_largest_sum_of_squares_in_the_world_is_close() {
    // 1 036 800 cells of sigma 65 535: sqrt(1036800) * 65535 = 1018.2... * 65535
    let squares = 1_036_800.0 * 65_535.0 * 65_535.0;
    let root = square_root(squares);
    assert!((root * root - squares).abs() / squares < 1e-15);
}

#[test]
fn a_negative_or_infinite_or_missing_number_has_root_zero() {
    for value in [-4.0, f64::NEG_INFINITY, f64::INFINITY, f64::NAN] {
        assert_eq!(square_root(value).to_bits(), 0.0_f64.to_bits());
    }
}
