#[doc = "Writer for register TASKS_READSTART"]
pub type W = crate::W<u32, super::TASKS_READSTART>;
#[doc = "Register TASKS_READSTART `reset()`'s with value 0"]
impl crate::ResetValue for super::TASKS_READSTART {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Start transfer from external flash memory to internal RAM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_READSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER,
}
impl From<TASKS_READSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_READSTART_AW) -> Self {
        match variant {
            TASKS_READSTART_AW::TRIGGER => true,
        }
    }
}
#[doc = "Write proxy for field `TASKS_READSTART`"]
pub struct TASKS_READSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_READSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_READSTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_READSTART_AW::TRIGGER)
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
    #[doc = "Bit 0 - Start transfer from external flash memory to internal RAM"]
    #[inline(always)]
    pub fn tasks_readstart(&mut self) -> TASKS_READSTART_W {
        TASKS_READSTART_W { w: self }
    }
}
