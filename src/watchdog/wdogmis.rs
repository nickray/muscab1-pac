#[doc = "Reader of register WDOGMIS"]
pub type R = crate::R<u32, super::WDOGMIS>;
#[doc = "Reader of field `MIS`"]
pub type MIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Masked Watchdog Interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0x01) != 0)
    }
}
