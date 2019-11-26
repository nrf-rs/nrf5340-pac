#[doc = "Reader of register CPUID"]
pub type R = crate::R<u32, super::CPUID>;
#[doc = "Reader of field `CPUID`"]
pub type CPUID_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - CPU ID"]
    #[inline(always)]
    pub fn cpuid(&self) -> CPUID_R {
        CPUID_R::new((self.bits & 0xff) as u8)
    }
}
