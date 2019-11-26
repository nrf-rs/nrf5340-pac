#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Reset reason"]
    pub resetreas: RESETREAS,
    _reserved1: [u8; 524usize],
    #[doc = "0x610 - ULP Network core control"]
    pub network: NETWORK,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct NETWORK {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - Force off power and clock in Network core"]
    pub forceoff: self::network::FORCEOFF,
}
#[doc = r"Register block"]
#[doc = "ULP Network core control"]
pub mod network;
#[doc = "Reset reason\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resetreas](resetreas) module"]
pub type RESETREAS = crate::Reg<u32, _RESETREAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETREAS;
#[doc = "`read()` method returns [resetreas::R](resetreas::R) reader structure"]
impl crate::Readable for RESETREAS {}
#[doc = "`write(|w| ..)` method takes [resetreas::W](resetreas::W) writer structure"]
impl crate::Writable for RESETREAS {}
#[doc = "Reset reason"]
pub mod resetreas;
