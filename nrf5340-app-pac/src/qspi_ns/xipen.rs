#[doc = "Reader of register XIPEN"]
pub type R = crate::R<u32, super::XIPEN>;
#[doc = "Writer for register XIPEN"]
pub type W = crate::W<u32, super::XIPEN>;
#[doc = "Register XIPEN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::XIPEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enable XIP AHB Slave interface and access to XIP memory range\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum XIPEN_A {
    #[doc = "0: Disable XIP interface"]
    DISABLE,
    #[doc = "1: Enable XIP interface"]
    ENABLE,
}
impl From<XIPEN_A> for bool {
    #[inline(always)]
    fn from(variant: XIPEN_A) -> Self {
        match variant {
            XIPEN_A::DISABLE => false,
            XIPEN_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `XIPEN`"]
pub type XIPEN_R = crate::R<bool, XIPEN_A>;
impl XIPEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIPEN_A {
        match self.bits {
            false => XIPEN_A::DISABLE,
            true => XIPEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XIPEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XIPEN_A::ENABLE
    }
}
#[doc = "Write proxy for field `XIPEN`"]
pub struct XIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIPEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable XIP interface"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(XIPEN_A::DISABLE)
    }
    #[doc = "Enable XIP interface"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(XIPEN_A::ENABLE)
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
impl R {
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&self) -> XIPEN_R {
        XIPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable XIP AHB Slave interface and access to XIP memory range"]
    #[inline(always)]
    pub fn xipen(&mut self) -> XIPEN_W {
        XIPEN_W { w: self }
    }
}
