#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 512usize],
    #[doc = "0x200 - Device info"]
    pub info: INFO,
    _reserved1: [u8; 212usize],
    #[doc = "0x300 - Unspecified"]
    pub trimcnf: [TRIMCNF; 32],
    _reserved2: [u8; 80usize],
    #[doc = "0x450 - Unspecified"]
    pub nfc: NFC,
    _reserved3: [u8; 1952usize],
    #[doc = "0xc00 - NIST800-90B RNG calibration data"]
    pub trng90b: TRNG90B,
    #[doc = "0xc20 - XOSC32M capacitor selection trim values"]
    pub xosc32mtrim: XOSC32MTRIM,
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
    #[doc = "0x00 - Description cluster: Address of the PAR register which will be written"]
    pub addr: self::trimcnf::ADDR,
    #[doc = "0x04 - Description cluster: Data"]
    pub data: self::trimcnf::DATA,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod trimcnf;
#[doc = r"Register block"]
#[repr(C)]
pub struct NFC {
    #[doc = "0x00 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader0: self::nfc::TAGHEADER0,
    #[doc = "0x04 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader1: self::nfc::TAGHEADER1,
    #[doc = "0x08 - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader2: self::nfc::TAGHEADER2,
    #[doc = "0x0c - Default header for NFC Tag. Software can read these values to populate NFCID1_3RD_LAST, NFCID1_2ND_LAST and NFCID1_LAST."]
    pub tagheader3: self::nfc::TAGHEADER3,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod nfc;
#[doc = r"Register block"]
#[repr(C)]
pub struct TRNG90B {
    #[doc = "0x00 - Amount of bytes for the required entropy bits"]
    pub bytes: self::trng90b::BYTES,
    #[doc = "0x04 - Repetition counter cutoff"]
    pub rccutoff: self::trng90b::RCCUTOFF,
    #[doc = "0x08 - Adaptive proportion cutoff"]
    pub apcutoff: self::trng90b::APCUTOFF,
    #[doc = "0x0c - Amount of bytes for the startup tests"]
    pub startup: self::trng90b::STARTUP,
    #[doc = "0x10 - Sample count for ring oscillator 1"]
    pub rosc1: self::trng90b::ROSC1,
    #[doc = "0x14 - Sample count for ring oscillator 2"]
    pub rosc2: self::trng90b::ROSC2,
    #[doc = "0x18 - Sample count for ring oscillator 3"]
    pub rosc3: self::trng90b::ROSC3,
    #[doc = "0x1c - Sample count for ring oscillator 4"]
    pub rosc4: self::trng90b::ROSC4,
}
#[doc = r"Register block"]
#[doc = "NIST800-90B RNG calibration data"]
pub mod trng90b;
#[doc = "XOSC32M capacitor selection trim values\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [xosc32mtrim](xosc32mtrim) module"]
pub type XOSC32MTRIM = crate::Reg<u32, _XOSC32MTRIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XOSC32MTRIM;
#[doc = "`read()` method returns [xosc32mtrim::R](xosc32mtrim::R) reader structure"]
impl crate::Readable for XOSC32MTRIM {}
#[doc = "XOSC32M capacitor selection trim values"]
pub mod xosc32mtrim;
