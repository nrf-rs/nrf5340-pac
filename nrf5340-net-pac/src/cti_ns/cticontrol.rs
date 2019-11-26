#[doc = "Reader of register CTICONTROL"]
pub type R = crate::R<u32, super::CTICONTROL>;
#[doc = "Writer for register CTICONTROL"]
pub type W = crate::W<u32, super::CTICONTROL>;
#[doc = "Register CTICONTROL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTICONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables or disables the CTI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLBEN_A {
    #[doc = "0: All cross-triggering mapping logic functionality is disabled."]
    DISABLED,
    #[doc = "1: Cross-triggering mapping logic functionality is enabled."]
    ENABLED,
}
impl From<GLBEN_A> for bool {
    #[inline(always)]
    fn from(variant: GLBEN_A) -> Self {
        match variant {
            GLBEN_A::DISABLED => false,
            GLBEN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `GLBEN`"]
pub type GLBEN_R = crate::R<bool, GLBEN_A>;
impl GLBEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLBEN_A {
        match self.bits {
            false => GLBEN_A::DISABLED,
            true => GLBEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GLBEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GLBEN_A::ENABLED
    }
}
#[doc = "Write proxy for field `GLBEN`"]
pub struct GLBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLBEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "All cross-triggering mapping logic functionality is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GLBEN_A::DISABLED)
    }
    #[doc = "Cross-triggering mapping logic functionality is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GLBEN_A::ENABLED)
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
    #[doc = "Bit 0 - Enables or disables the CTI"]
    #[inline(always)]
    pub fn glben(&self) -> GLBEN_R {
        GLBEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the CTI"]
    #[inline(always)]
    pub fn glben(&mut self) -> GLBEN_W {
        GLBEN_W { w: self }
    }
}
