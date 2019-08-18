#[doc = "Reader of register GPTALARM1"]
pub type R = crate::R<u32, super::GPTALARM1>;
#[doc = "Writer for register GPTALARM1"]
pub type W = crate::W<u32, super::GPTALARM1>;
#[doc = "Register GPTALARM1 `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTALARM1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field ` GPTALARM1_DATA`"]
pub type GPTALARM1_DATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field ` GPTALARM1_DATA`"]
pub struct GPTALARM1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTALARM1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm1_data(&self) -> GPTALARM1_DATA_R {
        GPTALARM1_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value that triggers the ALARM1 interrupt when the counter reaches that value"]
    #[inline(always)]
    pub fn gptalarm1_data(&mut self) -> GPTALARM1_DATA_W {
        GPTALARM1_DATA_W { w: self }
    }
}
