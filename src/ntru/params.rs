use crate::math;
use crate::ntru::errors::NTRUErrors;

pub fn check_params<
    const P: usize,
    const Q: usize,
    const W: usize,
    const Q12: usize,
    const P_PLUS_ONE: usize,
    const P_TWICE_MINUS_ONE: usize,
>() -> Result<(), NTRUErrors<'static>> {
    if !math::prime::is_prime(P) {
        return Err(NTRUErrors::ParamsError("P should be Prime number"));
    }
    if !math::prime::is_prime(Q) {
        return Err(NTRUErrors::ParamsError("Q should be Prime number"));
    }
    if !(W > 0) {
        return Err(NTRUErrors::ParamsError("W should be more then 0"));
    }
    if !(2 * P >= 3 * W) {
        return Err(NTRUErrors::ParamsError("2P should be more then 3W"));
    }
    if !(Q >= 16 * W + 1) {
        return Err(NTRUErrors::ParamsError("Q should be more or eq then W + 1"));
    }
    if !(Q % 6 == 1) {
        return Err(NTRUErrors::ParamsError("Q mod 6 should be eq 1"));
    }
    if P + 1 != P_PLUS_ONE {
        return Err(NTRUErrors::ParamsError("P_PLUS_ONE should be eq P + 1"));
    }
    if P * 2 - 1 != P_TWICE_MINUS_ONE {
        return Err(NTRUErrors::ParamsError(
            "P_TWICE_MINUS_ONE should be eq 2P-1",
        ));
    }

    Ok(())
}
