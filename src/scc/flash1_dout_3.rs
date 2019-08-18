#[doc = "Reader of register FLASH1_DOUT_3"]
pub type R = crate::R<u32, super::FLASH1_DOUT_3>;
#[doc = "Reader of field `scc_flash1_dout3`"]
pub type SCC_FLASH1_DOUT3_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 1 data output\\[127:96\\]"]
    #[inline(always)]
    pub fn scc_flash1_dout3(&self) -> SCC_FLASH1_DOUT3_R {
        SCC_FLASH1_DOUT3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
