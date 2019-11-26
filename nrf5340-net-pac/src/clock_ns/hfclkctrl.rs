#[doc = "Reader of register HFCLKCTRL"]
pub type R = crate::R<u32, super::HFCLKCTRL>;
#[doc = "Writer for register HFCLKCTRL"]
pub type W = crate::W<u32, super::HFCLKCTRL>;
#[doc = "Register HFCLKCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::HFCLKCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "High frequency clock HCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCLK_A {
    #[doc = "0: Divide HFCLK by 1"]
    DIV1,
    #[doc = "1: Divide HFCLK by 2"]
    DIV2,
}
impl From<HCLK_A> for bool {
    #[inline(always)]
    fn from(variant: HCLK_A) -> Self {
        match variant {
            HCLK_A::DIV1 => false,
            HCLK_A::DIV2 => true,
        }
    }
}
#[doc = "Reader of field `HCLK`"]
pub type HCLK_R = crate::R<bool, HCLK_A>;
impl HCLK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCLK_A {
        match self.bits {
            false => HCLK_A::DIV1,
            true => HCLK_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HCLK_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HCLK_A::DIV2
    }
}
#[doc = "Write proxy for field `HCLK`"]
pub struct HCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Divide HFCLK by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HCLK_A::DIV1)
    }
    #[doc = "Divide HFCLK by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HCLK_A::DIV2)
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
    #[doc = "Bit 0 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&self) -> HCLK_R {
        HCLK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - High frequency clock HCLK"]
    #[inline(always)]
    pub fn hclk(&mut self) -> HCLK_W {
        HCLK_W { w: self }
    }
}
