#[doc = "Reader of register CIDR3"]
pub type R = crate::R<u32, super::CIDR3>;
#[doc = "Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code\n\nValue on reset: 177"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMBL_3_A {
    #[doc = "177: Bits\\[31:24\\] of the identification code"]
    VALUE,
}
impl From<PRMBL_3_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_3_A) -> Self {
        match variant {
            PRMBL_3_A::VALUE => 177,
        }
    }
}
#[doc = "Reader of field `PRMBL_3`"]
pub type PRMBL_3_R = crate::R<u8, PRMBL_3_A>;
impl PRMBL_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRMBL_3_A> {
        use crate::Variant::*;
        match self.bits {
            177 => Val(PRMBL_3_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_3_A::VALUE
    }
}
impl R {
    #[doc = "Bits 0:7 - Preamble\\[3\\]. Contains bits\\[31:24\\] of the component identification code"]
    #[inline(always)]
    pub fn prmbl_3(&self) -> PRMBL_3_R {
        PRMBL_3_R::new((self.bits & 0xff) as u8)
    }
}
