#[doc = "Reader of register FLASH1_DOUT_1"]
pub type R = crate::R<u32, super::FLASH1_DOUT_1>;
#[doc = "Reader of field `scc_flash1_dout1`"]
pub type SCC_FLASH1_DOUT1_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[63:32\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout1(&self) -> SCC_FLASH1_DOUT1_R {
        SCC_FLASH1_DOUT1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
