#[doc = "Reader of register PART"]
pub type R = crate::R<u32, super::PART>;
#[doc = "Part code\n\nValue on reset: 21312"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PART_A {
    #[doc = "21312: nRF5340"]
    N5340,
    #[doc = "4294967295: Unspecified"]
    UNSPECIFIED,
}
impl From<PART_A> for u32 {
    #[inline(always)]
    fn from(variant: PART_A) -> Self {
        match variant {
            PART_A::N5340 => 21312,
            PART_A::UNSPECIFIED => 4294967295,
        }
    }
}
#[doc = "Reader of field `PART`"]
pub type PART_R = crate::R<u32, PART_A>;
impl PART_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, PART_A> {
        use crate::Variant::*;
        match self.bits {
            21312 => Val(PART_A::N5340),
            4294967295 => Val(PART_A::UNSPECIFIED),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `N5340`"]
    #[inline(always)]
    pub fn is_n5340(&self) -> bool {
        *self == PART_A::N5340
    }
    #[doc = "Checks if the value of the field is `UNSPECIFIED`"]
    #[inline(always)]
    pub fn is_unspecified(&self) -> bool {
        *self == PART_A::UNSPECIFIED
    }
}
impl R {
    #[doc = "Bits 0:31 - Part code"]
    #[inline(always)]
    pub fn part(&self) -> PART_R {
        PART_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
