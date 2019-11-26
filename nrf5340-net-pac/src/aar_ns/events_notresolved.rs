#[doc = "Reader of register EVENTS_NOTRESOLVED"]
pub type R = crate::R<u32, super::EVENTS_NOTRESOLVED>;
#[doc = "Writer for register EVENTS_NOTRESOLVED"]
pub type W = crate::W<u32, super::EVENTS_NOTRESOLVED>;
#[doc = "Register EVENTS_NOTRESOLVED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_NOTRESOLVED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Address not resolved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_NOTRESOLVED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_NOTRESOLVED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_NOTRESOLVED_A) -> Self {
        match variant {
            EVENTS_NOTRESOLVED_A::NOTGENERATED => false,
            EVENTS_NOTRESOLVED_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_NOTRESOLVED`"]
pub type EVENTS_NOTRESOLVED_R = crate::R<bool, EVENTS_NOTRESOLVED_A>;
impl EVENTS_NOTRESOLVED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_NOTRESOLVED_A {
        match self.bits {
            false => EVENTS_NOTRESOLVED_A::NOTGENERATED,
            true => EVENTS_NOTRESOLVED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_NOTRESOLVED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_NOTRESOLVED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_NOTRESOLVED`"]
pub struct EVENTS_NOTRESOLVED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_NOTRESOLVED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_NOTRESOLVED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_NOTRESOLVED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_NOTRESOLVED_A::GENERATED)
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
    #[doc = "Bit 0 - Address not resolved"]
    #[inline(always)]
    pub fn events_notresolved(&self) -> EVENTS_NOTRESOLVED_R {
        EVENTS_NOTRESOLVED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Address not resolved"]
    #[inline(always)]
    pub fn events_notresolved(&mut self) -> EVENTS_NOTRESOLVED_W {
        EVENTS_NOTRESOLVED_W { w: self }
    }
}
