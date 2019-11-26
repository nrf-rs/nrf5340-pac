use self::keyslot::{KEY, CONFIG};

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Access port protection"]
    pub approtect: APPROTECT,
    _reserved1: [u8; 8usize],
    #[doc = "0x0c - Enable external circuitry to be supplied from VDD pin. Applicable in 'High voltage mode' only."]
    pub extsupply: EXTSUPPLY,
    #[doc = "0x10 - GPIO reference voltage / external output supply voltage in 'High voltage mode'."]
    pub vreghvout: VREGHVOUT,
    #[doc = "0x14 - HFXO startup counter"]
    pub hfxocnt: HFXOCNT,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - Secure access port protection"]
    pub secureapprotect: SECUREAPPROTECT,
    #[doc = "0x20 - Erase protection"]
    pub eraseprotect: ERASEPROTECT,
    #[doc = "0x24 - SW-DP Target instance"]
    pub tinstance: TINSTANCE,
    #[doc = "0x28 - Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
    pub nfcpins: NFCPINS,
    _reserved8: [u8; 212usize],
    #[doc = "0x100 - Description collection: One time programmable memory"]
    pub otp: [OTP; 192],
    #[doc = "0x400 - Unspecified"]
    pub keyslot: KEYSLOT,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct KEYSLOT {
    #[doc = "0x00 - Unspecified"]
    pub config: [CONFIG; 128],
    #[doc = "0x400 - Unspecified"]
    pub key: [KEY; 128],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod keyslot;
#[doc = "Access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [approtect](approtect) module"]
pub type APPROTECT = crate::Reg<u32, _APPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APPROTECT;
#[doc = "`read()` method returns [approtect::R](approtect::R) reader structure"]
impl crate::Readable for APPROTECT {}
#[doc = "`write(|w| ..)` method takes [approtect::W](approtect::W) writer structure"]
impl crate::Writable for APPROTECT {}
#[doc = "Access port protection"]
pub mod approtect;
#[doc = "Enable external circuitry to be supplied from VDD pin. Applicable in 'High voltage mode' only.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [extsupply](extsupply) module"]
pub type EXTSUPPLY = crate::Reg<u32, _EXTSUPPLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTSUPPLY;
#[doc = "`read()` method returns [extsupply::R](extsupply::R) reader structure"]
impl crate::Readable for EXTSUPPLY {}
#[doc = "`write(|w| ..)` method takes [extsupply::W](extsupply::W) writer structure"]
impl crate::Writable for EXTSUPPLY {}
#[doc = "Enable external circuitry to be supplied from VDD pin. Applicable in 'High voltage mode' only."]
pub mod extsupply;
#[doc = "GPIO reference voltage / external output supply voltage in 'High voltage mode'.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vreghvout](vreghvout) module"]
pub type VREGHVOUT = crate::Reg<u32, _VREGHVOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREGHVOUT;
#[doc = "`read()` method returns [vreghvout::R](vreghvout::R) reader structure"]
impl crate::Readable for VREGHVOUT {}
#[doc = "`write(|w| ..)` method takes [vreghvout::W](vreghvout::W) writer structure"]
impl crate::Writable for VREGHVOUT {}
#[doc = "GPIO reference voltage / external output supply voltage in 'High voltage mode'."]
pub mod vreghvout;
#[doc = "HFXO startup counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hfxocnt](hfxocnt) module"]
pub type HFXOCNT = crate::Reg<u32, _HFXOCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFXOCNT;
#[doc = "`read()` method returns [hfxocnt::R](hfxocnt::R) reader structure"]
impl crate::Readable for HFXOCNT {}
#[doc = "`write(|w| ..)` method takes [hfxocnt::W](hfxocnt::W) writer structure"]
impl crate::Writable for HFXOCNT {}
#[doc = "HFXO startup counter"]
pub mod hfxocnt;
#[doc = "Secure access port protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [secureapprotect](secureapprotect) module"]
pub type SECUREAPPROTECT = crate::Reg<u32, _SECUREAPPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SECUREAPPROTECT;
#[doc = "`read()` method returns [secureapprotect::R](secureapprotect::R) reader structure"]
impl crate::Readable for SECUREAPPROTECT {}
#[doc = "`write(|w| ..)` method takes [secureapprotect::W](secureapprotect::W) writer structure"]
impl crate::Writable for SECUREAPPROTECT {}
#[doc = "Secure access port protection"]
pub mod secureapprotect;
#[doc = "Erase protection\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eraseprotect](eraseprotect) module"]
pub type ERASEPROTECT = crate::Reg<u32, _ERASEPROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPROTECT;
#[doc = "`read()` method returns [eraseprotect::R](eraseprotect::R) reader structure"]
impl crate::Readable for ERASEPROTECT {}
#[doc = "`write(|w| ..)` method takes [eraseprotect::W](eraseprotect::W) writer structure"]
impl crate::Writable for ERASEPROTECT {}
#[doc = "Erase protection"]
pub mod eraseprotect;
#[doc = "SW-DP Target instance\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tinstance](tinstance) module"]
pub type TINSTANCE = crate::Reg<u32, _TINSTANCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TINSTANCE;
#[doc = "`read()` method returns [tinstance::R](tinstance::R) reader structure"]
impl crate::Readable for TINSTANCE {}
#[doc = "`write(|w| ..)` method takes [tinstance::W](tinstance::W) writer structure"]
impl crate::Writable for TINSTANCE {}
#[doc = "SW-DP Target instance"]
pub mod tinstance;
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nfcpins](nfcpins) module"]
pub type NFCPINS = crate::Reg<u32, _NFCPINS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NFCPINS;
#[doc = "`read()` method returns [nfcpins::R](nfcpins::R) reader structure"]
impl crate::Readable for NFCPINS {}
#[doc = "`write(|w| ..)` method takes [nfcpins::W](nfcpins::W) writer structure"]
impl crate::Writable for NFCPINS {}
#[doc = "Setting of pins dedicated to NFC functionality: NFC antenna or GPIO"]
pub mod nfcpins;
#[doc = "Description collection: One time programmable memory\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [otp](otp) module"]
pub type OTP = crate::Reg<u32, _OTP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OTP;
#[doc = "`read()` method returns [otp::R](otp::R) reader structure"]
impl crate::Readable for OTP {}
#[doc = "`write(|w| ..)` method takes [otp::W](otp::W) writer structure"]
impl crate::Writable for OTP {}
#[doc = "Description collection: One time programmable memory"]
pub mod otp;
