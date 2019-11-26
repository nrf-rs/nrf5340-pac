#[doc = "Reader of register DFECTRL2"]
pub type R = crate::R<u32, super::DFECTRL2>;
#[doc = "Writer for register DFECTRL2"]
pub type W = crate::W<u32, super::DFECTRL2>;
#[doc = "Register DFECTRL2 `reset()`'s with value 0"]
impl crate::ResetValue for super::DFECTRL2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSWITCHOFFSET`"]
pub type TSWITCHOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSWITCHOFFSET`"]
pub struct TSWITCHOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TSWITCHOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | ((value as u32) & 0x1fff);
        self.w
    }
}
#[doc = "Reader of field `TSAMPLEOFFSET`"]
pub type TSAMPLEOFFSET_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TSAMPLEOFFSET`"]
pub struct TSAMPLEOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> TSAMPLEOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&self) -> TSWITCHOFFSET_R {
        TSWITCHOFFSET_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&self) -> TSAMPLEOFFSET_R {
        TSAMPLEOFFSET_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16M cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&mut self) -> TSWITCHOFFSET_W {
        TSWITCHOFFSET_W { w: self }
    }
    #[doc = "Bits 16:27 - Signed value offset before starting sampling in number of 16M cycles relative to the beginning of the REFERENCE state - 12 us after switching start"]
    #[inline(always)]
    pub fn tsampleoffset(&mut self) -> TSAMPLEOFFSET_W {
        TSAMPLEOFFSET_W { w: self }
    }
}
