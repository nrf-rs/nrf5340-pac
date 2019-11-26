#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vreqh](vreqh) module"]
pub type VREQH = crate::Reg<u32, _VREQH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREQH;
#[doc = "`read()` method returns [vreqh::R](vreqh::R) reader structure"]
impl crate::Readable for VREQH {}
#[doc = "`write(|w| ..)` method takes [vreqh::W](vreqh::W) writer structure"]
impl crate::Writable for VREQH {}
#[doc = "Request high voltage on RADIO After requesting high voltage, the user must wait until VREQHREADY is set to Ready"]
pub mod vreqh;
#[doc = "High voltage on RADIO is ready\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [vreqhready](vreqhready) module"]
pub type VREQHREADY = crate::Reg<u32, _VREQHREADY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VREQHREADY;
#[doc = "`read()` method returns [vreqhready::R](vreqhready::R) reader structure"]
impl crate::Readable for VREQHREADY {}
#[doc = "High voltage on RADIO is ready"]
pub mod vreqhready;
