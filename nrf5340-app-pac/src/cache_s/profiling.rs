#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ihit](ihit) module"]
pub type IHIT = crate::Reg<u32, _IHIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IHIT;
#[doc = "`read()` method returns [ihit::R](ihit::R) reader structure"]
impl crate::Readable for IHIT {}
#[doc = "Description cluster: Instruction fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod ihit;
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imiss](imiss) module"]
pub type IMISS = crate::Reg<u32, _IMISS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMISS;
#[doc = "`read()` method returns [imiss::R](imiss::R) reader structure"]
impl crate::Readable for IMISS {}
#[doc = "Description cluster: Instruction fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod imiss;
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dhit](dhit) module"]
pub type DHIT = crate::Reg<u32, _DHIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DHIT;
#[doc = "`read()` method returns [dhit::R](dhit::R) reader structure"]
impl crate::Readable for DHIT {}
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dhit;
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmiss](dmiss) module"]
pub type DMISS = crate::Reg<u32, _DMISS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMISS;
#[doc = "`read()` method returns [dmiss::R](dmiss::R) reader structure"]
impl crate::Readable for DMISS {}
#[doc = "Description cluster: Data fetch cache miss counter for cache region n, where n=0 means Flash and n=1 means XIP."]
pub mod dmiss;
