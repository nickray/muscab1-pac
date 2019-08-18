#[doc = "Reader of register IOPAD_IS_1"]
pub type R = crate::R<u32, super::IOPAD_IS_1>;
#[doc = "Writer for register IOPAD_IS_1"]
pub type W = crate::W<u32, super::IOPAD_IS_1>;
#[doc = "Register IOPAD_IS_1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IOPAD_IS_1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `input_select`"]
pub type INPUT_SELECT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `input_select`"]
pub struct INPUT_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Selects input mode on test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn input_select(&self) -> INPUT_SELECT_R {
        INPUT_SELECT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Selects input mode on test chip I/O PA37-PA32"]
    #[inline(always)]
    pub fn input_select(&mut self) -> INPUT_SELECT_W {
        INPUT_SELECT_W { w: self }
    }
}
