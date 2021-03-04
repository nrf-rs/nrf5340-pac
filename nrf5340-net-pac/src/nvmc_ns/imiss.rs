#[doc = "Register `IMISS` reader"]
pub struct R(crate::R<IMISS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMISS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMISS_SPEC>> for R {
    fn from(reader: crate::R<IMISS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IMISS` writer"]
pub struct W(crate::W<IMISS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IMISS_SPEC>;
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
impl core::convert::From<crate::W<IMISS_SPEC>> for W {
    fn from(writer: crate::W<IMISS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MISSES` reader - Number of cache misses Write zero to clear"]
pub struct MISSES_R(crate::FieldReader<u32, u32>);
impl MISSES_R {
    pub(crate) fn new(bits: u32) -> Self {
        MISSES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISSES_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISSES` writer - Number of cache misses Write zero to clear"]
pub struct MISSES_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Number of cache misses Write zero to clear"]
    #[inline(always)]
    pub fn misses(&self) -> MISSES_R {
        MISSES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache misses Write zero to clear"]
    #[inline(always)]
    pub fn misses(&mut self) -> MISSES_W {
        MISSES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I-code cache miss counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imiss](index.html) module"]
pub struct IMISS_SPEC;
impl crate::RegisterSpec for IMISS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imiss::R](R) reader structure"]
impl crate::Readable for IMISS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [imiss::W](W) writer structure"]
impl crate::Writable for IMISS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IMISS to value 0"]
impl crate::Resettable for IMISS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
