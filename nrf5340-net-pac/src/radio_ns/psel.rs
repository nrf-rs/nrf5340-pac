#[doc = "Description collection: Pin select for DFE pin n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dfegpio](dfegpio) module"]
pub type DFEGPIO = crate::Reg<u32, _DFEGPIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFEGPIO;
#[doc = "`read()` method returns [dfegpio::R](dfegpio::R) reader structure"]
impl crate::Readable for DFEGPIO {}
#[doc = "`write(|w| ..)` method takes [dfegpio::W](dfegpio::W) writer structure"]
impl crate::Writable for DFEGPIO {}
#[doc = "Description collection: Pin select for DFE pin n"]
pub mod dfegpio;
