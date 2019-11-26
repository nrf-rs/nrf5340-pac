#[doc = "Writer for register ERASE"]
pub type W = crate::W<u32, super::ERASE>;
#[doc = "Register ERASE `reset()`'s with value 0"]
impl crate::ResetValue for super::ERASE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Erase the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERASE_AW {
    #[doc = "1: Erase cache"]
    ERASE,
}
impl From<ERASE_AW> for bool {
    #[inline(always)]
    fn from(variant: ERASE_AW) -> Self {
        match variant {
            ERASE_AW::ERASE => true,
        }
    }
}
#[doc = "Write proxy for field `ERASE`"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERASE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Erase cache"]
    #[inline(always)]
    pub fn erase(self) -> &'a mut W {
        self.variant(ERASE_AW::ERASE)
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
    #[doc = "Bit 0 - Erase the cache"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
}
