#[doc = "Reader of register EVENTS_DISABLED"]
pub type R = crate::R<u32, super::EVENTS_DISABLED>;
#[doc = "Writer for register EVENTS_DISABLED"]
pub type W = crate::W<u32, super::EVENTS_DISABLED>;
#[doc = "Register EVENTS_DISABLED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_DISABLED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "RADIO has been disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_DISABLED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_DISABLED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_DISABLED_A) -> Self {
        match variant {
            EVENTS_DISABLED_A::NOTGENERATED => false,
            EVENTS_DISABLED_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_DISABLED`"]
pub type EVENTS_DISABLED_R = crate::R<bool, EVENTS_DISABLED_A>;
impl EVENTS_DISABLED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_DISABLED_A {
        match self.bits {
            false => EVENTS_DISABLED_A::NOTGENERATED,
            true => EVENTS_DISABLED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_DISABLED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_DISABLED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_DISABLED`"]
pub struct EVENTS_DISABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_DISABLED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_DISABLED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_DISABLED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_DISABLED_A::GENERATED)
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
    #[doc = "Bit 0 - RADIO has been disabled"]
    #[inline(always)]
    pub fn events_disabled(&self) -> EVENTS_DISABLED_R {
        EVENTS_DISABLED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RADIO has been disabled"]
    #[inline(always)]
    pub fn events_disabled(&mut self) -> EVENTS_DISABLED_W {
        EVENTS_DISABLED_W { w: self }
    }
}
