#[doc = "Reader of register EVENTS_SAMPLERDY"]
pub type R = crate::R<u32, super::EVENTS_SAMPLERDY>;
#[doc = "Writer for register EVENTS_SAMPLERDY"]
pub type W = crate::W<u32, super::EVENTS_SAMPLERDY>;
#[doc = "Register EVENTS_SAMPLERDY `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_SAMPLERDY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Event being generated for every new sample value written to the SAMPLE register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_SAMPLERDY_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_SAMPLERDY_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_SAMPLERDY_A) -> Self {
        match variant {
            EVENTS_SAMPLERDY_A::NOTGENERATED => false,
            EVENTS_SAMPLERDY_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_SAMPLERDY`"]
pub type EVENTS_SAMPLERDY_R = crate::R<bool, EVENTS_SAMPLERDY_A>;
impl EVENTS_SAMPLERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_SAMPLERDY_A {
        match self.bits {
            false => EVENTS_SAMPLERDY_A::NOTGENERATED,
            true => EVENTS_SAMPLERDY_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_SAMPLERDY_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_SAMPLERDY_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_SAMPLERDY`"]
pub struct EVENTS_SAMPLERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_SAMPLERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_SAMPLERDY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_SAMPLERDY_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_SAMPLERDY_A::GENERATED)
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
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&self) -> EVENTS_SAMPLERDY_R {
        EVENTS_SAMPLERDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event being generated for every new sample value written to the SAMPLE register"]
    #[inline(always)]
    pub fn events_samplerdy(&mut self) -> EVENTS_SAMPLERDY_W {
        EVENTS_SAMPLERDY_W { w: self }
    }
}
