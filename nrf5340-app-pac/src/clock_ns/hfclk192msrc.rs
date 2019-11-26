#[doc = "Reader of register HFCLK192MSRC"]
pub type R = crate::R<u32, super::HFCLK192MSRC>;
#[doc = "Writer for register HFCLK192MSRC"]
pub type W = crate::W<u32, super::HFCLK192MSRC>;
#[doc = "Register HFCLK192MSRC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::HFCLK192MSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Select which HFCLK192M source is started by the HFCLK192MSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: HFCLK192MSTART task starts HFINT oscillator"]
    HFINT,
    #[doc = "1: HFCLK192MSTART task starts HFXO oscillator"]
    HFXO,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        match variant {
            SRC_A::HFINT => false,
            SRC_A::HFXO => true,
        }
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<bool, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::HFINT,
            true => SRC_A::HFXO,
        }
    }
    #[doc = "Checks if the value of the field is `HFINT`"]
    #[inline(always)]
    pub fn is_hfint(&self) -> bool {
        *self == SRC_A::HFINT
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SRC_A::HFXO
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HFCLK192MSTART task starts HFINT oscillator"]
    #[inline(always)]
    pub fn hfint(self) -> &'a mut W {
        self.variant(SRC_A::HFINT)
    }
    #[doc = "HFCLK192MSTART task starts HFXO oscillator"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(SRC_A::HFXO)
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
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select which HFCLK192M source is started by the HFCLK192MSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
