#[doc = "Register `AMOUNT` reader"]
pub struct R(crate::R<AMOUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMOUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<AMOUNT_SPEC>> for R {
    fn from(reader: crate::R<AMOUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AMOUNT` reader - Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
pub struct AMOUNT_R(crate::FieldReader<u16, u16>);
impl AMOUNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        AMOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMOUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:14 - Number of buffer words transferred since last START. This register can be read after an END or STOPPED event."]
    #[inline(always)]
    pub fn amount(&self) -> AMOUNT_R {
        AMOUNT_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "Number of buffer words transferred since last START\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amount](index.html) module"]
pub struct AMOUNT_SPEC;
impl crate::RegisterSpec for AMOUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amount::R](R) reader structure"]
impl crate::Readable for AMOUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AMOUNT to value 0"]
impl crate::Resettable for AMOUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
