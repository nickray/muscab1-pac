#[doc = "Reader of register CLK_CTRL_ENABLE"]
pub type R = crate::R<u32, super::CLK_CTRL_ENABLE>;
#[doc = "Writer for register CLK_CTRL_ENABLE"]
pub type W = crate::W<u32, super::CLK_CTRL_ENABLE>;
#[doc = "Register CLK_CTRL_ENABLE `reset()`'s with value 0xffff"]
impl crate::ResetValue for super::CLK_CTRL_ENABLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff
    }
}
#[doc = "Reader of field `ctrl_enable_1hz`"]
pub type CTRL_ENABLE_1HZ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_1hz`"]
pub struct CTRL_ENABLE_1HZ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_1HZ_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_dapswclk`"]
pub type CTRL_ENABLE_DAPSWCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_dapswclk`"]
pub struct CTRL_ENABLE_DAPSWCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_DAPSWCLK_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_gpiohclk`"]
pub type CTRL_ENABLE_GPIOHCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_gpiohclk`"]
pub struct CTRL_ENABLE_GPIOHCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_GPIOHCLK_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_i2sclk0`"]
pub type CTRL_ENABLE_I2SCLK0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_i2sclk0`"]
pub struct CTRL_ENABLE_I2SCLK0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_I2SCLK0_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_i2sclk1`"]
pub type CTRL_ENABLE_I2SCLK1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_i2sclk1`"]
pub struct CTRL_ENABLE_I2SCLK1_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_I2SCLK1_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_i2sclk2`"]
pub type CTRL_ENABLE_I2SCLK2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_i2sclk2`"]
pub struct CTRL_ENABLE_I2SCLK2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_I2SCLK2_W<'a> {
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
#[doc = "Reader of field `ctrl_enable_mainclk`"]
pub type CTRL_ENABLE_MAINCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_mainclk`"]
pub struct CTRL_ENABLE_MAINCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_MAINCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_qspi_phy_clk`"]
pub type CTRL_ENABLE_QSPI_PHY_CLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_qspi_phy_clk`"]
pub struct CTRL_ENABLE_QSPI_PHY_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_QSPI_PHY_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_refclk`"]
pub type CTRL_ENABLE_REFCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_refclk`"]
pub struct CTRL_ENABLE_REFCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_REFCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_rm38kclk`"]
pub type CTRL_ENABLE_RM38KCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_rm38kclk`"]
pub struct CTRL_ENABLE_RM38KCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_RM38KCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_sccclk`"]
pub type CTRL_ENABLE_SCCCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_sccclk`"]
pub struct CTRL_ENABLE_SCCCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_SCCCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_sdphyclk`"]
pub type CTRL_ENABLE_SDPHYCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_sdphyclk`"]
pub struct CTRL_ENABLE_SDPHYCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_SDPHYCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `ctrl_enable_testclk`"]
pub type CTRL_ENABLE_TESTCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ctrl_enable_testclk`"]
pub struct CTRL_ENABLE_TESTCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_TESTCLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_1hz(&self) -> CTRL_ENABLE_1HZ_R {
        CTRL_ENABLE_1HZ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_dapswclk(&self) -> CTRL_ENABLE_DAPSWCLK_R {
        CTRL_ENABLE_DAPSWCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_gpiohclk(&self) -> CTRL_ENABLE_GPIOHCLK_R {
        CTRL_ENABLE_GPIOHCLK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk0(&self) -> CTRL_ENABLE_I2SCLK0_R {
        CTRL_ENABLE_I2SCLK0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk1(&self) -> CTRL_ENABLE_I2SCLK1_R {
        CTRL_ENABLE_I2SCLK1_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk2(&self) -> CTRL_ENABLE_I2SCLK2_R {
        CTRL_ENABLE_I2SCLK2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_mainclk(&self) -> CTRL_ENABLE_MAINCLK_R {
        CTRL_ENABLE_MAINCLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_qspi_phy_clk(&self) -> CTRL_ENABLE_QSPI_PHY_CLK_R {
        CTRL_ENABLE_QSPI_PHY_CLK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_refclk(&self) -> CTRL_ENABLE_REFCLK_R {
        CTRL_ENABLE_REFCLK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_rm38kclk(&self) -> CTRL_ENABLE_RM38KCLK_R {
        CTRL_ENABLE_RM38KCLK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sccclk(&self) -> CTRL_ENABLE_SCCCLK_R {
        CTRL_ENABLE_SCCCLK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sdphyclk(&self) -> CTRL_ENABLE_SDPHYCLK_R {
        CTRL_ENABLE_SDPHYCLK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_testclk(&self) -> CTRL_ENABLE_TESTCLK_R {
        CTRL_ENABLE_TESTCLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_1hz(&mut self) -> CTRL_ENABLE_1HZ_W {
        CTRL_ENABLE_1HZ_W { w: self }
    }
    #[doc = "Bit 1 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_dapswclk(&mut self) -> CTRL_ENABLE_DAPSWCLK_W {
        CTRL_ENABLE_DAPSWCLK_W { w: self }
    }
    #[doc = "Bit 2 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_gpiohclk(&mut self) -> CTRL_ENABLE_GPIOHCLK_W {
        CTRL_ENABLE_GPIOHCLK_W { w: self }
    }
    #[doc = "Bit 3 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk0(&mut self) -> CTRL_ENABLE_I2SCLK0_W {
        CTRL_ENABLE_I2SCLK0_W { w: self }
    }
    #[doc = "Bit 4 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk1(&mut self) -> CTRL_ENABLE_I2SCLK1_W {
        CTRL_ENABLE_I2SCLK1_W { w: self }
    }
    #[doc = "Bit 5 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_i2sclk2(&mut self) -> CTRL_ENABLE_I2SCLK2_W {
        CTRL_ENABLE_I2SCLK2_W { w: self }
    }
    #[doc = "Bit 8 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_mainclk(&mut self) -> CTRL_ENABLE_MAINCLK_W {
        CTRL_ENABLE_MAINCLK_W { w: self }
    }
    #[doc = "Bit 9 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_qspi_phy_clk(&mut self) -> CTRL_ENABLE_QSPI_PHY_CLK_W {
        CTRL_ENABLE_QSPI_PHY_CLK_W { w: self }
    }
    #[doc = "Bit 10 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_refclk(&mut self) -> CTRL_ENABLE_REFCLK_W {
        CTRL_ENABLE_REFCLK_W { w: self }
    }
    #[doc = "Bit 11 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_rm38kclk(&mut self) -> CTRL_ENABLE_RM38KCLK_W {
        CTRL_ENABLE_RM38KCLK_W { w: self }
    }
    #[doc = "Bit 12 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sccclk(&mut self) -> CTRL_ENABLE_SCCCLK_W {
        CTRL_ENABLE_SCCCLK_W { w: self }
    }
    #[doc = "Bit 13 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_sdphyclk(&mut self) -> CTRL_ENABLE_SDPHYCLK_W {
        CTRL_ENABLE_SDPHYCLK_W { w: self }
    }
    #[doc = "Bit 15 - 0: Disable; 1: Enable"]
    #[inline(always)]
    pub fn ctrl_enable_testclk(&mut self) -> CTRL_ENABLE_TESTCLK_W {
        CTRL_ENABLE_TESTCLK_W { w: self }
    }
}
