#[doc = "Writer for register TASKS_ERASESTART"]
pub type W = crate::W<u32, super::TASKS_ERASESTART>;
#[doc = "Register TASKS_ERASESTART `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_ERASESTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start external flash memory erase operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_ERASESTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_ERASESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_ERASESTART_AW) -> Self {
        match variant {
            TASKS_ERASESTART_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_ERASESTART`"]
pub struct TASKS_ERASESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_ERASESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_ERASESTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_ERASESTART_AW::TRIGGER)
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
    #[doc = "Bit 0 - Start external flash memory erase operation"]
    #[inline(always)]
    pub fn tasks_erasestart(&mut self) -> TASKS_ERASESTART_W {
        TASKS_ERASESTART_W { w: self }
    }
}
