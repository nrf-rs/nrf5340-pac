#[doc = "Reader of register CLEARPATTERN"]
pub type R = crate::R<u32, super::CLEARPATTERN>;
#[doc = "Writer for register CLEARPATTERN"]
pub type W = crate::W<u32, super::CLEARPATTERN>;
#[doc = "Register CLEARPATTERN `reset()`'s with value 0"]
impl crate::ResetValue for super::CLEARPATTERN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clears GPIO pattern array for antenna control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEARPATTERN_A {
    #[doc = "1: Clear the GPIO pattern"]
    CLEAR,
}
impl From<CLEARPATTERN_A> for bool {
    #[inline(always)]
    fn from(variant: CLEARPATTERN_A) -> Self {
        match variant {
            CLEARPATTERN_A::CLEAR => true,
        }
    }
}
#[doc = "Reader of field `CLEARPATTERN`"]
pub type CLEARPATTERN_R = crate::R<bool, CLEARPATTERN_A>;
impl CLEARPATTERN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<bool, CLEARPATTERN_A> {
        use crate::Variant::*;
        match self.bits {
            true => Val(CLEARPATTERN_A::CLEAR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == CLEARPATTERN_A::CLEAR
    }
}
#[doc = "Write proxy for field `CLEARPATTERN`"]
pub struct CLEARPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEARPATTERN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEARPATTERN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the GPIO pattern"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEARPATTERN_A::CLEAR)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&self) -> CLEARPATTERN_R {
        CLEARPATTERN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clears GPIO pattern array for antenna control"]
    #[inline(always)]
    pub fn clearpattern(&mut self) -> CLEARPATTERN_W {
        CLEARPATTERN_W { w: self }
    }
}
