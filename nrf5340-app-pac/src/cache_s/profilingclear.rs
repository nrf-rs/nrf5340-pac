#[doc = "Writer for register PROFILINGCLEAR"]
pub type W = crate::W<u32, super::PROFILINGCLEAR>;
#[doc = "Register PROFILINGCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::PROFILINGCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clearing the profiling counters\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLEAR_AW {
    #[doc = "1: Clear the profiling counters"]
    CLEAR,
}
impl From<CLEAR_AW> for bool {
    #[inline(always)]
    fn from(variant: CLEAR_AW) -> Self {
        match variant {
            CLEAR_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `CLEAR`"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLEAR_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clear the profiling counters"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLEAR_AW::CLEAR)
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
    #[doc = "Bit 0 - Clearing the profiling counters"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
}
