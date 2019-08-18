#[doc = "Reader of register GRETREG"]
pub type R = crate::R<u32, super::GRETREG>;
#[doc = "Writer for register GRETREG"]
pub type W = crate::W<u32, super::GRETREG>;
#[doc = "Register GRETREG `reset()`'s with value 0"]
impl crate::ResetValue for super::GRETREG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GRETREG`"]
pub type GRETREG_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GRETREG`"]
pub struct GRETREG_W<'a> {
    w: &'a mut W,
}
impl<'a> GRETREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn gretreg(&self) -> GRETREG_R {
        GRETREG_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - General Purpose Retention Register"]
    #[inline(always)]
    pub fn gretreg(&mut self) -> GRETREG_W {
        GRETREG_W { w: self }
    }
}
