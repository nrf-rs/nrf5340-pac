#[doc = "Reader of register CIDR1"]
pub type R = crate::R<u32, super::CIDR1>;
#[doc = "Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRMBL_1_A {
    #[doc = "0: Bits\\[11:8\\] of the identification code"]
    VALUE,
}
impl From<PRMBL_1_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMBL_1_A) -> Self {
        match variant {
            PRMBL_1_A::VALUE => 0,
        }
    }
}
#[doc = "Reader of field `PRMBL_1`"]
pub type PRMBL_1_R = crate::R<u8, PRMBL_1_A>;
impl PRMBL_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRMBL_1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRMBL_1_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == PRMBL_1_A::VALUE
    }
}
#[doc = "Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code\n\nValue on reset: 9"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLASS_A {
    #[doc = "9: Indicates that the component is a CoreSight component"]
    CORESIGHT,
}
impl From<CLASS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLASS_A) -> Self {
        match variant {
            CLASS_A::CORESIGHT => 9,
        }
    }
}
#[doc = "Reader of field `CLASS`"]
pub type CLASS_R = crate::R<u8, CLASS_A>;
impl CLASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLASS_A> {
        use crate::Variant::*;
        match self.bits {
            9 => Val(CLASS_A::CORESIGHT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CORESIGHT`"]
    #[inline(always)]
    pub fn is_coresight(&self) -> bool {
        *self == CLASS_A::CORESIGHT
    }
}
impl R {
    #[doc = "Bits 0:3 - Preamble\\[1\\]. Contains bits\\[11:8\\] of the component identification code"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Class of the component, for example, whether the component is a ROM table or a generic CoreSight component. Contains bits\\[15:12\\] of the component identification code"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
