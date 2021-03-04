#[doc = "Register `APCUTOFF` reader"]
pub struct R(crate::R<APCUTOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APCUTOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<APCUTOFF_SPEC>> for R {
    fn from(reader: crate::R<APCUTOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APCUTOFF` reader - Adaptive proportion cutoff"]
pub struct APCUTOFF_R(crate::FieldReader<u32, u32>);
impl APCUTOFF_R {
    pub(crate) fn new(bits: u32) -> Self {
        APCUTOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APCUTOFF_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Adaptive proportion cutoff"]
    #[inline(always)]
    pub fn apcutoff(&self) -> APCUTOFF_R {
        APCUTOFF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Adaptive proportion cutoff\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apcutoff](index.html) module"]
pub struct APCUTOFF_SPEC;
impl crate::RegisterSpec for APCUTOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apcutoff::R](R) reader structure"]
impl crate::Readable for APCUTOFF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APCUTOFF to value 0xffff_ffff"]
impl crate::Resettable for APCUTOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
