#[cfg(feature = "ntrup653")]
pub mod params653 {
    pub const P: usize = 653;
    pub const Q: usize = 4621;
    pub const W: usize = 288;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 4;
}

#[cfg(feature = "ntrup761")]
pub mod params761 {
    pub const P: usize = 761;
    pub const W: usize = 286;
    pub const Q: usize = 4591;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 6;
}

#[cfg(feature = "ntrup857")]
pub mod params857 {
    pub const P: usize = 857;
    pub const W: usize = 322;
    pub const Q: usize = 5167;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 8;
}

#[cfg(feature = "ntrup953")]
pub mod params953 {
    pub const P: usize = 953;
    pub const Q: usize = 6343;
    pub const W: usize = 396;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 10;
}

#[cfg(feature = "ntrup1013")]
pub mod params1013 {
    pub const P: usize = 1013;
    pub const Q: usize = 7177;
    pub const W: usize = 448;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 12;
}

pub mod params1277 {
    pub const P: usize = 1277;
    pub const Q: usize = 7879;
    pub const W: usize = 492;
    pub const Q12: usize = (Q - 1) / 2;
    pub const R3_BYTES: usize = (P + 3) / 4;
    pub const RQ_BYTES: usize = P * 2;
    pub const PUBLICKEYS_BYTES: usize = RQ_BYTES;
    pub const SECRETKEYS_BYTES: usize = R3_BYTES * 2;
    pub const DIFFICULT: usize = 14;
}

#[cfg(feature = "ntrup653")]
pub use params653 as params;

#[cfg(feature = "ntrup761")]
pub use params761 as params;

#[cfg(feature = "ntrup857")]
pub use params857 as params;

#[cfg(feature = "ntrup953")]
pub use params953 as params;

#[cfg(feature = "ntrup1013")]
pub use params1013 as params;

#[cfg(feature = "ntrup1277")]
pub use params1277 as params;

#[cfg(all(
    not(feature = "ntrup653"),
    not(feature = "ntrup761"),
    not(feature = "ntrup857"),
    not(feature = "ntrup953"),
    not(feature = "ntrup1013"),
    not(feature = "ntrup1277")
))]
pub use params1277 as params;

#[cfg(feature = "ntrup1277")]
pub use params1277 as params;
