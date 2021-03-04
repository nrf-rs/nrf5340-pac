#[doc = "Register `IR[%s]` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IR_SPEC>> for R {
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IR` reader - Identity Root, word n"]
pub struct IR_R(crate::FieldReader<u32, u32>);
impl IR_R {
    pub(crate) fn new(bits: u32) -> Self {
        IR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Identity Root, word n"]
    #[inline(always)]
    pub fn ir(&self) -> IR_R {
        IR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Description collection: Identity Root, word n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IR[%s]
to value 0xffff_ffff"]
impl crate::Resettable for IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
