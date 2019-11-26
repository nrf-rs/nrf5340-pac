#[doc = "Reader of register CODESIZE"]
pub type R = crate::R<u32, super::CODESIZE>;
#[doc = "Code memory size in number of pages\n\nValue on reset: 4294967295"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CODESIZE_A {
    #[doc = "128: 128 pages"]
    P128,
}
impl From<CODESIZE_A> for u32 {
    #[inline(always)]
    fn from(variant: CODESIZE_A) -> Self {
        match variant {
            CODESIZE_A::P128 => 128,
        }
    }
}
#[doc = "Reader of field `CODESIZE`"]
pub type CODESIZE_R = crate::R<u32, CODESIZE_A>;
impl CODESIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, CODESIZE_A> {
        use crate::Variant::*;
        match self.bits {
            128 => Val(CODESIZE_A::P128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `P128`"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        *self == CODESIZE_A::P128
    }
}
impl R {
    #[doc = "Bits 0:31 - Code memory size in number of pages"]
    #[inline(always)]
    pub fn codesize(&self) -> CODESIZE_R {
        CODESIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
