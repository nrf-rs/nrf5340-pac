#[doc = "Reader of register SWITCHPATTERN"]
pub type R = crate::R<u32, super::SWITCHPATTERN>;
#[doc = "Writer for register SWITCHPATTERN"]
pub type W = crate::W<u32, super::SWITCHPATTERN>;
#[doc = "Register SWITCHPATTERN `reset()`'s with value 0"]
impl crate::ResetValue for super::SWITCHPATTERN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SWITCHPATTERN`"]
pub type SWITCHPATTERN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SWITCHPATTERN`"]
pub struct SWITCHPATTERN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCHPATTERN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control"]
    #[inline(always)]
    pub fn switchpattern(&self) -> SWITCHPATTERN_R {
        SWITCHPATTERN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fill array of GPIO patterns for antenna control"]
    #[inline(always)]
    pub fn switchpattern(&mut self) -> SWITCHPATTERN_W {
        SWITCHPATTERN_W { w: self }
    }
}
