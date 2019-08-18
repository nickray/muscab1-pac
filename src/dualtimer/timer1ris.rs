#[doc = "Reader of register TIMER1RIS"]
pub type R = crate::R<u32, super::TIMER1RIS>;
#[doc = "Reader of field `RIS`"]
pub type RIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Raw Timer Interrupt"]
    #[inline(always)]
    pub fn ris(&self) -> RIS_R {
        RIS_R::new((self.bits & 0x01) != 0)
    }
}
