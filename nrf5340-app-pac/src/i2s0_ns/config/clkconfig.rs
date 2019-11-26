#[doc = "Reader of register CLKCONFIG"]
pub type R = crate::R<u32, super::CLKCONFIG>;
#[doc = "Writer for register CLKCONFIG"]
pub type W = crate::W<u32, super::CLKCONFIG>;
#[doc = "Register CLKCONFIG `reset()`'s with value 0"]
impl crate::ResetValue for super::CLKCONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSRC_A {
    #[doc = "0: 32MHz peripheral clock"]
    PCLK32M,
    #[doc = "1: Audio PLL clock"]
    ACLK,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        match variant {
            CLKSRC_A::PCLK32M => false,
            CLKSRC_A::ACLK => true,
        }
    }
}
#[doc = "Reader of field `CLKSRC`"]
pub type CLKSRC_R = crate::R<bool, CLKSRC_A>;
impl CLKSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::PCLK32M,
            true => CLKSRC_A::ACLK,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK32M`"]
    #[inline(always)]
    pub fn is_pclk32m(&self) -> bool {
        *self == CLKSRC_A::PCLK32M
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == CLKSRC_A::ACLK
    }
}
#[doc = "Write proxy for field `CLKSRC`"]
pub struct CLKSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "32MHz peripheral clock"]
    #[inline(always)]
    pub fn pclk32m(self) -> &'a mut W {
        self.variant(CLKSRC_A::PCLK32M)
    }
    #[doc = "Audio PLL clock"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(CLKSRC_A::ACLK)
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
#[doc = "Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable bypass"]
    DISABLE,
    #[doc = "1: Enable bypass"]
    ENABLE,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        match variant {
            BYPASS_A::DISABLE => false,
            BYPASS_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLE,
            true => BYPASS_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == BYPASS_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == BYPASS_A::ENABLE
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable bypass"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLE)
    }
    #[doc = "Enable bypass"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock source selection"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> CLKSRC_W {
        CLKSRC_W { w: self }
    }
    #[doc = "Bit 8 - Bypass clock generator. MCK will be equal to source input. If bypass is enabled the MCKFREQ setting has no effect."]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
}
