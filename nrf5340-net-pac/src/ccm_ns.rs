#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of keystream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encryption/decryption. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encryption/decryption"]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x0c - Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
    pub tasks_rateoverride: TASKS_RATEOVERRIDE,
    _reserved4: [u8; 112usize],
    #[doc = "0x80 - Subscribe configuration for task KSGEN"]
    pub subscribe_ksgen: SUBSCRIBE_KSGEN,
    #[doc = "0x84 - Subscribe configuration for task CRYPT"]
    pub subscribe_crypt: SUBSCRIBE_CRYPT,
    #[doc = "0x88 - Subscribe configuration for task STOP"]
    pub subscribe_stop: SUBSCRIBE_STOP,
    #[doc = "0x8c - Subscribe configuration for task RATEOVERRIDE"]
    pub subscribe_rateoverride: SUBSCRIBE_RATEOVERRIDE,
    _reserved8: [u8; 112usize],
    #[doc = "0x100 - Keystream generation complete"]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt complete"]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - Deprecated register - CCM error event"]
    pub events_error: EVENTS_ERROR,
    _reserved11: [u8; 116usize],
    #[doc = "0x180 - Publish configuration for event ENDKSGEN"]
    pub publish_endksgen: PUBLISH_ENDKSGEN,
    #[doc = "0x184 - Publish configuration for event ENDCRYPT"]
    pub publish_endcrypt: PUBLISH_ENDCRYPT,
    #[doc = "0x188 - Deprecated register - Publish configuration for event ERROR"]
    pub publish_error: PUBLISH_ERROR,
    _reserved14: [u8; 116usize],
    #[doc = "0x200 - Shortcuts between local events and tasks"]
    pub shorts: SHORTS,
    _reserved15: [u8; 256usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved17: [u8; 244usize],
    #[doc = "0x400 - MIC check result"]
    pub micstatus: MICSTATUS,
    _reserved18: [u8; 252usize],
    #[doc = "0x500 - Enable"]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode"]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to data structure holding the AES key and the NONCE vector"]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Input pointer"]
    pub inptr: INPTR,
    #[doc = "0x510 - Output pointer"]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to data area used for temporary storage"]
    pub scratchptr: SCRATCHPTR,
    #[doc = "0x518 - Length of keystream generated when MODE.LENGTH = Extended"]
    pub maxpacketsize: MAXPACKETSIZE,
    #[doc = "0x51c - Data rate override setting."]
    pub rateoverride: RATEOVERRIDE,
    #[doc = "0x520 - Header (S0) mask."]
    pub headermask: HEADERMASK,
}
#[doc = "Start generation of keystream. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_ksgen](tasks_ksgen) module"]
pub type TASKS_KSGEN = crate::Reg<u32, _TASKS_KSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_KSGEN;
#[doc = "`write(|w| ..)` method takes [tasks_ksgen::W](tasks_ksgen::W) writer structure"]
impl crate::Writable for TASKS_KSGEN {}
#[doc = "Start generation of keystream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "Start encryption/decryption. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_crypt](tasks_crypt) module"]
pub type TASKS_CRYPT = crate::Reg<u32, _TASKS_CRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CRYPT;
#[doc = "`write(|w| ..)` method takes [tasks_crypt::W](tasks_crypt::W) writer structure"]
impl crate::Writable for TASKS_CRYPT {}
#[doc = "Start encryption/decryption. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "Stop encryption/decryption\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop encryption/decryption"]
pub mod tasks_stop;
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_rateoverride](tasks_rateoverride) module"]
pub type TASKS_RATEOVERRIDE = crate::Reg<u32, _TASKS_RATEOVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RATEOVERRIDE;
#[doc = "`write(|w| ..)` method takes [tasks_rateoverride::W](tasks_rateoverride::W) writer structure"]
impl crate::Writable for TASKS_RATEOVERRIDE {}
#[doc = "Override DATARATE setting in MODE register with the contents of the RATEOVERRIDE register for any ongoing encryption/decryption"]
pub mod tasks_rateoverride;
#[doc = "Subscribe configuration for task KSGEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_ksgen](subscribe_ksgen) module"]
pub type SUBSCRIBE_KSGEN = crate::Reg<u32, _SUBSCRIBE_KSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_KSGEN;
#[doc = "`read()` method returns [subscribe_ksgen::R](subscribe_ksgen::R) reader structure"]
impl crate::Readable for SUBSCRIBE_KSGEN {}
#[doc = "`write(|w| ..)` method takes [subscribe_ksgen::W](subscribe_ksgen::W) writer structure"]
impl crate::Writable for SUBSCRIBE_KSGEN {}
#[doc = "Subscribe configuration for task KSGEN"]
pub mod subscribe_ksgen;
#[doc = "Subscribe configuration for task CRYPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_crypt](subscribe_crypt) module"]
pub type SUBSCRIBE_CRYPT = crate::Reg<u32, _SUBSCRIBE_CRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_CRYPT;
#[doc = "`read()` method returns [subscribe_crypt::R](subscribe_crypt::R) reader structure"]
impl crate::Readable for SUBSCRIBE_CRYPT {}
#[doc = "`write(|w| ..)` method takes [subscribe_crypt::W](subscribe_crypt::W) writer structure"]
impl crate::Writable for SUBSCRIBE_CRYPT {}
#[doc = "Subscribe configuration for task CRYPT"]
pub mod subscribe_crypt;
#[doc = "Subscribe configuration for task STOP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stop](subscribe_stop) module"]
pub type SUBSCRIBE_STOP = crate::Reg<u32, _SUBSCRIBE_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOP;
#[doc = "`read()` method returns [subscribe_stop::R](subscribe_stop::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOP {}
#[doc = "`write(|w| ..)` method takes [subscribe_stop::W](subscribe_stop::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOP {}
#[doc = "Subscribe configuration for task STOP"]
pub mod subscribe_stop;
#[doc = "Subscribe configuration for task RATEOVERRIDE\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_rateoverride](subscribe_rateoverride) module"]
pub type SUBSCRIBE_RATEOVERRIDE = crate::Reg<u32, _SUBSCRIBE_RATEOVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_RATEOVERRIDE;
#[doc = "`read()` method returns [subscribe_rateoverride::R](subscribe_rateoverride::R) reader structure"]
impl crate::Readable for SUBSCRIBE_RATEOVERRIDE {}
#[doc = "`write(|w| ..)` method takes [subscribe_rateoverride::W](subscribe_rateoverride::W) writer structure"]
impl crate::Writable for SUBSCRIBE_RATEOVERRIDE {}
#[doc = "Subscribe configuration for task RATEOVERRIDE"]
pub mod subscribe_rateoverride;
#[doc = "Keystream generation complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endksgen](events_endksgen) module"]
pub type EVENTS_ENDKSGEN = crate::Reg<u32, _EVENTS_ENDKSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDKSGEN;
#[doc = "`read()` method returns [events_endksgen::R](events_endksgen::R) reader structure"]
impl crate::Readable for EVENTS_ENDKSGEN {}
#[doc = "`write(|w| ..)` method takes [events_endksgen::W](events_endksgen::W) writer structure"]
impl crate::Writable for EVENTS_ENDKSGEN {}
#[doc = "Keystream generation complete"]
pub mod events_endksgen;
#[doc = "Encrypt/decrypt complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endcrypt](events_endcrypt) module"]
pub type EVENTS_ENDCRYPT = crate::Reg<u32, _EVENTS_ENDCRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDCRYPT;
#[doc = "`read()` method returns [events_endcrypt::R](events_endcrypt::R) reader structure"]
impl crate::Readable for EVENTS_ENDCRYPT {}
#[doc = "`write(|w| ..)` method takes [events_endcrypt::W](events_endcrypt::W) writer structure"]
impl crate::Writable for EVENTS_ENDCRYPT {}
#[doc = "Encrypt/decrypt complete"]
pub mod events_endcrypt;
#[doc = "Deprecated register - CCM error event\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "Deprecated register - CCM error event"]
pub mod events_error;
#[doc = "Publish configuration for event ENDKSGEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endksgen](publish_endksgen) module"]
pub type PUBLISH_ENDKSGEN = crate::Reg<u32, _PUBLISH_ENDKSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDKSGEN;
#[doc = "`read()` method returns [publish_endksgen::R](publish_endksgen::R) reader structure"]
impl crate::Readable for PUBLISH_ENDKSGEN {}
#[doc = "`write(|w| ..)` method takes [publish_endksgen::W](publish_endksgen::W) writer structure"]
impl crate::Writable for PUBLISH_ENDKSGEN {}
#[doc = "Publish configuration for event ENDKSGEN"]
pub mod publish_endksgen;
#[doc = "Publish configuration for event ENDCRYPT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endcrypt](publish_endcrypt) module"]
pub type PUBLISH_ENDCRYPT = crate::Reg<u32, _PUBLISH_ENDCRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDCRYPT;
#[doc = "`read()` method returns [publish_endcrypt::R](publish_endcrypt::R) reader structure"]
impl crate::Readable for PUBLISH_ENDCRYPT {}
#[doc = "`write(|w| ..)` method takes [publish_endcrypt::W](publish_endcrypt::W) writer structure"]
impl crate::Writable for PUBLISH_ENDCRYPT {}
#[doc = "Publish configuration for event ENDCRYPT"]
pub mod publish_endcrypt;
#[doc = "Deprecated register - Publish configuration for event ERROR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_error](publish_error) module"]
pub type PUBLISH_ERROR = crate::Reg<u32, _PUBLISH_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ERROR;
#[doc = "`read()` method returns [publish_error::R](publish_error::R) reader structure"]
impl crate::Readable for PUBLISH_ERROR {}
#[doc = "`write(|w| ..)` method takes [publish_error::W](publish_error::W) writer structure"]
impl crate::Writable for PUBLISH_ERROR {}
#[doc = "Deprecated register - Publish configuration for event ERROR"]
pub mod publish_error;
#[doc = "Shortcuts between local events and tasks\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts between local events and tasks"]
pub mod shorts;
#[doc = "Enable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "MIC check result\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [micstatus](micstatus) module"]
pub type MICSTATUS = crate::Reg<u32, _MICSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICSTATUS;
#[doc = "`read()` method returns [micstatus::R](micstatus::R) reader structure"]
impl crate::Readable for MICSTATUS {}
#[doc = "MIC check result"]
pub mod micstatus;
#[doc = "Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable"]
pub mod enable;
#[doc = "Operation mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Operation mode"]
pub mod mode;
#[doc = "Pointer to data structure holding the AES key and the NONCE vector\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnfptr](cnfptr) module"]
pub type CNFPTR = crate::Reg<u32, _CNFPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNFPTR;
#[doc = "`read()` method returns [cnfptr::R](cnfptr::R) reader structure"]
impl crate::Readable for CNFPTR {}
#[doc = "`write(|w| ..)` method takes [cnfptr::W](cnfptr::W) writer structure"]
impl crate::Writable for CNFPTR {}
#[doc = "Pointer to data structure holding the AES key and the NONCE vector"]
pub mod cnfptr;
#[doc = "Input pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inptr](inptr) module"]
pub type INPTR = crate::Reg<u32, _INPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPTR;
#[doc = "`read()` method returns [inptr::R](inptr::R) reader structure"]
impl crate::Readable for INPTR {}
#[doc = "`write(|w| ..)` method takes [inptr::W](inptr::W) writer structure"]
impl crate::Writable for INPTR {}
#[doc = "Input pointer"]
pub mod inptr;
#[doc = "Output pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outptr](outptr) module"]
pub type OUTPTR = crate::Reg<u32, _OUTPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPTR;
#[doc = "`read()` method returns [outptr::R](outptr::R) reader structure"]
impl crate::Readable for OUTPTR {}
#[doc = "`write(|w| ..)` method takes [outptr::W](outptr::W) writer structure"]
impl crate::Writable for OUTPTR {}
#[doc = "Output pointer"]
pub mod outptr;
#[doc = "Pointer to data area used for temporary storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scratchptr](scratchptr) module"]
pub type SCRATCHPTR = crate::Reg<u32, _SCRATCHPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCHPTR;
#[doc = "`read()` method returns [scratchptr::R](scratchptr::R) reader structure"]
impl crate::Readable for SCRATCHPTR {}
#[doc = "`write(|w| ..)` method takes [scratchptr::W](scratchptr::W) writer structure"]
impl crate::Writable for SCRATCHPTR {}
#[doc = "Pointer to data area used for temporary storage"]
pub mod scratchptr;
#[doc = "Length of keystream generated when MODE.LENGTH = Extended\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxpacketsize](maxpacketsize) module"]
pub type MAXPACKETSIZE = crate::Reg<u32, _MAXPACKETSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXPACKETSIZE;
#[doc = "`read()` method returns [maxpacketsize::R](maxpacketsize::R) reader structure"]
impl crate::Readable for MAXPACKETSIZE {}
#[doc = "`write(|w| ..)` method takes [maxpacketsize::W](maxpacketsize::W) writer structure"]
impl crate::Writable for MAXPACKETSIZE {}
#[doc = "Length of keystream generated when MODE.LENGTH = Extended"]
pub mod maxpacketsize;
#[doc = "Data rate override setting.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rateoverride](rateoverride) module"]
pub type RATEOVERRIDE = crate::Reg<u32, _RATEOVERRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RATEOVERRIDE;
#[doc = "`read()` method returns [rateoverride::R](rateoverride::R) reader structure"]
impl crate::Readable for RATEOVERRIDE {}
#[doc = "`write(|w| ..)` method takes [rateoverride::W](rateoverride::W) writer structure"]
impl crate::Writable for RATEOVERRIDE {}
#[doc = "Data rate override setting."]
pub mod rateoverride;
#[doc = "Header (S0) mask.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [headermask](headermask) module"]
pub type HEADERMASK = crate::Reg<u32, _HEADERMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HEADERMASK;
#[doc = "`read()` method returns [headermask::R](headermask::R) reader structure"]
impl crate::Readable for HEADERMASK {}
#[doc = "`write(|w| ..)` method takes [headermask::W](headermask::W) writer structure"]
impl crate::Writable for HEADERMASK {}
#[doc = "Header (S0) mask."]
pub mod headermask;
