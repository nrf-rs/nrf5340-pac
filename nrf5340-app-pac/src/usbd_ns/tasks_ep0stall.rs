#[doc = "Writer for register TASKS_EP0STALL"]
pub type W = crate::W<u32, super::TASKS_EP0STALL>;
#[doc = "Register TASKS_EP0STALL `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_EP0STALL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stalls data and status stage on control endpoint 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_EP0STALL_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_EP0STALL_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_EP0STALL_AW) -> Self {
        match variant {
            TASKS_EP0STALL_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_EP0STALL`"]
pub struct TASKS_EP0STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_EP0STALL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_EP0STALL_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_EP0STALL_AW::TRIGGER)
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
    #[doc = "Bit 0 - Stalls data and status stage on control endpoint 0"]
    #[inline(always)]
    pub fn tasks_ep0stall(&mut self) -> TASKS_EP0STALL_W {
        TASKS_EP0STALL_W { w: self }
    }
}
