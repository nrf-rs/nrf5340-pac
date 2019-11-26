#[doc = "Reader of register EVENTS_INVALIDOPERATION"]
pub type R = crate::R<u32, super::EVENTS_INVALIDOPERATION>;
#[doc = "Writer for register EVENTS_INVALIDOPERATION"]
pub type W = crate::W<u32, super::EVENTS_INVALIDOPERATION>;
#[doc = "Register EVENTS_INVALIDOPERATION `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_INVALIDOPERATION {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "An FPUIOC exception triggered by an invalid operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_INVALIDOPERATION_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_INVALIDOPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_INVALIDOPERATION_A) -> Self {
        match variant {
            EVENTS_INVALIDOPERATION_A::NOTGENERATED => false,
            EVENTS_INVALIDOPERATION_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_INVALIDOPERATION`"]
pub type EVENTS_INVALIDOPERATION_R = crate::R<bool, EVENTS_INVALIDOPERATION_A>;
impl EVENTS_INVALIDOPERATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_INVALIDOPERATION_A {
        match self.bits {
            false => EVENTS_INVALIDOPERATION_A::NOTGENERATED,
            true => EVENTS_INVALIDOPERATION_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_INVALIDOPERATION_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_INVALIDOPERATION_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_INVALIDOPERATION`"]
pub struct EVENTS_INVALIDOPERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_INVALIDOPERATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_INVALIDOPERATION_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_INVALIDOPERATION_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_INVALIDOPERATION_A::GENERATED)
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
    #[doc = "Bit 0 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_invalidoperation(&self) -> EVENTS_INVALIDOPERATION_R {
        EVENTS_INVALIDOPERATION_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUIOC exception triggered by an invalid operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_invalidoperation(&mut self) -> EVENTS_INVALIDOPERATION_W {
        EVENTS_INVALIDOPERATION_W { w: self }
    }
}
