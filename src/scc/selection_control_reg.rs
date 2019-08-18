#[doc = "Reader of register SELECTION_CONTROL_REG"]
pub type R = crate::R<u32, super::SELECTION_CONTROL_REG>;
#[doc = "Writer for register SELECTION_CONTROL_REG"]
pub type W = crate::W<u32, super::SELECTION_CONTROL_REG>;
#[doc = "Register SELECTION_CONTROL_REG `reset()`'s with value 0x0100_0200"]
impl crate::ResetValue for super::SELECTION_CONTROL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100_0200
    }
}
#[doc = "Reader of field `clock_phase_shifter_select`"]
pub type CLOCK_PHASE_SHIFTER_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `clock_phase_shifter_select`"]
pub struct CLOCK_PHASE_SHIFTER_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_PHASE_SHIFTER_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `clock_phase_shifter_bypass`"]
pub type CLOCK_PHASE_SHIFTER_BYPASS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `clock_phase_shifter_bypass`"]
pub struct CLOCK_PHASE_SHIFTER_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCK_PHASE_SHIFTER_BYPASS_W<'a> {
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
#[doc = "Reader of field `sdio_mask_delay`"]
pub type SDIO_MASK_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `sdio_mask_delay`"]
pub struct SDIO_MASK_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_MASK_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_select(&self) -> CLOCK_PHASE_SHIFTER_SELECT_R {
        CLOCK_PHASE_SHIFTER_SELECT_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_bypass(&self) -> CLOCK_PHASE_SHIFTER_BYPASS_R {
        CLOCK_PHASE_SHIFTER_BYPASS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - SDIO mask delay"]
    #[inline(always)]
    pub fn sdio_mask_delay(&self) -> SDIO_MASK_DELAY_R {
        SDIO_MASK_DELAY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_select(&mut self) -> CLOCK_PHASE_SHIFTER_SELECT_W {
        CLOCK_PHASE_SHIFTER_SELECT_W { w: self }
    }
    #[doc = "Bit 2 - QSPI input clock phase shift control"]
    #[inline(always)]
    pub fn clock_phase_shifter_bypass(&mut self) -> CLOCK_PHASE_SHIFTER_BYPASS_W {
        CLOCK_PHASE_SHIFTER_BYPASS_W { w: self }
    }
    #[doc = "Bits 8:9 - SDIO mask delay"]
    #[inline(always)]
    pub fn sdio_mask_delay(&mut self) -> SDIO_MASK_DELAY_W {
        SDIO_MASK_DELAY_W { w: self }
    }
}
