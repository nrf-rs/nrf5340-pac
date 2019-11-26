#[doc = "Reader of register PIDR1"]
pub type R = crate::R<u32, super::PIDR1>;
#[doc = "Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PART_1_A {
    #[doc = "13: Indicates bits\\[11:8\\] of the part number of the component"]
    PARTNUMBERH,
}
impl From<PART_1_A> for u8 {
    #[inline(always)]
    fn from(variant: PART_1_A) -> Self {
        match variant {
            PART_1_A::PARTNUMBERH => 13,
        }
    }
}
#[doc = "Reader of field `PART_1`"]
pub type PART_1_R = crate::R<u8, PART_1_A>;
impl PART_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PART_1_A> {
        use crate::Variant::*;
        match self.bits {
            13 => Val(PART_1_A::PARTNUMBERH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PARTNUMBERH`"]
    #[inline(always)]
    pub fn is_partnumber_h(&self) -> bool {
        *self == PART_1_A::PARTNUMBERH
    }
}
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_0_A {
    #[doc = "11: ARM. Bits\\[3:0\\] of the JEDEC JEP106 Identity Code"]
    ARM,
}
impl From<DES_0_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_0_A) -> Self {
        match variant {
            DES_0_A::ARM => 11,
        }
    }
}
#[doc = "Reader of field `DES_0`"]
pub type DES_0_R = crate::R<u8, DES_0_A>;
impl DES_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DES_0_A> {
        use crate::Variant::*;
        match self.bits {
            11 => Val(DES_0_A::ARM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM`"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == DES_0_A::ARM
    }
}
impl R {
    #[doc = "Bits 0:3 - Bits\\[11:8\\] of the 12-bit part number of the component. The designer of the component assigns this part number"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
