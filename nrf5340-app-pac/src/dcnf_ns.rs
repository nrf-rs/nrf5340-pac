#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1056usize],
    #[doc = "0x420 - CPU ID of this subsystem"]
    pub cpuid: CPUID,
    _reserved1: [u8; 28usize],
    #[doc = "0x440 - Unspecified"]
    pub extperi: [EXTPERI; 1],
    _reserved2: [u8; 28usize],
    #[doc = "0x460 - Unspecified"]
    pub extram: [EXTRAM; 1],
    _reserved3: [u8; 28usize],
    #[doc = "0x480 - Unspecified"]
    pub extcode: [EXTCODE; 1],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTPERI {
    #[doc = "0x00 - Description cluster: Control access for master connected to AMLI master port EXTPERI\\[n\\]"]
    pub protect: self::extperi::PROTECT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extperi;
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTRAM {
    #[doc = "0x00 - Description cluster: Control access from master connected to AMLI master port EXTRAM\\[n\\]"]
    pub protect: self::extram::PROTECT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extram;
#[doc = r"Register block"]
#[repr(C)]
pub struct EXTCODE {
    #[doc = "0x00 - Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]"]
    pub protect: self::extcode::PROTECT,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod extcode;
#[doc = "CPU ID of this subsystem\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cpuid](cpuid) module"]
pub type CPUID = crate::Reg<u32, _CPUID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPUID;
#[doc = "`read()` method returns [cpuid::R](cpuid::R) reader structure"]
impl crate::Readable for CPUID {}
#[doc = "CPU ID of this subsystem"]
pub mod cpuid;
