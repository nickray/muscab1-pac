#[doc = "Reader of register CLK_CTRL_SEL"]
pub type R = crate::R<u32, super::CLK_CTRL_SEL>;
#[doc = "Writer for register CLK_CTRL_SEL"]
pub type W = crate::W<u32, super::CLK_CTRL_SEL>;
#[doc = "Register CLK_CTRL_SEL `reset()`'s with value 0x72"]
impl crate::ResetValue for super::CLK_CTRL_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x72
    }
}
#[doc = "Reader of field `sel_premux_clk`"]
pub type SEL_PREMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_premux_clk`"]
pub struct SEL_PREMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_PREMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `sel_dapswmux_clk`"]
pub type SEL_DAPSWMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_dapswmux_clk`"]
pub struct SEL_DAPSWMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_DAPSWMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `sel_mainmux_clk`"]
pub type SEL_MAINMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_mainmux_clk`"]
pub struct SEL_MAINMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_MAINMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `sel_refmux_clk`"]
pub type SEL_REFMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_refmux_clk`"]
pub struct SEL_REFMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_REFMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `sel_rm38kmux_clk`"]
pub type SEL_RM38KMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_rm38kmux_clk`"]
pub struct SEL_RM38KMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_RM38KMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `sel_sccmux_clk`"]
pub type SEL_SCCMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_sccmux_clk`"]
pub struct SEL_SCCMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_SCCMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `sel_rm38p4_premux_clk`"]
pub type SEL_RM38P4_PREMUX_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `sel_rm38p4_premux_clk`"]
pub struct SEL_RM38P4_PREMUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_RM38P4_PREMUX_CLK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `ctrl_sel_test_mux_clk`"]
pub type CTRL_SEL_TEST_MUX_CLK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ctrl_sel_test_mux_clk`"]
pub struct CTRL_SEL_TEST_MUX_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_SEL_TEST_MUX_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: 32k 1: FASTCLK"]
    #[inline(always)]
    pub fn sel_premux_clk(&self) -> SEL_PREMUX_CLK_R {
        SEL_PREMUX_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0: PRE_MUX_CLK 1: TCK"]
    #[inline(always)]
    pub fn sel_dapswmux_clk(&self) -> SEL_DAPSWMUX_CLK_R {
        SEL_DAPSWMUX_CLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0: PLL0_CLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_mainmux_clk(&self) -> SEL_MAINMUX_CLK_R {
        SEL_MAINMUX_CLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
    #[inline(always)]
    pub fn sel_refmux_clk(&self) -> SEL_REFMUX_CLK_R {
        SEL_REFMUX_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0: REF_MUX_CLK 1: RM38K"]
    #[inline(always)]
    pub fn sel_rm38kmux_clk(&self) -> SEL_RM38KMUX_CLK_R {
        SEL_RM38KMUX_CLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0: SCCCLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_sccmux_clk(&self) -> SEL_SCCMUX_CLK_R {
        SEL_SCCMUX_CLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0: SYSSYSSUGCLK 1: NRM138P4"]
    #[inline(always)]
    pub fn sel_rm38p4_premux_clk(&self) -> SEL_RM38P4_PREMUX_CLK_R {
        SEL_RM38P4_PREMUX_CLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:11 - ctrl_sel_test_mux_clk"]
    #[inline(always)]
    pub fn ctrl_sel_test_mux_clk(&self) -> CTRL_SEL_TEST_MUX_CLK_R {
        CTRL_SEL_TEST_MUX_CLK_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: 32k 1: FASTCLK"]
    #[inline(always)]
    pub fn sel_premux_clk(&mut self) -> SEL_PREMUX_CLK_W {
        SEL_PREMUX_CLK_W { w: self }
    }
    #[doc = "Bit 1 - 0: PRE_MUX_CLK 1: TCK"]
    #[inline(always)]
    pub fn sel_dapswmux_clk(&mut self) -> SEL_DAPSWMUX_CLK_W {
        SEL_DAPSWMUX_CLK_W { w: self }
    }
    #[doc = "Bit 2 - 0: PLL0_CLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_mainmux_clk(&mut self) -> SEL_MAINMUX_CLK_W {
        SEL_MAINMUX_CLK_W { w: self }
    }
    #[doc = "Bit 3 - 0: PRE_MUX_CLK 1: PRE_PLL_CLK"]
    #[inline(always)]
    pub fn sel_refmux_clk(&mut self) -> SEL_REFMUX_CLK_W {
        SEL_REFMUX_CLK_W { w: self }
    }
    #[doc = "Bit 4 - 0: REF_MUX_CLK 1: RM38K"]
    #[inline(always)]
    pub fn sel_rm38kmux_clk(&mut self) -> SEL_RM38KMUX_CLK_W {
        SEL_RM38KMUX_CLK_W { w: self }
    }
    #[doc = "Bit 5 - 0: SCCCLK 1: PRE_MUX_CLK"]
    #[inline(always)]
    pub fn sel_sccmux_clk(&mut self) -> SEL_SCCMUX_CLK_W {
        SEL_SCCMUX_CLK_W { w: self }
    }
    #[doc = "Bit 6 - 0: SYSSYSSUGCLK 1: NRM138P4"]
    #[inline(always)]
    pub fn sel_rm38p4_premux_clk(&mut self) -> SEL_RM38P4_PREMUX_CLK_W {
        SEL_RM38P4_PREMUX_CLK_W { w: self }
    }
    #[doc = "Bits 7:11 - ctrl_sel_test_mux_clk"]
    #[inline(always)]
    pub fn ctrl_sel_test_mux_clk(&mut self) -> CTRL_SEL_TEST_MUX_CLK_W {
        CTRL_SEL_TEST_MUX_CLK_W { w: self }
    }
}
