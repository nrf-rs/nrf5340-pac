#[doc = "Reader of register CTITRIGINSTATUS"]
pub type R = crate::R<u32, super::CTITRIGINSTATUS>;
#[doc = "Processor Halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPUHALTED_A {
    #[doc = "1: Ctitrigin 0 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 0 is inactive"]
    INACTIVE,
}
impl From<CPUHALTED_A> for bool {
    #[inline(always)]
    fn from(variant: CPUHALTED_A) -> Self {
        match variant {
            CPUHALTED_A::ACTIVE => true,
            CPUHALTED_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CPUHALTED`"]
pub type CPUHALTED_R = crate::R<bool, CPUHALTED_A>;
impl CPUHALTED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPUHALTED_A {
        match self.bits {
            true => CPUHALTED_A::ACTIVE,
            false => CPUHALTED_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CPUHALTED_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CPUHALTED_A::INACTIVE
    }
}
#[doc = "DWT Comparator Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT0_A {
    #[doc = "1: Ctitrigin 1 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 1 is inactive"]
    INACTIVE,
}
impl From<DWTCOMPOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT0_A) -> Self {
        match variant {
            DWTCOMPOUT0_A::ACTIVE => true,
            DWTCOMPOUT0_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `DWTCOMPOUT0`"]
pub type DWTCOMPOUT0_R = crate::R<bool, DWTCOMPOUT0_A>;
impl DWTCOMPOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT0_A {
        match self.bits {
            true => DWTCOMPOUT0_A::ACTIVE,
            false => DWTCOMPOUT0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DWTCOMPOUT0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DWTCOMPOUT0_A::INACTIVE
    }
}
#[doc = "DWT Comparator Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT1_A {
    #[doc = "1: Ctitrigin 2 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 2 is inactive"]
    INACTIVE,
}
impl From<DWTCOMPOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT1_A) -> Self {
        match variant {
            DWTCOMPOUT1_A::ACTIVE => true,
            DWTCOMPOUT1_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `DWTCOMPOUT1`"]
pub type DWTCOMPOUT1_R = crate::R<bool, DWTCOMPOUT1_A>;
impl DWTCOMPOUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT1_A {
        match self.bits {
            true => DWTCOMPOUT1_A::ACTIVE,
            false => DWTCOMPOUT1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DWTCOMPOUT1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DWTCOMPOUT1_A::INACTIVE
    }
}
#[doc = "DWT Comparator Output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWTCOMPOUT2_A {
    #[doc = "1: Ctitrigin 3 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 3 is inactive"]
    INACTIVE,
}
impl From<DWTCOMPOUT2_A> for bool {
    #[inline(always)]
    fn from(variant: DWTCOMPOUT2_A) -> Self {
        match variant {
            DWTCOMPOUT2_A::ACTIVE => true,
            DWTCOMPOUT2_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `DWTCOMPOUT2`"]
pub type DWTCOMPOUT2_R = crate::R<bool, DWTCOMPOUT2_A>;
impl DWTCOMPOUT2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTCOMPOUT2_A {
        match self.bits {
            true => DWTCOMPOUT2_A::ACTIVE,
            false => DWTCOMPOUT2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DWTCOMPOUT2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DWTCOMPOUT2_A::INACTIVE
    }
}
#[doc = "ETM Event Output 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTOUT0_A {
    #[doc = "1: Ctitrigin 4 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 4 is inactive"]
    INACTIVE,
}
impl From<ETMEVTOUT0_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTOUT0_A) -> Self {
        match variant {
            ETMEVTOUT0_A::ACTIVE => true,
            ETMEVTOUT0_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTOUT0`"]
pub type ETMEVTOUT0_R = crate::R<bool, ETMEVTOUT0_A>;
impl ETMEVTOUT0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTOUT0_A {
        match self.bits {
            true => ETMEVTOUT0_A::ACTIVE,
            false => ETMEVTOUT0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTOUT0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTOUT0_A::INACTIVE
    }
}
#[doc = "ETM Event Output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTOUT1_A {
    #[doc = "1: Ctitrigin 5 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 5 is inactive"]
    INACTIVE,
}
impl From<ETMEVTOUT1_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTOUT1_A) -> Self {
        match variant {
            ETMEVTOUT1_A::ACTIVE => true,
            ETMEVTOUT1_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTOUT1`"]
pub type ETMEVTOUT1_R = crate::R<bool, ETMEVTOUT1_A>;
impl ETMEVTOUT1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTOUT1_A {
        match self.bits {
            true => ETMEVTOUT1_A::ACTIVE,
            false => ETMEVTOUT1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTOUT1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTOUT1_A::INACTIVE
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_A {
    #[doc = "1: Ctitrigin 6 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 6 is inactive"]
    INACTIVE,
}
impl From<UNUSED0_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED0_A) -> Self {
        match variant {
            UNUSED0_A::ACTIVE => true,
            UNUSED0_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `UNUSED0`"]
pub type UNUSED0_R = crate::R<bool, UNUSED0_A>;
impl UNUSED0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED0_A {
        match self.bits {
            true => UNUSED0_A::ACTIVE,
            false => UNUSED0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == UNUSED0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED0_A::INACTIVE
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED1_A {
    #[doc = "1: Ctitrigin 7 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigin 7 is inactive"]
    INACTIVE,
}
impl From<UNUSED1_A> for bool {
    #[inline(always)]
    fn from(variant: UNUSED1_A) -> Self {
        match variant {
            UNUSED1_A::ACTIVE => true,
            UNUSED1_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `UNUSED1`"]
pub type UNUSED1_R = crate::R<bool, UNUSED1_A>;
impl UNUSED1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNUSED1_A {
        match self.bits {
            true => UNUSED1_A::ACTIVE,
            false => UNUSED1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == UNUSED1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == UNUSED1_A::INACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Processor Halted"]
    #[inline(always)]
    pub fn cpuhalted(&self) -> CPUHALTED_R {
        CPUHALTED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DWT Comparator Output 0"]
    #[inline(always)]
    pub fn dwtcompout0(&self) -> DWTCOMPOUT0_R {
        DWTCOMPOUT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DWT Comparator Output 1"]
    #[inline(always)]
    pub fn dwtcompout1(&self) -> DWTCOMPOUT1_R {
        DWTCOMPOUT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DWT Comparator Output 2"]
    #[inline(always)]
    pub fn dwtcompout2(&self) -> DWTCOMPOUT2_R {
        DWTCOMPOUT2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ETM Event Output 0"]
    #[inline(always)]
    pub fn etmevtout0(&self) -> ETMEVTOUT0_R {
        ETMEVTOUT0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ETM Event Output 1"]
    #[inline(always)]
    pub fn etmevtout1(&self) -> ETMEVTOUT1_R {
        ETMEVTOUT1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> UNUSED1_R {
        UNUSED1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
