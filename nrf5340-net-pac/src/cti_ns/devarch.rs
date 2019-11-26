#[doc = "Reader of register DEVARCH"]
pub type R = crate::R<u32, super::DEVARCH>;
#[doc = "Reader of field `Architecture`"]
pub type ARCHITECTURE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Contains the CTI device architecture"]
    #[inline(always)]
    pub fn architecture(&self) -> ARCHITECTURE_R {
        ARCHITECTURE_R::new((self.bits & 0x01) != 0)
    }
}
