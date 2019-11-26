#[doc = "Writer for register CTIINTACK"]
pub type W = crate::W<u32, super::CTIINTACK>;
#[doc = "Register CTIINTACK `reset()`'s with value 0"]
impl crate::ResetValue for super::CTIINTACK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGREQ_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<DEBUGREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: DEBUGREQ_AW) -> Self {
        match variant {
            DEBUGREQ_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `DEBUGREQ`"]
pub struct DEBUGREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEBUGREQ_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(DEBUGREQ_AW::ACKNOWLEDGE)
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
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURESTART_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<CPURESTART_AW> for bool {
    #[inline(always)]
    fn from(variant: CPURESTART_AW) -> Self {
        match variant {
            CPURESTART_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `CPURESTART`"]
pub struct CPURESTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CPURESTART_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPURESTART_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(CPURESTART_AW::ACKNOWLEDGE)
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
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<UNUSED0_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED0_AW) -> Self {
        match variant {
            UNUSED0_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `UNUSED0`"]
pub struct UNUSED0_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED0_AW::ACKNOWLEDGE)
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
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED1_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<UNUSED1_AW> for bool {
    #[inline(always)]
    fn from(variant: UNUSED1_AW) -> Self {
        match variant {
            UNUSED1_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `UNUSED1`"]
pub struct UNUSED1_W<'a> {
    w: &'a mut W,
}
impl<'a> UNUSED1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UNUSED1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(UNUSED1_AW::ACKNOWLEDGE)
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
#[doc = "ETM Event Input 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN0_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<ETMEVTIN0_AW> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN0_AW) -> Self {
        match variant {
            ETMEVTIN0_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `ETMEVTIN0`"]
pub struct ETMEVTIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMEVTIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMEVTIN0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(ETMEVTIN0_AW::ACKNOWLEDGE)
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
#[doc = "ETM Event Input 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN1_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<ETMEVTIN1_AW> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN1_AW) -> Self {
        match variant {
            ETMEVTIN1_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `ETMEVTIN1`"]
pub struct ETMEVTIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMEVTIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMEVTIN1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(ETMEVTIN1_AW::ACKNOWLEDGE)
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
#[doc = "ETM Event Input 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN2_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<ETMEVTIN2_AW> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN2_AW) -> Self {
        match variant {
            ETMEVTIN2_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `ETMEVTIN2`"]
pub struct ETMEVTIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMEVTIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMEVTIN2_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(ETMEVTIN2_AW::ACKNOWLEDGE)
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
#[doc = "ETM Event Input 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN3_AW {
    #[doc = "1: Clears the ctitrigout"]
    ACKNOWLEDGE,
}
impl From<ETMEVTIN3_AW> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN3_AW) -> Self {
        match variant {
            ETMEVTIN3_AW::ACKNOWLEDGE => true,
        }
    }
}
#[doc = "Write proxy for field `ETMEVTIN3`"]
pub struct ETMEVTIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMEVTIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMEVTIN3_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Clears the ctitrigout"]
    #[inline(always)]
    pub fn acknowledge(self) -> &'a mut W {
        self.variant(ETMEVTIN3_AW::ACKNOWLEDGE)
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
impl W {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&mut self) -> DEBUGREQ_W {
        DEBUGREQ_W { w: self }
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&mut self) -> CPURESTART_W {
        CPURESTART_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&mut self) -> UNUSED0_W {
        UNUSED0_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&mut self) -> UNUSED1_W {
        UNUSED1_W { w: self }
    }
    #[doc = "Bit 4 - ETM Event Input 0"]
    #[inline(always)]
    pub fn etmevtin0(&mut self) -> ETMEVTIN0_W {
        ETMEVTIN0_W { w: self }
    }
    #[doc = "Bit 5 - ETM Event Input 1"]
    #[inline(always)]
    pub fn etmevtin1(&mut self) -> ETMEVTIN1_W {
        ETMEVTIN1_W { w: self }
    }
    #[doc = "Bit 6 - ETM Event Input 2"]
    #[inline(always)]
    pub fn etmevtin2(&mut self) -> ETMEVTIN2_W {
        ETMEVTIN2_W { w: self }
    }
    #[doc = "Bit 7 - ETM Event Input 3"]
    #[inline(always)]
    pub fn etmevtin3(&mut self) -> ETMEVTIN3_W {
        ETMEVTIN3_W { w: self }
    }
}
