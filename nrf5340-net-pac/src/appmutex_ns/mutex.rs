#[doc = "Reader of register MUTEX[%s]"]
pub type R = crate::R<u32, super::MUTEX>;
#[doc = "Writer for register MUTEX[%s]"]
pub type W = crate::W<u32, super::MUTEX>;
#[doc = "Register MUTEX[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::MUTEX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mutex register n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MUTEX_A {
    #[doc = "0: Mutex n is in unlocked state"]
    UNLOCKED,
    #[doc = "1: Mutex n is in locked state"]
    LOCKED,
}
impl From<MUTEX_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEX_A) -> Self {
        match variant {
            MUTEX_A::UNLOCKED => false,
            MUTEX_A::LOCKED => true,
        }
    }
}
#[doc = "Reader of field `MUTEX`"]
pub type MUTEX_R = crate::R<bool, MUTEX_A>;
impl MUTEX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUTEX_A {
        match self.bits {
            false => MUTEX_A::UNLOCKED,
            true => MUTEX_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == MUTEX_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == MUTEX_A::LOCKED
    }
}
#[doc = "Write proxy for field `MUTEX`"]
pub struct MUTEX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUTEX_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Mutex n is in unlocked state"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(MUTEX_A::UNLOCKED)
    }
    #[doc = "Mutex n is in locked state"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(MUTEX_A::LOCKED)
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
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&self) -> MUTEX_R {
        MUTEX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mutex register n"]
    #[inline(always)]
    pub fn mutex(&mut self) -> MUTEX_W {
        MUTEX_W { w: self }
    }
}
