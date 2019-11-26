#[doc = "Reader of register SWIDTH"]
pub type R = crate::R<u32, super::SWIDTH>;
#[doc = "Writer for register SWIDTH"]
pub type W = crate::W<u32, super::SWIDTH>;
#[doc = "Register SWIDTH `reset()`'s with value 0x01"]
impl crate::ResetValue for super::SWIDTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Sample and half-frame width\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWIDTH_A {
    #[doc = "0: 8 bit sample."]
    _8BIT,
    #[doc = "1: 16 bit sample."]
    _16BIT,
    #[doc = "2: 24 bit sample."]
    _24BIT,
    #[doc = "3: 32-bit sample."]
    _32BIT,
    #[doc = "4: 8 bit sample in a 16 bit half-frame."]
    _8BITIN16,
    #[doc = "5: 8 bit sample in a 32-bit half-frame."]
    _8BITIN32,
    #[doc = "6: 16 bit sample in a 32-bit half-frame."]
    _16BITIN32,
    #[doc = "7: 24 bit sample in a 32-bit half-frame."]
    _24BITIN32,
}
impl From<SWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: SWIDTH_A) -> Self {
        match variant {
            SWIDTH_A::_8BIT => 0,
            SWIDTH_A::_16BIT => 1,
            SWIDTH_A::_24BIT => 2,
            SWIDTH_A::_32BIT => 3,
            SWIDTH_A::_8BITIN16 => 4,
            SWIDTH_A::_8BITIN32 => 5,
            SWIDTH_A::_16BITIN32 => 6,
            SWIDTH_A::_24BITIN32 => 7,
        }
    }
}
#[doc = "Reader of field `SWIDTH`"]
pub type SWIDTH_R = crate::R<u8, SWIDTH_A>;
impl SWIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWIDTH_A {
        match self.bits {
            0 => SWIDTH_A::_8BIT,
            1 => SWIDTH_A::_16BIT,
            2 => SWIDTH_A::_24BIT,
            3 => SWIDTH_A::_32BIT,
            4 => SWIDTH_A::_8BITIN16,
            5 => SWIDTH_A::_8BITIN32,
            6 => SWIDTH_A::_16BITIN32,
            7 => SWIDTH_A::_24BITIN32,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8BIT`"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == SWIDTH_A::_8BIT
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == SWIDTH_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_24BIT`"]
    #[inline(always)]
    pub fn is_24bit(&self) -> bool {
        *self == SWIDTH_A::_24BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == SWIDTH_A::_32BIT
    }
    #[doc = "Checks if the value of the field is `_8BITIN16`"]
    #[inline(always)]
    pub fn is_8bit_in16(&self) -> bool {
        *self == SWIDTH_A::_8BITIN16
    }
    #[doc = "Checks if the value of the field is `_8BITIN32`"]
    #[inline(always)]
    pub fn is_8bit_in32(&self) -> bool {
        *self == SWIDTH_A::_8BITIN32
    }
    #[doc = "Checks if the value of the field is `_16BITIN32`"]
    #[inline(always)]
    pub fn is_16bit_in32(&self) -> bool {
        *self == SWIDTH_A::_16BITIN32
    }
    #[doc = "Checks if the value of the field is `_24BITIN32`"]
    #[inline(always)]
    pub fn is_24bit_in32(&self) -> bool {
        *self == SWIDTH_A::_24BITIN32
    }
}
#[doc = "Write proxy for field `SWIDTH`"]
pub struct SWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "8 bit sample."]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_8BIT)
    }
    #[doc = "16 bit sample."]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_16BIT)
    }
    #[doc = "24 bit sample."]
    #[inline(always)]
    pub fn _24bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_24BIT)
    }
    #[doc = "32-bit sample."]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(SWIDTH_A::_32BIT)
    }
    #[doc = "8 bit sample in a 16 bit half-frame."]
    #[inline(always)]
    pub fn _8bit_in16(self) -> &'a mut W {
        self.variant(SWIDTH_A::_8BITIN16)
    }
    #[doc = "8 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _8bit_in32(self) -> &'a mut W {
        self.variant(SWIDTH_A::_8BITIN32)
    }
    #[doc = "16 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _16bit_in32(self) -> &'a mut W {
        self.variant(SWIDTH_A::_16BITIN32)
    }
    #[doc = "24 bit sample in a 32-bit half-frame."]
    #[inline(always)]
    pub fn _24bit_in32(self) -> &'a mut W {
        self.variant(SWIDTH_A::_24BITIN32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample and half-frame width"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample and half-frame width"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SWIDTH_W {
        SWIDTH_W { w: self }
    }
}
