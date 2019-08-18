#[doc = "Reader of register RESET_SYNDROME"]
pub type R = crate::R<u32, super::RESET_SYNDROME>;
#[doc = "Writer for register RESET_SYNDROME"]
pub type W = crate::W<u32, super::RESET_SYNDROME>;
#[doc = "Register RESET_SYNDROME `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RESET_SYNDROME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Write proxy for field `PoR`"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Write proxy for field `NSWD`"]
pub struct NSWD_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWD_W<'a> {
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
#[doc = "Write proxy for field `SWD`"]
pub struct SWD_W<'a> {
    w: &'a mut W,
}
impl<'a> SWD_W<'a> {
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
#[doc = "Write proxy for field `S32KWD`"]
pub struct S32KWD_W<'a> {
    w: &'a mut W,
}
impl<'a> S32KWD_W<'a> {
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
#[doc = "Write proxy for field `SYSRSTREQ0`"]
pub struct SYSRSTREQ0_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTREQ0_W<'a> {
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
#[doc = "Write proxy for field `SYSRSTREQ1`"]
pub struct SYSRSTREQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSRSTREQ1_W<'a> {
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
#[doc = "Write proxy for field `LOCKUP0`"]
pub struct LOCKUP0_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP0_W<'a> {
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
#[doc = "Write proxy for field `LOCKUP1`"]
pub struct LOCKUP1_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKUP1_W<'a> {
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
#[doc = "Write proxy for field `RESETREQ`"]
pub struct RESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETREQ_W<'a> {
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
#[doc = "Write proxy for field `SWRESETREQ`"]
pub struct SWRESETREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRESETREQ_W<'a> {
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
impl R {}
impl W {
    #[doc = "Bit 0 - Power-on"]
    #[inline(always)]
    pub fn po_r(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - Non-secure watchdog"]
    #[inline(always)]
    pub fn nswd(&mut self) -> NSWD_W {
        NSWD_W { w: self }
    }
    #[doc = "Bit 2 - Secure watchdog"]
    #[inline(always)]
    pub fn swd(&mut self) -> SWD_W {
        SWD_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog on the S32KCLK clock"]
    #[inline(always)]
    pub fn s32kwd(&mut self) -> S32KWD_W {
        S32KWD_W { w: self }
    }
    #[doc = "Bit 4 - CPU 0 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq0(&mut self) -> SYSRSTREQ0_W {
        SYSRSTREQ0_W { w: self }
    }
    #[doc = "Bit 5 - CPU 1 System Reset Request"]
    #[inline(always)]
    pub fn sysrstreq1(&mut self) -> SYSRSTREQ1_W {
        SYSRSTREQ1_W { w: self }
    }
    #[doc = "Bit 6 - CPU 0 Lock-up Status"]
    #[inline(always)]
    pub fn lockup0(&mut self) -> LOCKUP0_W {
        LOCKUP0_W { w: self }
    }
    #[doc = "Bit 7 - CPU 1 Lock-up Status"]
    #[inline(always)]
    pub fn lockup1(&mut self) -> LOCKUP1_W {
        LOCKUP1_W { w: self }
    }
    #[doc = "Bit 8 - External Reset Request"]
    #[inline(always)]
    pub fn resetreq(&mut self) -> RESETREQ_W {
        RESETREQ_W { w: self }
    }
    #[doc = "Bit 9 - Software Reset Request"]
    #[inline(always)]
    pub fn swresetreq(&mut self) -> SWRESETREQ_W {
        SWRESETREQ_W { w: self }
    }
}
