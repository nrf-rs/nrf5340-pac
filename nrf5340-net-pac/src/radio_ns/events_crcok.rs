#[doc = "Reader of register EVENTS_CRCOK"]
pub type R = crate::R<u32, super::EVENTS_CRCOK>;
#[doc = "Writer for register EVENTS_CRCOK"]
pub type W = crate::W<u32, super::EVENTS_CRCOK>;
#[doc = "Register EVENTS_CRCOK `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_CRCOK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Packet received with CRC ok\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_CRCOK_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_CRCOK_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_CRCOK_A) -> Self {
        match variant {
            EVENTS_CRCOK_A::NOTGENERATED => false,
            EVENTS_CRCOK_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_CRCOK`"]
pub type EVENTS_CRCOK_R = crate::R<bool, EVENTS_CRCOK_A>;
impl EVENTS_CRCOK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_CRCOK_A {
        match self.bits {
            false => EVENTS_CRCOK_A::NOTGENERATED,
            true => EVENTS_CRCOK_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_CRCOK_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_CRCOK_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_CRCOK`"]
pub struct EVENTS_CRCOK_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_CRCOK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_CRCOK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_CRCOK_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_CRCOK_A::GENERATED)
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
    #[doc = "Bit 0 - Packet received with CRC ok"]
    #[inline(always)]
    pub fn events_crcok(&self) -> EVENTS_CRCOK_R {
        EVENTS_CRCOK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Packet received with CRC ok"]
    #[inline(always)]
    pub fn events_crcok(&mut self) -> EVENTS_CRCOK_W {
        EVENTS_CRCOK_W { w: self }
    }
}
