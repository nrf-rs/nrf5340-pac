#[doc = "Reader of register EVENTS_BCMATCH"]
pub type R = crate::R<u32, super::EVENTS_BCMATCH>;
#[doc = "Writer for register EVENTS_BCMATCH"]
pub type W = crate::W<u32, super::EVENTS_BCMATCH>;
#[doc = "Register EVENTS_BCMATCH `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_BCMATCH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Bit counter reached bit count value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_BCMATCH_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_BCMATCH_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_BCMATCH_A) -> Self {
        match variant {
            EVENTS_BCMATCH_A::NOTGENERATED => false,
            EVENTS_BCMATCH_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_BCMATCH`"]
pub type EVENTS_BCMATCH_R = crate::R<bool, EVENTS_BCMATCH_A>;
impl EVENTS_BCMATCH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_BCMATCH_A {
        match self.bits {
            false => EVENTS_BCMATCH_A::NOTGENERATED,
            true => EVENTS_BCMATCH_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_BCMATCH_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_BCMATCH_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_BCMATCH`"]
pub struct EVENTS_BCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_BCMATCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_BCMATCH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_BCMATCH_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_BCMATCH_A::GENERATED)
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
    #[doc = "Bit 0 - Bit counter reached bit count value"]
    #[inline(always)]
    pub fn events_bcmatch(&self) -> EVENTS_BCMATCH_R {
        EVENTS_BCMATCH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit counter reached bit count value"]
    #[inline(always)]
    pub fn events_bcmatch(&mut self) -> EVENTS_BCMATCH_W {
        EVENTS_BCMATCH_W { w: self }
    }
}
