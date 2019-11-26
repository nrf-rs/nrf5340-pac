#[doc = "Writer for register NONCE1"]
pub type W = crate::W<u32, super::NONCE1>;
#[doc = "Register NONCE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::NONCE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `NONCE1`"]
pub struct NONCE1_W<'a> {
    w: &'a mut W,
}
impl<'a> NONCE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of DMA NONCE"]
    #[inline(always)]
    pub fn nonce1(&mut self) -> NONCE1_W {
        NONCE1_W { w: self }
    }
}
