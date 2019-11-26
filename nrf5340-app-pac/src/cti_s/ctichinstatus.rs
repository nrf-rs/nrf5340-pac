#[doc = "Reader of register CTICHINSTATUS"]
pub type R = crate::R<u32, super::CTICHINSTATUS>;
#[doc = "Shows the status of the ctitrigin 0 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_0_A {
    #[doc = "1: Ctichin 0 is active"]
    ACTIVE,
    #[doc = "0: Ctichin 0 is inactive"]
    INACTIVE,
}
impl From<CTICHINSTATUS_0_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_0_A) -> Self {
        match variant {
            CTICHINSTATUS_0_A::ACTIVE => true,
            CTICHINSTATUS_0_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CTICHINSTATUS_0`"]
pub type CTICHINSTATUS_0_R = crate::R<bool, CTICHINSTATUS_0_A>;
impl CTICHINSTATUS_0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_0_A {
        match self.bits {
            true => CTICHINSTATUS_0_A::ACTIVE,
            false => CTICHINSTATUS_0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CTICHINSTATUS_0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CTICHINSTATUS_0_A::INACTIVE
    }
}
#[doc = "Shows the status of the ctitrigin 1 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_1_A {
    #[doc = "1: Ctichin 1 is active"]
    ACTIVE,
    #[doc = "0: Ctichin 1 is inactive"]
    INACTIVE,
}
impl From<CTICHINSTATUS_1_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_1_A) -> Self {
        match variant {
            CTICHINSTATUS_1_A::ACTIVE => true,
            CTICHINSTATUS_1_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CTICHINSTATUS_1`"]
pub type CTICHINSTATUS_1_R = crate::R<bool, CTICHINSTATUS_1_A>;
impl CTICHINSTATUS_1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_1_A {
        match self.bits {
            true => CTICHINSTATUS_1_A::ACTIVE,
            false => CTICHINSTATUS_1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CTICHINSTATUS_1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CTICHINSTATUS_1_A::INACTIVE
    }
}
#[doc = "Shows the status of the ctitrigin 2 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_2_A {
    #[doc = "1: Ctichin 2 is active"]
    ACTIVE,
    #[doc = "0: Ctichin 2 is inactive"]
    INACTIVE,
}
impl From<CTICHINSTATUS_2_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_2_A) -> Self {
        match variant {
            CTICHINSTATUS_2_A::ACTIVE => true,
            CTICHINSTATUS_2_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CTICHINSTATUS_2`"]
pub type CTICHINSTATUS_2_R = crate::R<bool, CTICHINSTATUS_2_A>;
impl CTICHINSTATUS_2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_2_A {
        match self.bits {
            true => CTICHINSTATUS_2_A::ACTIVE,
            false => CTICHINSTATUS_2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CTICHINSTATUS_2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CTICHINSTATUS_2_A::INACTIVE
    }
}
#[doc = "Shows the status of the ctitrigin 3 input.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTICHINSTATUS_3_A {
    #[doc = "1: Ctichin 3 is active"]
    ACTIVE,
    #[doc = "0: Ctichin 3 is inactive"]
    INACTIVE,
}
impl From<CTICHINSTATUS_3_A> for bool {
    #[inline(always)]
    fn from(variant: CTICHINSTATUS_3_A) -> Self {
        match variant {
            CTICHINSTATUS_3_A::ACTIVE => true,
            CTICHINSTATUS_3_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CTICHINSTATUS_3`"]
pub type CTICHINSTATUS_3_R = crate::R<bool, CTICHINSTATUS_3_A>;
impl CTICHINSTATUS_3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTICHINSTATUS_3_A {
        match self.bits {
            true => CTICHINSTATUS_3_A::ACTIVE,
            false => CTICHINSTATUS_3_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CTICHINSTATUS_3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CTICHINSTATUS_3_A::INACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Shows the status of the ctitrigin 0 input."]
    #[inline(always)]
    pub fn ctichinstatus_0(&self) -> CTICHINSTATUS_0_R {
        CTICHINSTATUS_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shows the status of the ctitrigin 1 input."]
    #[inline(always)]
    pub fn ctichinstatus_1(&self) -> CTICHINSTATUS_1_R {
        CTICHINSTATUS_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shows the status of the ctitrigin 2 input."]
    #[inline(always)]
    pub fn ctichinstatus_2(&self) -> CTICHINSTATUS_2_R {
        CTICHINSTATUS_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shows the status of the ctitrigin 3 input."]
    #[inline(always)]
    pub fn ctichinstatus_3(&self) -> CTICHINSTATUS_3_R {
        CTICHINSTATUS_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
