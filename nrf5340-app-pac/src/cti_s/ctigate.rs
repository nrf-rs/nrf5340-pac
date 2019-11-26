#[doc = "Reader of register CTIGATE"]
pub type R = crate::R<u32, super::CTIGATE>;
#[doc = "Writer for register CTIGATE"]
pub type W = crate::W<u32, super::CTIGATE>;
#[doc = "Register CTIGATE `reset()`'s with value 0x0f"]
impl crate::ResetValue for super::CTIGATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0f
    }
}
#[doc = "Enable ctichout0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_0_A {
    #[doc = "1: Enable ctichout channel 0 propagation"]
    ENABLED,
    #[doc = "0: Disable ctichout channel 0 propagation"]
    DISABLED,
}
impl From<CTIGATEEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_0_A) -> Self {
        match variant {
            CTIGATEEN_0_A::ENABLED => true,
            CTIGATEEN_0_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CTIGATEEN_0`"]
pub type CTIGATEEN_0_R = crate::R<bool, CTIGATEEN_0_A>;
impl CTIGATEEN_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_0_A {
        match self.bits {
            true => CTIGATEEN_0_A::ENABLED,
            false => CTIGATEEN_0_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_0_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_0_A::DISABLED
    }
}
#[doc = "Write proxy for field `CTIGATEEN_0`"]
pub struct CTIGATEEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ctichout channel 0 propagation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 0 propagation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_0_A::DISABLED)
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
#[doc = "Enable ctichout1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_1_A {
    #[doc = "1: Enable ctichout channel 1 propagation"]
    ENABLED,
    #[doc = "0: Disable ctichout channel 1 propagation"]
    DISABLED,
}
impl From<CTIGATEEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_1_A) -> Self {
        match variant {
            CTIGATEEN_1_A::ENABLED => true,
            CTIGATEEN_1_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CTIGATEEN_1`"]
pub type CTIGATEEN_1_R = crate::R<bool, CTIGATEEN_1_A>;
impl CTIGATEEN_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_1_A {
        match self.bits {
            true => CTIGATEEN_1_A::ENABLED,
            false => CTIGATEEN_1_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_1_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_1_A::DISABLED
    }
}
#[doc = "Write proxy for field `CTIGATEEN_1`"]
pub struct CTIGATEEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ctichout channel 1 propagation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 1 propagation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_1_A::DISABLED)
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
#[doc = "Enable ctichout2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_2_A {
    #[doc = "1: Enable ctichout channel 2 propagation"]
    ENABLED,
    #[doc = "0: Disable ctichout channel 2 propagation"]
    DISABLED,
}
impl From<CTIGATEEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_2_A) -> Self {
        match variant {
            CTIGATEEN_2_A::ENABLED => true,
            CTIGATEEN_2_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CTIGATEEN_2`"]
pub type CTIGATEEN_2_R = crate::R<bool, CTIGATEEN_2_A>;
impl CTIGATEEN_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_2_A {
        match self.bits {
            true => CTIGATEEN_2_A::ENABLED,
            false => CTIGATEEN_2_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_2_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_2_A::DISABLED
    }
}
#[doc = "Write proxy for field `CTIGATEEN_2`"]
pub struct CTIGATEEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ctichout channel 2 propagation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 2 propagation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_2_A::DISABLED)
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
#[doc = "Enable ctichout3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTIGATEEN_3_A {
    #[doc = "1: Enable ctichout channel 3 propagation"]
    ENABLED,
    #[doc = "0: Disable ctichout channel 3 propagation"]
    DISABLED,
}
impl From<CTIGATEEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: CTIGATEEN_3_A) -> Self {
        match variant {
            CTIGATEEN_3_A::ENABLED => true,
            CTIGATEEN_3_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CTIGATEEN_3`"]
pub type CTIGATEEN_3_R = crate::R<bool, CTIGATEEN_3_A>;
impl CTIGATEEN_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTIGATEEN_3_A {
        match self.bits {
            true => CTIGATEEN_3_A::ENABLED,
            false => CTIGATEEN_3_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTIGATEEN_3_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTIGATEEN_3_A::DISABLED
    }
}
#[doc = "Write proxy for field `CTIGATEEN_3`"]
pub struct CTIGATEEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTIGATEEN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTIGATEEN_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable ctichout channel 3 propagation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::ENABLED)
    }
    #[doc = "Disable ctichout channel 3 propagation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTIGATEEN_3_A::DISABLED)
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
impl R {
    #[doc = "Bit 0 - Enable ctichout0"]
    #[inline(always)]
    pub fn ctigateen_0(&self) -> CTIGATEEN_0_R {
        CTIGATEEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable ctichout1"]
    #[inline(always)]
    pub fn ctigateen_1(&self) -> CTIGATEEN_1_R {
        CTIGATEEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable ctichout2"]
    #[inline(always)]
    pub fn ctigateen_2(&self) -> CTIGATEEN_2_R {
        CTIGATEEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable ctichout3"]
    #[inline(always)]
    pub fn ctigateen_3(&self) -> CTIGATEEN_3_R {
        CTIGATEEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ctichout0"]
    #[inline(always)]
    pub fn ctigateen_0(&mut self) -> CTIGATEEN_0_W {
        CTIGATEEN_0_W { w: self }
    }
    #[doc = "Bit 1 - Enable ctichout1"]
    #[inline(always)]
    pub fn ctigateen_1(&mut self) -> CTIGATEEN_1_W {
        CTIGATEEN_1_W { w: self }
    }
    #[doc = "Bit 2 - Enable ctichout2"]
    #[inline(always)]
    pub fn ctigateen_2(&mut self) -> CTIGATEEN_2_W {
        CTIGATEEN_2_W { w: self }
    }
    #[doc = "Bit 3 - Enable ctichout3"]
    #[inline(always)]
    pub fn ctigateen_3(&mut self) -> CTIGATEEN_3_W {
        CTIGATEEN_3_W { w: self }
    }
}
