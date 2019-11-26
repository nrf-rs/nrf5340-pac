#[doc = "Reader of register CIDR0"]
pub type R = crate::R<u32, super::CIDR0>;
#[doc = "Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMBL_0_A {
    #[doc = "13: Bits\\[7:0\\] of the identification code"]
    VALUE,
}
impl From<PRMBL_0_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_0_A) -> Self {
        match variant {
            PRMBL_0_A::VALUE => 13,
        }
    }
}
#[doc = "Reader of field `PRMBL_0`"]
pub type PRMBL_0_R = crate::R<u8, PRMBL_0_A>;
impl PRMBL_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRMBL_0_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(PRMBL_0_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_0_A::VALUE
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[0\\]. Contains bits\\[7:0\\] of the component identification code"]
    #[inline(always)]
    pub fn prmbl_0(&self) -> PRMBL_0_R {
        PRMBL_0_R::new((self.bits & 0xff) as u8)
    }
}
