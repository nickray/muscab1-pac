#[doc = "Reader of register IOPAD_DS1_1"]
pub type R = crate::R<u32, super::IOPAD_DS1_1>;
#[doc = "Writer for register IOPAD_DS1_1"]
pub type W = crate::W<u32, super::IOPAD_DS1_1>;
#[doc = "Register IOPAD_DS1_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOPAD_DS1_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `drive_strength_1`"]
pub type DRIVE_STRENGTH_1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `drive_strength_1`"]
pub struct DRIVE_STRENGTH_1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_1(&self) -> DRIVE_STRENGTH_1_R {
        DRIVE_STRENGTH_1_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn drive_strength_1(&mut self) -> DRIVE_STRENGTH_1_W {
        DRIVE_STRENGTH_1_W { w: self }
    }
}
