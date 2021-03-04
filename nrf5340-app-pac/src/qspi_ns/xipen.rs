#[doc = "Register `XIPEN` reader"]
pub struct R(crate::R<XIPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XIPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<XIPEN_SPEC>> for R {
    fn from(reader: crate::R<XIPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XIPEN` writer"]
pub struct W(crate::W<XIPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XIPEN_SPEC>;
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
impl core::convert::From<crate::W<XIPEN_SPEC>> for W {
    fn from(writer: crate::W<XIPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable XIP AHB Slave interface and access to XIP memory range\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIPEN_A {
    #[doc = "0: Disable XIP interface"]
    DISABLE = 0,
    #[doc = "1: Enable XIP interface"]
    ENABLE = 1,
}
impl From<XIPEN_A> for bool {
    #[inline(always)]
    fn from(variant: XIPEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XIPEN` reader - Enable XIP AHB Slave interface and access to XIP memory range"]
pub struct XIPEN_R(crate::FieldReader<bool, XIPEN_A>);
impl XIPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIPEN_A {
        match self.bits {
            false => XIPEN_A::DISABLE,
            true => XIPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == XIPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == XIPEN_A::ENABLE
    }
}
impl core::ops::Deref for XIPEN_R {
    type Target = crate::FieldReader<bool, XIPEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPEN` writer - Enable XIP AHB Slave interface and access to XIP memory range"]
pub struct XIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIPEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable XIP interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XIPEN_A::DISABLE)
    }
    #[doc = "Enable XIP interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XIPEN_A::ENABLE)
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
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&self) -> XIPEN_R {
        XIPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&mut self) -> XIPEN_W {
        XIPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Execute in Place operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xipen](index.html) module"]
pub struct XIPEN_SPEC;
impl crate::RegisterSpec for XIPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xipen::R](R) reader structure"]
impl crate::Readable for XIPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xipen::W](W) writer structure"]
impl crate::Writable for XIPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XIPEN to value 0x01"]
impl crate::Resettable for XIPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
