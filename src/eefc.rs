//! EEFC - Enhanced Embedded Flash Controller

use crate::pac::{EFC0, EFC1};

//----------------------------------------------------------------------------
//Error Handling
//----------------------------------------------------------------------------
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub enum Error {
    AddressLargerThanFlash,
    AddressMisaligned,
    LengthNotMultiple2,
    LengthTooLong,
    EraseError,
    ProgrammingError,
    WriteError,
    VerifyError,
    UnlockError,
    LockError,
}
//----------------------------------------------------------------------------
///Constrained EFCx Peripheral
pub struct Registers{
    #[doc = "Opaque EFC Flash Mode Register"]
    pub fmr: FMR,
    #[doc = "Opaque EFC Flash Command Register"]
    pub fcr: FCR,
    #[doc = "Opaque EFC Flash Status Register"]
    pub fsr: FSR,
    #[doc = "Opaque EFC Flash Result Register"]
    pub frr: FRR,
}

///Opaqque EFCx Registers
pub struct FMR{}
pub struct FCR{}
pub struct FSR{}
pub struct FRR{}

//----------------------------------------------------------------------------
//Extentions
//----------------------------------------------------------------------------
/// Extension trait to constrain the EFCx Peripherals
pub trait EFCxExt {
    /// Constrains the EFCx peripherals to play nicely with the other abstractions
    fn constrain(self) -> Registers;
}

//Implement Extension Trait for EFCx Peripherals
impl EFCxExt for (EFC0, EFC1){
    fn constrain(self) -> Registers {
        Registers {
            fmr: FMR{},
            fcr: FCR{},
            fsr: FSR{},
            frr: FRR{},
        }
    }
}