#[doc = "Register `A4` reader"]
pub struct R(crate::R<A4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<A4_SPEC>> for R {
    fn from(reader: crate::R<A4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A4` writer"]
pub struct W(crate::W<A4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A4_SPEC>;
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
impl core::convert::From<crate::W<A4_SPEC>> for W {
    fn from(writer: crate::W<A4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A4` reader - Slope of fifth piecewise linear function"]
pub struct A4_R(crate::FieldReader<u16, u16>);
impl A4_R {
    pub(crate) fn new(bits: u16) -> Self {
        A4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A4_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A4` writer - Slope of fifth piecewise linear function"]
pub struct A4_W<'a> {
    w: &'a mut W,
}
impl<'a> A4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub fn a4(&self) -> A4_R {
        A4_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of fifth piecewise linear function"]
    #[inline(always)]
    pub fn a4(&mut self) -> A4_W {
        A4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of fifth piecewise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a4](index.html) module"]
pub struct A4_SPEC;
impl crate::RegisterSpec for A4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a4::R](R) reader structure"]
impl crate::Readable for A4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a4::W](W) writer structure"]
impl crate::Writable for A4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A4 to value 0x044e"]
impl crate::Resettable for A4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x044e
    }
}
