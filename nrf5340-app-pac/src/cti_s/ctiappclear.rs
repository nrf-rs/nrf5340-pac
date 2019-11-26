#[doc = "Writer for register CTIAPPCLEAR"]
pub type W = crate::W<u32, super::CTIAPPCLEAR>;
#[doc = "Register CTIAPPCLEAR `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIAPPCLEAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_0_AW {
    #[doc = "1: Clears the event for channel 0"]
    CLEAR,
}
impl From<APPCLEAR_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_0_AW) -> Self {
        match variant {
            APPCLEAR_0_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `APPCLEAR_0`"]
pub struct APPCLEAR_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the event for channel 0"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_0_AW::CLEAR)
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
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_1_AW {
    #[doc = "1: Clears the event for channel 1"]
    CLEAR,
}
impl From<APPCLEAR_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_1_AW) -> Self {
        match variant {
            APPCLEAR_1_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `APPCLEAR_1`"]
pub struct APPCLEAR_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the event for channel 1"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_1_AW::CLEAR)
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
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_2_AW {
    #[doc = "1: Clears the event for channel 2"]
    CLEAR,
}
impl From<APPCLEAR_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_2_AW) -> Self {
        match variant {
            APPCLEAR_2_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `APPCLEAR_2`"]
pub struct APPCLEAR_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the event for channel 2"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_2_AW::CLEAR)
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
#[doc = "Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPCLEAR_3_AW {
    #[doc = "1: Clears the event for channel 3"]
    CLEAR,
}
impl From<APPCLEAR_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPCLEAR_3_AW) -> Self {
        match variant {
            APPCLEAR_3_AW::CLEAR => true,
        }
    }
}
#[doc = "Write proxy for field `APPCLEAR_3`"]
pub struct APPCLEAR_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APPCLEAR_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPCLEAR_3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the event for channel 3"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(APPCLEAR_3_AW::CLEAR)
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
impl W {
    #[doc = "Bit 0 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_0(&mut self) -> APPCLEAR_0_W {
        APPCLEAR_0_W { w: self }
    }
    #[doc = "Bit 1 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_1(&mut self) -> APPCLEAR_1_W {
        APPCLEAR_1_W { w: self }
    }
    #[doc = "Bit 2 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_2(&mut self) -> APPCLEAR_2_W {
        APPCLEAR_2_W { w: self }
    }
    #[doc = "Bit 3 - Sets the corresponding bits in the CTIAPPSET to 0. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appclear_3(&mut self) -> APPCLEAR_3_W {
        APPCLEAR_3_W { w: self }
    }
}
