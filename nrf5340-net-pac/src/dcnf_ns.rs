#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1056usize],
    #[doc = "0x420 - CPU ID of this subsystem"]
    pub cpuid: crate::Reg<cpuid::CPUID_SPEC>,
}
#[doc = "CPUID register accessor: an alias for `Reg<CPUID_SPEC>`"]
pub type CPUID = crate::Reg<cpuid::CPUID_SPEC>;
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
