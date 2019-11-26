#[doc = "Reader of register CTIINEN[%s]"]
pub type R = crate::R<u32, super::CTIINEN>;
#[doc = "Writer for register CTIINEN[%s]"]
pub type W = crate::W<u32, super::CTIINEN>;
#[doc = "Register CTIINEN[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIINEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enables a cross trigger event to channel 0 when a ctitrigin input is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGINEN_0_A {
    #[doc = "0: Input trigger n events are ignored by channel 0"]
    DISABLED,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 0"]
    ENABLED,
}
impl From<TRIGINEN_0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGINEN_0_A) -> Self {
        match variant {
            TRIGINEN_0_A::DISABLED => false,
            TRIGINEN_0_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGINEN_0`"]
pub type TRIGINEN_0_R = crate::R<bool, TRIGINEN_0_A>;
impl TRIGINEN_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGINEN_0_A {
        match self.bits {
            false => TRIGINEN_0_A::DISABLED,
            true => TRIGINEN_0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGINEN_0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGINEN_0_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGINEN_0`"]
pub struct TRIGINEN_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGINEN_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGINEN_0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input trigger n events are ignored by channel 0"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGINEN_0_A::DISABLED)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 0"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGINEN_0_A::ENABLED)
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
#[doc = "Enables a cross trigger event to channel 1 when a ctitrigin input is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGINEN_1_A {
    #[doc = "0: Input trigger n events are ignored by channel 1"]
    DISABLED,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 1"]
    ENABLED,
}
impl From<TRIGINEN_1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGINEN_1_A) -> Self {
        match variant {
            TRIGINEN_1_A::DISABLED => false,
            TRIGINEN_1_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGINEN_1`"]
pub type TRIGINEN_1_R = crate::R<bool, TRIGINEN_1_A>;
impl TRIGINEN_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGINEN_1_A {
        match self.bits {
            false => TRIGINEN_1_A::DISABLED,
            true => TRIGINEN_1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGINEN_1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGINEN_1_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGINEN_1`"]
pub struct TRIGINEN_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGINEN_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGINEN_1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input trigger n events are ignored by channel 1"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGINEN_1_A::DISABLED)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 1"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGINEN_1_A::ENABLED)
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
#[doc = "Enables a cross trigger event to channel 2 when a ctitrigin input is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGINEN_2_A {
    #[doc = "0: Input trigger n events are ignored by channel 2"]
    DISABLED,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 2"]
    ENABLED,
}
impl From<TRIGINEN_2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGINEN_2_A) -> Self {
        match variant {
            TRIGINEN_2_A::DISABLED => false,
            TRIGINEN_2_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGINEN_2`"]
pub type TRIGINEN_2_R = crate::R<bool, TRIGINEN_2_A>;
impl TRIGINEN_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGINEN_2_A {
        match self.bits {
            false => TRIGINEN_2_A::DISABLED,
            true => TRIGINEN_2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGINEN_2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGINEN_2_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGINEN_2`"]
pub struct TRIGINEN_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGINEN_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGINEN_2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input trigger n events are ignored by channel 2"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGINEN_2_A::DISABLED)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 2"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGINEN_2_A::ENABLED)
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
#[doc = "Enables a cross trigger event to channel 3 when a ctitrigin input is activated\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGINEN_3_A {
    #[doc = "0: Input trigger n events are ignored by channel 3"]
    DISABLED,
    #[doc = "1: When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 3"]
    ENABLED,
}
impl From<TRIGINEN_3_A> for bool {
    #[inline(always)]
    fn from(variant: TRIGINEN_3_A) -> Self {
        match variant {
            TRIGINEN_3_A::DISABLED => false,
            TRIGINEN_3_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `TRIGINEN_3`"]
pub type TRIGINEN_3_R = crate::R<bool, TRIGINEN_3_A>;
impl TRIGINEN_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIGINEN_3_A {
        match self.bits {
            false => TRIGINEN_3_A::DISABLED,
            true => TRIGINEN_3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRIGINEN_3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRIGINEN_3_A::ENABLED
    }
}
#[doc = "Write proxy for field `TRIGINEN_3`"]
pub struct TRIGINEN_3_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGINEN_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGINEN_3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Input trigger n events are ignored by channel 3"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRIGINEN_3_A::DISABLED)
    }
    #[doc = "When an event is received on input trigger n (ctitrigin\\[n\\]) generate an event on channel 3"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TRIGINEN_3_A::ENABLED)
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
    #[doc = "Bit 0 - Enables a cross trigger event to channel 0 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_0(&self) -> TRIGINEN_0_R {
        TRIGINEN_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables a cross trigger event to channel 1 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_1(&self) -> TRIGINEN_1_R {
        TRIGINEN_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables a cross trigger event to channel 2 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_2(&self) -> TRIGINEN_2_R {
        TRIGINEN_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enables a cross trigger event to channel 3 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_3(&self) -> TRIGINEN_3_R {
        TRIGINEN_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables a cross trigger event to channel 0 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_0(&mut self) -> TRIGINEN_0_W {
        TRIGINEN_0_W { w: self }
    }
    #[doc = "Bit 1 - Enables a cross trigger event to channel 1 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_1(&mut self) -> TRIGINEN_1_W {
        TRIGINEN_1_W { w: self }
    }
    #[doc = "Bit 2 - Enables a cross trigger event to channel 2 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_2(&mut self) -> TRIGINEN_2_W {
        TRIGINEN_2_W { w: self }
    }
    #[doc = "Bit 3 - Enables a cross trigger event to channel 3 when a ctitrigin input is activated"]
    #[inline(always)]
    pub fn triginen_3(&mut self) -> TRIGINEN_3_W {
        TRIGINEN_3_W { w: self }
    }
}
