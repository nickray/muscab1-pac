#[doc = "Reader of register TIMER2MIS"]
pub type R = crate::R<u32, super::TIMER2MIS>;
#[doc = "Reader of field `MIS`"]
pub type MIS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Masked Timer Interrupt"]
    #[inline(always)]
    pub fn mis(&self) -> MIS_R {
        MIS_R::new((self.bits & 0x01) != 0)
    }
}
