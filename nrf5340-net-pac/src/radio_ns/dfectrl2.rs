#[doc = "Register `DFECTRL2` reader"]
pub struct R(crate::R<DFECTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFECTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFECTRL2_SPEC>> for R {
    fn from(reader: crate::R<DFECTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFECTRL2` writer"]
pub struct W(crate::W<DFECTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFECTRL2_SPEC>;
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
impl core::convert::From<crate::W<DFECTRL2_SPEC>> for W {
    fn from(writer: crate::W<DFECTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSWITCHOFFSET` reader - Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
pub struct TSWITCHOFFSET_R(crate::FieldReader<u16, u16>);
impl TSWITCHOFFSET_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSWITCHOFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSWITCHOFFSET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSWITCHOFFSET` writer - Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
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
#[doc = "Field `TSAMPLEOFFSET` reader - Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
pub struct TSAMPLEOFFSET_R(crate::FieldReader<u16, u16>);
impl TSAMPLEOFFSET_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSAMPLEOFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSAMPLEOFFSET_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSAMPLEOFFSET` writer - Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
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
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&self) -> TSWITCHOFFSET_R {
        TSWITCHOFFSET_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:27 - Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
    #[inline(always)]
    pub fn tsampleoffset(&self) -> TSAMPLEOFFSET_R {
        TSAMPLEOFFSET_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Signed value offset after the end of the CRC before starting switching in number of 16 MHz clock cycles"]
    #[inline(always)]
    pub fn tswitchoffset(&mut self) -> TSWITCHOFFSET_W {
        TSWITCHOFFSET_W { w: self }
    }
    #[doc = "Bits 16:27 - Signed value offset in number of 16 MHz clock cycles for fine tuning of the sampling instant for all IQ samples. With TSAMPLEOFFSET=0 the first sample is taken immediately at the start of the reference period"]
    #[inline(always)]
    pub fn tsampleoffset(&mut self) -> TSAMPLEOFFSET_W {
        TSAMPLEOFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start offset for Direction finding\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfectrl2](index.html) module"]
pub struct DFECTRL2_SPEC;
impl crate::RegisterSpec for DFECTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfectrl2::R](R) reader structure"]
impl crate::Readable for DFECTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfectrl2::W](W) writer structure"]
impl crate::Writable for DFECTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFECTRL2 to value 0"]
impl crate::Resettable for DFECTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
