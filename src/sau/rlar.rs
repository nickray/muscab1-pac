#[doc = "Reader of register RLAR"]
pub type R = crate::R<u32, super::RLAR>;
#[doc = "Writer for register RLAR"]
pub type W = crate::W<u32, super::RLAR>;
#[doc = "Register RLAR `reset()`'s with value 0"]
impl crate::ResetValue for super::RLAR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LADDR`"]
pub type LADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LADDR`"]
pub struct LADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> LADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff_ffff << 5)) | (((value as u32) & 0x07ff_ffff) << 5);
        self.w
    }
}
#[doc = "Reader of field `NSC`"]
pub type NSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSC`"]
pub struct NSC_W<'a> {
    w: &'a mut W,
}
impl<'a> NSC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 5:31 - Limit Address"]
    #[inline(always)]
    pub fn laddr(&self) -> LADDR_R {
        LADDR_R::new(((self.bits >> 5) & 0x07ff_ffff) as u32)
    }
    #[doc = "Bit 1 - Non-Secure Callable"]
    #[inline(always)]
    pub fn nsc(&self) -> NSC_R {
        NSC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - SAU Region enabled"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:31 - Limit Address"]
    #[inline(always)]
    pub fn laddr(&mut self) -> LADDR_W {
        LADDR_W { w: self }
    }
    #[doc = "Bit 1 - Non-Secure Callable"]
    #[inline(always)]
    pub fn nsc(&mut self) -> NSC_W {
        NSC_W { w: self }
    }
    #[doc = "Bit 0 - SAU Region enabled"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
