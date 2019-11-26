#[doc = "Reader of register MAINREGSTATUS"]
pub type R = crate::R<u32, super::MAINREGSTATUS>;
#[doc = "VREGH status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VREGH_A {
    #[doc = "0: Normal voltage mode. Voltage supplied on VDD."]
    INACTIVE,
    #[doc = "1: High voltage mode. Voltage supplied on VDDH."]
    ACTIVE,
}
impl From<VREGH_A> for bool {
    #[inline(always)]
    fn from(variant: VREGH_A) -> Self {
        match variant {
            VREGH_A::INACTIVE => false,
            VREGH_A::ACTIVE => true,
        }
    }
}
#[doc = "Reader of field `VREGH`"]
pub type VREGH_R = crate::R<bool, VREGH_A>;
impl VREGH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREGH_A {
        match self.bits {
            false => VREGH_A::INACTIVE,
            true => VREGH_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == VREGH_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == VREGH_A::ACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - VREGH status"]
    #[inline(always)]
    pub fn vregh(&self) -> VREGH_R {
        VREGH_R::new((self.bits & 0x01) != 0)
    }
}
