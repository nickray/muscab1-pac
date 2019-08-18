#[doc = "Reader of register CLK_STATUS"]
pub type R = crate::R<u32, super::CLK_STATUS>;
#[doc = "Reader of field `status_out_clk_mainclk_ready`"]
pub type STATUS_OUT_CLK_MAINCLK_READY_R = crate::R<bool, bool>;
#[doc = "Reader of field `status_lock_signal_pll0_clk`"]
pub type STATUS_LOCK_SIGNAL_PLL0_CLK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Clock ready (active)"]
    #[inline(always)]
    pub fn status_out_clk_mainclk_ready(&self) -> STATUS_OUT_CLK_MAINCLK_READY_R {
        STATUS_OUT_CLK_MAINCLK_READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL Lock Status"]
    #[inline(always)]
    pub fn status_lock_signal_pll0_clk(&self) -> STATUS_LOCK_SIGNAL_PLL0_CLK_R {
        STATUS_LOCK_SIGNAL_PLL0_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
