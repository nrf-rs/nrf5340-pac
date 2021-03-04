#[doc = "Register `CIDR0` reader"]
pub struct R(crate::R<CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CIDR0_SPEC>> for R {
    fn from(reader: crate::R<CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\]
of the component identification code.\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMBL_0_A {
    #[doc = "13: Bits\\[7:0\\]
of the identification code."]
    VALUE = 13,
}
impl From<PRMBL_0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRMBL_0` reader - Preamble\\[0\\]. Contains bits\\[7:0\\]
of the component identification code."]
pub struct PRMBL_0_R(crate::FieldReader<u8, PRMBL_0_A>);
impl PRMBL_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRMBL_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRMBL_0_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(PRMBL_0_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        **self == PRMBL_0_A::VALUE
    }
}
impl core::ops::Deref for PRMBL_0_R {
    type Target = crate::FieldReader<u8, PRMBL_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[0\\]. Contains bits\\[7:0\\]
of the component identification code."]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Component ID0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cidr0](index.html) module"]
pub struct CIDR0_SPEC;
impl crate::RegisterSpec for CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cidr0::R](R) reader structure"]
impl crate::Readable for CIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CIDR0 to value 0x0d"]
impl crate::Resettable for CIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
