#[doc = "Writer for register CTIAPPPULSE"]
pub type W = crate::W<u32, super::CTIAPPPULSE>;
#[doc = "Register CTIAPPPULSE `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIAPPPULSE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_0_AW {
    #[doc = "1: Generates an event pulse on channel 0"]
    GENERATE,
}
impl From<APPULSE_0_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_0_AW) -> Self {
        match variant {
            APPULSE_0_AW::GENERATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPULSE_0`"]
pub struct APPULSE_0_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an event pulse on channel 0"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_0_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_1_AW {
    #[doc = "1: Generates an event pulse on channel 1"]
    GENERATE,
}
impl From<APPULSE_1_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_1_AW) -> Self {
        match variant {
            APPULSE_1_AW::GENERATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPULSE_1`"]
pub struct APPULSE_1_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an event pulse on channel 1"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_1_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_2_AW {
    #[doc = "1: Generates an event pulse on channel 2"]
    GENERATE,
}
impl From<APPULSE_2_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_2_AW) -> Self {
        match variant {
            APPULSE_2_AW::GENERATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPULSE_2`"]
pub struct APPULSE_2_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an event pulse on channel 2"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_2_AW::GENERATE)
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
#[doc = "Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPULSE_3_AW {
    #[doc = "1: Generates an event pulse on channel 3"]
    GENERATE,
}
impl From<APPULSE_3_AW> for bool {
    #[inline(always)]
    fn from(variant: APPULSE_3_AW) -> Self {
        match variant {
            APPULSE_3_AW::GENERATE => true,
        }
    }
}
#[doc = "Write proxy for field `APPULSE_3`"]
pub struct APPULSE_3_W<'a> {
    w: &'a mut W,
}
impl<'a> APPULSE_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: APPULSE_3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generates an event pulse on channel 3"]
    #[inline(always)]
    pub fn generate(self) -> &'a mut W {
        self.variant(APPULSE_3_AW::GENERATE)
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
    #[doc = "Bit 0 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_0(&mut self) -> APPULSE_0_W {
        APPULSE_0_W { w: self }
    }
    #[doc = "Bit 1 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_1(&mut self) -> APPULSE_1_W {
        APPULSE_1_W { w: self }
    }
    #[doc = "Bit 2 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_2(&mut self) -> APPULSE_2_W {
        APPULSE_2_W { w: self }
    }
    #[doc = "Bit 3 - Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel."]
    #[inline(always)]
    pub fn appulse_3(&mut self) -> APPULSE_3_W {
        APPULSE_3_W { w: self }
    }
}
