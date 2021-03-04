#[doc = "Register `T2` reader"]
pub struct R(crate::R<T2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<T2_SPEC>> for R {
    fn from(reader: crate::R<T2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T2` writer"]
pub struct W(crate::W<T2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T2_SPEC>;
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
impl core::convert::From<crate::W<T2_SPEC>> for W {
    fn from(writer: crate::W<T2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T2` reader - Endpoint of third piecewise linear function"]
pub struct T2_R(crate::FieldReader<u8, u8>);
impl T2_R {
    pub(crate) fn new(bits: u8) -> Self {
        T2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T2` writer - Endpoint of third piecewise linear function"]
pub struct T2_W<'a> {
    w: &'a mut W,
}
impl<'a> T2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub fn t2(&self) -> T2_R {
        T2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Endpoint of third piecewise linear function"]
    #[inline(always)]
    pub fn t2(&mut self) -> T2_W {
        T2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint of third piecewise linear function\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t2](index.html) module"]
pub struct T2_SPEC;
impl crate::RegisterSpec for T2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t2::R](R) reader structure"]
impl crate::Readable for T2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t2::W](W) writer structure"]
impl crate::Writable for T2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T2 to value 0x10"]
impl crate::Resettable for T2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
