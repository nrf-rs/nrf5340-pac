#[doc = "Reader of register MHRMATCHMAS"]
pub type R = crate::R<u32, super::MHRMATCHMAS>;
#[doc = "Writer for register MHRMATCHMAS"]
pub type W = crate::W<u32, super::MHRMATCHMAS>;
#[doc = "Register MHRMATCHMAS `reset()`'s with value 0"]
impl crate::ResetValue for super::MHRMATCHMAS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MHRMATCHMAS`"]
pub type MHRMATCHMAS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MHRMATCHMAS`"]
pub struct MHRMATCHMAS_W<'a> {
    w: &'a mut W,
}
impl<'a> MHRMATCHMAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmas(&self) -> MHRMATCHMAS_R {
        MHRMATCHMAS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pattern mask"]
    #[inline(always)]
    pub fn mhrmatchmas(&mut self) -> MHRMATCHMAS_W {
        MHRMATCHMAS_W { w: self }
    }
}
