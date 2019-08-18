#[doc = "Reader of register DEVWRITEINSTR"]
pub type R = crate::R<u32, super::DEVWRITEINSTR>;
#[doc = "Writer for register DEVWRITEINSTR"]
pub type W = crate::W<u32, super::DEVWRITEINSTR>;
#[doc = "Register DEVWRITEINSTR `reset()`'s with value 0x02"]
impl crate::ResetValue for super::DEVWRITEINSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `WRITEDUMCLKCYCNUM`"]
pub type WRITEDUMCLKCYCNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WRITEDUMCLKCYCNUM`"]
pub struct WRITEDUMCLKCYCNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEDUMCLKCYCNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
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
#[doc = "Reader of field `WELDISABLE`"]
pub type WELDISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WELDISABLE`"]
pub struct WELDISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WELDISABLE_W<'a> {
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
#[doc = "Reader of field `WROPCODE`"]
pub type WROPCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WROPCODE`"]
pub struct WROPCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WROPCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Write Instruction"]
    #[inline(always)]
    pub fn writedumclkcycnum(&self) -> WRITEDUMCLKCYCNUM_R {
        WRITEDUMCLKCYCNUM_R::new(((self.bits >> 24) & 0x1f) as u8)
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
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldisable(&self) -> WELDISABLE_R {
        WELDISABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&self) -> WROPCODE_R {
        WROPCODE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - Number of Dummy Clock Cycles required by device for Write Instruction"]
    #[inline(always)]
    pub fn writedumclkcycnum(&mut self) -> WRITEDUMCLKCYCNUM_W {
        WRITEDUMCLKCYCNUM_W { w: self }
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
    #[doc = "Bit 8 - WEL Disable"]
    #[inline(always)]
    pub fn weldisable(&mut self) -> WELDISABLE_W {
        WELDISABLE_W { w: self }
    }
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline(always)]
    pub fn wropcode(&mut self) -> WROPCODE_W {
        WROPCODE_W { w: self }
    }
}
