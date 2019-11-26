#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 512usize],
    #[doc = "0x200 - Device info"]
    pub info: INFO,
    _reserved1: [u8; 84usize],
    #[doc = "0x280 - Description collection: Encryption Root, word n"]
    pub er: [ER; 4],
    #[doc = "0x290 - Description collection: Identity Root, word n"]
    pub ir: [IR; 4],
    #[doc = "0x2a0 - Device address type"]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0x2a4 - Description collection: Device address n"]
    pub deviceaddr: [DEVICEADDR; 2],
    _reserved5: [u8; 84usize],
    #[doc = "0x300 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
}
#[doc = r"Register block"]
#[repr(C)]
pub struct INFO {
    #[doc = "0x00 - Configuration identifier"]
    pub configid: self::info::CONFIGID,
    #[doc = "0x04 - Description collection: Device identifier"]
    pub deviceid: [self::info::DEVICEID; 2],
    #[doc = "0x0c - Part code"]
    pub part: self::info::PART,
    #[doc = "0x10 - Part Variant, Hardware version and Production configuration"]
    pub variant: self::info::VARIANT,
    #[doc = "0x14 - Package option"]
    pub package: self::info::PACKAGE,
    #[doc = "0x18 - RAM variant"]
    pub ram: self::info::RAM,
    #[doc = "0x1c - Flash variant"]
    pub flash: self::info::FLASH,
    #[doc = "0x20 - Code memory page size in bytes"]
    pub codepagesize: self::info::CODEPAGESIZE,
    #[doc = "0x24 - Code memory size"]
    pub codesize: self::info::CODESIZE,
    #[doc = "0x28 - Device type"]
    pub devicetype: self::info::DEVICETYPE,
}
#[doc = r"Register block"]
#[doc = "Device info"]
pub mod info;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRIMCNF {
    #[doc = "0x00 - Description cluster: Address"]
    pub addr: self::trimcnf::ADDR,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: self::trimcnf::DATA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = "Description collection: Encryption Root, word n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [er](er) module"]
pub type ER = crate::Reg<u32, _ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER;
#[doc = "`read()` method returns [er::R](er::R) reader structure"]
impl crate::Readable for ER {}
#[doc = "Description collection: Encryption Root, word n"]
pub mod er;
#[doc = "Description collection: Identity Root, word n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "Description collection: Identity Root, word n"]
pub mod ir;
#[doc = "Device address type\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceaddrtype](deviceaddrtype) module"]
pub type DEVICEADDRTYPE = crate::Reg<u32, _DEVICEADDRTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDRTYPE;
#[doc = "`read()` method returns [deviceaddrtype::R](deviceaddrtype::R) reader structure"]
impl crate::Readable for DEVICEADDRTYPE {}
#[doc = "Device address type"]
pub mod deviceaddrtype;
#[doc = "Description collection: Device address n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceaddr](deviceaddr) module"]
pub type DEVICEADDR = crate::Reg<u32, _DEVICEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDR;
#[doc = "`read()` method returns [deviceaddr::R](deviceaddr::R) reader structure"]
impl crate::Readable for DEVICEADDR {}
#[doc = "Description collection: Device address n"]
pub mod deviceaddr;
