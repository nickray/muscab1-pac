#[doc = "Reader of register INITSVRTOR1"]
pub type R = crate::R<u32, super::INITSVRTOR1>;
#[doc = "Writer for register INITSVRTOR1"]
pub type W = crate::W<u32, super::INITSVRTOR1>;
#[doc = "Register INITSVRTOR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::INITSVRTOR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INITSVTOR1`"]
pub type INITSVTOR1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INITSVTOR1`"]
pub struct INITSVTOR1_W<'a> {
    w: &'a mut W,
}
impl<'a> INITSVTOR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 1"]
    #[inline(always)]
    pub fn initsvtor1(&self) -> INITSVTOR1_R {
        INITSVTOR1_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 1"]
    #[inline(always)]
    pub fn initsvtor1(&mut self) -> INITSVTOR1_W {
        INITSVTOR1_W { w: self }
    }
}
