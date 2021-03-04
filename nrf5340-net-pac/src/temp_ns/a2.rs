#[doc = "Register `A2` reader"]
pub struct R(crate::R<A2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<A2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<A2_SPEC>> for R {
    fn from(reader: crate::R<A2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `A2` writer"]
pub struct W(crate::W<A2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<A2_SPEC>;
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
impl core::convert::From<crate::W<A2_SPEC>> for W {
    fn from(writer: crate::W<A2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `A2` reader - Slope of third piecewise linear function"]
pub struct A2_R(crate::FieldReader<u16, u16>);
impl A2_R {
    pub(crate) fn new(bits: u16) -> Self {
        A2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A2` writer - Slope of third piecewise linear function"]
pub struct A2_W<'a> {
    w: &'a mut W,
}
impl<'a> A2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Slope of third piecewise linear function"]
    #[inline(always)]
    pub fn a2(&self) -> A2_R {
        A2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of third piecewise linear function"]
    #[inline(always)]
    pub fn a2(&mut self) -> A2_W {
        A2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slope of third piecewise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [a2](index.html) module"]
pub struct A2_SPEC;
impl crate::RegisterSpec for A2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [a2::R](R) reader structure"]
impl crate::Readable for A2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [a2::W](W) writer structure"]
impl crate::Writable for A2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets A2 to value 0x0355"]
impl crate::Resettable for A2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0355
    }
}
