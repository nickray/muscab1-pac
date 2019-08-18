#[doc = "Reader of register CLK_POSTDIV_CTRL_TEST"]
pub type R = crate::R<u32, super::CLK_POSTDIV_CTRL_TEST>;
#[doc = "Writer for register CLK_POSTDIV_CTRL_TEST"]
pub type W = crate::W<u32, super::CLK_POSTDIV_CTRL_TEST>;
#[doc = "Register CLK_POSTDIV_CTRL_TEST `reset()`'s with value 0x0a"]
impl crate::ResetValue for super::CLK_POSTDIV_CTRL_TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0a
    }
}
#[doc = "Reader of field `postdiv_ctrl_test_div`"]
pub type POSTDIV_CTRL_TEST_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `postdiv_ctrl_test_div`"]
pub struct POSTDIV_CTRL_TEST_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> POSTDIV_CTRL_TEST_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - postdiv_ctrl_test_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_test_div(&self) -> POSTDIV_CTRL_TEST_DIV_R {
        POSTDIV_CTRL_TEST_DIV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - postdiv_ctrl_test_div"]
    #[inline(always)]
    pub fn postdiv_ctrl_test_div(&mut self) -> POSTDIV_CTRL_TEST_DIV_W {
        POSTDIV_CTRL_TEST_DIV_W { w: self }
    }
}
