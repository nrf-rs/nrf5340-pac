#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1280usize],
    #[doc = "0x500 - Enable CRYPTOCELL subsystem."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
}
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable CRYPTOCELL subsystem."]
pub mod enable;
