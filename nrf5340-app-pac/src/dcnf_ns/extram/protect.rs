#[doc = "Reader of register PROTECT"]
pub type R = crate::R<u32, super::PROTECT>;
#[doc = "Writer for register PROTECT"]
pub type W = crate::W<u32, super::PROTECT>;
#[doc = "Register PROTECT `reset()`'s with value 0"]
impl crate::ResetValue for super::PROTECT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Control access to slave 0 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE0_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE0_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE0_A) -> Self {
        match variant {
            SLAVE0_A::ALLOWED => false,
            SLAVE0_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE0`"]
pub type SLAVE0_R = crate::R<bool, SLAVE0_A>;
impl SLAVE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE0_A {
        match self.bits {
            false => SLAVE0_A::ALLOWED,
            true => SLAVE0_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE0_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE0_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE0`"]
pub struct SLAVE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE0_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE0_A::BLOCKED)
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
#[doc = "Control access to slave 1 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE1_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE1_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE1_A) -> Self {
        match variant {
            SLAVE1_A::ALLOWED => false,
            SLAVE1_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE1`"]
pub type SLAVE1_R = crate::R<bool, SLAVE1_A>;
impl SLAVE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE1_A {
        match self.bits {
            false => SLAVE1_A::ALLOWED,
            true => SLAVE1_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE1_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE1_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE1`"]
pub struct SLAVE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE1_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE1_A::BLOCKED)
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
#[doc = "Control access to slave 2 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE2_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE2_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE2_A) -> Self {
        match variant {
            SLAVE2_A::ALLOWED => false,
            SLAVE2_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE2`"]
pub type SLAVE2_R = crate::R<bool, SLAVE2_A>;
impl SLAVE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE2_A {
        match self.bits {
            false => SLAVE2_A::ALLOWED,
            true => SLAVE2_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE2_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE2_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE2`"]
pub struct SLAVE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE2_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE2_A::BLOCKED)
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
#[doc = "Control access to slave 3 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE3_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE3_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE3_A) -> Self {
        match variant {
            SLAVE3_A::ALLOWED => false,
            SLAVE3_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE3`"]
pub type SLAVE3_R = crate::R<bool, SLAVE3_A>;
impl SLAVE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE3_A {
        match self.bits {
            false => SLAVE3_A::ALLOWED,
            true => SLAVE3_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE3_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE3_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE3`"]
pub struct SLAVE3_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE3_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE3_A::BLOCKED)
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
#[doc = "Control access to slave 4 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE4_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE4_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE4_A) -> Self {
        match variant {
            SLAVE4_A::ALLOWED => false,
            SLAVE4_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE4`"]
pub type SLAVE4_R = crate::R<bool, SLAVE4_A>;
impl SLAVE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE4_A {
        match self.bits {
            false => SLAVE4_A::ALLOWED,
            true => SLAVE4_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE4_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE4_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE4`"]
pub struct SLAVE4_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE4_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE4_A::BLOCKED)
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
#[doc = "Control access to slave 5 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE5_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE5_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE5_A) -> Self {
        match variant {
            SLAVE5_A::ALLOWED => false,
            SLAVE5_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE5`"]
pub type SLAVE5_R = crate::R<bool, SLAVE5_A>;
impl SLAVE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE5_A {
        match self.bits {
            false => SLAVE5_A::ALLOWED,
            true => SLAVE5_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE5_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE5_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE5`"]
pub struct SLAVE5_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE5_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE5_A::BLOCKED)
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
#[doc = "Control access to slave 6 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE6_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE6_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE6_A) -> Self {
        match variant {
            SLAVE6_A::ALLOWED => false,
            SLAVE6_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE6`"]
pub type SLAVE6_R = crate::R<bool, SLAVE6_A>;
impl SLAVE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE6_A {
        match self.bits {
            false => SLAVE6_A::ALLOWED,
            true => SLAVE6_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE6_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE6_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE6`"]
pub struct SLAVE6_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE6_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE6_A::BLOCKED)
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
#[doc = "Control access to slave 7 of master EXTRAM\\[n\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLAVE7_A {
    #[doc = "0: Access to slave is allowed"]
    ALLOWED,
    #[doc = "1: Access to slave is blocked"]
    BLOCKED,
}
impl From<SLAVE7_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE7_A) -> Self {
        match variant {
            SLAVE7_A::ALLOWED => false,
            SLAVE7_A::BLOCKED => true,
        }
    }
}
#[doc = "Reader of field `SLAVE7`"]
pub type SLAVE7_R = crate::R<bool, SLAVE7_A>;
impl SLAVE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE7_A {
        match self.bits {
            false => SLAVE7_A::ALLOWED,
            true => SLAVE7_A::BLOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `ALLOWED`"]
    #[inline(always)]
    pub fn is_allowed(&self) -> bool {
        *self == SLAVE7_A::ALLOWED
    }
    #[doc = "Checks if the value of the field is `BLOCKED`"]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SLAVE7_A::BLOCKED
    }
}
#[doc = "Write proxy for field `SLAVE7`"]
pub struct SLAVE7_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLAVE7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Access to slave is allowed"]
    #[inline(always)]
    pub fn allowed(self) -> &'a mut W {
        self.variant(SLAVE7_A::ALLOWED)
    }
    #[doc = "Access to slave is blocked"]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut W {
        self.variant(SLAVE7_A::BLOCKED)
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
impl R {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&self) -> SLAVE0_R {
        SLAVE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&self) -> SLAVE1_R {
        SLAVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&self) -> SLAVE2_R {
        SLAVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&self) -> SLAVE3_R {
        SLAVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&self) -> SLAVE4_R {
        SLAVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&self) -> SLAVE5_R {
        SLAVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&self) -> SLAVE6_R {
        SLAVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&self) -> SLAVE7_R {
        SLAVE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control access to slave 0 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave0(&mut self) -> SLAVE0_W {
        SLAVE0_W { w: self }
    }
    #[doc = "Bit 1 - Control access to slave 1 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave1(&mut self) -> SLAVE1_W {
        SLAVE1_W { w: self }
    }
    #[doc = "Bit 2 - Control access to slave 2 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave2(&mut self) -> SLAVE2_W {
        SLAVE2_W { w: self }
    }
    #[doc = "Bit 3 - Control access to slave 3 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave3(&mut self) -> SLAVE3_W {
        SLAVE3_W { w: self }
    }
    #[doc = "Bit 4 - Control access to slave 4 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave4(&mut self) -> SLAVE4_W {
        SLAVE4_W { w: self }
    }
    #[doc = "Bit 5 - Control access to slave 5 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave5(&mut self) -> SLAVE5_W {
        SLAVE5_W { w: self }
    }
    #[doc = "Bit 6 - Control access to slave 6 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave6(&mut self) -> SLAVE6_W {
        SLAVE6_W { w: self }
    }
    #[doc = "Bit 7 - Control access to slave 7 of master EXTRAM\\[n\\]"]
    #[inline(always)]
    pub fn slave7(&mut self) -> SLAVE7_W {
        SLAVE7_W { w: self }
    }
}
