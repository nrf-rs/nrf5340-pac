#[doc = "Reader of register INTPEND"]
pub type R = crate::R<u32, super::INTPEND>;
#[doc = "Read pending status of interrupt for event RECEIVE\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE0_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE0_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE0_A) -> Self {
        match variant {
            RECEIVE0_A::NOTPENDING => false,
            RECEIVE0_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE0`"]
pub type RECEIVE0_R = crate::R<bool, RECEIVE0_A>;
impl RECEIVE0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE0_A {
        match self.bits {
            false => RECEIVE0_A::NOTPENDING,
            true => RECEIVE0_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE0_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE0_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE1_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE1_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE1_A) -> Self {
        match variant {
            RECEIVE1_A::NOTPENDING => false,
            RECEIVE1_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE1`"]
pub type RECEIVE1_R = crate::R<bool, RECEIVE1_A>;
impl RECEIVE1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE1_A {
        match self.bits {
            false => RECEIVE1_A::NOTPENDING,
            true => RECEIVE1_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE1_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE1_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE2_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE2_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE2_A) -> Self {
        match variant {
            RECEIVE2_A::NOTPENDING => false,
            RECEIVE2_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE2`"]
pub type RECEIVE2_R = crate::R<bool, RECEIVE2_A>;
impl RECEIVE2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE2_A {
        match self.bits {
            false => RECEIVE2_A::NOTPENDING,
            true => RECEIVE2_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE2_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE2_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE3_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE3_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE3_A) -> Self {
        match variant {
            RECEIVE3_A::NOTPENDING => false,
            RECEIVE3_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE3`"]
pub type RECEIVE3_R = crate::R<bool, RECEIVE3_A>;
impl RECEIVE3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE3_A {
        match self.bits {
            false => RECEIVE3_A::NOTPENDING,
            true => RECEIVE3_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE3_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE3_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE4_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE4_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE4_A) -> Self {
        match variant {
            RECEIVE4_A::NOTPENDING => false,
            RECEIVE4_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE4`"]
pub type RECEIVE4_R = crate::R<bool, RECEIVE4_A>;
impl RECEIVE4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE4_A {
        match self.bits {
            false => RECEIVE4_A::NOTPENDING,
            true => RECEIVE4_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE4_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE4_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE5_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE5_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE5_A) -> Self {
        match variant {
            RECEIVE5_A::NOTPENDING => false,
            RECEIVE5_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE5`"]
pub type RECEIVE5_R = crate::R<bool, RECEIVE5_A>;
impl RECEIVE5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE5_A {
        match self.bits {
            false => RECEIVE5_A::NOTPENDING,
            true => RECEIVE5_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE5_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE5_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE6_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE6_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE6_A) -> Self {
        match variant {
            RECEIVE6_A::NOTPENDING => false,
            RECEIVE6_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE6`"]
pub type RECEIVE6_R = crate::R<bool, RECEIVE6_A>;
impl RECEIVE6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE6_A {
        match self.bits {
            false => RECEIVE6_A::NOTPENDING,
            true => RECEIVE6_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE6_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE6_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE7_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE7_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE7_A) -> Self {
        match variant {
            RECEIVE7_A::NOTPENDING => false,
            RECEIVE7_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE7`"]
pub type RECEIVE7_R = crate::R<bool, RECEIVE7_A>;
impl RECEIVE7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE7_A {
        match self.bits {
            false => RECEIVE7_A::NOTPENDING,
            true => RECEIVE7_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE7_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE7_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE8_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE8_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE8_A) -> Self {
        match variant {
            RECEIVE8_A::NOTPENDING => false,
            RECEIVE8_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE8`"]
pub type RECEIVE8_R = crate::R<bool, RECEIVE8_A>;
impl RECEIVE8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE8_A {
        match self.bits {
            false => RECEIVE8_A::NOTPENDING,
            true => RECEIVE8_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE8_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE8_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE9_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE9_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE9_A) -> Self {
        match variant {
            RECEIVE9_A::NOTPENDING => false,
            RECEIVE9_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE9`"]
pub type RECEIVE9_R = crate::R<bool, RECEIVE9_A>;
impl RECEIVE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE9_A {
        match self.bits {
            false => RECEIVE9_A::NOTPENDING,
            true => RECEIVE9_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE9_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE9_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE10_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE10_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE10_A) -> Self {
        match variant {
            RECEIVE10_A::NOTPENDING => false,
            RECEIVE10_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE10`"]
pub type RECEIVE10_R = crate::R<bool, RECEIVE10_A>;
impl RECEIVE10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE10_A {
        match self.bits {
            false => RECEIVE10_A::NOTPENDING,
            true => RECEIVE10_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE10_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE10_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE11_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE11_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE11_A) -> Self {
        match variant {
            RECEIVE11_A::NOTPENDING => false,
            RECEIVE11_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE11`"]
pub type RECEIVE11_R = crate::R<bool, RECEIVE11_A>;
impl RECEIVE11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE11_A {
        match self.bits {
            false => RECEIVE11_A::NOTPENDING,
            true => RECEIVE11_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE11_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE11_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE12_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE12_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE12_A) -> Self {
        match variant {
            RECEIVE12_A::NOTPENDING => false,
            RECEIVE12_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE12`"]
pub type RECEIVE12_R = crate::R<bool, RECEIVE12_A>;
impl RECEIVE12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE12_A {
        match self.bits {
            false => RECEIVE12_A::NOTPENDING,
            true => RECEIVE12_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE12_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE12_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE13_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE13_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE13_A) -> Self {
        match variant {
            RECEIVE13_A::NOTPENDING => false,
            RECEIVE13_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE13`"]
pub type RECEIVE13_R = crate::R<bool, RECEIVE13_A>;
impl RECEIVE13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE13_A {
        match self.bits {
            false => RECEIVE13_A::NOTPENDING,
            true => RECEIVE13_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE13_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE13_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE14_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE14_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE14_A) -> Self {
        match variant {
            RECEIVE14_A::NOTPENDING => false,
            RECEIVE14_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE14`"]
pub type RECEIVE14_R = crate::R<bool, RECEIVE14_A>;
impl RECEIVE14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE14_A {
        match self.bits {
            false => RECEIVE14_A::NOTPENDING,
            true => RECEIVE14_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE14_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE14_A::PENDING
    }
}
#[doc = "Read pending status of interrupt for event RECEIVE\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RECEIVE15_A {
    #[doc = "0: Read: Not pending"]
    NOTPENDING,
    #[doc = "1: Read: Pending"]
    PENDING,
}
impl From<RECEIVE15_A> for bool {
    #[inline(always)]
    fn from(variant: RECEIVE15_A) -> Self {
        match variant {
            RECEIVE15_A::NOTPENDING => false,
            RECEIVE15_A::PENDING => true,
        }
    }
}
#[doc = "Reader of field `RECEIVE15`"]
pub type RECEIVE15_R = crate::R<bool, RECEIVE15_A>;
impl RECEIVE15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RECEIVE15_A {
        match self.bits {
            false => RECEIVE15_A::NOTPENDING,
            true => RECEIVE15_A::PENDING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTPENDING`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RECEIVE15_A::NOTPENDING
    }
    #[doc = "Checks if the value of the field is `PENDING`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RECEIVE15_A::PENDING
    }
}
impl R {
    #[doc = "Bit 0 - Read pending status of interrupt for event RECEIVE\\[0\\]"]
    #[inline(always)]
    pub fn receive0(&self) -> RECEIVE0_R {
        RECEIVE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read pending status of interrupt for event RECEIVE\\[1\\]"]
    #[inline(always)]
    pub fn receive1(&self) -> RECEIVE1_R {
        RECEIVE1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Read pending status of interrupt for event RECEIVE\\[2\\]"]
    #[inline(always)]
    pub fn receive2(&self) -> RECEIVE2_R {
        RECEIVE2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Read pending status of interrupt for event RECEIVE\\[3\\]"]
    #[inline(always)]
    pub fn receive3(&self) -> RECEIVE3_R {
        RECEIVE3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Read pending status of interrupt for event RECEIVE\\[4\\]"]
    #[inline(always)]
    pub fn receive4(&self) -> RECEIVE4_R {
        RECEIVE4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Read pending status of interrupt for event RECEIVE\\[5\\]"]
    #[inline(always)]
    pub fn receive5(&self) -> RECEIVE5_R {
        RECEIVE5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Read pending status of interrupt for event RECEIVE\\[6\\]"]
    #[inline(always)]
    pub fn receive6(&self) -> RECEIVE6_R {
        RECEIVE6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Read pending status of interrupt for event RECEIVE\\[7\\]"]
    #[inline(always)]
    pub fn receive7(&self) -> RECEIVE7_R {
        RECEIVE7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Read pending status of interrupt for event RECEIVE\\[8\\]"]
    #[inline(always)]
    pub fn receive8(&self) -> RECEIVE8_R {
        RECEIVE8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Read pending status of interrupt for event RECEIVE\\[9\\]"]
    #[inline(always)]
    pub fn receive9(&self) -> RECEIVE9_R {
        RECEIVE9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Read pending status of interrupt for event RECEIVE\\[10\\]"]
    #[inline(always)]
    pub fn receive10(&self) -> RECEIVE10_R {
        RECEIVE10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Read pending status of interrupt for event RECEIVE\\[11\\]"]
    #[inline(always)]
    pub fn receive11(&self) -> RECEIVE11_R {
        RECEIVE11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Read pending status of interrupt for event RECEIVE\\[12\\]"]
    #[inline(always)]
    pub fn receive12(&self) -> RECEIVE12_R {
        RECEIVE12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Read pending status of interrupt for event RECEIVE\\[13\\]"]
    #[inline(always)]
    pub fn receive13(&self) -> RECEIVE13_R {
        RECEIVE13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Read pending status of interrupt for event RECEIVE\\[14\\]"]
    #[inline(always)]
    pub fn receive14(&self) -> RECEIVE14_R {
        RECEIVE14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Read pending status of interrupt for event RECEIVE\\[15\\]"]
    #[inline(always)]
    pub fn receive15(&self) -> RECEIVE15_R {
        RECEIVE15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
