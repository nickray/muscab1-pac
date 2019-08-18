#[doc = "Reader of register CTRL_BYPASS_DIV"]
pub type R = crate::R<u32, super::CTRL_BYPASS_DIV>;
#[doc = "Writer for register CTRL_BYPASS_DIV"]
pub type W = crate::W<u32, super::CTRL_BYPASS_DIV>;
#[doc = "Register CTRL_BYPASS_DIV `reset()`'s with value 0x01"]
impl crate::ResetValue for super::CTRL_BYPASS_DIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Reader of field `bypass_div_pll_div_prediv_clk`"]
pub type BYPASS_DIV_PLL_DIV_PREDIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_div_pll_div_prediv_clk`"]
pub struct BYPASS_DIV_PLL_DIV_PREDIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_DIV_PLL_DIV_PREDIV_CLK_W<'a> {
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
#[doc = "Reader of field `bypass_qspi_div_clk`"]
pub type BYPASS_QSPI_DIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_qspi_div_clk`"]
pub struct BYPASS_QSPI_DIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_QSPI_DIV_CLK_W<'a> {
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
#[doc = "Reader of field `bypass_rtc_div_clk`"]
pub type BYPASS_RTC_DIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_rtc_div_clk`"]
pub struct BYPASS_RTC_DIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_RTC_DIV_CLK_W<'a> {
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
#[doc = "Reader of field `bypass_sd_div_clk`"]
pub type BYPASS_SD_DIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_sd_div_clk`"]
pub struct BYPASS_SD_DIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_SD_DIV_CLK_W<'a> {
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
#[doc = "Reader of field `bypass_test_div_clk`"]
pub type BYPASS_TEST_DIV_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bypass_test_div_clk`"]
pub struct BYPASS_TEST_DIV_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_TEST_DIV_CLK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_div_pll_div_prediv_clk(&self) -> BYPASS_DIV_PLL_DIV_PREDIV_CLK_R {
        BYPASS_DIV_PLL_DIV_PREDIV_CLK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_qspi_div_clk(&self) -> BYPASS_QSPI_DIV_CLK_R {
        BYPASS_QSPI_DIV_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_rtc_div_clk(&self) -> BYPASS_RTC_DIV_CLK_R {
        BYPASS_RTC_DIV_CLK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_sd_div_clk(&self) -> BYPASS_SD_DIV_CLK_R {
        BYPASS_SD_DIV_CLK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_test_div_clk(&self) -> BYPASS_TEST_DIV_CLK_R {
        BYPASS_TEST_DIV_CLK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_div_pll_div_prediv_clk(&mut self) -> BYPASS_DIV_PLL_DIV_PREDIV_CLK_W {
        BYPASS_DIV_PLL_DIV_PREDIV_CLK_W { w: self }
    }
    #[doc = "Bit 3 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_qspi_div_clk(&mut self) -> BYPASS_QSPI_DIV_CLK_W {
        BYPASS_QSPI_DIV_CLK_W { w: self }
    }
    #[doc = "Bit 4 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_rtc_div_clk(&mut self) -> BYPASS_RTC_DIV_CLK_W {
        BYPASS_RTC_DIV_CLK_W { w: self }
    }
    #[doc = "Bit 5 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_sd_div_clk(&mut self) -> BYPASS_SD_DIV_CLK_W {
        BYPASS_SD_DIV_CLK_W { w: self }
    }
    #[doc = "Bit 6 - 0: Not bypass 1: bypass"]
    #[inline(always)]
    pub fn bypass_test_div_clk(&mut self) -> BYPASS_TEST_DIV_CLK_W {
        BYPASS_TEST_DIV_CLK_W { w: self }
    }
}
