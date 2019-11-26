#[doc = "Reader of register VREQH"]
pub type R = crate::R<u32, super::VREQH>;
#[doc = "Writer for register VREQH"]
pub type W = crate::W<u32, super::VREQH>;
#[doc = "Register VREQH `reset()`'s with value 0"]
impl crate::ResetValue for super::VREQH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Request high voltage\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREQH_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<VREQH_A> for bool {
    #[inline(always)]
    fn from(variant: VREQH_A) -> Self {
        match variant {
            VREQH_A::DISABLED => false,
            VREQH_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `VREQH`"]
pub type VREQH_R = crate::R<bool, VREQH_A>;
impl VREQH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREQH_A {
        match self.bits {
            false => VREQH_A::DISABLED,
            true => VREQH_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREQH_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREQH_A::ENABLED
    }
}
#[doc = "Write proxy for field `VREQH`"]
pub struct VREQH_W<'a> {
    w: &'a mut W,
}
impl<'a> VREQH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VREQH_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREQH_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREQH_A::ENABLED)
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
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&self) -> VREQH_R {
        VREQH_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Request high voltage"]
    #[inline(always)]
    pub fn vreqh(&mut self) -> VREQH_W {
        VREQH_W { w: self }
    }
}
