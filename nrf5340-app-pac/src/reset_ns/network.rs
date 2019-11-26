#[doc = "Force off power and clock in Network core\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [forceoff](forceoff) module"]
pub type FORCEOFF = crate::Reg<u32, _FORCEOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FORCEOFF;
#[doc = "`read()` method returns [forceoff::R](forceoff::R) reader structure"]
impl crate::Readable for FORCEOFF {}
#[doc = "`write(|w| ..)` method takes [forceoff::W](forceoff::W) writer structure"]
impl crate::Writable for FORCEOFF {}
#[doc = "Force off power and clock in Network core"]
pub mod forceoff;
