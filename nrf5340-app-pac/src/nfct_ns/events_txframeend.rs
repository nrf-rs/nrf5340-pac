#[doc = "Reader of register EVENTS_TXFRAMEEND"]
pub type R = crate::R<u32, super::EVENTS_TXFRAMEEND>;
#[doc = "Writer for register EVENTS_TXFRAMEEND"]
pub type W = crate::W<u32, super::EVENTS_TXFRAMEEND>;
#[doc = "Register EVENTS_TXFRAMEEND `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_TXFRAMEEND {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Marks the end of the last transmitted on-air symbol of a frame\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_TXFRAMEEND_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_TXFRAMEEND_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_TXFRAMEEND_A) -> Self {
        match variant {
            EVENTS_TXFRAMEEND_A::NOTGENERATED => false,
            EVENTS_TXFRAMEEND_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_TXFRAMEEND`"]
pub type EVENTS_TXFRAMEEND_R = crate::R<bool, EVENTS_TXFRAMEEND_A>;
impl EVENTS_TXFRAMEEND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_TXFRAMEEND_A {
        match self.bits {
            false => EVENTS_TXFRAMEEND_A::NOTGENERATED,
            true => EVENTS_TXFRAMEEND_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_TXFRAMEEND_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_TXFRAMEEND_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_TXFRAMEEND`"]
pub struct EVENTS_TXFRAMEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_TXFRAMEEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_TXFRAMEEND_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_TXFRAMEEND_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_TXFRAMEEND_A::GENERATED)
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
    #[doc = "Bit 0 - Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub fn events_txframeend(&self) -> EVENTS_TXFRAMEEND_R {
        EVENTS_TXFRAMEEND_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Marks the end of the last transmitted on-air symbol of a frame"]
    #[inline(always)]
    pub fn events_txframeend(&mut self) -> EVENTS_TXFRAMEEND_W {
        EVENTS_TXFRAMEEND_W { w: self }
    }
}
