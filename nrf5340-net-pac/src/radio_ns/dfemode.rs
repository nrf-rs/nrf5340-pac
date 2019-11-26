#[doc = "Reader of register DFEMODE"]
pub type R = crate::R<u32, super::DFEMODE>;
#[doc = "Writer for register DFEMODE"]
pub type W = crate::W<u32, super::DFEMODE>;
#[doc = "Register DFEMODE `reset()`'s with value 0"]
impl crate::ResetValue for super::DFEMODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Direction finding operation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFEOPMODE_A {
    #[doc = "0: Direction finding mode disabled"]
    DISABLED,
    #[doc = "2: Direction finding mode set to AoD"]
    AOD,
    #[doc = "3: Direction finding mode set to AoA"]
    AOA,
}
impl From<DFEOPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DFEOPMODE_A) -> Self {
        match variant {
            DFEOPMODE_A::DISABLED => 0,
            DFEOPMODE_A::AOD => 2,
            DFEOPMODE_A::AOA => 3,
        }
    }
}
#[doc = "Reader of field `DFEOPMODE`"]
pub type DFEOPMODE_R = crate::R<u8, DFEOPMODE_A>;
impl DFEOPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DFEOPMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DFEOPMODE_A::DISABLED),
            2 => Val(DFEOPMODE_A::AOD),
            3 => Val(DFEOPMODE_A::AOA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DFEOPMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `AOD`"]
    #[inline(always)]
    pub fn is_ao_d(&self) -> bool {
        *self == DFEOPMODE_A::AOD
    }
    #[doc = "Checks if the value of the field is `AOA`"]
    #[inline(always)]
    pub fn is_ao_a(&self) -> bool {
        *self == DFEOPMODE_A::AOA
    }
}
#[doc = "Write proxy for field `DFEOPMODE`"]
pub struct DFEOPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFEOPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFEOPMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Direction finding mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::DISABLED)
    }
    #[doc = "Direction finding mode set to AoD"]
    #[inline(always)]
    pub fn ao_d(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::AOD)
    }
    #[doc = "Direction finding mode set to AoA"]
    #[inline(always)]
    pub fn ao_a(self) -> &'a mut W {
        self.variant(DFEOPMODE_A::AOA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&self) -> DFEOPMODE_R {
        DFEOPMODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Direction finding operation mode"]
    #[inline(always)]
    pub fn dfeopmode(&mut self) -> DFEOPMODE_W {
        DFEOPMODE_W { w: self }
    }
}
