#[doc = "Reader of register B3"]
pub type R = crate::R<u32, super::B3>;
#[doc = "Writer for register B3"]
pub type W = crate::W<u32, super::B3>;
#[doc = "Register B3 `reset()`'s with value 0x0fcb"]
impl crate::ResetValue for super::B3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0fcb
    }
}
#[doc = "Reader of field `B3`"]
pub type B3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `B3`"]
pub struct B3_W<'a> {
    w: &'a mut W,
}
impl<'a> B3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn b3(&self) -> B3_R {
        B3_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - y-intercept of 4th piece wise linear function"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3_W {
        B3_W { w: self }
    }
}
