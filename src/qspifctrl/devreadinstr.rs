#[doc = "Reader of register DEVREADINSTR"]
pub type R = crate::R<u32, super::DEVREADINSTR>;
#[doc = "Writer for register DEVREADINSTR"]
pub type W = crate::W<u32, super::DEVREADINSTR>;
#[doc = "Register DEVREADINSTR `reset()`'s with value 0x03"]
impl crate::ResetValue for super::DEVREADINSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `READDUMCLKCYCNUM`"]
pub type READDUMCLKCYCNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `READDUMCLKCYCNUM`"]
pub struct READDUMCLKCYCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> READDUMCLKCYCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `DATATRTYPESSPI`"]
pub type DATATRTYPESSPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATATRTYPESSPI`"]
pub struct DATATRTYPESSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> DATATRTYPESSPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ADDRTRTYPESSPI`"]
pub type ADDRTRTYPESSPI_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDRTRTYPESSPI`"]
pub struct ADDRTRTYPESSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRTRTYPESSPI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DDRBITEN`"]
pub type DDRBITEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDRBITEN`"]
pub struct DDRBITEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DDRBITEN_W<'a> {
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
#[doc = "Reader of field `INSTRTYPE`"]
pub type INSTRTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSTRTYPE`"]
pub struct INSTRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> INSTRTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `ROPCODE`"]
pub type ROPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ROPCODE`"]
pub struct ROPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ROPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Read Instruction"]
    #[inline(always)]
    pub fn readdumclkcycnum(&self) -> READDUMCLKCYCNUM_R {
        READDUMCLKCYCNUM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&self) -> MODEBITEN_R {
        MODEBITEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn datatrtypesspi(&self) -> DATATRTYPESSPI_R {
        DATATRTYPESSPI_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn addrtrtypesspi(&self) -> ADDRTRTYPESSPI_R {
        ADDRTRTYPESSPI_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 10 - DDR Bit Enable"]
    #[inline(always)]
    pub fn ddrbiten(&self) -> DDRBITEN_R {
        DDRBITEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&self) -> INSTRTYPE_R {
        INSTRTYPE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:7 - Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    pub fn ropcode(&self) -> ROPCODE_R {
        ROPCODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Read Instruction"]
    #[inline(always)]
    pub fn readdumclkcycnum(&mut self) -> READDUMCLKCYCNUM_W {
        READDUMCLKCYCNUM_W { w: self }
    }
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline(always)]
    pub fn modebiten(&mut self) -> MODEBITEN_W {
        MODEBITEN_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn datatrtypesspi(&mut self) -> DATATRTYPESSPI_W {
        DATATRTYPESSPI_W { w: self }
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI modes"]
    #[inline(always)]
    pub fn addrtrtypesspi(&mut self) -> ADDRTRTYPESSPI_W {
        ADDRTRTYPESSPI_W { w: self }
    }
    #[doc = "Bit 10 - DDR Bit Enable"]
    #[inline(always)]
    pub fn ddrbiten(&mut self) -> DDRBITEN_W {
        DDRBITEN_W { w: self }
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline(always)]
    pub fn instrtype(&mut self) -> INSTRTYPE_W {
        INSTRTYPE_W { w: self }
    }
    #[doc = "Bits 0:7 - Read Opcode to use when not in XIP mode"]
    #[inline(always)]
    pub fn ropcode(&mut self) -> ROPCODE_W {
        ROPCODE_W { w: self }
    }
}
