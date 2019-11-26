#[doc = "Reader of register CIDR2"]
pub type R = crate::R<u32, super::CIDR2>;
#[doc = "Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMBL_2_A {
    #[doc = "5: Bits\\[23:16\\] of the identification code"]
    VALUE,
}
impl From<PRMBL_2_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_2_A) -> Self {
        match variant {
            PRMBL_2_A::VALUE => 5,
        }
    }
}
#[doc = "Reader of field `PRMBL_2`"]
pub type PRMBL_2_R = crate::R<u8, PRMBL_2_A>;
impl PRMBL_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRMBL_2_A> {
        use crate::Variant::*;
        match self.bits {
            5 => Val(PRMBL_2_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_2_A::VALUE
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[2\\]. Contains bits\\[23:16\\] of the component identification code"]
    #[inline(always)]
    pub fn prmbl_2(&self) -> PRMBL_2_R {
        PRMBL_2_R::new((self.bits & 0xff) as u8)
    }
}
