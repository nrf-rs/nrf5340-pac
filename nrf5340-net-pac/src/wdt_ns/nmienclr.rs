#[doc = "Register `NMIENCLR` reader"]
pub struct R(crate::R<NMIENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMIENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<NMIENCLR_SPEC>> for R {
    fn from(reader: crate::R<NMIENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMIENCLR` writer"]
pub struct W(crate::W<NMIENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMIENCLR_SPEC>;
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
impl core::convert::From<crate::W<NMIENCLR_SPEC>> for W {
    fn from(writer: crate::W<NMIENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Write '1' to disable interrupt for event TIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Write '1' to disable interrupt for event TIMEOUT"]
pub struct TIMEOUT_R(crate::FieldReader<bool, TIMEOUT_A>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::DISABLED,
            true => TIMEOUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TIMEOUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TIMEOUT_A::ENABLED
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event TIMEOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<TIMEOUT_AW> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` writer - Write '1' to disable interrupt for event TIMEOUT"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TIMEOUT_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_A {
    #[doc = "0: Read: Disabled"]
    DISABLED = 0,
    #[doc = "1: Read: Enabled"]
    ENABLED = 1,
}
impl From<STOPPED_A> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to disable interrupt for event STOPPED"]
pub struct STOPPED_R(crate::FieldReader<bool, STOPPED_A>);
impl STOPPED_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPPED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPPED_A {
        match self.bits {
            false => STOPPED_A::DISABLED,
            true => STOPPED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == STOPPED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == STOPPED_A::ENABLED
    }
}
impl core::ops::Deref for STOPPED_R {
    type Target = crate::FieldReader<bool, STOPPED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Write '1' to disable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPPED_AW {
    #[doc = "1: Disable"]
    CLEAR = 1,
}
impl From<STOPPED_AW> for bool {
    #[inline(always)]
    fn from(variant: STOPPED_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to disable interrupt for event STOPPED"]
pub struct STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPPED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPPED_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(STOPPED_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> STOPPED_R {
        STOPPED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> STOPPED_W {
        STOPPED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Disable interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmienclr](index.html) module"]
pub struct NMIENCLR_SPEC;
impl crate::RegisterSpec for NMIENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nmienclr::R](R) reader structure"]
impl crate::Readable for NMIENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmienclr::W](W) writer structure"]
impl crate::Writable for NMIENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMIENCLR to value 0"]
impl crate::Resettable for NMIENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
