#[doc = "Reader of register EVENTS_ACCOF"]
pub type R = crate::R<u32, super::EVENTS_ACCOF>;
#[doc = "Writer for register EVENTS_ACCOF"]
pub type W = crate::W<u32, super::EVENTS_ACCOF>;
#[doc = "Register EVENTS_ACCOF `reset()`'s with value 0"]
impl crate::ResetValue for super::EVENTS_ACCOF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ACC or ACCDBL register overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EVENTS_ACCOF_A {
    #[doc = "0: Event not generated"]
    NOTGENERATED,
    #[doc = "1: Event generated"]
    GENERATED,
}
impl From<EVENTS_ACCOF_A> for bool {
    #[inline(always)]
    fn from(variant: EVENTS_ACCOF_A) -> Self {
        match variant {
            EVENTS_ACCOF_A::NOTGENERATED => false,
            EVENTS_ACCOF_A::GENERATED => true,
        }
    }
}
#[doc = "Reader of field `EVENTS_ACCOF`"]
pub type EVENTS_ACCOF_R = crate::R<bool, EVENTS_ACCOF_A>;
impl EVENTS_ACCOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EVENTS_ACCOF_A {
        match self.bits {
            false => EVENTS_ACCOF_A::NOTGENERATED,
            true => EVENTS_ACCOF_A::GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTGENERATED`"]
    #[inline(always)]
    pub fn is_not_generated(&self) -> bool {
        *self == EVENTS_ACCOF_A::NOTGENERATED
    }
    #[doc = "Checks if the value of the field is `GENERATED`"]
    #[inline(always)]
    pub fn is_generated(&self) -> bool {
        *self == EVENTS_ACCOF_A::GENERATED
    }
}
#[doc = "Write proxy for field `EVENTS_ACCOF`"]
pub struct EVENTS_ACCOF_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_ACCOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EVENTS_ACCOF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Event not generated"]
    #[inline(always)]
    pub fn not_generated(self) -> &'a mut W {
        self.variant(EVENTS_ACCOF_A::NOTGENERATED)
    }
    #[doc = "Event generated"]
    #[inline(always)]
    pub fn generated(self) -> &'a mut W {
        self.variant(EVENTS_ACCOF_A::GENERATED)
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
    #[doc = "Bit 0 - ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub fn events_accof(&self) -> EVENTS_ACCOF_R {
        EVENTS_ACCOF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACC or ACCDBL register overflow"]
    #[inline(always)]
    pub fn events_accof(&mut self) -> EVENTS_ACCOF_W {
        EVENTS_ACCOF_W { w: self }
    }
}
