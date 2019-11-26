#[doc = "Reader of register FORCEOFF"]
pub type R = crate::R<u32, super::FORCEOFF>;
#[doc = "Writer for register FORCEOFF"]
pub type W = crate::W<u32, super::FORCEOFF>;
#[doc = "Register FORCEOFF `reset()`'s with value 0x01"]
impl crate::ResetValue for super::FORCEOFF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Force off power and clock in Network core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFF_A {
    #[doc = "0: Release force off signal"]
    RELEASE,
    #[doc = "1: Hold force off signal"]
    HOLD,
}
impl From<FORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFF_A) -> Self {
        match variant {
            FORCEOFF_A::RELEASE => false,
            FORCEOFF_A::HOLD => true,
        }
    }
}
#[doc = "Reader of field `FORCEOFF`"]
pub type FORCEOFF_R = crate::R<bool, FORCEOFF_A>;
impl FORCEOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFF_A {
        match self.bits {
            false => FORCEOFF_A::RELEASE,
            true => FORCEOFF_A::HOLD,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == FORCEOFF_A::RELEASE
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == FORCEOFF_A::HOLD
    }
}
#[doc = "Write proxy for field `FORCEOFF`"]
pub struct FORCEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Release force off signal"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(FORCEOFF_A::RELEASE)
    }
    #[doc = "Hold force off signal"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(FORCEOFF_A::HOLD)
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
    #[doc = "Bit 0 - Force off power and clock in Network core"]
    #[inline(always)]
    pub fn forceoff(&self) -> FORCEOFF_R {
        FORCEOFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force off power and clock in Network core"]
    #[inline(always)]
    pub fn forceoff(&mut self) -> FORCEOFF_W {
        FORCEOFF_W { w: self }
    }
}
