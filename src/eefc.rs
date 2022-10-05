//! EEFC - Enhanced Embedded Flash Controller

use crate::pac::{EFC0, EFC1, efc0};

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
///Constrained EFC0 Peripheral
pub struct ConstrPeripheral{
    #[doc = "Opaque EFCx Flash Mode Register"]
    pub fmr: FMR,
    #[doc = "Opaque EFCx Flash Command Register"]
    pub fcr: FCR,
    #[doc = "Opaque EFCx Flash Status Register"]
    pub fsr: FSR,
    #[doc = "Opaque EFCx Flash Result Register"]
    pub frr: FRR,
}

//Opaqque EFCx Registers
pub struct FMR{}
pub struct FCR{}
pub struct FSR{}
pub struct FRR{}

impl FMR{
    pub fn fmr(&mut self) -> &efc0::FMR{
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe{
            &(*EFC0::ptr()).fmr}
    }
}

impl FCR{
    pub fn fcr(&mut self) -> &efc0::FCR{
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe{
            &(*EFC0::ptr()).fcr}
    }
}

impl FSR{
    pub fn fsr(&mut self) -> &efc0::FSR{
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe{
            &(*EFC0::ptr()).fsr}
    }
}

impl FRR{
    pub fn frr(&mut self) -> &efc0::FRR{
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe{
            &(*EFC0::ptr()).frr}
    }
}

//----------------------------------------------------------------------------
//Extentions
//----------------------------------------------------------------------------
/// Extension trait to constrain the EFCx Peripherals
pub trait EFCxExt {
    /// Constrains the EFCx peripherals to play nicely with the other abstractions
    fn constrain(self) -> ConstrPeripheral;
}

//Implement Extension Trait for EFCx Peripherals
impl EFCxExt for EFC0{
    fn constrain(self) -> ConstrPeripheral {
        ConstrPeripheral {
            fmr: FMR{},
            fcr: FCR{},
            fsr: FSR{},
            frr: FRR{},
        }
    }
}