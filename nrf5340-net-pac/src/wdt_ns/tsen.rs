#[doc = "Writer for register TSEN"]
pub type W = crate::W<u32, super::TSEN>;
#[doc = "Register TSEN `reset()`'s with value 0"]
impl crate::ResetValue for super::TSEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Task stop enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TSEN_AW {
    #[doc = "1850885685: Value to allow stopping the watchdog"]
    ENABLE,
}
impl From<TSEN_AW> for u32 {
    #[inline(always)]
    fn from(variant: TSEN_AW) -> Self {
        match variant {
            TSEN_AW::ENABLE => 1850885685,
        }
    }
}
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSEN_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Value to allow stopping the watchdog"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TSEN_AW::ENABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Task stop enable register"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
}
