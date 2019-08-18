#[doc = "Reader of register FLASH0_DOUT_2"]
pub type R = crate::R<u32, super::FLASH0_DOUT_2>;
#[doc = "Reader of field `scc_flash0_dout2`"]
pub type SCC_FLASH0_DOUT2_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[95:64\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout2(&self) -> SCC_FLASH0_DOUT2_R {
        SCC_FLASH0_DOUT2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
