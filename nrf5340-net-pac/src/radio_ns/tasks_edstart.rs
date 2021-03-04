#[doc = "Register `TASKS_EDSTART` writer"]
pub struct W(crate::W<TASKS_EDSTART_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TASKS_EDSTART_SPEC>;
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
impl core::convert::From<crate::W<TASKS_EDSTART_SPEC>> for W {
    fn from(writer: crate::W<TASKS_EDSTART_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TASKS_EDSTART_AW {
    #[doc = "1: Trigger task"]
    TRIGGER = 1,
}
impl From<TASKS_EDSTART_AW> for bool {
    #[inline(always)]
    fn from(variant: TASKS_EDSTART_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_EDSTART` writer - Start the energy detect measurement used in IEEE 802.15.4 mode"]
pub struct TASKS_EDSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> TASKS_EDSTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TASKS_EDSTART_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut W {
        self.variant(TASKS_EDSTART_AW::TRIGGER)
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
impl W {
    #[doc = "Bit 0 - Start the energy detect measurement used in IEEE 802.15.4 mode"]
    #[inline(always)]
    pub fn tasks_edstart(&mut self) -> TASKS_EDSTART_W {
        TASKS_EDSTART_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Start the energy detect measurement used in IEEE 802.15.4 mode\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_edstart](index.html) module"]
pub struct TASKS_EDSTART_SPEC;
impl crate::RegisterSpec for TASKS_EDSTART_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tasks_edstart::W](W) writer structure"]
impl crate::Writable for TASKS_EDSTART_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TASKS_EDSTART to value 0"]
impl crate::Resettable for TASKS_EDSTART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
