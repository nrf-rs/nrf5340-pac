#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bypass](bypass) module"]
pub type BYPASS = crate::Reg<u32, _BYPASS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BYPASS;
#[doc = "`read()` method returns [bypass::R](bypass::R) reader structure"]
impl crate::Readable for BYPASS {}
#[doc = "`write(|w| ..)` method takes [bypass::W](bypass::W) writer structure"]
impl crate::Writable for BYPASS {}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
pub mod bypass;
#[doc = "Control usage of internal load capacitors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intcap](intcap) module"]
pub type INTCAP = crate::Reg<u32, _INTCAP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTCAP;
#[doc = "`read()` method returns [intcap::R](intcap::R) reader structure"]
impl crate::Readable for INTCAP {}
#[doc = "`write(|w| ..)` method takes [intcap::W](intcap::W) writer structure"]
impl crate::Writable for INTCAP {}
#[doc = "Control usage of internal load capacitors"]
pub mod intcap;
