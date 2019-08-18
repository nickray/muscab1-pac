#[doc = "Reader of register INTR_CTRL"]
pub type R = crate::R<u32, super::INTR_CTRL>;
#[doc = "Writer for register INTR_CTRL"]
pub type W = crate::W<u32, super::INTR_CTRL>;
#[doc = "Register INTR_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::INTR_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QSPI_MPC_CFG_INIT_VALUE`"]
pub type QSPI_MPC_CFG_INIT_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI_MPC_CFG_INIT_VALUE`"]
pub struct QSPI_MPC_CFG_INIT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI_MPC_CFG_INIT_VALUE_W<'a> {
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
#[doc = "Reader of field `SRAM_MPC_CFG_INIT_VALUE`"]
pub type SRAM_MPC_CFG_INIT_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM_MPC_CFG_INIT_VALUE`"]
pub struct SRAM_MPC_CFG_INIT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_MPC_CFG_INIT_VALUE_W<'a> {
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
#[doc = "Reader of field `AZ_MPC_CFG_INIT_VALUE`"]
pub type AZ_MPC_CFG_INIT_VALUE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AZ_MPC_CFG_INIT_VALUE`"]
pub struct AZ_MPC_CFG_INIT_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> AZ_MPC_CFG_INIT_VALUE_W<'a> {
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
    #[doc = "Bit 3 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn qspi_mpc_cfg_init_value(&self) -> QSPI_MPC_CFG_INIT_VALUE_R {
        QSPI_MPC_CFG_INIT_VALUE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn sram_mpc_cfg_init_value(&self) -> SRAM_MPC_CFG_INIT_VALUE_R {
        SRAM_MPC_CFG_INIT_VALUE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn az_mpc_cfg_init_value(&self) -> AZ_MPC_CFG_INIT_VALUE_R {
        AZ_MPC_CFG_INIT_VALUE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn qspi_mpc_cfg_init_value(&mut self) -> QSPI_MPC_CFG_INIT_VALUE_W {
        QSPI_MPC_CFG_INIT_VALUE_W { w: self }
    }
    #[doc = "Bit 5 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn sram_mpc_cfg_init_value(&mut self) -> SRAM_MPC_CFG_INIT_VALUE_W {
        SRAM_MPC_CFG_INIT_VALUE_W { w: self }
    }
    #[doc = "Bit 6 - 0: Secure mode 1: Non-secure mode"]
    #[inline(always)]
    pub fn az_mpc_cfg_init_value(&mut self) -> AZ_MPC_CFG_INIT_VALUE_W {
        AZ_MPC_CFG_INIT_VALUE_W { w: self }
    }
}
