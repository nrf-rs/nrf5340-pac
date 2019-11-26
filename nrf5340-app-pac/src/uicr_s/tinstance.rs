#[doc = "Reader of register TINSTANCE"]
pub type R = crate::R<u32, super::TINSTANCE>;
#[doc = "Writer for register TINSTANCE"]
pub type W = crate::W<u32, super::TINSTANCE>;
#[doc = "Register TINSTANCE `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TINSTANCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `TINSTANCE`"]
pub type TINSTANCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TINSTANCE`"]
pub struct TINSTANCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TINSTANCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&self) -> TINSTANCE_R {
        TINSTANCE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31 - TINSTANCE bits are negated and used in the SW-DP DLPIDR.TINSTANCE field. E.g. 0xF in this field is translated to 0x0 in DLPIDR.TINSTANCE field."]
    #[inline(always)]
    pub fn tinstance(&mut self) -> TINSTANCE_W {
        TINSTANCE_W { w: self }
    }
}
