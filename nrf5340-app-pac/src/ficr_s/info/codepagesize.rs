#[doc = "Reader of register CODEPAGESIZE"]
pub type R = crate::R<u32, super::CODEPAGESIZE>;
#[doc = "Code memory page size in bytes\n\nValue on reset: 4096"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODEPAGESIZE_A {
    #[doc = "4096: 4 kByte"]
    K4096,
}
impl From<CODEPAGESIZE_A> for u32 {
    #[inline(always)]
    fn from(variant: CODEPAGESIZE_A) -> Self {
        match variant {
            CODEPAGESIZE_A::K4096 => 4096,
        }
    }
}
#[doc = "Reader of field `CODEPAGESIZE`"]
pub type CODEPAGESIZE_R = crate::R<u32, CODEPAGESIZE_A>;
impl CODEPAGESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CODEPAGESIZE_A> {
        use crate::Variant::*;
        match self.bits {
            4096 => Val(CODEPAGESIZE_A::K4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K4096`"]
    #[inline(always)]
    pub fn is_k4096(&self) -> bool {
        *self == CODEPAGESIZE_A::K4096
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory page size in bytes"]
    #[inline(always)]
    pub fn codepagesize(&self) -> CODEPAGESIZE_R {
        CODEPAGESIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
