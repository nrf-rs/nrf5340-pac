#[doc = "Reader of register PIDR0"]
pub type R = crate::R<u32, super::PIDR0>;
#[doc = "Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number\n\nValue on reset: 33"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PART_0_A {
    #[doc = "33: Indicates bits\\[7:0\\] of the part number of the component"]
    PARTNUMBERL,
}
impl From<PART_0_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_0_A) -> Self {
        match variant {
            PART_0_A::PARTNUMBERL => 33,
        }
    }
}
#[doc = "Reader of field `PART_0`"]
pub type PART_0_R = crate::R<u8, PART_0_A>;
impl PART_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PART_0_A> {
        use crate::Variant::*;
        match self.bits {
            33 => Val(PART_0_A::PARTNUMBERL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PARTNUMBERL`"]
    #[inline(always)]
    pub fn is_partnumber_l(&self) -> bool {
        *self == PART_0_A::PARTNUMBERL
    }
}
impl R {
    #[doc = "Bits 0:7 - Bits\\[7:0\\] of the 12-bit part number of the component. The designer of the component assigns this part number"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
