#[doc = "Register `COUNTERTOP` reader"]
pub struct R(crate::R<COUNTERTOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNTERTOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<COUNTERTOP_SPEC>> for R {
    fn from(reader: crate::R<COUNTERTOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COUNTERTOP` writer"]
pub struct W(crate::W<COUNTERTOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNTERTOP_SPEC>;
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
impl core::convert::From<crate::W<COUNTERTOP_SPEC>> for W {
    fn from(writer: crate::W<COUNTERTOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTERTOP` reader - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
pub struct COUNTERTOP_R(crate::FieldReader<u16, u16>);
impl COUNTERTOP_R {
    pub(crate) fn new(bits: u16) -> Self {
        COUNTERTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTERTOP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNTERTOP` writer - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
pub struct COUNTERTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNTERTOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub fn countertop(&self) -> COUNTERTOP_R {
        COUNTERTOP_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Value up to which the pulse generator counter counts. This register is ignored when DECODER.MODE=WaveForm and only values from RAM are used."]
    #[inline(always)]
    pub fn countertop(&mut self) -> COUNTERTOP_W {
        COUNTERTOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value up to which the pulse generator counter counts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [countertop](index.html) module"]
pub struct COUNTERTOP_SPEC;
impl crate::RegisterSpec for COUNTERTOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [countertop::R](R) reader structure"]
impl crate::Readable for COUNTERTOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [countertop::W](W) writer structure"]
impl crate::Writable for COUNTERTOP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COUNTERTOP to value 0x03ff"]
impl crate::Resettable for COUNTERTOP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
