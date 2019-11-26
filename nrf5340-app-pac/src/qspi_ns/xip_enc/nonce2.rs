#[doc = "Writer for register NONCE2"]
pub type W = crate::W<u32, super::NONCE2>;
#[doc = "Register NONCE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::NONCE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NONCE2`"]
pub struct NONCE2_W<'a> {
    w: &'a mut W,
}
impl<'a> NONCE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 95:64 of XIP NONCE"]
    #[inline(always)]
    pub fn nonce2(&mut self) -> NONCE2_W {
        NONCE2_W { w: self }
    }
}
