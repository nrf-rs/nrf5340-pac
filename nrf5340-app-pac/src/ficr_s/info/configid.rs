#[doc = "Reader of register CONFIGID"]
pub type R = crate::R<u32, super::CONFIGID>;
#[doc = "Reader of field `HWID`"]
pub type HWID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Identification number for the HW"]
    #[inline(always)]
    pub fn hwid(&self) -> HWID_R {
        HWID_R::new((self.bits & 0xffff) as u16)
    }
}
