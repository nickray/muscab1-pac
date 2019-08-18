#[doc = "Reader of register INITSVRTOR0"]
pub type R = crate::R<u32, super::INITSVRTOR0>;
#[doc = "Writer for register INITSVRTOR0"]
pub type W = crate::W<u32, super::INITSVRTOR0>;
#[doc = "Register INITSVRTOR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::INITSVRTOR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INITSVTOR0`"]
pub type INITSVTOR0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INITSVTOR0`"]
pub struct INITSVTOR0_W<'a> {
    w: &'a mut W,
}
impl<'a> INITSVTOR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | (((value as u32) & 0x01ff_ffff) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 0"]
    #[inline(always)]
    pub fn initsvtor0(&self) -> INITSVTOR0_R {
        INITSVTOR0_R::new(((self.bits >> 7) & 0x01ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 7:31 - Default Secure Vector table offset at reset for CPU 0"]
    #[inline(always)]
    pub fn initsvtor0(&mut self) -> INITSVTOR0_W {
        INITSVTOR0_W { w: self }
    }
}
