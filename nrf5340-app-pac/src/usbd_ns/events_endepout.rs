#[doc = "Reader of register EVENTS_ENDEPOUT[%s]"]
pub type R = crate::R<u32, super::EVENTS_ENDEPOUT>;
#[doc = "Writer for register EVENTS_ENDEPOUT[%s]"]
pub type W = crate::W<u32, super::EVENTS_ENDEPOUT>;
#[doc = "Register EVENTS_ENDEPOUT[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_ENDEPOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ENDEPOUT_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_ENDEPOUT_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ENDEPOUT_A) -> Self {
        match variant {
            EVENTS_ENDEPOUT_A::NOTGENERATED => false,
            EVENTS_ENDEPOUT_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_ENDEPOUT`"]
pub type EVENTS_ENDEPOUT_R = crate::R<bool, EVENTS_ENDEPOUT_A>;
impl EVENTS_ENDEPOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ENDEPOUT_A {
        match self.bits {
            false => EVENTS_ENDEPOUT_A::NOTGENERATED,
            true => EVENTS_ENDEPOUT_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ENDEPOUT_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ENDEPOUT_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_ENDEPOUT`"]
pub struct EVENTS_ENDEPOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ENDEPOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_ENDEPOUT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPOUT_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ENDEPOUT_A::GENERATED)
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
    #[doc = "Bit 0 - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&self) -> EVENTS_ENDEPOUT_R {
        EVENTS_ENDEPOUT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The whole EPOUT\\[n\\] buffer has been consumed. The buffer can be accessed safely by software."]
    #[inline(always)]
    pub fn events_endepout(&mut self) -> EVENTS_ENDEPOUT_W {
        EVENTS_ENDEPOUT_W { w: self }
    }
}
