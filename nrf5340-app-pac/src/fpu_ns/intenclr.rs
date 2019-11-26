#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<INVALIDOPERATION_A> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_A) -> Self {
        match variant {
            INVALIDOPERATION_A::DISABLED => false,
            INVALIDOPERATION_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `INVALIDOPERATION`"]
pub type INVALIDOPERATION_R = crate::R<bool, INVALIDOPERATION_A>;
impl INVALIDOPERATION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVALIDOPERATION_A {
        match self.bits {
            false => INVALIDOPERATION_A::DISABLED,
            true => INVALIDOPERATION_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INVALIDOPERATION_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INVALIDOPERATION_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event INVALIDOPERATION\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INVALIDOPERATION_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<INVALIDOPERATION_AW> for bool {
    #[inline(always)]
    fn from(variant: INVALIDOPERATION_AW) -> Self {
        match variant {
            INVALIDOPERATION_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `INVALIDOPERATION`"]
pub struct INVALIDOPERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALIDOPERATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INVALIDOPERATION_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INVALIDOPERATION_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<DIVIDEBYZERO_A> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_A) -> Self {
        match variant {
            DIVIDEBYZERO_A::DISABLED => false,
            DIVIDEBYZERO_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DIVIDEBYZERO`"]
pub type DIVIDEBYZERO_R = crate::R<bool, DIVIDEBYZERO_A>;
impl DIVIDEBYZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVIDEBYZERO_A {
        match self.bits {
            false => DIVIDEBYZERO_A::DISABLED,
            true => DIVIDEBYZERO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIVIDEBYZERO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIVIDEBYZERO_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event DIVIDEBYZERO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIVIDEBYZERO_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<DIVIDEBYZERO_AW> for bool {
    #[inline(always)]
    fn from(variant: DIVIDEBYZERO_AW) -> Self {
        match variant {
            DIVIDEBYZERO_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `DIVIDEBYZERO`"]
pub struct DIVIDEBYZERO_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDEBYZERO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVIDEBYZERO_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DIVIDEBYZERO_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<OVERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_A) -> Self {
        match variant {
            OVERFLOW_A::DISABLED => false,
            OVERFLOW_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `OVERFLOW`"]
pub type OVERFLOW_R = crate::R<bool, OVERFLOW_A>;
impl OVERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERFLOW_A {
        match self.bits {
            false => OVERFLOW_A::DISABLED,
            true => OVERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVERFLOW_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event OVERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVERFLOW_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<OVERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVERFLOW_AW) -> Self {
        match variant {
            OVERFLOW_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `OVERFLOW`"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVERFLOW_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVERFLOW_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_A) -> Self {
        match variant {
            UNDERFLOW_A::DISABLED => false,
            UNDERFLOW_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `UNDERFLOW`"]
pub type UNDERFLOW_R = crate::R<bool, UNDERFLOW_A>;
impl UNDERFLOW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNDERFLOW_A {
        match self.bits {
            false => UNDERFLOW_A::DISABLED,
            true => UNDERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UNDERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UNDERFLOW_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event UNDERFLOW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNDERFLOW_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<UNDERFLOW_AW> for bool {
    #[inline(always)]
    fn from(variant: UNDERFLOW_AW) -> Self {
        match variant {
            UNDERFLOW_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `UNDERFLOW`"]
pub struct UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> UNDERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNDERFLOW_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UNDERFLOW_AW::CLEAR)
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
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<INEXACT_A> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_A) -> Self {
        match variant {
            INEXACT_A::DISABLED => false,
            INEXACT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `INEXACT`"]
pub type INEXACT_R = crate::R<bool, INEXACT_A>;
impl INEXACT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INEXACT_A {
        match self.bits {
            false => INEXACT_A::DISABLED,
            true => INEXACT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == INEXACT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == INEXACT_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event INEXACT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INEXACT_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<INEXACT_AW> for bool {
    #[inline(always)]
    fn from(variant: INEXACT_AW) -> Self {
        match variant {
            INEXACT_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `INEXACT`"]
pub struct INEXACT_W<'a> {
    w: &'a mut W,
}
impl<'a> INEXACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INEXACT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(INEXACT_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_A {
    #[doc = "0: Read: Disabled"]
    DISABLED,
    #[doc = "1: Read: Enabled"]
    ENABLED,
}
impl From<DENORMALINPUT_A> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_A) -> Self {
        match variant {
            DENORMALINPUT_A::DISABLED => false,
            DENORMALINPUT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DENORMALINPUT`"]
pub type DENORMALINPUT_R = crate::R<bool, DENORMALINPUT_A>;
impl DENORMALINPUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DENORMALINPUT_A {
        match self.bits {
            false => DENORMALINPUT_A::DISABLED,
            true => DENORMALINPUT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DENORMALINPUT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DENORMALINPUT_A::ENABLED
    }
}
#[doc = "Write '1' to disable interrupt for event DENORMALINPUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DENORMALINPUT_AW {
    #[doc = "1: Disable"]
    CLEAR,
}
impl From<DENORMALINPUT_AW> for bool {
    #[inline(always)]
    fn from(variant: DENORMALINPUT_AW) -> Self {
        match variant {
            DENORMALINPUT_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `DENORMALINPUT`"]
pub struct DENORMALINPUT_W<'a> {
    w: &'a mut W,
}
impl<'a> DENORMALINPUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DENORMALINPUT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DENORMALINPUT_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&self) -> INVALIDOPERATION_R {
        INVALIDOPERATION_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&self) -> DIVIDEBYZERO_R {
        DIVIDEBYZERO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&self) -> UNDERFLOW_R {
        UNDERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&self) -> INEXACT_R {
        INEXACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&self) -> DENORMALINPUT_R {
        DENORMALINPUT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to disable interrupt for event INVALIDOPERATION"]
    #[inline(always)]
    pub fn invalidoperation(&mut self) -> INVALIDOPERATION_W {
        INVALIDOPERATION_W { w: self }
    }
    #[doc = "Bit 1 - Write '1' to disable interrupt for event DIVIDEBYZERO"]
    #[inline(always)]
    pub fn dividebyzero(&mut self) -> DIVIDEBYZERO_W {
        DIVIDEBYZERO_W { w: self }
    }
    #[doc = "Bit 2 - Write '1' to disable interrupt for event OVERFLOW"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 3 - Write '1' to disable interrupt for event UNDERFLOW"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UNDERFLOW_W {
        UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - Write '1' to disable interrupt for event INEXACT"]
    #[inline(always)]
    pub fn inexact(&mut self) -> INEXACT_W {
        INEXACT_W { w: self }
    }
    #[doc = "Bit 5 - Write '1' to disable interrupt for event DENORMALINPUT"]
    #[inline(always)]
    pub fn denormalinput(&mut self) -> DENORMALINPUT_W {
        DENORMALINPUT_W { w: self }
    }
}
