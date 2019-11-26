#[doc = "Reader of register CTEINLINECONF"]
pub type R = crate::R<u32, super::CTEINLINECONF>;
#[doc = "Writer for register CTEINLINECONF"]
pub type W = crate::W<u32, super::CTEINLINECONF>;
#[doc = "Register CTEINLINECONF `reset()`'s with value 0x2800"]
impl crate::ResetValue for super::CTEINLINECONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2800
    }
}
#[doc = "Enable parsing of CTEInfo from received packet in BLE modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINLINECTRLEN_A {
    #[doc = "1: Parsing of CTEInfo is enabled"]
    ENABLED,
    #[doc = "0: Parsing of CTEInfo is disabled"]
    DISABLED,
}
impl From<CTEINLINECTRLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTEINLINECTRLEN_A) -> Self {
        match variant {
            CTEINLINECTRLEN_A::ENABLED => true,
            CTEINLINECTRLEN_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `CTEINLINECTRLEN`"]
pub type CTEINLINECTRLEN_R = crate::R<bool, CTEINLINECTRLEN_A>;
impl CTEINLINECTRLEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEINLINECTRLEN_A {
        match self.bits {
            true => CTEINLINECTRLEN_A::ENABLED,
            false => CTEINLINECTRLEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTEINLINECTRLEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTEINLINECTRLEN_A::DISABLED
    }
}
#[doc = "Write proxy for field `CTEINLINECTRLEN`"]
pub struct CTEINLINECTRLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEINLINECTRLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEINLINECTRLEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Parsing of CTEInfo is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTEINLINECTRLEN_A::ENABLED)
    }
    #[doc = "Parsing of CTEInfo is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTEINLINECTRLEN_A::DISABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "CTEInfo is S1 byte or not\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINFOINS1_A {
    #[doc = "1: CTEInfo is in S1 byte (data PDU)"]
    INS1,
    #[doc = "0: CTEInfo is NOT in S1 byte (advertising PDU)"]
    NOTINS1,
}
impl From<CTEINFOINS1_A> for bool {
    #[inline(always)]
    fn from(variant: CTEINFOINS1_A) -> Self {
        match variant {
            CTEINFOINS1_A::INS1 => true,
            CTEINFOINS1_A::NOTINS1 => false,
        }
    }
}
#[doc = "Reader of field `CTEINFOINS1`"]
pub type CTEINFOINS1_R = crate::R<bool, CTEINFOINS1_A>;
impl CTEINFOINS1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEINFOINS1_A {
        match self.bits {
            true => CTEINFOINS1_A::INS1,
            false => CTEINFOINS1_A::NOTINS1,
        }
    }
    #[doc = "Checks if the value of the field is `INS1`"]
    #[inline(always)]
    pub fn is_in_s1(&self) -> bool {
        *self == CTEINFOINS1_A::INS1
    }
    #[doc = "Checks if the value of the field is `NOTINS1`"]
    #[inline(always)]
    pub fn is_not_in_s1(&self) -> bool {
        *self == CTEINFOINS1_A::NOTINS1
    }
}
#[doc = "Write proxy for field `CTEINFOINS1`"]
pub struct CTEINFOINS1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEINFOINS1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEINFOINS1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CTEInfo is in S1 byte (data PDU)"]
    #[inline(always)]
    pub fn in_s1(self) -> &'a mut W {
        self.variant(CTEINFOINS1_A::INS1)
    }
    #[doc = "CTEInfo is NOT in S1 byte (advertising PDU)"]
    #[inline(always)]
    pub fn not_in_s1(self) -> &'a mut W {
        self.variant(CTEINFOINS1_A::NOTINS1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Sampling/switching if CRC is not OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEERRORHANDLING_A {
    #[doc = "1: Sampling and antenna switching also when CRC is not OK"]
    YES,
    #[doc = "0: No sampling and antenna switching when CRC is not OK"]
    NO,
}
impl From<CTEERRORHANDLING_A> for bool {
    #[inline(always)]
    fn from(variant: CTEERRORHANDLING_A) -> Self {
        match variant {
            CTEERRORHANDLING_A::YES => true,
            CTEERRORHANDLING_A::NO => false,
        }
    }
}
#[doc = "Reader of field `CTEERRORHANDLING`"]
pub type CTEERRORHANDLING_R = crate::R<bool, CTEERRORHANDLING_A>;
impl CTEERRORHANDLING_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTEERRORHANDLING_A {
        match self.bits {
            true => CTEERRORHANDLING_A::YES,
            false => CTEERRORHANDLING_A::NO,
        }
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CTEERRORHANDLING_A::YES
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CTEERRORHANDLING_A::NO
    }
}
#[doc = "Write proxy for field `CTEERRORHANDLING`"]
pub struct CTEERRORHANDLING_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEERRORHANDLING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEERRORHANDLING_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Sampling and antenna switching also when CRC is not OK"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CTEERRORHANDLING_A::YES)
    }
    #[doc = "No sampling and antenna switching when CRC is not OK"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CTEERRORHANDLING_A::NO)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Max range of CTETime\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTETIMEVALIDRANGE_A {
    #[doc = "0: 20 in 8us unit (default) Set to 20 if parsed CTETime is larger han 20"]
    _20,
    #[doc = "1: 31 in 8us unit"]
    _31,
    #[doc = "2: 63 in 8us unit"]
    _63,
}
impl From<CTETIMEVALIDRANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: CTETIMEVALIDRANGE_A) -> Self {
        match variant {
            CTETIMEVALIDRANGE_A::_20 => 0,
            CTETIMEVALIDRANGE_A::_31 => 1,
            CTETIMEVALIDRANGE_A::_63 => 2,
        }
    }
}
#[doc = "Reader of field `CTETIMEVALIDRANGE`"]
pub type CTETIMEVALIDRANGE_R = crate::R<u8, CTETIMEVALIDRANGE_A>;
impl CTETIMEVALIDRANGE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTETIMEVALIDRANGE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTETIMEVALIDRANGE_A::_20),
            1 => Val(CTETIMEVALIDRANGE_A::_31),
            2 => Val(CTETIMEVALIDRANGE_A::_63),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_20
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_31
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == CTETIMEVALIDRANGE_A::_63
    }
}
#[doc = "Write proxy for field `CTETIMEVALIDRANGE`"]
pub struct CTETIMEVALIDRANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTETIMEVALIDRANGE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTETIMEVALIDRANGE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "20 in 8us unit (default) Set to 20 if parsed CTETime is larger han 20"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_20)
    }
    #[doc = "31 in 8us unit"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_31)
    }
    #[doc = "63 in 8us unit"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut W {
        self.variant(CTETIMEVALIDRANGE_A::_63)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINLINERXMODE1US_A {
    #[doc = "1: 4us"]
    _4US,
    #[doc = "2: 2us"]
    _2US,
    #[doc = "3: 1us"]
    _1US,
    #[doc = "4: 0.5us"]
    _500NS,
    #[doc = "5: 0.25us"]
    _250NS,
    #[doc = "6: 0.125us"]
    _125NS,
}
impl From<CTEINLINERXMODE1US_A> for u8 {
    #[inline(always)]
    fn from(variant: CTEINLINERXMODE1US_A) -> Self {
        match variant {
            CTEINLINERXMODE1US_A::_4US => 1,
            CTEINLINERXMODE1US_A::_2US => 2,
            CTEINLINERXMODE1US_A::_1US => 3,
            CTEINLINERXMODE1US_A::_500NS => 4,
            CTEINLINERXMODE1US_A::_250NS => 5,
            CTEINLINERXMODE1US_A::_125NS => 6,
        }
    }
}
#[doc = "Reader of field `CTEINLINERXMODE1US`"]
pub type CTEINLINERXMODE1US_R = crate::R<u8, CTEINLINERXMODE1US_A>;
impl CTEINLINERXMODE1US_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTEINLINERXMODE1US_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CTEINLINERXMODE1US_A::_4US),
            2 => Val(CTEINLINERXMODE1US_A::_2US),
            3 => Val(CTEINLINERXMODE1US_A::_1US),
            4 => Val(CTEINLINERXMODE1US_A::_500NS),
            5 => Val(CTEINLINERXMODE1US_A::_250NS),
            6 => Val(CTEINLINERXMODE1US_A::_125NS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == CTEINLINERXMODE1US_A::_125NS
    }
}
#[doc = "Write proxy for field `CTEINLINERXMODE1US`"]
pub struct CTEINLINERXMODE1US_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEINLINERXMODE1US_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEINLINERXMODE1US_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE1US_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | (((value as u32) & 0x07) << 10);
        self.w
    }
}
#[doc = "Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTEINLINERXMODE2US_A {
    #[doc = "1: 4us"]
    _4US,
    #[doc = "2: 2us"]
    _2US,
    #[doc = "3: 1us"]
    _1US,
    #[doc = "4: 0.5us"]
    _500NS,
    #[doc = "5: 0.25us"]
    _250NS,
    #[doc = "6: 0.125us"]
    _125NS,
}
impl From<CTEINLINERXMODE2US_A> for u8 {
    #[inline(always)]
    fn from(variant: CTEINLINERXMODE2US_A) -> Self {
        match variant {
            CTEINLINERXMODE2US_A::_4US => 1,
            CTEINLINERXMODE2US_A::_2US => 2,
            CTEINLINERXMODE2US_A::_1US => 3,
            CTEINLINERXMODE2US_A::_500NS => 4,
            CTEINLINERXMODE2US_A::_250NS => 5,
            CTEINLINERXMODE2US_A::_125NS => 6,
        }
    }
}
#[doc = "Reader of field `CTEINLINERXMODE2US`"]
pub type CTEINLINERXMODE2US_R = crate::R<u8, CTEINLINERXMODE2US_A>;
impl CTEINLINERXMODE2US_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTEINLINERXMODE2US_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(CTEINLINERXMODE2US_A::_4US),
            2 => Val(CTEINLINERXMODE2US_A::_2US),
            3 => Val(CTEINLINERXMODE2US_A::_1US),
            4 => Val(CTEINLINERXMODE2US_A::_500NS),
            5 => Val(CTEINLINERXMODE2US_A::_250NS),
            6 => Val(CTEINLINERXMODE2US_A::_125NS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_4US`"]
    #[inline(always)]
    pub fn is_4us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_4US
    }
    #[doc = "Checks if the value of the field is `_2US`"]
    #[inline(always)]
    pub fn is_2us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_2US
    }
    #[doc = "Checks if the value of the field is `_1US`"]
    #[inline(always)]
    pub fn is_1us(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_1US
    }
    #[doc = "Checks if the value of the field is `_500NS`"]
    #[inline(always)]
    pub fn is_500ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_500NS
    }
    #[doc = "Checks if the value of the field is `_250NS`"]
    #[inline(always)]
    pub fn is_250ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_250NS
    }
    #[doc = "Checks if the value of the field is `_125NS`"]
    #[inline(always)]
    pub fn is_125ns(&self) -> bool {
        *self == CTEINLINERXMODE2US_A::_125NS
    }
}
#[doc = "Write proxy for field `CTEINLINERXMODE2US`"]
pub struct CTEINLINERXMODE2US_W<'a> {
    w: &'a mut W,
}
impl<'a> CTEINLINERXMODE2US_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTEINLINERXMODE2US_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "4us"]
    #[inline(always)]
    pub fn _4us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_4US)
    }
    #[doc = "2us"]
    #[inline(always)]
    pub fn _2us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_2US)
    }
    #[doc = "1us"]
    #[inline(always)]
    pub fn _1us(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_1US)
    }
    #[doc = "0.5us"]
    #[inline(always)]
    pub fn _500ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_500NS)
    }
    #[doc = "0.25us"]
    #[inline(always)]
    pub fn _250ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_250NS)
    }
    #[doc = "0.125us"]
    #[inline(always)]
    pub fn _125ns(self) -> &'a mut W {
        self.variant(CTEINLINERXMODE2US_A::_125NS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `S0CONF`"]
pub type S0CONF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0CONF`"]
pub struct S0CONF_W<'a> {
    w: &'a mut W,
}
impl<'a> S0CONF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `S0MASK`"]
pub type S0MASK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S0MASK`"]
pub struct S0MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> S0MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&self) -> CTEINLINECTRLEN_R {
        CTEINLINECTRLEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&self) -> CTEINFOINS1_R {
        CTEINFOINS1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&self) -> CTEERRORHANDLING_R {
        CTEERRORHANDLING_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&self) -> CTETIMEVALIDRANGE_R {
        CTETIMEVALIDRANGE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set"]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&self) -> CTEINLINERXMODE1US_R {
        CTEINLINERXMODE1US_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set"]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&self) -> CTEINLINERXMODE2US_R {
        CTEINLINERXMODE2US_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&self) -> S0CONF_R {
        S0CONF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&self) -> S0MASK_R {
        S0MASK_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable parsing of CTEInfo from received packet in BLE modes"]
    #[inline(always)]
    pub fn cteinlinectrlen(&mut self) -> CTEINLINECTRLEN_W {
        CTEINLINECTRLEN_W { w: self }
    }
    #[doc = "Bit 3 - CTEInfo is S1 byte or not"]
    #[inline(always)]
    pub fn cteinfoins1(&mut self) -> CTEINFOINS1_W {
        CTEINFOINS1_W { w: self }
    }
    #[doc = "Bit 4 - Sampling/switching if CRC is not OK"]
    #[inline(always)]
    pub fn cteerrorhandling(&mut self) -> CTEERRORHANDLING_W {
        CTEERRORHANDLING_W { w: self }
    }
    #[doc = "Bits 6:7 - Max range of CTETime"]
    #[inline(always)]
    pub fn ctetimevalidrange(&mut self) -> CTETIMEVALIDRANGE_W {
        CTETIMEVALIDRANGE_W { w: self }
    }
    #[doc = "Bits 10:12 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set"]
    #[inline(always)]
    pub fn cteinlinerxmode1us(&mut self) -> CTEINLINERXMODE1US_W {
        CTEINLINERXMODE1US_W { w: self }
    }
    #[doc = "Bits 13:15 - Spacing between samples for the samples in the SWITCHING period when CTEINLINEMODE is set"]
    #[inline(always)]
    pub fn cteinlinerxmode2us(&mut self) -> CTEINLINERXMODE2US_W {
        CTEINLINERXMODE2US_W { w: self }
    }
    #[doc = "Bits 16:23 - S0 bit pattern to match"]
    #[inline(always)]
    pub fn s0conf(&mut self) -> S0CONF_W {
        S0CONF_W { w: self }
    }
    #[doc = "Bits 24:31 - S0 bit mask to set which bit to match"]
    #[inline(always)]
    pub fn s0mask(&mut self) -> S0MASK_W {
        S0MASK_W { w: self }
    }
}
