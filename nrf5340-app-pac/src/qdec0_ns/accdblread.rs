#[doc = "Register `ACCDBLREAD` reader"]
pub struct R(crate::R<ACCDBLREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACCDBLREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACCDBLREAD_SPEC>> for R {
    fn from(reader: crate::R<ACCDBLREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACCDBLREAD` reader - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
pub struct ACCDBLREAD_R(crate::FieldReader<u8, u8>);
impl ACCDBLREAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACCDBLREAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACCDBLREAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Snapshot of the ACCDBL register. This field is updated when the READCLRACC or RDCLRDBL task is triggered."]
    #[inline(always)]
    pub fn accdblread(&self) -> ACCDBLREAD_R {
        ACCDBLREAD_R::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "Snapshot of the ACCDBL, updated by the READCLRACC or RDCLRDBL task\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [accdblread](index.html) module"]
pub struct ACCDBLREAD_SPEC;
impl crate::RegisterSpec for ACCDBLREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [accdblread::R](R) reader structure"]
impl crate::Readable for ACCDBLREAD_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ACCDBLREAD to value 0"]
impl crate::Resettable for ACCDBLREAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
