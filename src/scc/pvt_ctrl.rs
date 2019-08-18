#[doc = "Reader of register PVT_CTRL"]
pub type R = crate::R<u32, super::PVT_CTRL>;
#[doc = "Writer for register PVT_CTRL"]
pub type W = crate::W<u32, super::PVT_CTRL>;
#[doc = "Register PVT_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::PVT_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TSTSENNUM`"]
pub type TSTSENNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TSTSENNUM`"]
pub struct TSTSENNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTSENNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Select PVT sensor to write to and read from"]
    #[inline(always)]
    pub fn tstsennum(&self) -> TSTSENNUM_R {
        TSTSENNUM_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Select PVT sensor to write to and read from"]
    #[inline(always)]
    pub fn tstsennum(&mut self) -> TSTSENNUM_W {
        TSTSENNUM_W { w: self }
    }
}
