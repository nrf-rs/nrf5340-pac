#[doc = "Register `HFCLK192MSRC` reader"]
pub struct R(crate::R<HFCLK192MSRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLK192MSRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HFCLK192MSRC_SPEC>> for R {
    fn from(reader: crate::R<HFCLK192MSRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFCLK192MSRC` writer"]
pub struct W(crate::W<HFCLK192MSRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCLK192MSRC_SPEC>;
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
impl core::convert::From<crate::W<HFCLK192MSRC_SPEC>> for W {
    fn from(writer: crate::W<HFCLK192MSRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select which HFCLK192M source is started by the HFCLK192MSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: HFCLK192MSTART task starts HFINT oscillator"]
    HFINT = 0,
    #[doc = "1: HFCLK192MSTART task starts HFXO oscillator"]
    HFXO = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
pub struct SRC_R(crate::FieldReader<bool, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::HFINT,
            true => SRC_A::HFXO,
        }
    }
    #[doc = "Checks if the value of the field is `HFINT`"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        **self == SRC_A::HFINT
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        **self == SRC_A::HFXO
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC` writer - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HFCLK192MSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn hfint(self) -> &'a mut W {
        self.variant(SRC_A::HFINT)
    }
    #[doc = "HFCLK192MSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(SRC_A::HFXO)
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
impl R {
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock source for HFCLK192M\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclk192msrc](index.html) module"]
pub struct HFCLK192MSRC_SPEC;
impl crate::RegisterSpec for HFCLK192MSRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclk192msrc::R](R) reader structure"]
impl crate::Readable for HFCLK192MSRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfclk192msrc::W](W) writer structure"]
impl crate::Writable for HFCLK192MSRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFCLK192MSRC to value 0x01"]
impl crate::Resettable for HFCLK192MSRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
