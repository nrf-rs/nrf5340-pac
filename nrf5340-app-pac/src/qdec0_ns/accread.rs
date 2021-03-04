#[doc = "Register `ACCREAD` reader"]
pub struct R(crate::R<ACCREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACCREAD_SPEC>> for R {
    fn from(reader: crate::R<ACCREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCREAD` reader - Snapshot of the ACC register."]
pub struct ACCREAD_R(crate::FieldReader<u32, u32>);
impl ACCREAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACCREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCREAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Snapshot of the ACC register."]
    #[inline(always)]
    pub fn accread(&self) -> ACCREAD_R {
        ACCREAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Snapshot of the ACC register, updated by the READCLRACC or RDCLRACC task\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accread](index.html) module"]
pub struct ACCREAD_SPEC;
impl crate::RegisterSpec for ACCREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accread::R](R) reader structure"]
impl crate::Readable for ACCREAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCREAD to value 0"]
impl crate::Resettable for ACCREAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
