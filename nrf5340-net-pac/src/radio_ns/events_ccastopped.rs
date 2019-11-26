#[doc = "Reader of register EVENTS_CCASTOPPED"]
pub type R = crate::R<u32, super::EVENTS_CCASTOPPED>;
#[doc = "Writer for register EVENTS_CCASTOPPED"]
pub type W = crate::W<u32, super::EVENTS_CCASTOPPED>;
#[doc = "Register EVENTS_CCASTOPPED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_CCASTOPPED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The CCA has stopped\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_CCASTOPPED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_CCASTOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_CCASTOPPED_A) -> Self {
        match variant {
            EVENTS_CCASTOPPED_A::NOTGENERATED => false,
            EVENTS_CCASTOPPED_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_CCASTOPPED`"]
pub type EVENTS_CCASTOPPED_R = crate::R<bool, EVENTS_CCASTOPPED_A>;
impl EVENTS_CCASTOPPED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_CCASTOPPED_A {
        match self.bits {
            false => EVENTS_CCASTOPPED_A::NOTGENERATED,
            true => EVENTS_CCASTOPPED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_CCASTOPPED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_CCASTOPPED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_CCASTOPPED`"]
pub struct EVENTS_CCASTOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_CCASTOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_CCASTOPPED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_CCASTOPPED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_CCASTOPPED_A::GENERATED)
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
    #[doc = "Bit 0 - The CCA has stopped"]
    #[inline(always)]
    pub fn events_ccastopped(&self) -> EVENTS_CCASTOPPED_R {
        EVENTS_CCASTOPPED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The CCA has stopped"]
    #[inline(always)]
    pub fn events_ccastopped(&mut self) -> EVENTS_CCASTOPPED_W {
        EVENTS_CCASTOPPED_W { w: self }
    }
}
