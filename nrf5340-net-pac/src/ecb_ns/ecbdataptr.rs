#[doc = "Register `ECBDATAPTR` reader"]
pub struct R(crate::R<ECBDATAPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECBDATAPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ECBDATAPTR_SPEC>> for R {
    fn from(reader: crate::R<ECBDATAPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECBDATAPTR` writer"]
pub struct W(crate::W<ECBDATAPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECBDATAPTR_SPEC>;
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
impl core::convert::From<crate::W<ECBDATAPTR_SPEC>> for W {
    fn from(writer: crate::W<ECBDATAPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ECBDATAPTR` reader - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub struct ECBDATAPTR_R(crate::FieldReader<u32, u32>);
impl ECBDATAPTR_R {
    pub(crate) fn new(bits: u32) -> Self {
        ECBDATAPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ECBDATAPTR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ECBDATAPTR` writer - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
pub struct ECBDATAPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ECBDATAPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&self) -> ECBDATAPTR_R {
        ECBDATAPTR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer to the ECB data structure (see Table 1 ECB data structure overview)"]
    #[inline(always)]
    pub fn ecbdataptr(&mut self) -> ECBDATAPTR_W {
        ECBDATAPTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ECB block encrypt memory pointers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecbdataptr](index.html) module"]
pub struct ECBDATAPTR_SPEC;
impl crate::RegisterSpec for ECBDATAPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecbdataptr::R](R) reader structure"]
impl crate::Readable for ECBDATAPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecbdataptr::W](W) writer structure"]
impl crate::Writable for ECBDATAPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECBDATAPTR to value 0"]
impl crate::Resettable for ECBDATAPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
