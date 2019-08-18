#[doc = "Reader of register GPTALARM0"]
pub type R = crate::R<u32, super::GPTALARM0>;
#[doc = "Writer for register GPTALARM0"]
pub type W = crate::W<u32, super::GPTALARM0>;
#[doc = "Register GPTALARM0 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTALARM0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field ` GPTALARM0_DATA`"]
pub type GPTALARM0_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field ` GPTALARM0_DATA`"]
pub struct GPTALARM0_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTALARM0_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm0_data(&self) -> GPTALARM0_DATA_R {
        GPTALARM0_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value that triggers the ALARM0 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm0_data(&mut self) -> GPTALARM0_DATA_W {
        GPTALARM0_DATA_W { w: self }
    }
}
