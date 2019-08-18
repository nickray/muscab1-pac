#[doc = "Reader of register DBG_CTRL"]
pub type R = crate::R<u32, super::DBG_CTRL>;
#[doc = "Writer for register DBG_CTRL"]
pub type W = crate::W<u32, super::DBG_CTRL>;
#[doc = "Register DBG_CTRL `reset()`'s with value 0x1f"]
impl crate::ResetValue for super::DBG_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1f
    }
}
#[doc = "Reader of field `SSE_200_DBGENIN`"]
pub type SSE_200_DBGENIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSE_200_DBGENIN`"]
pub struct SSE_200_DBGENIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_200_DBGENIN_W<'a> {
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
#[doc = "Reader of field `SSE_200_NIDENIN`"]
pub type SSE_200_NIDENIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSE_200_NIDENIN`"]
pub struct SSE_200_NIDENIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_200_NIDENIN_W<'a> {
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
#[doc = "Reader of field `SSE_200_SPIDENIN`"]
pub type SSE_200_SPIDENIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSE_200_SPIDENIN`"]
pub struct SSE_200_SPIDENIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_200_SPIDENIN_W<'a> {
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
#[doc = "Reader of field `SSE_200_SPNIDENIN`"]
pub type SSE_200_SPNIDENIN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSE_200_SPNIDENIN`"]
pub struct SSE_200_SPNIDENIN_W<'a> {
    w: &'a mut W,
}
impl<'a> SSE_200_SPNIDENIN_W<'a> {
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
#[doc = "Reader of field `TODBGENSEL0`"]
pub type TODBGENSEL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TODBGENSEL0`"]
pub struct TODBGENSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> TODBGENSEL0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `TODBGENSEL1`"]
pub type TODBGENSEL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TODBGENSEL1`"]
pub struct TODBGENSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> TODBGENSEL1_W<'a> {
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
#[doc = "Reader of field `DBG_DCU_FORCE`"]
pub type DBG_DCU_FORCE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBG_DCU_FORCE`"]
pub struct DBG_DCU_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBG_DCU_FORCE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_dbgenin(&self) -> SSE_200_DBGENIN_R {
        SSE_200_DBGENIN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_nidenin(&self) -> SSE_200_NIDENIN_R {
        SSE_200_NIDENIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spidenin(&self) -> SSE_200_SPIDENIN_R {
        SSE_200_SPIDENIN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spnidenin(&self) -> SSE_200_SPNIDENIN_R {
        SSE_200_SPNIDENIN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel0(&self) -> TODBGENSEL0_R {
        TODBGENSEL0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel1(&self) -> TODBGENSEL1_R {
        TODBGENSEL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - SSE-200 debug ports control"]
    #[inline(always)]
    pub fn dbg_dcu_force(&self) -> DBG_DCU_FORCE_R {
        DBG_DCU_FORCE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_dbgenin(&mut self) -> SSE_200_DBGENIN_W {
        SSE_200_DBGENIN_W { w: self }
    }
    #[doc = "Bit 1 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_nidenin(&mut self) -> SSE_200_NIDENIN_W {
        SSE_200_NIDENIN_W { w: self }
    }
    #[doc = "Bit 2 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spidenin(&mut self) -> SSE_200_SPIDENIN_W {
        SSE_200_SPIDENIN_W { w: self }
    }
    #[doc = "Bit 3 - 0: Not enable 1: Enable"]
    #[inline(always)]
    pub fn sse_200_spnidenin(&mut self) -> SSE_200_SPNIDENIN_W {
        SSE_200_SPNIDENIN_W { w: self }
    }
    #[doc = "Bit 7 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel0(&mut self) -> TODBGENSEL0_W {
        TODBGENSEL0_W { w: self }
    }
    #[doc = "Bit 8 - 0: Enable 1: Mask or bypass"]
    #[inline(always)]
    pub fn todbgensel1(&mut self) -> TODBGENSEL1_W {
        TODBGENSEL1_W { w: self }
    }
    #[doc = "Bits 30:31 - SSE-200 debug ports control"]
    #[inline(always)]
    pub fn dbg_dcu_force(&mut self) -> DBG_DCU_FORCE_W {
        DBG_DCU_FORCE_W { w: self }
    }
}
