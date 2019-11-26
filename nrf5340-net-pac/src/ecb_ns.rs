#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt"]
    pub tasks_startecb: TASKS_STARTECB,
    #[doc = "0x04 - Abort a possible executing ECB operation"]
    pub tasks_stopecb: TASKS_STOPECB,
    _reserved2: [u8; 120usize],
    #[doc = "0x80 - Subscribe configuration for task STARTECB"]
    pub subscribe_startecb: SUBSCRIBE_STARTECB,
    #[doc = "0x84 - Subscribe configuration for task STOPECB"]
    pub subscribe_stopecb: SUBSCRIBE_STOPECB,
    _reserved4: [u8; 120usize],
    #[doc = "0x100 - ECB block encrypt complete"]
    pub events_endecb: EVENTS_ENDECB,
    #[doc = "0x104 - ECB block encrypt aborted because of a STOPECB task or due to an error"]
    pub events_errorecb: EVENTS_ERRORECB,
    _reserved6: [u8; 120usize],
    #[doc = "0x180 - Publish configuration for event ENDECB"]
    pub publish_endecb: PUBLISH_ENDECB,
    #[doc = "0x184 - Publish configuration for event ERRORECB"]
    pub publish_errorecb: PUBLISH_ERRORECB,
    _reserved8: [u8; 380usize],
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 504usize],
    #[doc = "0x504 - ECB block encrypt memory pointers"]
    pub ecbdataptr: ECBDATAPTR,
}
#[doc = "Start ECB block encrypt\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_startecb](tasks_startecb) module"]
pub type TASKS_STARTECB = crate::Reg<u32, _TASKS_STARTECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTECB;
#[doc = "`write(|w| ..)` method takes [tasks_startecb::W](tasks_startecb::W) writer structure"]
impl crate::Writable for TASKS_STARTECB {}
#[doc = "Start ECB block encrypt"]
pub mod tasks_startecb;
#[doc = "Abort a possible executing ECB operation\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stopecb](tasks_stopecb) module"]
pub type TASKS_STOPECB = crate::Reg<u32, _TASKS_STOPECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPECB;
#[doc = "`write(|w| ..)` method takes [tasks_stopecb::W](tasks_stopecb::W) writer structure"]
impl crate::Writable for TASKS_STOPECB {}
#[doc = "Abort a possible executing ECB operation"]
pub mod tasks_stopecb;
#[doc = "Subscribe configuration for task STARTECB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_startecb](subscribe_startecb) module"]
pub type SUBSCRIBE_STARTECB = crate::Reg<u32, _SUBSCRIBE_STARTECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STARTECB;
#[doc = "`read()` method returns [subscribe_startecb::R](subscribe_startecb::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STARTECB {}
#[doc = "`write(|w| ..)` method takes [subscribe_startecb::W](subscribe_startecb::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STARTECB {}
#[doc = "Subscribe configuration for task STARTECB"]
pub mod subscribe_startecb;
#[doc = "Subscribe configuration for task STOPECB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [subscribe_stopecb](subscribe_stopecb) module"]
pub type SUBSCRIBE_STOPECB = crate::Reg<u32, _SUBSCRIBE_STOPECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SUBSCRIBE_STOPECB;
#[doc = "`read()` method returns [subscribe_stopecb::R](subscribe_stopecb::R) reader structure"]
impl crate::Readable for SUBSCRIBE_STOPECB {}
#[doc = "`write(|w| ..)` method takes [subscribe_stopecb::W](subscribe_stopecb::W) writer structure"]
impl crate::Writable for SUBSCRIBE_STOPECB {}
#[doc = "Subscribe configuration for task STOPECB"]
pub mod subscribe_stopecb;
#[doc = "ECB block encrypt complete\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endecb](events_endecb) module"]
pub type EVENTS_ENDECB = crate::Reg<u32, _EVENTS_ENDECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDECB;
#[doc = "`read()` method returns [events_endecb::R](events_endecb::R) reader structure"]
impl crate::Readable for EVENTS_ENDECB {}
#[doc = "`write(|w| ..)` method takes [events_endecb::W](events_endecb::W) writer structure"]
impl crate::Writable for EVENTS_ENDECB {}
#[doc = "ECB block encrypt complete"]
pub mod events_endecb;
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_errorecb](events_errorecb) module"]
pub type EVENTS_ERRORECB = crate::Reg<u32, _EVENTS_ERRORECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERRORECB;
#[doc = "`read()` method returns [events_errorecb::R](events_errorecb::R) reader structure"]
impl crate::Readable for EVENTS_ERRORECB {}
#[doc = "`write(|w| ..)` method takes [events_errorecb::W](events_errorecb::W) writer structure"]
impl crate::Writable for EVENTS_ERRORECB {}
#[doc = "ECB block encrypt aborted because of a STOPECB task or due to an error"]
pub mod events_errorecb;
#[doc = "Publish configuration for event ENDECB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_endecb](publish_endecb) module"]
pub type PUBLISH_ENDECB = crate::Reg<u32, _PUBLISH_ENDECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ENDECB;
#[doc = "`read()` method returns [publish_endecb::R](publish_endecb::R) reader structure"]
impl crate::Readable for PUBLISH_ENDECB {}
#[doc = "`write(|w| ..)` method takes [publish_endecb::W](publish_endecb::W) writer structure"]
impl crate::Writable for PUBLISH_ENDECB {}
#[doc = "Publish configuration for event ENDECB"]
pub mod publish_endecb;
#[doc = "Publish configuration for event ERRORECB\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_errorecb](publish_errorecb) module"]
pub type PUBLISH_ERRORECB = crate::Reg<u32, _PUBLISH_ERRORECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_ERRORECB;
#[doc = "`read()` method returns [publish_errorecb::R](publish_errorecb::R) reader structure"]
impl crate::Readable for PUBLISH_ERRORECB {}
#[doc = "`write(|w| ..)` method takes [publish_errorecb::W](publish_errorecb::W) writer structure"]
impl crate::Writable for PUBLISH_ERRORECB {}
#[doc = "Publish configuration for event ERRORECB"]
pub mod publish_errorecb;
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
#[doc = "ECB block encrypt memory pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ecbdataptr](ecbdataptr) module"]
pub type ECBDATAPTR = crate::Reg<u32, _ECBDATAPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECBDATAPTR;
#[doc = "`read()` method returns [ecbdataptr::R](ecbdataptr::R) reader structure"]
impl crate::Readable for ECBDATAPTR {}
#[doc = "`write(|w| ..)` method takes [ecbdataptr::W](ecbdataptr::W) writer structure"]
impl crate::Writable for ECBDATAPTR {}
#[doc = "ECB block encrypt memory pointers"]
pub mod ecbdataptr;
