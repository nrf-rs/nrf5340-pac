#[doc = "Writer for register KEY2"]
pub type W = crate::W<u32, super::KEY2>;
#[doc = "Register KEY2 `reset()`'s with value 0"]
impl crate::ResetValue for super::KEY2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `KEY2`"]
pub struct KEY2_W<'a> {
    w: &'a mut W,
}
impl<'a> KEY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 95:64 of DMA AES KEY"]
    #[inline(always)]
    pub fn key2(&mut self) -> KEY2_W {
        KEY2_W { w: self }
    }
}
