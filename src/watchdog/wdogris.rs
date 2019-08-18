#[doc = "Reader of register WDOGRIS"]
pub type R = crate::R<u32, super::WDOGRIS>;
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw watchdog Interrupt"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x01) != 0)
    }
}
