#[doc = "Reader of register PLL_POSTDIV_CTRL_PLL0_CLK"]
pub type R = crate::R<u32, super::PLL_POSTDIV_CTRL_PLL0_CLK>;
#[doc = "Writer for register PLL_POSTDIV_CTRL_PLL0_CLK"]
pub type W = crate::W<u32, super::PLL_POSTDIV_CTRL_PLL0_CLK>;
#[doc = "Register PLL_POSTDIV_CTRL_PLL0_CLK `reset()`'s with value 0x01"]
impl crate::ResetValue for super::PLL_POSTDIV_CTRL_PLL0_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `pll_postdiv_ctrl_pll0_clk`"]
pub type PLL_POSTDIV_CTRL_PLL0_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `pll_postdiv_ctrl_pll0_clk`"]
pub struct PLL_POSTDIV_CTRL_PLL0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_POSTDIV_CTRL_PLL0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - pll_postdiv_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_postdiv_ctrl_pll0_clk(&self) -> PLL_POSTDIV_CTRL_PLL0_CLK_R {
        PLL_POSTDIV_CTRL_PLL0_CLK_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - pll_postdiv_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_postdiv_ctrl_pll0_clk(&mut self) -> PLL_POSTDIV_CTRL_PLL0_CLK_W {
        PLL_POSTDIV_CTRL_PLL0_CLK_W { w: self }
    }
}
