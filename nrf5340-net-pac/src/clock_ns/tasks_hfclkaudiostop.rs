#[doc = "Writer for register TASKS_HFCLKAUDIOSTOP"]
pub type W = crate::W<u32, super::TASKS_HFCLKAUDIOSTOP>;
#[doc = "Register TASKS_HFCLKAUDIOSTOP `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_HFCLKAUDIOSTOP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Stop HFCLKAUDIO source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_HFCLKAUDIOSTOP_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_HFCLKAUDIOSTOP_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_HFCLKAUDIOSTOP_AW) -> Self {
        match variant {
            TASKS_HFCLKAUDIOSTOP_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_HFCLKAUDIOSTOP`"]
pub struct TASKS_HFCLKAUDIOSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_HFCLKAUDIOSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_HFCLKAUDIOSTOP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_HFCLKAUDIOSTOP_AW::TRIGGER)
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
    #[doc = "Bit 0 - Stop HFCLKAUDIO source"]
    #[inline(always)]
    pub fn tasks_hfclkaudiostop(&mut self) -> TASKS_HFCLKAUDIOSTOP_W {
        TASKS_HFCLKAUDIOSTOP_W { w: self }
    }
}
