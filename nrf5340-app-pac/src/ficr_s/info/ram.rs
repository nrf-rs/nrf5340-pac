#[doc = "Register `RAM` reader"]
pub struct R(crate::R<RAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RAM_SPEC>> for R {
    fn from(reader: crate::R<RAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "RAM variant\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum RAM_A {
    #[doc = "16: 16 kByte RAM"]
    K16 = 16,
    #[doc = "32: 32 kByte RAM"]
    K32 = 32,
    #[doc = "64: 64 kByte RAM"]
    K64 = 64,
    #[doc = "128: 128 kByte RAM"]
    K128 = 128,
    #[doc = "256: 256 kByte RAM"]
    K256 = 256,
    #[doc = "512: 512 kByte RAM"]
    K512 = 512,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED = 4294967295,
}
impl From<RAM_A> for u32 {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RAM` reader - RAM variant"]
pub struct RAM_R(crate::FieldReader<u32, RAM_A>);
impl RAM_R {
    pub(crate) fn new(bits: u32) -> Self {
        RAM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, RAM_A> {
        use crate::Variant::*;
        match self.bits {
            16 => Val(RAM_A::K16),
            32 => Val(RAM_A::K32),
            64 => Val(RAM_A::K64),
            128 => Val(RAM_A::K128),
            256 => Val(RAM_A::K256),
            512 => Val(RAM_A::K512),
            4294967295 => Val(RAM_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K16`"]
    #[inline(always)]
    pub fn is_k16(&self) -> bool {
        **self == RAM_A::K16
    }
    #[doc = "Checks if the value of the field is `K32`"]
    #[inline(always)]
    pub fn is_k32(&self) -> bool {
        **self == RAM_A::K32
    }
    #[doc = "Checks if the value of the field is `K64`"]
    #[inline(always)]
    pub fn is_k64(&self) -> bool {
        **self == RAM_A::K64
    }
    #[doc = "Checks if the value of the field is `K128`"]
    #[inline(always)]
    pub fn is_k128(&self) -> bool {
        **self == RAM_A::K128
    }
    #[doc = "Checks if the value of the field is `K256`"]
    #[inline(always)]
    pub fn is_k256(&self) -> bool {
        **self == RAM_A::K256
    }
    #[doc = "Checks if the value of the field is `K512`"]
    #[inline(always)]
    pub fn is_k512(&self) -> bool {
        **self == RAM_A::K512
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        **self == RAM_A::UNSPECIFIED
    }
}
impl core::ops::Deref for RAM_R {
    type Target = crate::FieldReader<u32, RAM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - RAM variant"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "RAM variant\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ram](index.html) module"]
pub struct RAM_SPEC;
impl crate::RegisterSpec for RAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ram::R](R) reader structure"]
impl crate::Readable for RAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAM to value 0xffff_ffff"]
impl crate::Resettable for RAM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
