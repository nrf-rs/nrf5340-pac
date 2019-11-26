#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - Voltage supply detected on VBUS"]
    pub events_usbdetected: EVENTS_USBDETECTED,
    #[doc = "0x104 - Voltage supply removed from VBUS"]
    pub events_usbremoved: EVENTS_USBREMOVED,
    #[doc = "0x108 - USB 3.3 V supply ready"]
    pub events_usbpwrrdy: EVENTS_USBPWRRDY,
    _reserved3: [u8; 116usize],
    #[doc = "0x180 - Publish configuration for event USBDETECTED"]
    pub publish_usbdetected: PUBLISH_USBDETECTED,
    #[doc = "0x184 - Publish configuration for event USBREMOVED"]
    pub publish_usbremoved: PUBLISH_USBREMOVED,
    #[doc = "0x188 - Publish configuration for event USBPWRRDY"]
    pub publish_usbpwrrdy: PUBLISH_USBPWRRDY,
    _reserved6: [u8; 372usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 244usize],
    #[doc = "0x400 - USB supply status"]
    pub usbregstatus: USBREGSTATUS,
}
#[doc = "Voltage supply detected on VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_usbdetected](events_usbdetected) module"]
pub type EVENTS_USBDETECTED = crate::Reg<u32, _EVENTS_USBDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBDETECTED;
#[doc = "`read()` method returns [events_usbdetected::R](events_usbdetected::R) reader structure"]
impl crate::Readable for EVENTS_USBDETECTED {}
#[doc = "`write(|w| ..)` method takes [events_usbdetected::W](events_usbdetected::W) writer structure"]
impl crate::Writable for EVENTS_USBDETECTED {}
#[doc = "Voltage supply detected on VBUS"]
pub mod events_usbdetected;
#[doc = "Voltage supply removed from VBUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_usbremoved](events_usbremoved) module"]
pub type EVENTS_USBREMOVED = crate::Reg<u32, _EVENTS_USBREMOVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBREMOVED;
#[doc = "`read()` method returns [events_usbremoved::R](events_usbremoved::R) reader structure"]
impl crate::Readable for EVENTS_USBREMOVED {}
#[doc = "`write(|w| ..)` method takes [events_usbremoved::W](events_usbremoved::W) writer structure"]
impl crate::Writable for EVENTS_USBREMOVED {}
#[doc = "Voltage supply removed from VBUS"]
pub mod events_usbremoved;
#[doc = "USB 3.3 V supply ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_usbpwrrdy](events_usbpwrrdy) module"]
pub type EVENTS_USBPWRRDY = crate::Reg<u32, _EVENTS_USBPWRRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_USBPWRRDY;
#[doc = "`read()` method returns [events_usbpwrrdy::R](events_usbpwrrdy::R) reader structure"]
impl crate::Readable for EVENTS_USBPWRRDY {}
#[doc = "`write(|w| ..)` method takes [events_usbpwrrdy::W](events_usbpwrrdy::W) writer structure"]
impl crate::Writable for EVENTS_USBPWRRDY {}
#[doc = "USB 3.3 V supply ready"]
pub mod events_usbpwrrdy;
#[doc = "Publish configuration for event USBDETECTED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_usbdetected](publish_usbdetected) module"]
pub type PUBLISH_USBDETECTED = crate::Reg<u32, _PUBLISH_USBDETECTED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_USBDETECTED;
#[doc = "`read()` method returns [publish_usbdetected::R](publish_usbdetected::R) reader structure"]
impl crate::Readable for PUBLISH_USBDETECTED {}
#[doc = "`write(|w| ..)` method takes [publish_usbdetected::W](publish_usbdetected::W) writer structure"]
impl crate::Writable for PUBLISH_USBDETECTED {}
#[doc = "Publish configuration for event USBDETECTED"]
pub mod publish_usbdetected;
#[doc = "Publish configuration for event USBREMOVED\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_usbremoved](publish_usbremoved) module"]
pub type PUBLISH_USBREMOVED = crate::Reg<u32, _PUBLISH_USBREMOVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_USBREMOVED;
#[doc = "`read()` method returns [publish_usbremoved::R](publish_usbremoved::R) reader structure"]
impl crate::Readable for PUBLISH_USBREMOVED {}
#[doc = "`write(|w| ..)` method takes [publish_usbremoved::W](publish_usbremoved::W) writer structure"]
impl crate::Writable for PUBLISH_USBREMOVED {}
#[doc = "Publish configuration for event USBREMOVED"]
pub mod publish_usbremoved;
#[doc = "Publish configuration for event USBPWRRDY\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [publish_usbpwrrdy](publish_usbpwrrdy) module"]
pub type PUBLISH_USBPWRRDY = crate::Reg<u32, _PUBLISH_USBPWRRDY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PUBLISH_USBPWRRDY;
#[doc = "`read()` method returns [publish_usbpwrrdy::R](publish_usbpwrrdy::R) reader structure"]
impl crate::Readable for PUBLISH_USBPWRRDY {}
#[doc = "`write(|w| ..)` method takes [publish_usbpwrrdy::W](publish_usbpwrrdy::W) writer structure"]
impl crate::Writable for PUBLISH_USBPWRRDY {}
#[doc = "Publish configuration for event USBPWRRDY"]
pub mod publish_usbpwrrdy;
#[doc = "Enable or disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "Enable or disable interrupt"]
pub mod inten;
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
#[doc = "USB supply status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [usbregstatus](usbregstatus) module"]
pub type USBREGSTATUS = crate::Reg<u32, _USBREGSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBREGSTATUS;
#[doc = "`read()` method returns [usbregstatus::R](usbregstatus::R) reader structure"]
impl crate::Readable for USBREGSTATUS {}
#[doc = "USB supply status"]
pub mod usbregstatus;
