#[doc = "Reader of register EVENTS_FRAMESTART"]
pub type R = crate::R<u32, super::EVENTS_FRAMESTART>;
#[doc = "Writer for register EVENTS_FRAMESTART"]
pub type W = crate::W<u32, super::EVENTS_FRAMESTART>;
#[doc = "Register EVENTS_FRAMESTART `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_FRAMESTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "IEEE 802.15.4 length field received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_FRAMESTART_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_FRAMESTART_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_FRAMESTART_A) -> Self {
        match variant {
            EVENTS_FRAMESTART_A::NOTGENERATED => false,
            EVENTS_FRAMESTART_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_FRAMESTART`"]
pub type EVENTS_FRAMESTART_R = crate::R<bool, EVENTS_FRAMESTART_A>;
impl EVENTS_FRAMESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_FRAMESTART_A {
        match self.bits {
            false => EVENTS_FRAMESTART_A::NOTGENERATED,
            true => EVENTS_FRAMESTART_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_FRAMESTART_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_FRAMESTART_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_FRAMESTART`"]
pub struct EVENTS_FRAMESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_FRAMESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_FRAMESTART_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_FRAMESTART_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_FRAMESTART_A::GENERATED)
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
    #[doc = "Bit 0 - IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub fn events_framestart(&self) -> EVENTS_FRAMESTART_R {
        EVENTS_FRAMESTART_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IEEE 802.15.4 length field received"]
    #[inline(always)]
    pub fn events_framestart(&mut self) -> EVENTS_FRAMESTART_W {
        EVENTS_FRAMESTART_W { w: self }
    }
}
