#[doc = "Reader of register EVENTS_RSSIEND"]
pub type R = crate::R<u32, super::EVENTS_RSSIEND>;
#[doc = "Writer for register EVENTS_RSSIEND"]
pub type W = crate::W<u32, super::EVENTS_RSSIEND>;
#[doc = "Register EVENTS_RSSIEND `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_RSSIEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sampling of receive signal strength complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RSSIEND_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_RSSIEND_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_RSSIEND_A) -> Self {
        match variant {
            EVENTS_RSSIEND_A::NOTGENERATED => false,
            EVENTS_RSSIEND_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_RSSIEND`"]
pub type EVENTS_RSSIEND_R = crate::R<bool, EVENTS_RSSIEND_A>;
impl EVENTS_RSSIEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_RSSIEND_A {
        match self.bits {
            false => EVENTS_RSSIEND_A::NOTGENERATED,
            true => EVENTS_RSSIEND_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_RSSIEND_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_RSSIEND_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_RSSIEND`"]
pub struct EVENTS_RSSIEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_RSSIEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_RSSIEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_RSSIEND_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_RSSIEND_A::GENERATED)
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
    #[doc = "Bit 0 - Sampling of receive signal strength complete"]
    #[inline(always)]
    pub fn events_rssiend(&self) -> EVENTS_RSSIEND_R {
        EVENTS_RSSIEND_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sampling of receive signal strength complete"]
    #[inline(always)]
    pub fn events_rssiend(&mut self) -> EVENTS_RSSIEND_W {
        EVENTS_RSSIEND_W { w: self }
    }
}
