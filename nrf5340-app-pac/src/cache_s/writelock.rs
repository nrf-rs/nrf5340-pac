#[doc = "Reader of register WRITELOCK"]
pub type R = crate::R<u32, super::WRITELOCK>;
#[doc = "Writer for register WRITELOCK"]
pub type W = crate::W<u32, super::WRITELOCK>;
#[doc = "Register WRITELOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::WRITELOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Lock cache updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRITELOCK_A {
    #[doc = "0: Cache updates unlocked"]
    UNLOCKED,
    #[doc = "1: Cache updates locked"]
    LOCKED,
}
impl From<WRITELOCK_A> for bool {
    #[inline(always)]
    fn from(variant: WRITELOCK_A) -> Self {
        match variant {
            WRITELOCK_A::UNLOCKED => false,
            WRITELOCK_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `WRITELOCK`"]
pub type WRITELOCK_R = crate::R<bool, WRITELOCK_A>;
impl WRITELOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRITELOCK_A {
        match self.bits {
            false => WRITELOCK_A::UNLOCKED,
            true => WRITELOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == WRITELOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == WRITELOCK_A::LOCKED
    }
}
#[doc = "Write proxy for field `WRITELOCK`"]
pub struct WRITELOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITELOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRITELOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Cache updates unlocked"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(WRITELOCK_A::UNLOCKED)
    }
    #[doc = "Cache updates locked"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(WRITELOCK_A::LOCKED)
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
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&self) -> WRITELOCK_R {
        WRITELOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock cache updates"]
    #[inline(always)]
    pub fn writelock(&mut self) -> WRITELOCK_W {
        WRITELOCK_W { w: self }
    }
}
