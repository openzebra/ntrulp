use crate::math;
use crate::ntru::errors::NTRUErrors;

pub fn check_params<const P: usize, const Q: usize, const W: usize, const Q12: usize>(
) -> Result<(), NTRUErrors> {
    if !math::prime::is_prime(P) {
        return Err(NTRUErrors::PMustBePrimeNumber);
    }
    if !math::prime::is_prime(Q) {
        return Err(NTRUErrors::QMustbePrimeNumber);
    }
    if !(W > 0) {
        return Err(NTRUErrors::WCannotBeLessZero);
    }
    if !(2 * P >= 3 * W) {
        return Err(NTRUErrors::DubblePShouldBeMoreOrEqTripleW);
    }
    if !(Q >= 16 * W + 1) {
        return Err(NTRUErrors::QShouldBeMoreOrEq17MulWPlusOne);
    }
    if !(Q % 6 == 1) {
        // spec allows 5 but these tests do not
        return Err(NTRUErrors::QModeSixShouldBeEqOne);
    }

    Ok(())
}
