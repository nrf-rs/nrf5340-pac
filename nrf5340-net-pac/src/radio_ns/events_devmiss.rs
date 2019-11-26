#[doc = "Reader of register EVENTS_DEVMISS"]
pub type R = crate::R<u32, super::EVENTS_DEVMISS>;
#[doc = "Writer for register EVENTS_DEVMISS"]
pub type W = crate::W<u32, super::EVENTS_DEVMISS>;
#[doc = "Register EVENTS_DEVMISS `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_DEVMISS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "No device address match occurred on the last received packet\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_DEVMISS_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_DEVMISS_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_DEVMISS_A) -> Self {
        match variant {
            EVENTS_DEVMISS_A::NOTGENERATED => false,
            EVENTS_DEVMISS_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_DEVMISS`"]
pub type EVENTS_DEVMISS_R = crate::R<bool, EVENTS_DEVMISS_A>;
impl EVENTS_DEVMISS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_DEVMISS_A {
        match self.bits {
            false => EVENTS_DEVMISS_A::NOTGENERATED,
            true => EVENTS_DEVMISS_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_DEVMISS_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_DEVMISS_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_DEVMISS`"]
pub struct EVENTS_DEVMISS_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_DEVMISS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_DEVMISS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_DEVMISS_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_DEVMISS_A::GENERATED)
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
    #[doc = "Bit 0 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmiss(&self) -> EVENTS_DEVMISS_R {
        EVENTS_DEVMISS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No device address match occurred on the last received packet"]
    #[inline(always)]
    pub fn events_devmiss(&mut self) -> EVENTS_DEVMISS_W {
        EVENTS_DEVMISS_W { w: self }
    }
}
