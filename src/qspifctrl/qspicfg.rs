#[doc = "Reader of register QSPICFG"]
pub type R = crate::R<u32, super::QSPICFG>;
#[doc = "Writer for register QSPICFG"]
pub type W = crate::W<u32, super::QSPICFG>;
#[doc = "Register QSPICFG `reset()`'s with value 0x8078_0081"]
impl crate::ResetValue for super::QSPICFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x8078_0081
    }
}
#[doc = "Reader of field `PIPLIDLE`"]
pub type PIPLIDLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIPLIDLE`"]
pub struct PIPLIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIPLIDLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `PIPLPHYEN`"]
pub type PIPLPHYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PIPLPHYEN`"]
pub struct PIPLPHYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PIPLPHYEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `DTREN`"]
pub type DTREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DTREN`"]
pub struct DTREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `AHBDECEN`"]
pub type AHBDECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AHBDECEN`"]
pub struct AHBDECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHBDECEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `MAMOBRDIV`"]
pub type MAMOBRDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAMOBRDIV`"]
pub struct MAMOBRDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MAMOBRDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | (((value as u32) & 0x0f) << 19);
        self.w
    }
}
#[doc = "Reader of field `ENTRXIPMODEIMM`"]
pub type ENTRXIPMODEIMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTRXIPMODEIMM`"]
pub struct ENTRXIPMODEIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRXIPMODEIMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `ENTRXIPMODEONR`"]
pub type ENTRXIPMODEONR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENTRXIPMODEONR`"]
pub struct ENTRXIPMODEONR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTRXIPMODEONR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `ENAHBADDRRM`"]
pub type ENAHBADDRRM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENAHBADDRRM`"]
pub struct ENAHBADDRRM_W<'a> {
    w: &'a mut W,
}
impl<'a> ENAHBADDRRM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `ENDMAPIF`"]
pub type ENDMAPIF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDMAPIF`"]
pub struct ENDMAPIF_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDMAPIF_W<'a> {
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
#[doc = "Reader of field `WPPINDRV`"]
pub type WPPINDRV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WPPINDRV`"]
pub struct WPPINDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> WPPINDRV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `PERCSLINES`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERCSLINES_A {
    #[doc = "n_ss_out: 0b1110"]
    SS0,
    #[doc = "n_ss_out: 0b1101"]
    SS1,
    #[doc = "n_ss_out: 0b1011"]
    SS2,
    #[doc = "n_ss_out: 0b0111"]
    SS3,
    #[doc = "n_ss_out: 0b1111 (no peripheral selected)"]
    SSINACTIVE,
}
impl From<PERCSLINES_A> for u8 {
    #[inline(always)]
    fn from(variant: PERCSLINES_A) -> Self {
        match variant {
            PERCSLINES_A::SS0 => 0,
            PERCSLINES_A::SS1 => 1,
            PERCSLINES_A::SS2 => 3,
            PERCSLINES_A::SS3 => 7,
            PERCSLINES_A::SSINACTIVE => 15,
        }
    }
}
#[doc = "Reader of field `PERCSLINES`"]
pub type PERCSLINES_R = crate::R<u8, PERCSLINES_A>;
impl PERCSLINES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PERCSLINES_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PERCSLINES_A::SS0),
            1 => Val(PERCSLINES_A::SS1),
            3 => Val(PERCSLINES_A::SS2),
            7 => Val(PERCSLINES_A::SS3),
            15 => Val(PERCSLINES_A::SSINACTIVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SS0`"]
    #[inline(always)]
    pub fn is_ss0(&self) -> bool {
        *self == PERCSLINES_A::SS0
    }
    #[doc = "Checks if the value of the field is `SS1`"]
    #[inline(always)]
    pub fn is_ss1(&self) -> bool {
        *self == PERCSLINES_A::SS1
    }
    #[doc = "Checks if the value of the field is `SS2`"]
    #[inline(always)]
    pub fn is_ss2(&self) -> bool {
        *self == PERCSLINES_A::SS2
    }
    #[doc = "Checks if the value of the field is `SS3`"]
    #[inline(always)]
    pub fn is_ss3(&self) -> bool {
        *self == PERCSLINES_A::SS3
    }
    #[doc = "Checks if the value of the field is `SSINACTIVE`"]
    #[inline(always)]
    pub fn is_ssinactive(&self) -> bool {
        *self == PERCSLINES_A::SSINACTIVE
    }
}
#[doc = "Write proxy for field `PERCSLINES`"]
pub struct PERCSLINES_W<'a> {
    w: &'a mut W,
}
impl<'a> PERCSLINES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERCSLINES_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "n_ss_out: 0b1110"]
    #[inline(always)]
    pub fn ss0(self) -> &'a mut W {
        self.variant(PERCSLINES_A::SS0)
    }
    #[doc = "n_ss_out: 0b1101"]
    #[inline(always)]
    pub fn ss1(self) -> &'a mut W {
        self.variant(PERCSLINES_A::SS1)
    }
    #[doc = "n_ss_out: 0b1011"]
    #[inline(always)]
    pub fn ss2(self) -> &'a mut W {
        self.variant(PERCSLINES_A::SS2)
    }
    #[doc = "n_ss_out: 0b0111"]
    #[inline(always)]
    pub fn ss3(self) -> &'a mut W {
        self.variant(PERCSLINES_A::SS3)
    }
    #[doc = "n_ss_out: 0b1111 (no peripheral selected)"]
    #[inline(always)]
    pub fn ssinactive(self) -> &'a mut W {
        self.variant(PERCSLINES_A::SSINACTIVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | (((value as u32) & 0x0f) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `PERSELDEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERSELDEC_A {
    #[doc = "Only 1 of 4 selects n_ss_out is active"]
    DISABLED,
    #[doc = "Allow external 4-to-16 decode"]
    ENABLED,
}
impl From<PERSELDEC_A> for bool {
    #[inline(always)]
    fn from(variant: PERSELDEC_A) -> Self {
        match variant {
            PERSELDEC_A::DISABLED => false,
            PERSELDEC_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `PERSELDEC`"]
pub type PERSELDEC_R = crate::R<bool, PERSELDEC_A>;
impl PERSELDEC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERSELDEC_A {
        match self.bits {
            false => PERSELDEC_A::DISABLED,
            true => PERSELDEC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PERSELDEC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PERSELDEC_A::ENABLED
    }
}
#[doc = "Write proxy for field `PERSELDEC`"]
pub struct PERSELDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSELDEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERSELDEC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Only 1 of 4 selects n_ss_out is active"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PERSELDEC_A::DISABLED)
    }
    #[doc = "Allow external 4-to-16 decode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PERSELDEC_A::ENABLED)
    }
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
#[doc = "Reader of field `LEGIPMODEEN`"]
pub type LEGIPMODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LEGIPMODEEN`"]
pub struct LEGIPMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEGIPMODEEN_W<'a> {
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
#[doc = "Reader of field `ENDIRACCCTR`"]
pub type ENDIRACCCTR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENDIRACCCTR`"]
pub struct ENDIRACCCTR_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDIRACCCTR_W<'a> {
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
#[doc = "Reader of field `PHYMODEEN`"]
pub type PHYMODEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYMODEEN`"]
pub struct PHYMODEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYMODEEN_W<'a> {
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
#[doc = "Reader of field `CLKPHASE`"]
pub type CLKPHASE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPHASE`"]
pub struct CLKPHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPHASE_W<'a> {
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
#[doc = "Reader of field `CLKPOLARITY`"]
pub type CLKPOLARITY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLKPOLARITY`"]
pub struct CLKPOLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPOLARITY_W<'a> {
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
#[doc = "Reader of field `QSPIEN`"]
pub type QSPIEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPIEN`"]
pub struct QSPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPIEN_W<'a> {
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
impl R {
    #[doc = "Bit 31 - Serial Interface and QSPI pipeline is IDLE"]
    #[inline(always)]
    pub fn piplidle(&self) -> PIPLIDLE_R {
        PIPLIDLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Pipeline PHY Mode enable"]
    #[inline(always)]
    pub fn piplphyen(&self) -> PIPLPHYEN_R {
        PIPLPHYEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtren(&self) -> DTREN_R {
        DTREN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable AHB Decoder"]
    #[inline(always)]
    pub fn ahbdecen(&self) -> AHBDECEN_R {
        AHBDECEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22 - Master mode baud rate divisor (2 to 32)"]
    #[inline(always)]
    pub fn mamobrdiv(&self) -> MAMOBRDIV_R {
        MAMOBRDIV_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - Enter XIP Mode immediately"]
    #[inline(always)]
    pub fn entrxipmodeimm(&self) -> ENTRXIPMODEIMM_R {
        ENTRXIPMODEIMM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enter XIP Mode on next READ"]
    #[inline(always)]
    pub fn entrxipmodeonr(&self) -> ENTRXIPMODEONR_R {
        ENTRXIPMODEONR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable AHB Address Re-mapping"]
    #[inline(always)]
    pub fn enahbaddrrm(&self) -> ENAHBADDRRM_R {
        ENAHBADDRRM_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable DMA Peripheral Interface"]
    #[inline(always)]
    pub fn endmapif(&self) -> ENDMAPIF_R {
        ENDMAPIF_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Set to drive the WP pin of Flash device"]
    #[inline(always)]
    pub fn wppindrv(&self) -> WPPINDRV_R {
        WPPINDRV_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Peripheral chip select lines"]
    #[inline(always)]
    pub fn percslines(&self) -> PERCSLINES_R {
        PERCSLINES_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Peripheral select decode"]
    #[inline(always)]
    pub fn perseldec(&self) -> PERSELDEC_R {
        PERSELDEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn legipmodeen(&self) -> LEGIPMODEEN_R {
        LEGIPMODEEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn endiraccctr(&self) -> ENDIRACCCTR_R {
        ENDIRACCCTR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PHY Mode enable"]
    #[inline(always)]
    pub fn phymodeen(&self) -> PHYMODEEN_R {
        PHYMODEEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Clock phase, this maps to the standard SPI CPHA transfer format"]
    #[inline(always)]
    pub fn clkphase(&self) -> CLKPHASE_R {
        CLKPHASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
    #[inline(always)]
    pub fn clkpolarity(&self) -> CLKPOLARITY_R {
        CLKPOLARITY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QSPIEN_R {
        QSPIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Serial Interface and QSPI pipeline is IDLE"]
    #[inline(always)]
    pub fn piplidle(&mut self) -> PIPLIDLE_W {
        PIPLIDLE_W { w: self }
    }
    #[doc = "Bit 25 - Pipeline PHY Mode enable"]
    #[inline(always)]
    pub fn piplphyen(&mut self) -> PIPLPHYEN_W {
        PIPLPHYEN_W { w: self }
    }
    #[doc = "Bit 24 - Enable DTR Protocol"]
    #[inline(always)]
    pub fn dtren(&mut self) -> DTREN_W {
        DTREN_W { w: self }
    }
    #[doc = "Bit 23 - Enable AHB Decoder"]
    #[inline(always)]
    pub fn ahbdecen(&mut self) -> AHBDECEN_W {
        AHBDECEN_W { w: self }
    }
    #[doc = "Bits 19:22 - Master mode baud rate divisor (2 to 32)"]
    #[inline(always)]
    pub fn mamobrdiv(&mut self) -> MAMOBRDIV_W {
        MAMOBRDIV_W { w: self }
    }
    #[doc = "Bit 18 - Enter XIP Mode immediately"]
    #[inline(always)]
    pub fn entrxipmodeimm(&mut self) -> ENTRXIPMODEIMM_W {
        ENTRXIPMODEIMM_W { w: self }
    }
    #[doc = "Bit 17 - Enter XIP Mode on next READ"]
    #[inline(always)]
    pub fn entrxipmodeonr(&mut self) -> ENTRXIPMODEONR_W {
        ENTRXIPMODEONR_W { w: self }
    }
    #[doc = "Bit 16 - Enable AHB Address Re-mapping"]
    #[inline(always)]
    pub fn enahbaddrrm(&mut self) -> ENAHBADDRRM_W {
        ENAHBADDRRM_W { w: self }
    }
    #[doc = "Bit 15 - Enable DMA Peripheral Interface"]
    #[inline(always)]
    pub fn endmapif(&mut self) -> ENDMAPIF_W {
        ENDMAPIF_W { w: self }
    }
    #[doc = "Bit 14 - Set to drive the WP pin of Flash device"]
    #[inline(always)]
    pub fn wppindrv(&mut self) -> WPPINDRV_W {
        WPPINDRV_W { w: self }
    }
    #[doc = "Bits 10:13 - Peripheral chip select lines"]
    #[inline(always)]
    pub fn percslines(&mut self) -> PERCSLINES_W {
        PERCSLINES_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral select decode"]
    #[inline(always)]
    pub fn perseldec(&mut self) -> PERSELDEC_W {
        PERSELDEC_W { w: self }
    }
    #[doc = "Bit 8 - Legacy IP Mode Enable"]
    #[inline(always)]
    pub fn legipmodeen(&mut self) -> LEGIPMODEEN_W {
        LEGIPMODEEN_W { w: self }
    }
    #[doc = "Bit 7 - Enable Direct Access Controller"]
    #[inline(always)]
    pub fn endiraccctr(&mut self) -> ENDIRACCCTR_W {
        ENDIRACCCTR_W { w: self }
    }
    #[doc = "Bit 3 - PHY Mode enable"]
    #[inline(always)]
    pub fn phymodeen(&mut self) -> PHYMODEEN_W {
        PHYMODEEN_W { w: self }
    }
    #[doc = "Bit 2 - Clock phase, this maps to the standard SPI CPHA transfer format"]
    #[inline(always)]
    pub fn clkphase(&mut self) -> CLKPHASE_W {
        CLKPHASE_W { w: self }
    }
    #[doc = "Bit 1 - Clock polarity outside SPI word. This maps to the standard SPI CPOL transfer format"]
    #[inline(always)]
    pub fn clkpolarity(&mut self) -> CLKPOLARITY_W {
        CLKPOLARITY_W { w: self }
    }
    #[doc = "Bit 0 - QSPI Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QSPIEN_W {
        QSPIEN_W { w: self }
    }
}
