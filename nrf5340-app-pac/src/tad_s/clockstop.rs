#[doc = "Register `CLOCKSTOP` writer"]
pub struct W(crate::W<CLOCKSTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKSTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<CLOCKSTOP_SPEC>> for W {
    fn from(writer: crate::W<CLOCKSTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOP_AW {
    #[doc = "1: Stop all trace and debug clocks."]
    STOP = 1,
}
impl From<STOP_AW> for bool {
    #[inline(always)]
    fn from(variant: STOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` writer - "]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOP_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Stop all trace and debug clocks."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_AW::STOP)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop all trace and debug clocks.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clockstop](index.html) module"]
pub struct CLOCKSTOP_SPEC;
impl crate::RegisterSpec for CLOCKSTOP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clockstop::W](W) writer structure"]
impl crate::Writable for CLOCKSTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKSTOP to value 0"]
impl crate::Resettable for CLOCKSTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
