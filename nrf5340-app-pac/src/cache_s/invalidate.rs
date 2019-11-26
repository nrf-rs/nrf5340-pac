#[doc = "Writer for register INVALIDATE"]
pub type W = crate::W<u32, super::INVALIDATE>;
#[doc = "Register INVALIDATE `reset()`'s with value 0"]
impl crate::ResetValue for super::INVALIDATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Invalidate the cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDATE_AW {
    #[doc = "1: Invalidate the cache"]
    INVALIDATE,
}
impl From<INVALIDATE_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALIDATE_AW) -> Self {
        match variant {
            INVALIDATE_AW::INVALIDATE => true,
        }
    }
}
#[doc = "Write proxy for field `INVALIDATE`"]
pub struct INVALIDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVALIDATE_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(self) -> &'a mut W {
        self.variant(INVALIDATE_AW::INVALIDATE)
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
    #[doc = "Bit 0 - Invalidate the cache"]
    #[inline(always)]
    pub fn invalidate(&mut self) -> INVALIDATE_W {
        INVALIDATE_W { w: self }
    }
}
