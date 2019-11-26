#[doc = "Reader of register HFCLKALWAYSRUN"]
pub type R = crate::R<u32, super::HFCLKALWAYSRUN>;
#[doc = "Ensure clock is always running\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALWAYSRUN_A {
    #[doc = "0: Use automatic clock control"]
    AUTOMATIC,
    #[doc = "1: Ensure clock is always running"]
    ALWAYSRUN,
}
impl From<ALWAYSRUN_A> for bool {
    #[inline(always)]
    fn from(variant: ALWAYSRUN_A) -> Self {
        match variant {
            ALWAYSRUN_A::AUTOMATIC => false,
            ALWAYSRUN_A::ALWAYSRUN => true,
        }
    }
}
#[doc = "Reader of field `ALWAYSRUN`"]
pub type ALWAYSRUN_R = crate::R<bool, ALWAYSRUN_A>;
impl ALWAYSRUN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALWAYSRUN_A {
        match self.bits {
            false => ALWAYSRUN_A::AUTOMATIC,
            true => ALWAYSRUN_A::ALWAYSRUN,
        }
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == ALWAYSRUN_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `ALWAYSRUN`"]
    #[inline(always)]
    pub fn is_always_run(&self) -> bool {
        *self == ALWAYSRUN_A::ALWAYSRUN
    }
}
impl R {
    #[doc = "Bit 0 - Ensure clock is always running"]
    #[inline(always)]
    pub fn alwaysrun(&self) -> ALWAYSRUN_R {
        ALWAYSRUN_R::new((self.bits & 0x01) != 0)
    }
}
