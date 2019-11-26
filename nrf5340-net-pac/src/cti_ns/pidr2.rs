#[doc = "Reader of register PIDR2"]
pub type R = crate::R<u32, super::PIDR2>;
#[doc = "Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DES_1_A {
    #[doc = "3: ARM. Bits\\[6:4\\] of the JEDEC JEP106 Identity Code"]
    ARM,
}
impl From<DES_1_A> for u8 {
    #[inline(always)]
    fn from(variant: DES_1_A) -> Self {
        match variant {
            DES_1_A::ARM => 3,
        }
    }
}
#[doc = "Reader of field `DES_1`"]
pub type DES_1_R = crate::R<u8, DES_1_A>;
impl DES_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DES_1_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(DES_1_A::ARM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ARM`"]
    #[inline(always)]
    pub fn is_arm(&self) -> bool {
        *self == DES_1_A::ARM
    }
}
#[doc = "Reader of field `JEDEC`"]
pub type JEDEC_R = crate::R<bool, bool>;
#[doc = "Peripheral revision\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVISION_A {
    #[doc = "0: This device is at r0p0"]
    REV0P0,
}
impl From<REVISION_A> for u8 {
    #[inline(always)]
    fn from(variant: REVISION_A) -> Self {
        match variant {
            REVISION_A::REV0P0 => 0,
        }
    }
}
#[doc = "Reader of field `REVISION`"]
pub type REVISION_R = crate::R<u8, REVISION_A>;
impl REVISION_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVISION_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVISION_A::REV0P0),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REV0P0`"]
    #[inline(always)]
    pub fn is_rev0p0(&self) -> bool {
        *self == REVISION_A::REV0P0
    }
}
impl R {
    #[doc = "Bits 0:2 - Together, PIDR1.DES_0, PIDR2.DES_1, and PIDR4.DES_2 identify the designer of the component"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Always 1. Indicates that the JEDEC-assigned designer ID is used"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
