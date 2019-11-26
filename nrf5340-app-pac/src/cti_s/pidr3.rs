#[doc = "Reader of register PIDR3"]
pub type R = crate::R<u32, super::PIDR3>;
#[doc = "Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMOD_A {
    #[doc = "0: Indicates that the customer has not modified this component"]
    UNMODIFIED,
}
impl From<CMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMOD_A) -> Self {
        match variant {
            CMOD_A::UNMODIFIED => 0,
        }
    }
}
#[doc = "Reader of field `CMOD`"]
pub type CMOD_R = crate::R<u8, CMOD_A>;
impl CMOD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CMOD_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CMOD_A::UNMODIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNMODIFIED`"]
    #[inline(always)]
    pub fn is_unmodified(&self) -> bool {
        *self == CMOD_A::UNMODIFIED
    }
}
#[doc = "Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. ARM recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REVAND_A {
    #[doc = "0: Indicates that there are no errata fixes to this component"]
    NOERRATA,
}
impl From<REVAND_A> for u8 {
    #[inline(always)]
    fn from(variant: REVAND_A) -> Self {
        match variant {
            REVAND_A::NOERRATA => 0,
        }
    }
}
#[doc = "Reader of field `REVAND`"]
pub type REVAND_R = crate::R<u8, REVAND_A>;
impl REVAND_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, REVAND_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(REVAND_A::NOERRATA),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOERRATA`"]
    #[inline(always)]
    pub fn is_no_errata(&self) -> bool {
        *self == REVAND_A::NOERRATA
    }
}
impl R {
    #[doc = "Bits 0:3 - Customer Modified. Indicates whether the customer has modified the behavior of the component. In most cases, this field is 0b0000. Customers change this value when they make authorized modifications to this component."]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates minor errata fixes specific to the revision of the component being used, for example metal fixes after implementation. In most cases, this field is 0b0000. ARM recommends that the component designers ensure that a metal fix can change this field if required, for example, by driving it from registers that reset to 0b0000."]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
