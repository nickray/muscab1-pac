#[doc = "Reader of register IOPAD_DSO_1"]
pub type R = crate::R<u32, super::IOPAD_DSO_1>;
#[doc = "Writer for register IOPAD_DSO_1"]
pub type W = crate::W<u32, super::IOPAD_DSO_1>;
#[doc = "Register IOPAD_DSO_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOPAD_DSO_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `drive_strength_0`"]
pub type DRIVE_STRENGTH_0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `drive_strength_0`"]
pub struct DRIVE_STRENGTH_0_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_0(&self) -> DRIVE_STRENGTH_0_R {
        DRIVE_STRENGTH_0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Least significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_0(&mut self) -> DRIVE_STRENGTH_0_W {
        DRIVE_STRENGTH_0_W { w: self }
    }
}
