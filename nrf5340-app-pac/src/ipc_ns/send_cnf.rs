#[doc = "Reader of register SEND_CNF[%s]"]
pub type R = crate::R<u32, super::SEND_CNF>;
#[doc = "Writer for register SEND_CNF[%s]"]
pub type W = crate::W<u32, super::SEND_CNF>;
#[doc = "Register SEND_CNF[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::SEND_CNF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable broadcasting on IPC channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN0_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN0_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN0_A) -> Self {
        match variant {
            CHEN0_A::DISABLE => false,
            CHEN0_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN0`"]
pub type CHEN0_R = crate::R<bool, CHEN0_A>;
impl CHEN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN0_A {
        match self.bits {
            false => CHEN0_A::DISABLE,
            true => CHEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN0_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN0`"]
pub struct CHEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN0_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN0_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN1_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN1_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN1_A) -> Self {
        match variant {
            CHEN1_A::DISABLE => false,
            CHEN1_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN1`"]
pub type CHEN1_R = crate::R<bool, CHEN1_A>;
impl CHEN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN1_A {
        match self.bits {
            false => CHEN1_A::DISABLE,
            true => CHEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN1_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN1`"]
pub struct CHEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN1_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN1_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN2_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN2_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN2_A) -> Self {
        match variant {
            CHEN2_A::DISABLE => false,
            CHEN2_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN2`"]
pub type CHEN2_R = crate::R<bool, CHEN2_A>;
impl CHEN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN2_A {
        match self.bits {
            false => CHEN2_A::DISABLE,
            true => CHEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN2_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN2`"]
pub struct CHEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN2_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN2_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN3_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN3_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN3_A) -> Self {
        match variant {
            CHEN3_A::DISABLE => false,
            CHEN3_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN3`"]
pub type CHEN3_R = crate::R<bool, CHEN3_A>;
impl CHEN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN3_A {
        match self.bits {
            false => CHEN3_A::DISABLE,
            true => CHEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN3_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN3`"]
pub struct CHEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN3_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN3_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN4_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN4_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN4_A) -> Self {
        match variant {
            CHEN4_A::DISABLE => false,
            CHEN4_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN4`"]
pub type CHEN4_R = crate::R<bool, CHEN4_A>;
impl CHEN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN4_A {
        match self.bits {
            false => CHEN4_A::DISABLE,
            true => CHEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN4_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN4`"]
pub struct CHEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN4_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN4_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN5_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN5_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN5_A) -> Self {
        match variant {
            CHEN5_A::DISABLE => false,
            CHEN5_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN5`"]
pub type CHEN5_R = crate::R<bool, CHEN5_A>;
impl CHEN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN5_A {
        match self.bits {
            false => CHEN5_A::DISABLE,
            true => CHEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN5_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN5`"]
pub struct CHEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN5_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN5_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN6_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN6_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN6_A) -> Self {
        match variant {
            CHEN6_A::DISABLE => false,
            CHEN6_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN6`"]
pub type CHEN6_R = crate::R<bool, CHEN6_A>;
impl CHEN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN6_A {
        match self.bits {
            false => CHEN6_A::DISABLE,
            true => CHEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN6_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN6`"]
pub struct CHEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN6_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN6_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN7_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN7_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN7_A) -> Self {
        match variant {
            CHEN7_A::DISABLE => false,
            CHEN7_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN7`"]
pub type CHEN7_R = crate::R<bool, CHEN7_A>;
impl CHEN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN7_A {
        match self.bits {
            false => CHEN7_A::DISABLE,
            true => CHEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN7_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN7`"]
pub struct CHEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN7_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN7_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN8_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN8_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN8_A) -> Self {
        match variant {
            CHEN8_A::DISABLE => false,
            CHEN8_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN8`"]
pub type CHEN8_R = crate::R<bool, CHEN8_A>;
impl CHEN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN8_A {
        match self.bits {
            false => CHEN8_A::DISABLE,
            true => CHEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN8_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN8`"]
pub struct CHEN8_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN8_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN8_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN8_A::ENABLE)
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
#[doc = "Enable broadcasting on IPC channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN9_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN9_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN9_A) -> Self {
        match variant {
            CHEN9_A::DISABLE => false,
            CHEN9_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN9`"]
pub type CHEN9_R = crate::R<bool, CHEN9_A>;
impl CHEN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN9_A {
        match self.bits {
            false => CHEN9_A::DISABLE,
            true => CHEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN9_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN9`"]
pub struct CHEN9_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN9_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN9_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN10_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN10_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN10_A) -> Self {
        match variant {
            CHEN10_A::DISABLE => false,
            CHEN10_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN10`"]
pub type CHEN10_R = crate::R<bool, CHEN10_A>;
impl CHEN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN10_A {
        match self.bits {
            false => CHEN10_A::DISABLE,
            true => CHEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN10_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN10`"]
pub struct CHEN10_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN10_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN10_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN10_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN11_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN11_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN11_A) -> Self {
        match variant {
            CHEN11_A::DISABLE => false,
            CHEN11_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN11`"]
pub type CHEN11_R = crate::R<bool, CHEN11_A>;
impl CHEN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN11_A {
        match self.bits {
            false => CHEN11_A::DISABLE,
            true => CHEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN11_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN11`"]
pub struct CHEN11_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN11_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN11_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN11_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN12_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN12_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN12_A) -> Self {
        match variant {
            CHEN12_A::DISABLE => false,
            CHEN12_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN12`"]
pub type CHEN12_R = crate::R<bool, CHEN12_A>;
impl CHEN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN12_A {
        match self.bits {
            false => CHEN12_A::DISABLE,
            true => CHEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN12_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN12`"]
pub struct CHEN12_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN12_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN12_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN12_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN13_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN13_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN13_A) -> Self {
        match variant {
            CHEN13_A::DISABLE => false,
            CHEN13_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN13`"]
pub type CHEN13_R = crate::R<bool, CHEN13_A>;
impl CHEN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN13_A {
        match self.bits {
            false => CHEN13_A::DISABLE,
            true => CHEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN13_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN13`"]
pub struct CHEN13_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN13_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN13_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN13_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN14_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN14_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN14_A) -> Self {
        match variant {
            CHEN14_A::DISABLE => false,
            CHEN14_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN14`"]
pub type CHEN14_R = crate::R<bool, CHEN14_A>;
impl CHEN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN14_A {
        match self.bits {
            false => CHEN14_A::DISABLE,
            true => CHEN14_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN14_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN14`"]
pub struct CHEN14_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN14_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN14_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN14_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Enable broadcasting on IPC channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CHEN15_A {
    #[doc = "0: Disable broadcast"]
    DISABLE,
    #[doc = "1: Enable broadcast"]
    ENABLE,
}
impl From<CHEN15_A> for bool {
    #[inline(always)]
    fn from(variant: CHEN15_A) -> Self {
        match variant {
            CHEN15_A::DISABLE => false,
            CHEN15_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `CHEN15`"]
pub type CHEN15_R = crate::R<bool, CHEN15_A>;
impl CHEN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHEN15_A {
        match self.bits {
            false => CHEN15_A::DISABLE,
            true => CHEN15_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CHEN15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CHEN15_A::ENABLE
    }
}
#[doc = "Write proxy for field `CHEN15`"]
pub struct CHEN15_W<'a> {
    w: &'a mut W,
}
impl<'a> CHEN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHEN15_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable broadcast"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CHEN15_A::DISABLE)
    }
    #[doc = "Enable broadcast"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CHEN15_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&self) -> CHEN0_R {
        CHEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&self) -> CHEN1_R {
        CHEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&self) -> CHEN2_R {
        CHEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&self) -> CHEN3_R {
        CHEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&self) -> CHEN4_R {
        CHEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&self) -> CHEN5_R {
        CHEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&self) -> CHEN6_R {
        CHEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&self) -> CHEN7_R {
        CHEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&self) -> CHEN8_R {
        CHEN8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&self) -> CHEN9_R {
        CHEN9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&self) -> CHEN10_R {
        CHEN10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&self) -> CHEN11_R {
        CHEN11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&self) -> CHEN12_R {
        CHEN12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&self) -> CHEN13_R {
        CHEN13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&self) -> CHEN14_R {
        CHEN14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&self) -> CHEN15_R {
        CHEN15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable broadcasting on IPC channel 0"]
    #[inline(always)]
    pub fn chen0(&mut self) -> CHEN0_W {
        CHEN0_W { w: self }
    }
    #[doc = "Bit 1 - Enable broadcasting on IPC channel 1"]
    #[inline(always)]
    pub fn chen1(&mut self) -> CHEN1_W {
        CHEN1_W { w: self }
    }
    #[doc = "Bit 2 - Enable broadcasting on IPC channel 2"]
    #[inline(always)]
    pub fn chen2(&mut self) -> CHEN2_W {
        CHEN2_W { w: self }
    }
    #[doc = "Bit 3 - Enable broadcasting on IPC channel 3"]
    #[inline(always)]
    pub fn chen3(&mut self) -> CHEN3_W {
        CHEN3_W { w: self }
    }
    #[doc = "Bit 4 - Enable broadcasting on IPC channel 4"]
    #[inline(always)]
    pub fn chen4(&mut self) -> CHEN4_W {
        CHEN4_W { w: self }
    }
    #[doc = "Bit 5 - Enable broadcasting on IPC channel 5"]
    #[inline(always)]
    pub fn chen5(&mut self) -> CHEN5_W {
        CHEN5_W { w: self }
    }
    #[doc = "Bit 6 - Enable broadcasting on IPC channel 6"]
    #[inline(always)]
    pub fn chen6(&mut self) -> CHEN6_W {
        CHEN6_W { w: self }
    }
    #[doc = "Bit 7 - Enable broadcasting on IPC channel 7"]
    #[inline(always)]
    pub fn chen7(&mut self) -> CHEN7_W {
        CHEN7_W { w: self }
    }
    #[doc = "Bit 8 - Enable broadcasting on IPC channel 8"]
    #[inline(always)]
    pub fn chen8(&mut self) -> CHEN8_W {
        CHEN8_W { w: self }
    }
    #[doc = "Bit 9 - Enable broadcasting on IPC channel 9"]
    #[inline(always)]
    pub fn chen9(&mut self) -> CHEN9_W {
        CHEN9_W { w: self }
    }
    #[doc = "Bit 10 - Enable broadcasting on IPC channel 10"]
    #[inline(always)]
    pub fn chen10(&mut self) -> CHEN10_W {
        CHEN10_W { w: self }
    }
    #[doc = "Bit 11 - Enable broadcasting on IPC channel 11"]
    #[inline(always)]
    pub fn chen11(&mut self) -> CHEN11_W {
        CHEN11_W { w: self }
    }
    #[doc = "Bit 12 - Enable broadcasting on IPC channel 12"]
    #[inline(always)]
    pub fn chen12(&mut self) -> CHEN12_W {
        CHEN12_W { w: self }
    }
    #[doc = "Bit 13 - Enable broadcasting on IPC channel 13"]
    #[inline(always)]
    pub fn chen13(&mut self) -> CHEN13_W {
        CHEN13_W { w: self }
    }
    #[doc = "Bit 14 - Enable broadcasting on IPC channel 14"]
    #[inline(always)]
    pub fn chen14(&mut self) -> CHEN14_W {
        CHEN14_W { w: self }
    }
    #[doc = "Bit 15 - Enable broadcasting on IPC channel 15"]
    #[inline(always)]
    pub fn chen15(&mut self) -> CHEN15_W {
        CHEN15_W { w: self }
    }
}
