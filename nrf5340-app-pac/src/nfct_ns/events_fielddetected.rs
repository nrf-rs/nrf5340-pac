#[doc = "Reader of register EVENTS_FIELDDETECTED"]
pub type R = crate::R<u32, super::EVENTS_FIELDDETECTED>;
#[doc = "Writer for register EVENTS_FIELDDETECTED"]
pub type W = crate::W<u32, super::EVENTS_FIELDDETECTED>;
#[doc = "Register EVENTS_FIELDDETECTED `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_FIELDDETECTED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Remote NFC field detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_FIELDDETECTED_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_FIELDDETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_FIELDDETECTED_A) -> Self {
        match variant {
            EVENTS_FIELDDETECTED_A::NOTGENERATED => false,
            EVENTS_FIELDDETECTED_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_FIELDDETECTED`"]
pub type EVENTS_FIELDDETECTED_R = crate::R<bool, EVENTS_FIELDDETECTED_A>;
impl EVENTS_FIELDDETECTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_FIELDDETECTED_A {
        match self.bits {
            false => EVENTS_FIELDDETECTED_A::NOTGENERATED,
            true => EVENTS_FIELDDETECTED_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_FIELDDETECTED_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_FIELDDETECTED_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_FIELDDETECTED`"]
pub struct EVENTS_FIELDDETECTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_FIELDDETECTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_FIELDDETECTED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_FIELDDETECTED_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_FIELDDETECTED_A::GENERATED)
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
    #[doc = "Bit 0 - Remote NFC field detected"]
    #[inline(always)]
    pub fn events_fielddetected(&self) -> EVENTS_FIELDDETECTED_R {
        EVENTS_FIELDDETECTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote NFC field detected"]
    #[inline(always)]
    pub fn events_fielddetected(&mut self) -> EVENTS_FIELDDETECTED_W {
        EVENTS_FIELDDETECTED_W { w: self }
    }
}
