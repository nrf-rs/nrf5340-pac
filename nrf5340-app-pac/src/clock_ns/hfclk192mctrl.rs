#[doc = "Reader of register HFCLK192MCTRL"]
pub type R = crate::R<u32, super::HFCLK192MCTRL>;
#[doc = "Writer for register HFCLK192MCTRL"]
pub type W = crate::W<u32, super::HFCLK192MCTRL>;
#[doc = "Register HFCLK192MCTRL `reset()`'s with value 0x02"]
impl crate::ResetValue for super::HFCLK192MCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "High frequency clock HCLK192M\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HCLK192M_A {
    #[doc = "0: Divide HFCLK192M by 1"]
    DIV1,
    #[doc = "1: Divide HFCLK192M by 2"]
    DIV2,
    #[doc = "2: Divide HFCLK192M by 4"]
    DIV4,
}
impl From<HCLK192M_A> for u8 {
    #[inline(always)]
    fn from(variant: HCLK192M_A) -> Self {
        match variant {
            HCLK192M_A::DIV1 => 0,
            HCLK192M_A::DIV2 => 1,
            HCLK192M_A::DIV4 => 2,
        }
    }
}
#[doc = "Reader of field `HCLK192M`"]
pub type HCLK192M_R = crate::R<u8, HCLK192M_A>;
impl HCLK192M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, HCLK192M_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(HCLK192M_A::DIV1),
            1 => Val(HCLK192M_A::DIV2),
            2 => Val(HCLK192M_A::DIV4),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HCLK192M_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HCLK192M_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HCLK192M_A::DIV4
    }
}
#[doc = "Write proxy for field `HCLK192M`"]
pub struct HCLK192M_W<'a> {
    w: &'a mut W,
}
impl<'a> HCLK192M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HCLK192M_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Divide HFCLK192M by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV1)
    }
    #[doc = "Divide HFCLK192M by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV2)
    }
    #[doc = "Divide HFCLK192M by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HCLK192M_A::DIV4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&self) -> HCLK192M_R {
        HCLK192M_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High frequency clock HCLK192M"]
    #[inline(always)]
    pub fn hclk192m(&mut self) -> HCLK192M_W {
        HCLK192M_W { w: self }
    }
}
