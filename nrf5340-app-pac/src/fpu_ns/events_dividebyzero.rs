#[doc = "Reader of register EVENTS_DIVIDEBYZERO"]
pub type R = crate::R<u32, super::EVENTS_DIVIDEBYZERO>;
#[doc = "Writer for register EVENTS_DIVIDEBYZERO"]
pub type W = crate::W<u32, super::EVENTS_DIVIDEBYZERO>;
#[doc = "Register EVENTS_DIVIDEBYZERO `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_DIVIDEBYZERO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_DIVIDEBYZERO_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_DIVIDEBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_DIVIDEBYZERO_A) -> Self {
        match variant {
            EVENTS_DIVIDEBYZERO_A::NOTGENERATED => false,
            EVENTS_DIVIDEBYZERO_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_DIVIDEBYZERO`"]
pub type EVENTS_DIVIDEBYZERO_R = crate::R<bool, EVENTS_DIVIDEBYZERO_A>;
impl EVENTS_DIVIDEBYZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_DIVIDEBYZERO_A {
        match self.bits {
            false => EVENTS_DIVIDEBYZERO_A::NOTGENERATED,
            true => EVENTS_DIVIDEBYZERO_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_DIVIDEBYZERO_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_DIVIDEBYZERO_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_DIVIDEBYZERO`"]
pub struct EVENTS_DIVIDEBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_DIVIDEBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_DIVIDEBYZERO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_DIVIDEBYZERO_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_DIVIDEBYZERO_A::GENERATED)
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
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&self) -> EVENTS_DIVIDEBYZERO_R {
        EVENTS_DIVIDEBYZERO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - An FPUDZC exception triggered by a floating-point divide-by-zero operation has occurred in the FPU"]
    #[inline(always)]
    pub fn events_dividebyzero(&mut self) -> EVENTS_DIVIDEBYZERO_W {
        EVENTS_DIVIDEBYZERO_W { w: self }
    }
}
