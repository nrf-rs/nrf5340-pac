#[doc = "Bits 31:0 of DMA AES KEY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key0](key0) module"]
pub type KEY0 = crate::Reg<u32, _KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY0;
#[doc = "`write(|w| ..)` method takes [key0::W](key0::W) writer structure"]
impl crate::Writable for KEY0 {}
#[doc = "Bits 31:0 of DMA AES KEY"]
pub mod key0;
#[doc = "Bits 63:32 of DMA AES KEY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "Bits 63:32 of DMA AES KEY"]
pub mod key1;
#[doc = "Bits 95:64 of DMA AES KEY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "Bits 95:64 of DMA AES KEY"]
pub mod key2;
#[doc = "Bits 127:96 of DMA AES KEY\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key3](key3) module"]
pub type KEY3 = crate::Reg<u32, _KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY3;
#[doc = "`write(|w| ..)` method takes [key3::W](key3::W) writer structure"]
impl crate::Writable for KEY3 {}
#[doc = "Bits 127:96 of DMA AES KEY"]
pub mod key3;
#[doc = "Bits 31:0 of DMA NONCE\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonce0](nonce0) module"]
pub type NONCE0 = crate::Reg<u32, _NONCE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONCE0;
#[doc = "`write(|w| ..)` method takes [nonce0::W](nonce0::W) writer structure"]
impl crate::Writable for NONCE0 {}
#[doc = "Bits 31:0 of DMA NONCE"]
pub mod nonce0;
#[doc = "Bits 63:32 of DMA NONCE\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonce1](nonce1) module"]
pub type NONCE1 = crate::Reg<u32, _NONCE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONCE1;
#[doc = "`write(|w| ..)` method takes [nonce1::W](nonce1::W) writer structure"]
impl crate::Writable for NONCE1 {}
#[doc = "Bits 63:32 of DMA NONCE"]
pub mod nonce1;
#[doc = "Bits 95:64 of DMA NONCE\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nonce2](nonce2) module"]
pub type NONCE2 = crate::Reg<u32, _NONCE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NONCE2;
#[doc = "`write(|w| ..)` method takes [nonce2::W](nonce2::W) writer structure"]
impl crate::Writable for NONCE2 {}
#[doc = "Bits 95:64 of DMA NONCE"]
pub mod nonce2;
#[doc = "Enable stream cipher for XIP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable stream cipher for XIP"]
pub mod enable;
