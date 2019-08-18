#[doc = "Reader of register FLASH0_DOUT_1"]
pub type R = crate::R<u32, super::FLASH0_DOUT_1>;
#[doc = "Reader of field `scc_flash0_dout1`"]
pub type SCC_FLASH0_DOUT1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[63:32\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout1(&self) -> SCC_FLASH0_DOUT1_R {
        SCC_FLASH0_DOUT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
