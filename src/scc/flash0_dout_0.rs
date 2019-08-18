#[doc = "Reader of register FLASH0_DOUT_0"]
pub type R = crate::R<u32, super::FLASH0_DOUT_0>;
#[doc = "Reader of field `scc_flash0_dout0`"]
pub type SCC_FLASH0_DOUT0_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - eFlash 0 data output\\[31:0\\]"]
    #[inline(always)]
    pub fn scc_flash0_dout0(&self) -> SCC_FLASH0_DOUT0_R {
        SCC_FLASH0_DOUT0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
