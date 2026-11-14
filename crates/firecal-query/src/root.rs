//! The square root with only `+ - * /`, because `core` has no `sqrt` and the same arithmetic gives the same
//! bits natively and in wasm.

const NEWTON_STEPS: u32 = 8;
const HALVED_EXPONENT_BIAS: u64 = 0x1FF8_0000_0000_0000;

/// The square root of a finite `value` of zero or more; zero for anything else.
///
/// Halving the exponent bits gives a guess within a few percent, and eight Newton steps square that error
/// eight times over, far below one unit in the last place. The result is deterministic, but not promised to
/// be the correctly rounded root: it can differ from `f64::sqrt` in the last bit.
pub(crate) fn square_root(value: f64) -> f64 {
    if !value.is_finite() || value <= 0.0 {
        return 0.0;
    }
    let mut guess = f64::from_bits((value.to_bits() >> 1) + HALVED_EXPONENT_BIAS);
    for _ in 0..NEWTON_STEPS {
        guess = guess.midpoint(value / guess);
    }
    guess
}

#[cfg(test)]
mod tests;
