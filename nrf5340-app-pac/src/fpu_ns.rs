#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 256usize],
    #[doc = "0x100 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    pub events_invalidoperation: EVENTS_INVALIDOPERATION,
    #[doc = "0x104 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    pub events_dividebyzero: EVENTS_DIVIDEBYZERO,
    #[doc = "0x108 - An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
    pub events_overflow: EVENTS_OVERFLOW,
    #[doc = "0x10c - An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
    pub events_underflow: EVENTS_UNDERFLOW,
    #[doc = "0x110 - An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
    pub events_inexact: EVENTS_INEXACT,
    #[doc = "0x114 - An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
    pub events_denormalinput: EVENTS_DENORMALINPUT,
    _reserved6: [u8; 488usize],
    #[doc = "0x300 - Enable or disable interrupt"]
    pub inten: INTEN,
    #[doc = "0x304 - Enable interrupt"]
    pub intenset: INTENSET,
    #[doc = "0x308 - Disable interrupt"]
    pub intenclr: INTENCLR,
}
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_invalidoperation](events_invalidoperation) module"]
pub type EVENTS_INVALIDOPERATION = crate::Reg<u32, _EVENTS_INVALIDOPERATION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_INVALIDOPERATION;
#[doc = "`read()` method returns [events_invalidoperation::R](events_invalidoperation::R) reader structure"]
impl crate::Readable for EVENTS_INVALIDOPERATION {}
#[doc = "`write(|w| ..)` method takes [events_invalidoperation::W](events_invalidoperation::W) writer structure"]
impl crate::Writable for EVENTS_INVALIDOPERATION {}
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
pub mod events_invalidoperation;
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_dividebyzero](events_dividebyzero) module"]
pub type EVENTS_DIVIDEBYZERO = crate::Reg<u32, _EVENTS_DIVIDEBYZERO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DIVIDEBYZERO;
#[doc = "`read()` method returns [events_dividebyzero::R](events_dividebyzero::R) reader structure"]
impl crate::Readable for EVENTS_DIVIDEBYZERO {}
#[doc = "`write(|w| ..)` method takes [events_dividebyzero::W](events_dividebyzero::W) writer structure"]
impl crate::Writable for EVENTS_DIVIDEBYZERO {}
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
pub mod events_dividebyzero;
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_overflow](events_overflow) module"]
pub type EVENTS_OVERFLOW = crate::Reg<u32, _EVENTS_OVERFLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_OVERFLOW;
#[doc = "`read()` method returns [events_overflow::R](events_overflow::R) reader structure"]
impl crate::Readable for EVENTS_OVERFLOW {}
#[doc = "`write(|w| ..)` method takes [events_overflow::W](events_overflow::W) writer structure"]
impl crate::Writable for EVENTS_OVERFLOW {}
#[doc = "An FPUOFC exception triggered by a floating-point overflow has occurred in the FPU"]
pub mod events_overflow;
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_underflow](events_underflow) module"]
pub type EVENTS_UNDERFLOW = crate::Reg<u32, _EVENTS_UNDERFLOW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_UNDERFLOW;
#[doc = "`read()` method returns [events_underflow::R](events_underflow::R) reader structure"]
impl crate::Readable for EVENTS_UNDERFLOW {}
#[doc = "`write(|w| ..)` method takes [events_underflow::W](events_underflow::W) writer structure"]
impl crate::Writable for EVENTS_UNDERFLOW {}
#[doc = "An FPUUFC exception triggered by a floating-point underflow has occurred in the FPU"]
pub mod events_underflow;
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_inexact](events_inexact) module"]
pub type EVENTS_INEXACT = crate::Reg<u32, _EVENTS_INEXACT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_INEXACT;
#[doc = "`read()` method returns [events_inexact::R](events_inexact::R) reader structure"]
impl crate::Readable for EVENTS_INEXACT {}
#[doc = "`write(|w| ..)` method takes [events_inexact::W](events_inexact::W) writer structure"]
impl crate::Writable for EVENTS_INEXACT {}
#[doc = "An FPUIXC exception triggered by an inexact floating-point operation has occurred in the FPU"]
pub mod events_inexact;
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_denormalinput](events_denormalinput) module"]
pub type EVENTS_DENORMALINPUT = crate::Reg<u32, _EVENTS_DENORMALINPUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DENORMALINPUT;
#[doc = "`read()` method returns [events_denormalinput::R](events_denormalinput::R) reader structure"]
impl crate::Readable for EVENTS_DENORMALINPUT {}
#[doc = "`write(|w| ..)` method takes [events_denormalinput::W](events_denormalinput::W) writer structure"]
impl crate::Writable for EVENTS_DENORMALINPUT {}
#[doc = "An FPUIDC exception triggered by a denormal floating-point input has occurred in the FPU"]
pub mod events_denormalinput;
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
