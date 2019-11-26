#[doc = "Writer for register TASKS_ACTIVATE"]
pub type W = crate::W<u32, super::TASKS_ACTIVATE>;
#[doc = "Register TASKS_ACTIVATE `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_ACTIVATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Activate NFCT peripheral for incoming and outgoing frames, change state to activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_ACTIVATE_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_ACTIVATE_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_ACTIVATE_AW) -> Self {
        match variant {
            TASKS_ACTIVATE_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_ACTIVATE`"]
pub struct TASKS_ACTIVATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_ACTIVATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_ACTIVATE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_ACTIVATE_AW::TRIGGER)
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
impl W {
    #[doc = "Bit 0 - Activate NFCT peripheral for incoming and outgoing frames, change state to activated"]
    #[inline(always)]
    pub fn tasks_activate(&mut self) -> TASKS_ACTIVATE_W {
        TASKS_ACTIVATE_W { w: self }
    }
}
