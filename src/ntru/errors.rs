#[derive(Debug)]
pub enum NTRUErrors {
    PMustBePrimeNumber,
    QMustbePrimeNumber,
    WCannotBeLessZero,
    DubblePShouldBeMoreOrEqTripleW,
    QShouldBeMoreOrEq17MulWPlusOne,
    QModeSixShouldBeEqOne,
    KeyPairGen,
    KeysIsEmpty,
}
