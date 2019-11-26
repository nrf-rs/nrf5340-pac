#[doc = "Reader of register IMISS"]
pub type R = crate::R<u32, super::IMISS>;
#[doc = "Reader of field `MISSES`"]
pub type MISSES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of instruction cache misses"]
    #[inline(always)]
    pub fn misses(&self) -> MISSES_R {
        MISSES_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
