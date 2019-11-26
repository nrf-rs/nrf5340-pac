#[doc = "Reader of register DEVTYPE"]
pub type R = crate::R<u32, super::DEVTYPE>;
#[doc = "Major classification of the type of the debug component as specified in the ARM Architecture Specification for this debug and trace component\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAJOR_A {
    #[doc = "4: Indicates that this component allows a debugger to control other components in a CoreSight SoC-400 system"]
    CONTROLLER,
}
impl From<MAJOR_A> for u8 {
    #[inline(always)]
    fn from(variant: MAJOR_A) -> Self {
        match variant {
            MAJOR_A::CONTROLLER => 4,
        }
    }
}
#[doc = "Reader of field `MAJOR`"]
pub type MAJOR_R = crate::R<u8, MAJOR_A>;
impl MAJOR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MAJOR_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(MAJOR_A::CONTROLLER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CONTROLLER`"]
    #[inline(always)]
    pub fn is_controller(&self) -> bool {
        *self == MAJOR_A::CONTROLLER
    }
}
#[doc = "Sub-classification of the type of the debug component as specified in the ARM Architecture Specification within the major classification as specified in the MAJOR field.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SUB_A {
    #[doc = "1: Indicates that this component is a sub-triggering component"]
    CROSSTRIGGER,
}
impl From<SUB_A> for u8 {
    #[inline(always)]
    fn from(variant: SUB_A) -> Self {
        match variant {
            SUB_A::CROSSTRIGGER => 1,
        }
    }
}
#[doc = "Reader of field `SUB`"]
pub type SUB_R = crate::R<u8, SUB_A>;
impl SUB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SUB_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(SUB_A::CROSSTRIGGER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CROSSTRIGGER`"]
    #[inline(always)]
    pub fn is_crosstrigger(&self) -> bool {
        *self == SUB_A::CROSSTRIGGER
    }
}
impl R {
    #[doc = "Bits 0:3 - Major classification of the type of the debug component as specified in the ARM Architecture Specification for this debug and trace component"]
    #[inline(always)]
    pub fn major(&self) -> MAJOR_R {
        MAJOR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sub-classification of the type of the debug component as specified in the ARM Architecture Specification within the major classification as specified in the MAJOR field."]
    #[inline(always)]
    pub fn sub(&self) -> SUB_R {
        SUB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
