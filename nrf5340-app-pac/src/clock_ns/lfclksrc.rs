#[doc = "Reader of register LFCLKSRC"]
pub type R = crate::R<u32, super::LFCLKSRC>;
#[doc = "Writer for register LFCLKSRC"]
pub type W = crate::W<u32, super::LFCLKSRC>;
#[doc = "Register LFCLKSRC `reset()`'s with value 0x01"]
impl crate::ResetValue for super::LFCLKSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Select which LFCLK source is started by the LFCLKSTART task\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: 32.768 kHz ultra-low power RC oscillator"]
    LFULP,
    #[doc = "1: 32.768 kHz RC oscillator"]
    LFRC,
    #[doc = "2: 32.768 kHz crystal oscillator"]
    LFXO,
    #[doc = "3: 32.768 kHz synthesized from HFCLK"]
    LFSYNT,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        match variant {
            SRC_A::LFULP => 0,
            SRC_A::LFRC => 1,
            SRC_A::LFXO => 2,
            SRC_A::LFSYNT => 3,
        }
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            0 => SRC_A::LFULP,
            1 => SRC_A::LFRC,
            2 => SRC_A::LFXO,
            3 => SRC_A::LFSYNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LFULP`"]
    #[inline(always)]
    pub fn is_lfulp(&self) -> bool {
        *self == SRC_A::LFULP
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        *self == SRC_A::LFRC
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SRC_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFSYNT`"]
    #[inline(always)]
    pub fn is_lfsynt(&self) -> bool {
        *self == SRC_A::LFSYNT
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
            self.bits(variant.into())
        }
    }
    #[doc = "32.768 kHz ultra-low power RC oscillator"]
    #[inline(always)]
    pub fn lfulp(self) -> &'a mut W {
        self.variant(SRC_A::LFULP)
    }
    #[doc = "32.768 kHz RC oscillator"]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(SRC_A::LFRC)
    }
    #[doc = "32.768 kHz crystal oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(SRC_A::LFXO)
    }
    #[doc = "32.768 kHz synthesized from HFCLK"]
    #[inline(always)]
    pub fn lfsynt(self) -> &'a mut W {
        self.variant(SRC_A::LFSYNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select which LFCLK source is started by the LFCLKSTART task"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
