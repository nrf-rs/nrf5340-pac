#[doc = "Register `DHIT` reader"]
pub struct R(crate::R<DHIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DHIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DHIT_SPEC>> for R {
    fn from(reader: crate::R<DHIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HITS` reader - Number of data cache hits"]
pub struct HITS_R(crate::FieldReader<u32, u32>);
impl HITS_R {
    pub(crate) fn new(bits: u32) -> Self {
        HITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of data cache hits"]
    #[inline(always)]
    pub fn hits(&self) -> HITS_R {
        HITS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Description cluster: Data fetch cache hit counter for cache region n, where n=0 means Flash and n=1 means XIP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dhit](index.html) module"]
pub struct DHIT_SPEC;
impl crate::RegisterSpec for DHIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dhit::R](R) reader structure"]
impl crate::Readable for DHIT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DHIT to value 0"]
impl crate::Resettable for DHIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
