#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: crate::Reg<resetreas::RESETREAS_SPEC>,
    _reserved1: [u8; 524usize],
    #[doc = "0x610 - ULP network core control"]
    pub network: NETWORK,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct NETWORK {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Force network core off"]
    pub forceoff: crate::Reg<self::network::forceoff::FORCEOFF_SPEC>,
}
#[doc = r"Register block"]
#[doc = "ULP network core control"]
pub mod network;
#[doc = "RESETREAS register accessor: an alias for `Reg<RESETREAS_SPEC>`"]
pub type RESETREAS = crate::Reg<resetreas::RESETREAS_SPEC>;
#[doc = "Reset reason"]
pub mod resetreas;
