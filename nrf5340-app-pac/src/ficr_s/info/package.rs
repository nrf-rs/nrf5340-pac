#[doc = "Register `PACKAGE` reader"]
pub struct R(crate::R<PACKAGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACKAGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PACKAGE_SPEC>> for R {
    fn from(reader: crate::R<PACKAGE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Package option\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PACKAGE_A {
    #[doc = "8192: QKxx - 94-pin aQFN"]
    QK = 8192,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<PACKAGE_A> for u32 {
    #[inline(always)]
    fn from(variant: PACKAGE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PACKAGE` reader - Package option"]
pub struct PACKAGE_R(crate::FieldReader<u32, PACKAGE_A>);
impl PACKAGE_R {
    pub(crate) fn new(bits: u32) -> Self {
        PACKAGE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PACKAGE_A> {
        use crate::Variant::*;
        match self.bits {
            8192 => Val(PACKAGE_A::QK),
            4294967295 => Val(PACKAGE_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `QK`"]
    #[inline(always)]
    pub fn is_qk(&self) -> bool {
        **self == PACKAGE_A::QK
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == PACKAGE_A::UNSPECIFIED
    }
}
impl core::ops::Deref for PACKAGE_R {
    type Target = crate::FieldReader<u32, PACKAGE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Package option"]
    #[inline(always)]
    pub fn package(&self) -> PACKAGE_R {
        PACKAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Package option\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [package](index.html) module"]
pub struct PACKAGE_SPEC;
impl crate::RegisterSpec for PACKAGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [package::R](R) reader structure"]
impl crate::Readable for PACKAGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PACKAGE to value 0xffff_ffff"]
impl crate::Resettable for PACKAGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
