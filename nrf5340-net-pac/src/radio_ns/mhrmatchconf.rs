#[doc = "Reader of register MHRMATCHCONF"]
pub type R = crate::R<u32, super::MHRMATCHCONF>;
#[doc = "Writer for register MHRMATCHCONF"]
pub type W = crate::W<u32, super::MHRMATCHCONF>;
#[doc = "Register MHRMATCHCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::MHRMATCHCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MHRMATCHCONF`"]
pub type MHRMATCHCONF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `MHRMATCHCONF`"]
pub struct MHRMATCHCONF_W<'a> {
    w: &'a mut W,
}
impl<'a> MHRMATCHCONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&self) -> MHRMATCHCONF_R {
        MHRMATCHCONF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Search pattern configuration"]
    #[inline(always)]
    pub fn mhrmatchconf(&mut self) -> MHRMATCHCONF_W {
        MHRMATCHCONF_W { w: self }
    }
}
