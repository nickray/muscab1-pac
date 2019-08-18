#[doc = "Reader of register DEVSIZE"]
pub type R = crate::R<u32, super::DEVSIZE>;
#[doc = "Writer for register DEVSIZE"]
pub type W = crate::W<u32, super::DEVSIZE>;
#[doc = "Register DEVSIZE `reset()`'s with value 0x0010_1002"]
impl crate::ResetValue for super::DEVSIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0010_1002
    }
}
#[doc = "Reader of field `FDEVSIZECS3`"]
pub type FDEVSIZECS3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDEVSIZECS3`"]
pub struct FDEVSIZECS3_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEVSIZECS3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | (((value as u32) & 0x03) << 27);
        self.w
    }
}
#[doc = "Reader of field `FDEVSIZECS2`"]
pub type FDEVSIZECS2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDEVSIZECS2`"]
pub struct FDEVSIZECS2_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEVSIZECS2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
#[doc = "Reader of field `FDEVSIZECS1`"]
pub type FDEVSIZECS1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDEVSIZECS1`"]
pub struct FDEVSIZECS1_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEVSIZECS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | (((value as u32) & 0x03) << 23);
        self.w
    }
}
#[doc = "Reader of field `FDEVSIZECS0`"]
pub type FDEVSIZECS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FDEVSIZECS0`"]
pub struct FDEVSIZECS0_W<'a> {
    w: &'a mut W,
}
impl<'a> FDEVSIZECS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `BYTEPERBLKNUM`"]
pub type BYTEPERBLKNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYTEPERBLKNUM`"]
pub struct BYTEPERBLKNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEPERBLKNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `BYTEPERDEVPGNUM`"]
pub type BYTEPERDEVPGNUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BYTEPERDEVPGNUM`"]
pub struct BYTEPERDEVPGNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTEPERDEVPGNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 4)) | (((value as u32) & 0x0fff) << 4);
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
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 27:28 - Size of Flash Device connected to CS\\[3\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs3(&self) -> FDEVSIZECS3_R {
        FDEVSIZECS3_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bits 25:26 - Size of Flash Device connected to CS\\[2\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs2(&self) -> FDEVSIZECS2_R {
        FDEVSIZECS2_R::new(((self.bits >> 25) & 0x03) as u8)
    }
    #[doc = "Bits 23:24 - Size of Flash Device connected to CS\\[1\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs1(&self) -> FDEVSIZECS1_R {
        FDEVSIZECS1_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bits 21:22 - Size of Flash Device connected to CS\\[0\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs0(&self) -> FDEVSIZECS0_R {
        FDEVSIZECS0_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - Number of bytes per block"]
    #[inline(always)]
    pub fn byteperblknum(&self) -> BYTEPERBLKNUM_R {
        BYTEPERBLKNUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 4:15 - Number of bytes per device page"]
    #[inline(always)]
    pub fn byteperdevpgnum(&self) -> BYTEPERDEVPGNUM_R {
        BYTEPERDEVPGNUM_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - Number of address bytes"]
    #[inline(always)]
    pub fn addrbytenum(&self) -> ADDRBYTENUM_R {
        ADDRBYTENUM_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 27:28 - Size of Flash Device connected to CS\\[3\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs3(&mut self) -> FDEVSIZECS3_W {
        FDEVSIZECS3_W { w: self }
    }
    #[doc = "Bits 25:26 - Size of Flash Device connected to CS\\[2\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs2(&mut self) -> FDEVSIZECS2_W {
        FDEVSIZECS2_W { w: self }
    }
    #[doc = "Bits 23:24 - Size of Flash Device connected to CS\\[1\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs1(&mut self) -> FDEVSIZECS1_W {
        FDEVSIZECS1_W { w: self }
    }
    #[doc = "Bits 21:22 - Size of Flash Device connected to CS\\[0\\] pin"]
    #[inline(always)]
    pub fn fdevsizecs0(&mut self) -> FDEVSIZECS0_W {
        FDEVSIZECS0_W { w: self }
    }
    #[doc = "Bits 16:20 - Number of bytes per block"]
    #[inline(always)]
    pub fn byteperblknum(&mut self) -> BYTEPERBLKNUM_W {
        BYTEPERBLKNUM_W { w: self }
    }
    #[doc = "Bits 4:15 - Number of bytes per device page"]
    #[inline(always)]
    pub fn byteperdevpgnum(&mut self) -> BYTEPERDEVPGNUM_W {
        BYTEPERDEVPGNUM_W { w: self }
    }
    #[doc = "Bits 0:3 - Number of address bytes"]
    #[inline(always)]
    pub fn addrbytenum(&mut self) -> ADDRBYTENUM_W {
        ADDRBYTENUM_W { w: self }
    }
}
