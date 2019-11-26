#[doc = "Reader of register CTIAPPSET"]
pub type R = crate::R<u32, super::CTIAPPSET>;
#[doc = "Writer for register CTIAPPSET"]
pub type W = crate::W<u32, super::CTIAPPSET>;
#[doc = "Register CTIAPPSET `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIAPPSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Application trigger event for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_0_A {
    #[doc = "0: Application trigger 0 is inactive."]
    INACTIVE,
    #[doc = "1: Application trigger 0 is active."]
    ACTIVE,
}
impl From<APPSET_0_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_0_A) -> Self {
        match variant {
            APPSET_0_A::INACTIVE => false,
            APPSET_0_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `APPSET_0`"]
pub type APPSET_0_R = crate::R<bool, APPSET_0_A>;
impl APPSET_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_0_A {
        match self.bits {
            false => APPSET_0_A::INACTIVE,
            true => APPSET_0_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_0_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_0_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_0_AW {
    #[doc = "1: Generate channel event for channel 0."]
    ACTIVATE,
}
impl From<APPSET_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_0_AW) -> Self {
        match variant {
            APPSET_0_AW::ACTIVATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPSET_0`"]
pub struct APPSET_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSET_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPSET_0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate channel event for channel 0."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_0_AW::ACTIVATE)
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
#[doc = "Application trigger event for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_1_A {
    #[doc = "0: Application trigger 1 is inactive."]
    INACTIVE,
    #[doc = "1: Application trigger 1 is active."]
    ACTIVE,
}
impl From<APPSET_1_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_1_A) -> Self {
        match variant {
            APPSET_1_A::INACTIVE => false,
            APPSET_1_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `APPSET_1`"]
pub type APPSET_1_R = crate::R<bool, APPSET_1_A>;
impl APPSET_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_1_A {
        match self.bits {
            false => APPSET_1_A::INACTIVE,
            true => APPSET_1_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_1_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_1_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_1_AW {
    #[doc = "1: Generate channel event for channel 1."]
    ACTIVATE,
}
impl From<APPSET_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_1_AW) -> Self {
        match variant {
            APPSET_1_AW::ACTIVATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPSET_1`"]
pub struct APPSET_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSET_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPSET_1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate channel event for channel 1."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_1_AW::ACTIVATE)
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
#[doc = "Application trigger event for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_2_A {
    #[doc = "0: Application trigger 2 is inactive."]
    INACTIVE,
    #[doc = "1: Application trigger 2 is active."]
    ACTIVE,
}
impl From<APPSET_2_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_2_A) -> Self {
        match variant {
            APPSET_2_A::INACTIVE => false,
            APPSET_2_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `APPSET_2`"]
pub type APPSET_2_R = crate::R<bool, APPSET_2_A>;
impl APPSET_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_2_A {
        match self.bits {
            false => APPSET_2_A::INACTIVE,
            true => APPSET_2_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_2_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_2_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_2_AW {
    #[doc = "1: Generate channel event for channel 2."]
    ACTIVATE,
}
impl From<APPSET_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_2_AW) -> Self {
        match variant {
            APPSET_2_AW::ACTIVATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPSET_2`"]
pub struct APPSET_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSET_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPSET_2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate channel event for channel 2."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_2_AW::ACTIVATE)
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
#[doc = "Application trigger event for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_3_A {
    #[doc = "0: Application trigger 3 is inactive."]
    INACTIVE,
    #[doc = "1: Application trigger 3 is active."]
    ACTIVE,
}
impl From<APPSET_3_A> for bool {
    #[inline(always)]
    fn from(variant: APPSET_3_A) -> Self {
        match variant {
            APPSET_3_A::INACTIVE => false,
            APPSET_3_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `APPSET_3`"]
pub type APPSET_3_R = crate::R<bool, APPSET_3_A>;
impl APPSET_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPSET_3_A {
        match self.bits {
            false => APPSET_3_A::INACTIVE,
            true => APPSET_3_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == APPSET_3_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == APPSET_3_A::ACTIVE
    }
}
#[doc = "Application trigger event for channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPSET_3_AW {
    #[doc = "1: Generate channel event for channel 3."]
    ACTIVATE,
}
impl From<APPSET_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPSET_3_AW) -> Self {
        match variant {
            APPSET_3_AW::ACTIVATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPSET_3`"]
pub struct APPSET_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APPSET_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPSET_3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate channel event for channel 3."]
    #[inline(always)]
    pub fn activate(self) -> &'a mut W {
        self.variant(APPSET_3_AW::ACTIVATE)
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
    #[doc = "Bit 0 - Application trigger event for channel 0"]
    #[inline(always)]
    pub fn appset_0(&self) -> APPSET_0_R {
        APPSET_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Application trigger event for channel 1"]
    #[inline(always)]
    pub fn appset_1(&self) -> APPSET_1_R {
        APPSET_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Application trigger event for channel 2"]
    #[inline(always)]
    pub fn appset_2(&self) -> APPSET_2_R {
        APPSET_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Application trigger event for channel 3"]
    #[inline(always)]
    pub fn appset_3(&self) -> APPSET_3_R {
        APPSET_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Application trigger event for channel 0"]
    #[inline(always)]
    pub fn appset_0(&mut self) -> APPSET_0_W {
        APPSET_0_W { w: self }
    }
    #[doc = "Bit 1 - Application trigger event for channel 1"]
    #[inline(always)]
    pub fn appset_1(&mut self) -> APPSET_1_W {
        APPSET_1_W { w: self }
    }
    #[doc = "Bit 2 - Application trigger event for channel 2"]
    #[inline(always)]
    pub fn appset_2(&mut self) -> APPSET_2_W {
        APPSET_2_W { w: self }
    }
    #[doc = "Bit 3 - Application trigger event for channel 3"]
    #[inline(always)]
    pub fn appset_3(&mut self) -> APPSET_3_W {
        APPSET_3_W { w: self }
    }
}
