#[doc = "Reader of register CTIOUTEN[%s]"]
pub type R = crate::R<u32, super::CTIOUTEN>;
#[doc = "Writer for register CTIOUTEN[%s]"]
pub type W = crate::W<u32, super::CTIOUTEN>;
#[doc = "Register CTIOUTEN[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIOUTEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables a cross trigger event to ctitrigout when channel 0 when is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_0_A {
    #[doc = "0: Channel 0 is ignored by output trigger n"]
    DISABLED,
    #[doc = "1: When an event occur on channel 0, generate an event on output event n (ctitrigout\\[n\\])"]
    ENABLED,
}
impl From<TRIGOUTEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_0_A) -> Self {
        match variant {
            TRIGOUTEN_0_A::DISABLED => false,
            TRIGOUTEN_0_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGOUTEN_0`"]
pub type TRIGOUTEN_0_R = crate::R<bool, TRIGOUTEN_0_A>;
impl TRIGOUTEN_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_0_A {
        match self.bits {
            false => TRIGOUTEN_0_A::DISABLED,
            true => TRIGOUTEN_0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGOUTEN_0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGOUTEN_0_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGOUTEN_0`"]
pub struct TRIGOUTEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 0 is ignored by output trigger n"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_0_A::DISABLED)
    }
    #[doc = "When an event occur on channel 0, generate an event on output event n (ctitrigout\\[n\\])"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_0_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 1 when is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_1_A {
    #[doc = "0: Channel 1 is ignored by output trigger n"]
    DISABLED,
    #[doc = "1: When an event occur on channel 1, generate an event on output event n (ctitrigout\\[n\\])"]
    ENABLED,
}
impl From<TRIGOUTEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_1_A) -> Self {
        match variant {
            TRIGOUTEN_1_A::DISABLED => false,
            TRIGOUTEN_1_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGOUTEN_1`"]
pub type TRIGOUTEN_1_R = crate::R<bool, TRIGOUTEN_1_A>;
impl TRIGOUTEN_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_1_A {
        match self.bits {
            false => TRIGOUTEN_1_A::DISABLED,
            true => TRIGOUTEN_1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGOUTEN_1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGOUTEN_1_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGOUTEN_1`"]
pub struct TRIGOUTEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 1 is ignored by output trigger n"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_1_A::DISABLED)
    }
    #[doc = "When an event occur on channel 1, generate an event on output event n (ctitrigout\\[n\\])"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_1_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 2 when is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_2_A {
    #[doc = "0: Channel 2 is ignored by output trigger n"]
    DISABLED,
    #[doc = "1: When an event occur on channel 2, generate an event on output event n (ctitrigout\\[n\\])"]
    ENABLED,
}
impl From<TRIGOUTEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_2_A) -> Self {
        match variant {
            TRIGOUTEN_2_A::DISABLED => false,
            TRIGOUTEN_2_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGOUTEN_2`"]
pub type TRIGOUTEN_2_R = crate::R<bool, TRIGOUTEN_2_A>;
impl TRIGOUTEN_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_2_A {
        match self.bits {
            false => TRIGOUTEN_2_A::DISABLED,
            true => TRIGOUTEN_2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGOUTEN_2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGOUTEN_2_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGOUTEN_2`"]
pub struct TRIGOUTEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 2 is ignored by output trigger n"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_2_A::DISABLED)
    }
    #[doc = "When an event occur on channel 2, generate an event on output event n (ctitrigout\\[n\\])"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_2_A::ENABLED)
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
#[doc = "Enables a cross trigger event to ctitrigout when channel 3 when is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGOUTEN_3_A {
    #[doc = "0: Channel 3 is ignored by output trigger n"]
    DISABLED,
    #[doc = "1: When an event occur on channel 3, generate an event on output event n (ctitrigout\\[n\\])"]
    ENABLED,
}
impl From<TRIGOUTEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGOUTEN_3_A) -> Self {
        match variant {
            TRIGOUTEN_3_A::DISABLED => false,
            TRIGOUTEN_3_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGOUTEN_3`"]
pub type TRIGOUTEN_3_R = crate::R<bool, TRIGOUTEN_3_A>;
impl TRIGOUTEN_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGOUTEN_3_A {
        match self.bits {
            false => TRIGOUTEN_3_A::DISABLED,
            true => TRIGOUTEN_3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGOUTEN_3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGOUTEN_3_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGOUTEN_3`"]
pub struct TRIGOUTEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGOUTEN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGOUTEN_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Channel 3 is ignored by output trigger n"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_3_A::DISABLED)
    }
    #[doc = "When an event occur on channel 3, generate an event on output event n (ctitrigout\\[n\\])"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGOUTEN_3_A::ENABLED)
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
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 when is activated"]
    #[inline(always)]
    pub fn trigouten_0(&self) -> TRIGOUTEN_0_R {
        TRIGOUTEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 when is activated"]
    #[inline(always)]
    pub fn trigouten_1(&self) -> TRIGOUTEN_1_R {
        TRIGOUTEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 when is activated"]
    #[inline(always)]
    pub fn trigouten_2(&self) -> TRIGOUTEN_2_R {
        TRIGOUTEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 when is activated"]
    #[inline(always)]
    pub fn trigouten_3(&self) -> TRIGOUTEN_3_R {
        TRIGOUTEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a cross trigger event to ctitrigout when channel 0 when is activated"]
    #[inline(always)]
    pub fn trigouten_0(&mut self) -> TRIGOUTEN_0_W {
        TRIGOUTEN_0_W { w: self }
    }
    #[doc = "Bit 1 - Enables a cross trigger event to ctitrigout when channel 1 when is activated"]
    #[inline(always)]
    pub fn trigouten_1(&mut self) -> TRIGOUTEN_1_W {
        TRIGOUTEN_1_W { w: self }
    }
    #[doc = "Bit 2 - Enables a cross trigger event to ctitrigout when channel 2 when is activated"]
    #[inline(always)]
    pub fn trigouten_2(&mut self) -> TRIGOUTEN_2_W {
        TRIGOUTEN_2_W { w: self }
    }
    #[doc = "Bit 3 - Enables a cross trigger event to ctitrigout when channel 3 when is activated"]
    #[inline(always)]
    pub fn trigouten_3(&mut self) -> TRIGOUTEN_3_W {
        TRIGOUTEN_3_W { w: self }
    }
}
