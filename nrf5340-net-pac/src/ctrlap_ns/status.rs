#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Status bit for device debug interface mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBGIFACEMODE_A {
    #[doc = "0: No debugger attached"]
    DISABLED,
    #[doc = "1: Debugger is attached and device is in debug interface mode"]
    ENABLED,
}
impl From<DBGIFACEMODE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGIFACEMODE_A) -> Self {
        match variant {
            DBGIFACEMODE_A::DISABLED => false,
            DBGIFACEMODE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `DBGIFACEMODE`"]
pub type DBGIFACEMODE_R = crate::R<bool, DBGIFACEMODE_A>;
impl DBGIFACEMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGIFACEMODE_A {
        match self.bits {
            false => DBGIFACEMODE_A::DISABLED,
            true => DBGIFACEMODE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBGIFACEMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBGIFACEMODE_A::ENABLED
    }
}
#[doc = "Status bit for access port protection in non-secure mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APPROTECT_A {
    #[doc = "0: Non-secure mode access port protection is currently disabled"]
    DISABLED,
    #[doc = "1: Non-secure mode access port protection is currently enabled"]
    ENABLED,
}
impl From<APPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: APPROTECT_A) -> Self {
        match variant {
            APPROTECT_A::DISABLED => false,
            APPROTECT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `APPROTECT`"]
pub type APPROTECT_R = crate::R<bool, APPROTECT_A>;
impl APPROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APPROTECT_A {
        match self.bits {
            false => APPROTECT_A::DISABLED,
            true => APPROTECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APPROTECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APPROTECT_A::ENABLED
    }
}
#[doc = "Status bit for access port protection in secure mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SECUREAPPROTECT_A {
    #[doc = "0: Secure mode access port protection is currently disabled"]
    DISABLED,
    #[doc = "1: Secure mode access port protection is currently enabled"]
    ENABLED,
}
impl From<SECUREAPPROTECT_A> for bool {
    #[inline(always)]
    fn from(variant: SECUREAPPROTECT_A) -> Self {
        match variant {
            SECUREAPPROTECT_A::DISABLED => false,
            SECUREAPPROTECT_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `SECUREAPPROTECT`"]
pub type SECUREAPPROTECT_R = crate::R<bool, SECUREAPPROTECT_A>;
impl SECUREAPPROTECT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECUREAPPROTECT_A {
        match self.bits {
            false => SECUREAPPROTECT_A::DISABLED,
            true => SECUREAPPROTECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SECUREAPPROTECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SECUREAPPROTECT_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 0 - Status bit for device debug interface mode"]
    #[inline(always)]
    pub fn dbgifacemode(&self) -> DBGIFACEMODE_R {
        DBGIFACEMODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status bit for access port protection in non-secure mode"]
    #[inline(always)]
    pub fn approtect(&self) -> APPROTECT_R {
        APPROTECT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status bit for access port protection in secure mode"]
    #[inline(always)]
    pub fn secureapprotect(&self) -> SECUREAPPROTECT_R {
        SECUREAPPROTECT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
