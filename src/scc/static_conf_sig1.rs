#[doc = "Reader of register STATIC_CONF_SIG1"]
pub type R = crate::R<u32, super::STATIC_CONF_SIG1>;
#[doc = "Writer for register STATIC_CONF_SIG1"]
pub type W = crate::W<u32, super::STATIC_CONF_SIG1>;
#[doc = "Register STATIC_CONF_SIG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::STATIC_CONF_SIG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TISBYPASSIN`"]
pub type TISBYPASSIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TISBYPASSIN`"]
pub struct TISBYPASSIN_W<'a> {
    w: &'a mut W,
}
impl<'a> TISBYPASSIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `TISBYPASSACK`"]
pub type TISBYPASSACK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TISBYPASSACK`"]
pub struct TISBYPASSACK_W<'a> {
    w: &'a mut W,
}
impl<'a> TISBYPASSACK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIHSBYPASS`"]
pub type TIHSBYPASS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIHSBYPASS`"]
pub struct TIHSBYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> TIHSBYPASS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `TINIDENSEL`"]
pub type TINIDENSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TINIDENSEL`"]
pub struct TINIDENSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TINIDENSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `TODBGENSEL`"]
pub type TODBGENSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TODBGENSEL`"]
pub struct TODBGENSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TODBGENSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
    #[inline(always)]
    pub fn tisbypassin(&self) -> TISBYPASSIN_R {
        TISBYPASSIN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
    #[inline(always)]
    pub fn tisbypassack(&self) -> TISBYPASSACK_R {
        TISBYPASSACK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
    #[inline(always)]
    pub fn tihsbypass(&self) -> TIHSBYPASS_R {
        TIHSBYPASS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - NIDEN mask on CTITRIGINT"]
    #[inline(always)]
    pub fn tinidensel(&self) -> TINIDENSEL_R {
        TINIDENSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - DBGEN mask on CTITRIGOUT"]
    #[inline(always)]
    pub fn todbgensel(&self) -> TODBGENSEL_R {
        TODBGENSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cross Trigger Interface synchronous bypass on CTITRIGIN"]
    #[inline(always)]
    pub fn tisbypassin(&mut self) -> TISBYPASSIN_W {
        TISBYPASSIN_W { w: self }
    }
    #[doc = "Bits 8:11 - Cross Trigger Interface synchronous bypass on CTITRIGOUTACK"]
    #[inline(always)]
    pub fn tisbypassack(&mut self) -> TISBYPASSACK_W {
        TISBYPASSACK_W { w: self }
    }
    #[doc = "Bits 12:15 - Cross Trigger Interface handshake bypass on CTITRIGOUT"]
    #[inline(always)]
    pub fn tihsbypass(&mut self) -> TIHSBYPASS_W {
        TIHSBYPASS_W { w: self }
    }
    #[doc = "Bits 16:23 - NIDEN mask on CTITRIGINT"]
    #[inline(always)]
    pub fn tinidensel(&mut self) -> TINIDENSEL_W {
        TINIDENSEL_W { w: self }
    }
    #[doc = "Bits 24:27 - DBGEN mask on CTITRIGOUT"]
    #[inline(always)]
    pub fn todbgensel(&mut self) -> TODBGENSEL_W {
        TODBGENSEL_W { w: self }
    }
}
