#[doc = "Register `COUNTER` reader"]
pub struct R(crate::R<COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<COUNTER_SPEC>> for R {
    fn from(reader: crate::R<COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER` reader - Counter value"]
pub struct COUNTER_R(crate::FieldReader<u32, u32>);
impl COUNTER_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTER_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Counter value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Current counter value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](index.html) module"]
pub struct COUNTER_SPEC;
impl crate::RegisterSpec for COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [counter::R](R) reader structure"]
impl crate::Readable for COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for COUNTER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
