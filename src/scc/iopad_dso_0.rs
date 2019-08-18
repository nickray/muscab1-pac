#[doc = "Reader of register IOPAD_DSO_0"]
pub type R = crate::R<u32, super::IOPAD_DSO_0>;
#[doc = "Writer for register IOPAD_DSO_0"]
pub type W = crate::W<u32, super::IOPAD_DSO_0>;
#[doc = "Register IOPAD_DSO_0 `reset()`'s with value 0xfff0_0000"]
impl crate::ResetValue for super::IOPAD_DSO_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xfff0_0000
    }
}
#[doc = "Reader of field `drive_strength0`"]
pub type DRIVE_STRENGTH0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `drive_strength0`"]
pub struct DRIVE_STRENGTH0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength0(&self) -> DRIVE_STRENGTH0_R {
        DRIVE_STRENGTH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength0(&mut self) -> DRIVE_STRENGTH0_W {
        DRIVE_STRENGTH0_W { w: self }
    }
}
