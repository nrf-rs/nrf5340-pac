#[doc = "Reader of register LFCLKSRCCOPY"]
pub type R = crate::R<u32, super::LFCLKSRCCOPY>;
#[doc = "Clock source\n\nValue on reset: 1"]
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
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
