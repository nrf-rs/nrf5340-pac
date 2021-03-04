#[doc = "Register `NRFFW[%s]` reader"]
pub struct R(crate::R<NRFFW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NRFFW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NRFFW_SPEC>> for R {
    fn from(reader: crate::R<NRFFW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NRFFW[%s]` writer"]
pub struct W(crate::W<NRFFW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NRFFW_SPEC>;
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
impl core::convert::From<crate::W<NRFFW_SPEC>> for W {
    fn from(writer: crate::W<NRFFW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NRFFW` reader - Reserved for Nordic firmware design"]
pub struct NRFFW_R(crate::FieldReader<u32, u32>);
impl NRFFW_R {
    pub(crate) fn new(bits: u32) -> Self {
        NRFFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NRFFW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NRFFW` writer - Reserved for Nordic firmware design"]
pub struct NRFFW_W<'a> {
    w: &'a mut W,
}
impl<'a> NRFFW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&self) -> NRFFW_R {
        NRFFW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved for Nordic firmware design"]
    #[inline(always)]
    pub fn nrffw(&mut self) -> NRFFW_W {
        NRFFW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Reserved for Nordic firmware design\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrffw](index.html) module"]
pub struct NRFFW_SPEC;
impl crate::RegisterSpec for NRFFW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nrffw::R](R) reader structure"]
impl crate::Readable for NRFFW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nrffw::W](W) writer structure"]
impl crate::Writable for NRFFW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NRFFW[%s]
to value 0xffff_ffff"]
impl crate::Resettable for NRFFW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
