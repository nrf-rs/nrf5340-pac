#[doc = "Reader of register CTITRIGOUTSTATUS"]
pub type R = crate::R<u32, super::CTITRIGOUTSTATUS>;
#[doc = "Processor debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEBUGREQ_A {
    #[doc = "1: Ctitrigout 0 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 0 is inactive"]
    INACTIVE,
}
impl From<DEBUGREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DEBUGREQ_A) -> Self {
        match variant {
            DEBUGREQ_A::ACTIVE => true,
            DEBUGREQ_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `DEBUGREQ`"]
pub type DEBUGREQ_R = crate::R<bool, DEBUGREQ_A>;
impl DEBUGREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBUGREQ_A {
        match self.bits {
            true => DEBUGREQ_A::ACTIVE,
            false => DEBUGREQ_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DEBUGREQ_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DEBUGREQ_A::INACTIVE
    }
}
#[doc = "Processor Restart\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPURESTART_A {
    #[doc = "1: Ctitrigout 1 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 1 is inactive"]
    INACTIVE,
}
impl From<CPURESTART_A> for bool {
    #[inline(always)]
    fn from(variant: CPURESTART_A) -> Self {
        match variant {
            CPURESTART_A::ACTIVE => true,
            CPURESTART_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `CPURESTART`"]
pub type CPURESTART_R = crate::R<bool, CPURESTART_A>;
impl CPURESTART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPURESTART_A {
        match self.bits {
            true => CPURESTART_A::ACTIVE,
            false => CPURESTART_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == CPURESTART_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == CPURESTART_A::INACTIVE
    }
}
#[doc = "N/A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNUSED0_A {
    #[doc = "1: Ctitrigout 2 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 2 is inactive"]
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
    #[doc = "1: Ctitrigout 3 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 3 is inactive"]
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
#[doc = "ETM Event Input 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN0_A {
    #[doc = "1: Ctitrigout 4 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 4 is inactive"]
    INACTIVE,
}
impl From<ETMEVTIN0_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN0_A) -> Self {
        match variant {
            ETMEVTIN0_A::ACTIVE => true,
            ETMEVTIN0_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTIN0`"]
pub type ETMEVTIN0_R = crate::R<bool, ETMEVTIN0_A>;
impl ETMEVTIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN0_A {
        match self.bits {
            true => ETMEVTIN0_A::ACTIVE,
            false => ETMEVTIN0_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTIN0_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTIN0_A::INACTIVE
    }
}
#[doc = "ETM Event Input 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN1_A {
    #[doc = "1: Ctitrigout 5 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 5 is inactive"]
    INACTIVE,
}
impl From<ETMEVTIN1_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN1_A) -> Self {
        match variant {
            ETMEVTIN1_A::ACTIVE => true,
            ETMEVTIN1_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTIN1`"]
pub type ETMEVTIN1_R = crate::R<bool, ETMEVTIN1_A>;
impl ETMEVTIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN1_A {
        match self.bits {
            true => ETMEVTIN1_A::ACTIVE,
            false => ETMEVTIN1_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTIN1_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTIN1_A::INACTIVE
    }
}
#[doc = "ETM Event Input 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN2_A {
    #[doc = "1: Ctitrigout 6 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 6 is inactive"]
    INACTIVE,
}
impl From<ETMEVTIN2_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN2_A) -> Self {
        match variant {
            ETMEVTIN2_A::ACTIVE => true,
            ETMEVTIN2_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTIN2`"]
pub type ETMEVTIN2_R = crate::R<bool, ETMEVTIN2_A>;
impl ETMEVTIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN2_A {
        match self.bits {
            true => ETMEVTIN2_A::ACTIVE,
            false => ETMEVTIN2_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTIN2_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTIN2_A::INACTIVE
    }
}
#[doc = "ETM Event Input 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMEVTIN3_A {
    #[doc = "1: Ctitrigout 7 is active"]
    ACTIVE,
    #[doc = "0: Ctitrigout 7 is inactive"]
    INACTIVE,
}
impl From<ETMEVTIN3_A> for bool {
    #[inline(always)]
    fn from(variant: ETMEVTIN3_A) -> Self {
        match variant {
            ETMEVTIN3_A::ACTIVE => true,
            ETMEVTIN3_A::INACTIVE => false,
        }
    }
}
#[doc = "Reader of field `ETMEVTIN3`"]
pub type ETMEVTIN3_R = crate::R<bool, ETMEVTIN3_A>;
impl ETMEVTIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETMEVTIN3_A {
        match self.bits {
            true => ETMEVTIN3_A::ACTIVE,
            false => ETMEVTIN3_A::INACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == ETMEVTIN3_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == ETMEVTIN3_A::INACTIVE
    }
}
impl R {
    #[doc = "Bit 0 - Processor debug request"]
    #[inline(always)]
    pub fn debugreq(&self) -> DEBUGREQ_R {
        DEBUGREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Processor Restart"]
    #[inline(always)]
    pub fn cpurestart(&self) -> CPURESTART_R {
        CPURESTART_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn unused0(&self) -> UNUSED0_R {
        UNUSED0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn unused1(&self) -> UNUSED1_R {
        UNUSED1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ETM Event Input 0"]
    #[inline(always)]
    pub fn etmevtin0(&self) -> ETMEVTIN0_R {
        ETMEVTIN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ETM Event Input 1"]
    #[inline(always)]
    pub fn etmevtin1(&self) -> ETMEVTIN1_R {
        ETMEVTIN1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ETM Event Input 2"]
    #[inline(always)]
    pub fn etmevtin2(&self) -> ETMEVTIN2_R {
        ETMEVTIN2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ETM Event Input 3"]
    #[inline(always)]
    pub fn etmevtin3(&self) -> ETMEVTIN3_R {
        ETMEVTIN3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
