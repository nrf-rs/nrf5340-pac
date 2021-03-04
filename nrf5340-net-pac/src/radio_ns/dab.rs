#[doc = "Register `DAB[%s]` reader"]
pub struct R(crate::R<DAB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DAB_SPEC>> for R {
    fn from(reader: crate::R<DAB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAB[%s]` writer"]
pub struct W(crate::W<DAB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAB_SPEC>;
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
impl core::convert::From<crate::W<DAB_SPEC>> for W {
    fn from(writer: crate::W<DAB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAB` reader - Device address base segment n"]
pub struct DAB_R(crate::FieldReader<u32, u32>);
impl DAB_R {
    pub(crate) fn new(bits: u32) -> Self {
        DAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAB` writer - Device address base segment n"]
pub struct DAB_W<'a> {
    w: &'a mut W,
}
impl<'a> DAB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    pub fn dab(&self) -> DAB_R {
        DAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Device address base segment n"]
    #[inline(always)]
    pub fn dab(&mut self) -> DAB_W {
        DAB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Device address base segment n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dab](index.html) module"]
pub struct DAB_SPEC;
impl crate::RegisterSpec for DAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dab::R](R) reader structure"]
impl crate::Readable for DAB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dab::W](W) writer structure"]
impl crate::Writable for DAB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAB[%s]
to value 0"]
impl crate::Resettable for DAB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
