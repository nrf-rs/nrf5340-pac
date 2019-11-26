#[doc = "Reader of register HEADERMASK"]
pub type R = crate::R<u32, super::HEADERMASK>;
#[doc = "Writer for register HEADERMASK"]
pub type W = crate::W<u32, super::HEADERMASK>;
#[doc = "Register HEADERMASK `reset()`'s with value 0xe3"]
impl crate::ResetValue for super::HEADERMASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xe3
    }
}
#[doc = "Reader of field `HEADERMASK`"]
pub type HEADERMASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HEADERMASK`"]
pub struct HEADERMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> HEADERMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&self) -> HEADERMASK_R {
        HEADERMASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Header (S0) mask"]
    #[inline(always)]
    pub fn headermask(&mut self) -> HEADERMASK_W {
        HEADERMASK_W { w: self }
    }
}
