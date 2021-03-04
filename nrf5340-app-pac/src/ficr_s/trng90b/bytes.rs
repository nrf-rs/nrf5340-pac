#[doc = "Register `BYTES` reader"]
pub struct R(crate::R<BYTES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BYTES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BYTES_SPEC>> for R {
    fn from(reader: crate::R<BYTES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BYTES` reader - Amount of bytes for the required entropy bits"]
pub struct BYTES_R(crate::FieldReader<u32, u32>);
impl BYTES_R {
    pub(crate) fn new(bits: u32) -> Self {
        BYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Amount of bytes for the required entropy bits"]
    #[inline(always)]
    pub fn bytes(&self) -> BYTES_R {
        BYTES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Amount of bytes for the required entropy bits\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bytes](index.html) module"]
pub struct BYTES_SPEC;
impl crate::RegisterSpec for BYTES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bytes::R](R) reader structure"]
impl crate::Readable for BYTES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BYTES to value 0x0210"]
impl crate::Resettable for BYTES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0210
    }
}
