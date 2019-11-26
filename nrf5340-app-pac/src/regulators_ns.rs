#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1064usize],
    #[doc = "0x428 - Main supply status"]
    pub mainregstatus: MAINREGSTATUS,
    _reserved1: [u8; 212usize],
    #[doc = "0x500 - System OFF register"]
    pub systemoff: SYSTEMOFF,
    _reserved2: [u8; 12usize],
    #[doc = "0x510 - Power-fail comparator configuration"]
    pub pofcon: POFCON,
    _reserved3: [u8; 496usize],
    #[doc = "0x704 - Unspecified"]
    pub vregmain: VREGMAIN,
    _reserved4: [u8; 504usize],
    #[doc = "0x900 - Unspecified"]
    pub vregradio: VREGRADIO,
    _reserved5: [u8; 504usize],
    #[doc = "0xb00 - Unspecified"]
    pub vregh: VREGH,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGMAIN {
    #[doc = "0x00 - DC/DC enable register for VREGMAIN"]
    pub dcdcen: self::vregmain::DCDCEN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregmain;
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGRADIO {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - DC/DC enable register for VREGRADIO"]
    pub dcdcen: self::vregradio::DCDCEN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregradio;
#[doc = r"Register block"]
#[repr(C)]
pub struct VREGH {
    #[doc = "0x00 - DC/DC enable register for VREGH"]
    pub dcdcen: self::vregh::DCDCEN,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod vregh;
#[doc = "Main supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mainregstatus](mainregstatus) module"]
pub type MAINREGSTATUS = crate::Reg<u32, _MAINREGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAINREGSTATUS;
#[doc = "`read()` method returns [mainregstatus::R](mainregstatus::R) reader structure"]
impl crate::Readable for MAINREGSTATUS {}
#[doc = "Main supply status"]
pub mod mainregstatus;
#[doc = "System OFF register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systemoff](systemoff) module"]
pub type SYSTEMOFF = crate::Reg<u32, _SYSTEMOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEMOFF;
#[doc = "`write(|w| ..)` method takes [systemoff::W](systemoff::W) writer structure"]
impl crate::Writable for SYSTEMOFF {}
#[doc = "System OFF register"]
pub mod systemoff;
#[doc = "Power-fail comparator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pofcon](pofcon) module"]
pub type POFCON = crate::Reg<u32, _POFCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POFCON;
#[doc = "`read()` method returns [pofcon::R](pofcon::R) reader structure"]
impl crate::Readable for POFCON {}
#[doc = "`write(|w| ..)` method takes [pofcon::W](pofcon::W) writer structure"]
impl crate::Writable for POFCON {}
#[doc = "Power-fail comparator configuration"]
pub mod pofcon;
