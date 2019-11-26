#[doc = "Reader of register EVENTS_READY"]
pub type R = crate::R<u32, super::EVENTS_READY>;
#[doc = "Writer for register EVENTS_READY"]
pub type W = crate::W<u32, super::EVENTS_READY>;
#[doc = "Register EVENTS_READY `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_READY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "LPCOMP is ready and output is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_READY_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_READY_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_READY_A) -> Self {
        match variant {
            EVENTS_READY_A::NOTGENERATED => false,
            EVENTS_READY_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_READY`"]
pub type EVENTS_READY_R = crate::R<bool, EVENTS_READY_A>;
impl EVENTS_READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_READY_A {
        match self.bits {
            false => EVENTS_READY_A::NOTGENERATED,
            true => EVENTS_READY_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_READY_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_READY_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_READY`"]
pub struct EVENTS_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_READY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_READY_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_READY_A::GENERATED)
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
    #[doc = "Bit 0 - LPCOMP is ready and output is valid"]
    #[inline(always)]
    pub fn events_ready(&self) -> EVENTS_READY_R {
        EVENTS_READY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPCOMP is ready and output is valid"]
    #[inline(always)]
    pub fn events_ready(&mut self) -> EVENTS_READY_W {
        EVENTS_READY_W { w: self }
    }
}
