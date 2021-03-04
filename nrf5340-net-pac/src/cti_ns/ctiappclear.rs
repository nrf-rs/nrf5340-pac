#[doc = "Register `CTIAPPCLEAR` writer"]
pub struct W(crate::W<CTIAPPCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTIAPPCLEAR_SPEC>;
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
impl core::convert::From<crate::W<CTIAPPCLEAR_SPEC>> for W {
    fn from(writer: crate::W<CTIAPPCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_0_AW {
    #[doc = "1: Clears the event for channel 0."]
    CLEAR = 1,
}
impl From<APPCLEAR_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_0` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub struct APPCLEAR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the event for channel 0."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_0_AW::CLEAR)
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
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_1_AW {
    #[doc = "1: Clears the event for channel 1."]
    CLEAR = 1,
}
impl From<APPCLEAR_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_1` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub struct APPCLEAR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the event for channel 1."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_1_AW::CLEAR)
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
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_2_AW {
    #[doc = "1: Clears the event for channel 2."]
    CLEAR = 1,
}
impl From<APPCLEAR_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_2` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub struct APPCLEAR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the event for channel 2."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_2_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_3_AW {
    #[doc = "1: Clears the event for channel 3."]
    CLEAR = 1,
}
impl From<APPCLEAR_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APPCLEAR_3` writer - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
pub struct APPCLEAR_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clears the event for channel 3."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_3_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_0(&mut self) -> APPCLEAR_0_W {
        APPCLEAR_0_W { w: self }
    }
    #[doc = "Bit 1 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_1(&mut self) -> APPCLEAR_1_W {
        APPCLEAR_1_W { w: self }
    }
    #[doc = "Bit 2 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_2(&mut self) -> APPCLEAR_2_W {
        APPCLEAR_2_W { w: self }
    }
    #[doc = "Bit 3 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_3(&mut self) -> APPCLEAR_3_W {
        APPCLEAR_3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTI Application Trigger Clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctiappclear](index.html) module"]
pub struct CTIAPPCLEAR_SPEC;
impl crate::RegisterSpec for CTIAPPCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctiappclear::W](W) writer structure"]
impl crate::Writable for CTIAPPCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTIAPPCLEAR to value 0"]
impl crate::Resettable for CTIAPPCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
