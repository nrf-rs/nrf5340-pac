#[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [protect](protect) module"]
pub type PROTECT = crate::Reg<u32, _PROTECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PROTECT;
#[doc = "`read()` method returns [protect::R](protect::R) reader structure"]
impl crate::Readable for PROTECT {}
#[doc = "`write(|w| ..)` method takes [protect::W](protect::W) writer structure"]
impl crate::Writable for PROTECT {}
#[doc = "Description cluster: Control access from master connected to AMLI master port EXTCODE\\[n\\]"]
pub mod protect;
