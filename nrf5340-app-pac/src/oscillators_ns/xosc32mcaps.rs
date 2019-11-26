#[doc = "Reader of register XOSC32MCAPS"]
pub type R = crate::R<u32, super::XOSC32MCAPS>;
#[doc = "Writer for register XOSC32MCAPS"]
pub type W = crate::W<u32, super::XOSC32MCAPS>;
#[doc = "Register XOSC32MCAPS `reset()`'s with value 0"]
impl crate::ResetValue for super::XOSC32MCAPS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAPVALUE`"]
pub type CAPVALUE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAPVALUE`"]
pub struct CAPVALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPVALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Enable on-chip capacitors on XC1 and XC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Capacitor disabled (use external caps)"]
    DISABLED,
    #[doc = "1: Capacitor enabled"]
    ENABLED,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        match variant {
            ENABLE_A::DISABLED => false,
            ENABLE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capacitor disabled (use external caps)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Capacitor enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&self) -> CAPVALUE_R {
        CAPVALUE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Value representing capacitance, calculated using provided equation"]
    #[inline(always)]
    pub fn capvalue(&mut self) -> CAPVALUE_W {
        CAPVALUE_W { w: self }
    }
    #[doc = "Bit 8 - Enable on-chip capacitors on XC1 and XC2"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
