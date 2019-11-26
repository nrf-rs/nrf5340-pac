#[doc = "Reader of register EXTSUPPLY"]
pub type R = crate::R<u32, super::EXTSUPPLY>;
#[doc = "Writer for register EXTSUPPLY"]
pub type W = crate::W<u32, super::EXTSUPPLY>;
#[doc = "Register EXTSUPPLY `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::EXTSUPPLY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Enable external circuitry to be supplied from VDD pin (output of VREGH stage).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTSUPPLY_A {
    #[doc = "1: No current can be drawn from the VDD pin."]
    DISABLED,
    #[doc = "0: It is allowed to supply external circuitry from the VDD pin."]
    ENABLED,
}
impl From<EXTSUPPLY_A> for bool {
    #[inline(always)]
    fn from(variant: EXTSUPPLY_A) -> Self {
        match variant {
            EXTSUPPLY_A::DISABLED => true,
            EXTSUPPLY_A::ENABLED => false,
        }
    }
}
#[doc = "Reader of field `EXTSUPPLY`"]
pub type EXTSUPPLY_R = crate::R<bool, EXTSUPPLY_A>;
impl EXTSUPPLY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTSUPPLY_A {
        match self.bits {
            true => EXTSUPPLY_A::DISABLED,
            false => EXTSUPPLY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTSUPPLY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTSUPPLY_A::ENABLED
    }
}
#[doc = "Write proxy for field `EXTSUPPLY`"]
pub struct EXTSUPPLY_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTSUPPLY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTSUPPLY_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No current can be drawn from the VDD pin."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTSUPPLY_A::DISABLED)
    }
    #[doc = "It is allowed to supply external circuitry from the VDD pin."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTSUPPLY_A::ENABLED)
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
    #[doc = "Bit 0 - Enable external circuitry to be supplied from VDD pin (output of VREGH stage)."]
    #[inline(always)]
    pub fn extsupply(&self) -> EXTSUPPLY_R {
        EXTSUPPLY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable external circuitry to be supplied from VDD pin (output of VREGH stage)."]
    #[inline(always)]
    pub fn extsupply(&mut self) -> EXTSUPPLY_W {
        EXTSUPPLY_W { w: self }
    }
}
