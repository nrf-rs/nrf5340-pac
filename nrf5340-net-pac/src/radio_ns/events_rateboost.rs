#[doc = "Reader of register EVENTS_RATEBOOST"]
pub type R = crate::R<u32, super::EVENTS_RATEBOOST>;
#[doc = "Writer for register EVENTS_RATEBOOST"]
pub type W = crate::W<u32, super::EVENTS_RATEBOOST>;
#[doc = "Register EVENTS_RATEBOOST `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_RATEBOOST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_RATEBOOST_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_RATEBOOST_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_RATEBOOST_A) -> Self {
        match variant {
            EVENTS_RATEBOOST_A::NOTGENERATED => false,
            EVENTS_RATEBOOST_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_RATEBOOST`"]
pub type EVENTS_RATEBOOST_R = crate::R<bool, EVENTS_RATEBOOST_A>;
impl EVENTS_RATEBOOST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_RATEBOOST_A {
        match self.bits {
            false => EVENTS_RATEBOOST_A::NOTGENERATED,
            true => EVENTS_RATEBOOST_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_RATEBOOST_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_RATEBOOST_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_RATEBOOST`"]
pub struct EVENTS_RATEBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_RATEBOOST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_RATEBOOST_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_RATEBOOST_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_RATEBOOST_A::GENERATED)
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
    #[doc = "Bit 0 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub fn events_rateboost(&self) -> EVENTS_RATEBOOST_R {
        EVENTS_RATEBOOST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ble_LR CI field received, receive mode is changed from Ble_LR125Kbit to Ble_LR500Kbit."]
    #[inline(always)]
    pub fn events_rateboost(&mut self) -> EVENTS_RATEBOOST_W {
        EVENTS_RATEBOOST_W { w: self }
    }
}
