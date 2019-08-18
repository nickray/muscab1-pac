#[doc = "Reader of register FLASHCMDCTRL"]
pub type R = crate::R<u32, super::FLASHCMDCTRL>;
#[doc = "Writer for register FLASHCMDCTRL"]
pub type W = crate::W<u32, super::FLASHCMDCTRL>;
#[doc = "Register FLASHCMDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::FLASHCMDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CMDOPCODE`"]
pub type CMDOPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMDOPCODE`"]
pub struct CMDOPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDOPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `RDATAEN`"]
pub type RDATAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RDATAEN`"]
pub struct RDATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATAEN_W<'a> {
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
#[doc = "Reader of field `RDATABYTENUM`"]
pub type RDATABYTENUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDATABYTENUM`"]
pub struct RDATABYTENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RDATABYTENUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `CMDADDREN`"]
pub type CMDADDREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDADDREN`"]
pub struct CMDADDREN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDADDREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `MODEBITEN`"]
pub type MODEBITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODEBITEN`"]
pub struct MODEBITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODEBITEN_W<'a> {
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
#[doc = "Reader of field `ADDRBYTENUM`"]
pub type ADDRBYTENUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRBYTENUM`"]
pub struct ADDRBYTENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRBYTENUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `WRDATAEN`"]
pub type WRDATAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WRDATAEN`"]
pub struct WRDATAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDATAEN_W<'a> {
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
#[doc = "Reader of field `WRDATABYTENUM`"]
pub type WRDATABYTENUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRDATABYTENUM`"]
pub struct WRDATABYTENUM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRDATABYTENUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Reader of field `DUMCYCNUM`"]
pub type DUMCYCNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DUMCYCNUM`"]
pub struct DUMCYCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> DUMCYCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 7)) | (((value as u32) & 0x1f) << 7);
        self.w
    }
}
#[doc = "Reader of field `CMDEXINPROG`"]
pub type CMDEXINPROG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDEXINPROG`"]
pub struct CMDEXINPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEXINPROG_W<'a> {
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
#[doc = "Reader of field `CMDEXEC`"]
pub type CMDEXEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMDEXEC`"]
pub struct CMDEXEC_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDEXEC_W<'a> {
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
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&self) -> CMDOPCODE_R {
        CMDOPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn rdataen(&self) -> RDATAEN_R {
        RDATAEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn rdatabytenum(&self) -> RDATABYTENUM_R {
        RDATABYTENUM_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn cmdaddren(&self) -> CMDADDREN_R {
        CMDADDREN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&self) -> MODEBITEN_R {
        MODEBITEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn addrbytenum(&self) -> ADDRBYTENUM_R {
        ADDRBYTENUM_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn wrdataen(&self) -> WRDATAEN_R {
        WRDATAEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn wrdatabytenum(&self) -> WRDATABYTENUM_R {
        WRDATABYTENUM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn dumcycnum(&self) -> DUMCYCNUM_R {
        DUMCYCNUM_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 1 - Command execution in progress"]
    #[inline(always)]
    pub fn cmdexinprog(&self) -> CMDEXINPROG_R {
        CMDEXINPROG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Execute the command"]
    #[inline(always)]
    pub fn cmdexec(&self) -> CMDEXEC_R {
        CMDEXEC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline(always)]
    pub fn cmdopcode(&mut self) -> CMDOPCODE_W {
        CMDOPCODE_W { w: self }
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline(always)]
    pub fn rdataen(&mut self) -> RDATAEN_W {
        RDATAEN_W { w: self }
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline(always)]
    pub fn rdatabytenum(&mut self) -> RDATABYTENUM_W {
        RDATABYTENUM_W { w: self }
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline(always)]
    pub fn cmdaddren(&mut self) -> CMDADDREN_W {
        CMDADDREN_W { w: self }
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&mut self) -> MODEBITEN_W {
        MODEBITEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline(always)]
    pub fn addrbytenum(&mut self) -> ADDRBYTENUM_W {
        ADDRBYTENUM_W { w: self }
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline(always)]
    pub fn wrdataen(&mut self) -> WRDATAEN_W {
        WRDATAEN_W { w: self }
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline(always)]
    pub fn wrdatabytenum(&mut self) -> WRDATABYTENUM_W {
        WRDATABYTENUM_W { w: self }
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline(always)]
    pub fn dumcycnum(&mut self) -> DUMCYCNUM_W {
        DUMCYCNUM_W { w: self }
    }
    #[doc = "Bit 1 - Command execution in progress"]
    #[inline(always)]
    pub fn cmdexinprog(&mut self) -> CMDEXINPROG_W {
        CMDEXINPROG_W { w: self }
    }
    #[doc = "Bit 0 - Execute the command"]
    #[inline(always)]
    pub fn cmdexec(&mut self) -> CMDEXEC_W {
        CMDEXEC_W { w: self }
    }
}
