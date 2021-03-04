#[doc = "Register `ONESHOTEN[%s]` reader"]
pub struct R(crate::R<ONESHOTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ONESHOTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ONESHOTEN_SPEC>> for R {
    fn from(reader: crate::R<ONESHOTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ONESHOTEN[%s]` writer"]
pub struct W(crate::W<ONESHOTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ONESHOTEN_SPEC>;
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
impl core::convert::From<crate::W<ONESHOTEN_SPEC>> for W {
    fn from(writer: crate::W<ONESHOTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable one-shot operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTEN_A {
    #[doc = "0: Disable one-shot operation"]
    DISABLE = 0,
    #[doc = "1: Enable one-shot operation"]
    ENABLE = 1,
}
impl From<ONESHOTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOTEN` reader - Enable one-shot operation"]
pub struct ONESHOTEN_R(crate::FieldReader<bool, ONESHOTEN_A>);
impl ONESHOTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONESHOTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTEN_A {
        match self.bits {
            false => ONESHOTEN_A::DISABLE,
            true => ONESHOTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == ONESHOTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == ONESHOTEN_A::ENABLE
    }
}
impl core::ops::Deref for ONESHOTEN_R {
    type Target = crate::FieldReader<bool, ONESHOTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONESHOTEN` writer - Enable one-shot operation"]
pub struct ONESHOTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable one-shot operation"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ONESHOTEN_A::DISABLE)
    }
    #[doc = "Enable one-shot operation"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ONESHOTEN_A::ENABLE)
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
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&self) -> ONESHOTEN_R {
        ONESHOTEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable one-shot operation"]
    #[inline(always)]
    pub fn oneshoten(&mut self) -> ONESHOTEN_W {
        ONESHOTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Description collection: Enable one-shot operation for Capture/Compare channel n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oneshoten](index.html) module"]
pub struct ONESHOTEN_SPEC;
impl crate::RegisterSpec for ONESHOTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oneshoten::R](R) reader structure"]
impl crate::Readable for ONESHOTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oneshoten::W](W) writer structure"]
impl crate::Writable for ONESHOTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ONESHOTEN[%s]
to value 0"]
impl crate::Resettable for ONESHOTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
