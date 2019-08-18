#[doc = "Reader of register SFSR"]
pub type R = crate::R<u32, super::SFSR>;
#[doc = "Writer for register SFSR"]
pub type W = crate::W<u32, super::SFSR>;
#[doc = "Register SFSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SFSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LSERR`"]
pub type LSERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSERR`"]
pub struct LSERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SFARVALID`"]
pub type SFARVALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFARVALID`"]
pub struct SFARVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> SFARVALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `LSPERR`"]
pub type LSPERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LSPERR`"]
pub struct LSPERR_W<'a> {
    w: &'a mut W,
}
impl<'a> LSPERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `INVTRAN`"]
pub type INVTRAN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVTRAN`"]
pub struct INVTRAN_W<'a> {
    w: &'a mut W,
}
impl<'a> INVTRAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `AUVIOL`"]
pub type AUVIOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUVIOL`"]
pub struct AUVIOL_W<'a> {
    w: &'a mut W,
}
impl<'a> AUVIOL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `INVER`"]
pub type INVER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVER`"]
pub struct INVER_W<'a> {
    w: &'a mut W,
}
impl<'a> INVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `INVIS`"]
pub type INVIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVIS`"]
pub struct INVIS_W<'a> {
    w: &'a mut W,
}
impl<'a> INVIS_W<'a> {
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
#[doc = "Reader of field `INVEP`"]
pub type INVEP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVEP`"]
pub struct INVEP_W<'a> {
    w: &'a mut W,
}
impl<'a> INVEP_W<'a> {
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
    #[doc = "Bit 7 - Lazy state error flag"]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Secure fault address valid"]
    #[inline(always)]
    pub fn sfarvalid(&self) -> SFARVALID_R {
        SFARVALID_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lazy state preservation error flag"]
    #[inline(always)]
    pub fn lsperr(&self) -> LSPERR_R {
        LSPERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Invalid transition flag"]
    #[inline(always)]
    pub fn invtran(&self) -> INVTRAN_R {
        INVTRAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Attribution unit violation flag"]
    #[inline(always)]
    pub fn auviol(&self) -> AUVIOL_R {
        AUVIOL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invalid exception return flag"]
    #[inline(always)]
    pub fn inver(&self) -> INVER_R {
        INVER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Invalid integrity signature flag"]
    #[inline(always)]
    pub fn invis(&self) -> INVIS_R {
        INVIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Invalid entry pointd"]
    #[inline(always)]
    pub fn invep(&self) -> INVEP_R {
        INVEP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Lazy state error flag"]
    #[inline(always)]
    pub fn lserr(&mut self) -> LSERR_W {
        LSERR_W { w: self }
    }
    #[doc = "Bit 6 - Secure fault address valid"]
    #[inline(always)]
    pub fn sfarvalid(&mut self) -> SFARVALID_W {
        SFARVALID_W { w: self }
    }
    #[doc = "Bit 5 - Lazy state preservation error flag"]
    #[inline(always)]
    pub fn lsperr(&mut self) -> LSPERR_W {
        LSPERR_W { w: self }
    }
    #[doc = "Bit 4 - Invalid transition flag"]
    #[inline(always)]
    pub fn invtran(&mut self) -> INVTRAN_W {
        INVTRAN_W { w: self }
    }
    #[doc = "Bit 3 - Attribution unit violation flag"]
    #[inline(always)]
    pub fn auviol(&mut self) -> AUVIOL_W {
        AUVIOL_W { w: self }
    }
    #[doc = "Bit 2 - Invalid exception return flag"]
    #[inline(always)]
    pub fn inver(&mut self) -> INVER_W {
        INVER_W { w: self }
    }
    #[doc = "Bit 1 - Invalid integrity signature flag"]
    #[inline(always)]
    pub fn invis(&mut self) -> INVIS_W {
        INVIS_W { w: self }
    }
    #[doc = "Bit 0 - Invalid entry pointd"]
    #[inline(always)]
    pub fn invep(&mut self) -> INVEP_W {
        INVEP_W { w: self }
    }
}
