#[doc = "Register `FLASH` reader"]
pub struct R(crate::R<FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FLASH_SPEC>> for R {
    fn from(reader: crate::R<FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Flash variant\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FLASH_A {
    #[doc = "128: 128 kByte FLASH"]
    K128 = 128,
    #[doc = "256: 256 kByte FLASH"]
    K256 = 256,
    #[doc = "512: 512 kByte FLASH"]
    K512 = 512,
    #[doc = "1024: 1 MByte FLASH"]
    K1024 = 1024,
    #[doc = "2048: 2 MByte FLASH"]
    K2048 = 2048,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<FLASH_A> for u32 {
    #[inline(always)]
    fn from(variant: FLASH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FLASH` reader - Flash variant"]
pub struct FLASH_R(crate::FieldReader<u32, FLASH_A>);
impl FLASH_R {
    pub(crate) fn new(bits: u32) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FLASH_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(FLASH_A::K128),
            256 => Val(FLASH_A::K256),
            512 => Val(FLASH_A::K512),
            1024 => Val(FLASH_A::K1024),
            2048 => Val(FLASH_A::K2048),
            4294967295 => Val(FLASH_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K128`"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        **self == FLASH_A::K128
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        **self == FLASH_A::K256
    }
    #[doc = "Checks if the value of the field is `K512`"]
    #[inline(always)]
    pub fn is_k512(&self) -> bool {
        **self == FLASH_A::K512
    }
    #[doc = "Checks if the value of the field is `K1024`"]
    #[inline(always)]
    pub fn is_k1024(&self) -> bool {
        **self == FLASH_A::K1024
    }
    #[doc = "Checks if the value of the field is `K2048`"]
    #[inline(always)]
    pub fn is_k2048(&self) -> bool {
        **self == FLASH_A::K2048
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == FLASH_A::UNSPECIFIED
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<u32, FLASH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Flash variant"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Flash variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash](index.html) module"]
pub struct FLASH_SPEC;
impl crate::RegisterSpec for FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash::R](R) reader structure"]
impl crate::Readable for FLASH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FLASH to value 0xffff_ffff"]
impl crate::Resettable for FLASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
