#[doc = "Reader of register DEBUGLOCK"]
pub type R = crate::R<u32, super::DEBUGLOCK>;
#[doc = "Writer for register DEBUGLOCK"]
pub type W = crate::W<u32, super::DEBUGLOCK>;
#[doc = "Register DEBUGLOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::DEBUGLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Lock debug mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGLOCK_A {
    #[doc = "0: Debug mode unlocked"]
    UNLOCKED,
    #[doc = "1: Debug mode locked"]
    LOCKED,
}
impl From<DEBUGLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGLOCK_A) -> Self {
        match variant {
            DEBUGLOCK_A::UNLOCKED => false,
            DEBUGLOCK_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `DEBUGLOCK`"]
pub type DEBUGLOCK_R = crate::R<bool, DEBUGLOCK_A>;
impl DEBUGLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGLOCK_A {
        match self.bits {
            false => DEBUGLOCK_A::UNLOCKED,
            true => DEBUGLOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == DEBUGLOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == DEBUGLOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `DEBUGLOCK`"]
pub struct DEBUGLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Debug mode unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(DEBUGLOCK_A::UNLOCKED)
    }
    #[doc = "Debug mode locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(DEBUGLOCK_A::LOCKED)
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
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&self) -> DEBUGLOCK_R {
        DEBUGLOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock debug mode"]
    #[inline(always)]
    pub fn debuglock(&mut self) -> DEBUGLOCK_W {
        DEBUGLOCK_W { w: self }
    }
}
