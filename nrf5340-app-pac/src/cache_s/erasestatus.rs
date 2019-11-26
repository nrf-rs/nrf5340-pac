#[doc = "Reader of register ERASESTATUS"]
pub type R = crate::R<u32, super::ERASESTATUS>;
#[doc = "Writer for register ERASESTATUS"]
pub type W = crate::W<u32, super::ERASESTATUS>;
#[doc = "Register ERASESTATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASESTATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Cache erase status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASESTATUS_A {
    #[doc = "0: Erase is not complete or hasn't started"]
    IDLE,
    #[doc = "1: Cache erase is finished"]
    FINISHED,
}
impl From<ERASESTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: ERASESTATUS_A) -> Self {
        match variant {
            ERASESTATUS_A::IDLE => false,
            ERASESTATUS_A::FINISHED => true,
        }
    }
}
#[doc = "Reader of field `ERASESTATUS`"]
pub type ERASESTATUS_R = crate::R<bool, ERASESTATUS_A>;
impl ERASESTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERASESTATUS_A {
        match self.bits {
            false => ERASESTATUS_A::IDLE,
            true => ERASESTATUS_A::FINISHED,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == ERASESTATUS_A::IDLE
    }
    #[doc = "Checks if the value of the field is `FINISHED`"]
    #[inline(always)]
    pub fn is_finished(&self) -> bool {
        *self == ERASESTATUS_A::FINISHED
    }
}
#[doc = "Write proxy for field `ERASESTATUS`"]
pub struct ERASESTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASESTATUS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASESTATUS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Erase is not complete or hasn't started"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(ERASESTATUS_A::IDLE)
    }
    #[doc = "Cache erase is finished"]
    #[inline(always)]
    pub fn finished(self) -> &'a mut W {
        self.variant(ERASESTATUS_A::FINISHED)
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
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&self) -> ERASESTATUS_R {
        ERASESTATUS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache erase status"]
    #[inline(always)]
    pub fn erasestatus(&mut self) -> ERASESTATUS_W {
        ERASESTATUS_W { w: self }
    }
}
