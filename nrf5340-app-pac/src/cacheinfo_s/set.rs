#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\].\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [way](way) module"]
pub type WAY = crate::Reg<u32, _WAY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAY;
#[doc = "`read()` method returns [way::R](way::R) reader structure"]
impl crate::Readable for WAY {}
#[doc = "`write(|w| ..)` method takes [way::W](way::W) writer structure"]
impl crate::Writable for WAY {}
#[doc = "Description collection: Cache information for SET\\[n\\], WAY\\[o\\]."]
pub mod way;
