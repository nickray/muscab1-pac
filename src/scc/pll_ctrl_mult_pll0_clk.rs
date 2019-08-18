#[doc = "Reader of register PLL_CTRL_MULT_PLL0_CLK"]
pub type R = crate::R<u32, super::PLL_CTRL_MULT_PLL0_CLK>;
#[doc = "Writer for register PLL_CTRL_MULT_PLL0_CLK"]
pub type W = crate::W<u32, super::PLL_CTRL_MULT_PLL0_CLK>;
#[doc = "Register PLL_CTRL_MULT_PLL0_CLK `reset()`'s with value 0x1388"]
impl crate::ResetValue for super::PLL_CTRL_MULT_PLL0_CLK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1388
    }
}
#[doc = "Reader of field `pll_mult_ctrl_pll0_clk`"]
pub type PLL_MULT_CTRL_PLL0_CLK_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `pll_mult_ctrl_pll0_clk`"]
pub struct PLL_MULT_CTRL_PLL0_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL_MULT_CTRL_PLL0_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - pll_mult_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_mult_ctrl_pll0_clk(&self) -> PLL_MULT_CTRL_PLL0_CLK_R {
        PLL_MULT_CTRL_PLL0_CLK_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - pll_mult_ctrl_pll0_clk"]
    #[inline(always)]
    pub fn pll_mult_ctrl_pll0_clk(&mut self) -> PLL_MULT_CTRL_PLL0_CLK_W {
        PLL_MULT_CTRL_PLL0_CLK_W { w: self }
    }
}
