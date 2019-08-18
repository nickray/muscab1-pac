#[doc = "Reader of register IOPAD_DS1_0"]
pub type R = crate::R<u32, super::IOPAD_DS1_0>;
#[doc = "Writer for register IOPAD_DS1_0"]
pub type W = crate::W<u32, super::IOPAD_DS1_0>;
#[doc = "Register IOPAD_DS1_0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOPAD_DS1_0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `drive_strength1`"]
pub type DRIVE_STRENGTH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `drive_strength1`"]
pub struct DRIVE_STRENGTH1_W<'a> {
    w: &'a mut W,
}
impl<'a> DRIVE_STRENGTH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength1(&self) -> DRIVE_STRENGTH1_R {
        DRIVE_STRENGTH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Most significant bits of the two-bit values that define drive strengths of test chip I/O PA31-PA0"]
    #[inline(always)]
    pub fn drive_strength1(&mut self) -> DRIVE_STRENGTH1_W {
        DRIVE_STRENGTH1_W { w: self }
    }
}
