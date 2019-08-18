#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `bit[4]`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BIT4_A {
    #[doc = "Read-As-Zero - Writes ignored"]
    RAZWI,
    #[doc = "Bus Error"]
    BUSERROR,
}
impl From<BIT4_A> for bool {
    #[inline(always)]
    fn from(variant: BIT4_A) -> Self {
        match variant {
            BIT4_A::RAZWI => false,
            BIT4_A::BUSERROR => true,
        }
    }
}
#[doc = "Reader of field `bit[4]`"]
pub type BIT4_R = crate::R<bool, BIT4_A>;
impl BIT4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIT4_A {
        match self.bits {
            false => BIT4_A::RAZWI,
            true => BIT4_A::BUSERROR,
        }
    }
    #[doc = "Checks if the value of the field is `RAZWI`"]
    #[inline(always)]
    pub fn is_razwi(&self) -> bool {
        *self == BIT4_A::RAZWI
    }
    #[doc = "Checks if the value of the field is `BUSERROR`"]
    #[inline(always)]
    pub fn is_buserror(&self) -> bool {
        *self == BIT4_A::BUSERROR
    }
}
#[doc = "Write proxy for field `bit[4]`"]
pub struct BIT4_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BIT4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Read-As-Zero - Writes ignored"]
    #[inline(always)]
    pub fn razwi(self) -> &'a mut W {
        self.variant(BIT4_A::RAZWI)
    }
    #[doc = "Bus Error"]
    #[inline(always)]
    pub fn buserror(self) -> &'a mut W {
        self.variant(BIT4_A::BUSERROR)
    }
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
#[doc = "Reader of field `bit[6]`"]
pub type BIT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit[6]`"]
pub struct BIT6_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT6_W<'a> {
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
#[doc = "Reader of field `bit[7]`"]
pub type BIT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit[7]`"]
pub struct BIT7_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT7_W<'a> {
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
#[doc = "Reader of field `bit[8]`"]
pub type BIT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit[8]`"]
pub struct BIT8_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT8_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `bit[31]`"]
pub type BIT31_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit[31]`"]
pub struct BIT31_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - Security error response configuration"]
    #[inline(always)]
    pub fn bit4(&self) -> BIT4_R {
        BIT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data interface gating request"]
    #[inline(always)]
    pub fn bit6(&self) -> BIT6_R {
        BIT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Data interface gating acknowledge (RO)"]
    #[inline(always)]
    pub fn bit7(&self) -> BIT7_R {
        BIT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Auto-increment"]
    #[inline(always)]
    pub fn bit8(&self) -> BIT8_R {
        BIT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Security lockdown"]
    #[inline(always)]
    pub fn bit31(&self) -> BIT31_R {
        BIT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Security error response configuration"]
    #[inline(always)]
    pub fn bit4(&mut self) -> BIT4_W {
        BIT4_W { w: self }
    }
    #[doc = "Bit 6 - Data interface gating request"]
    #[inline(always)]
    pub fn bit6(&mut self) -> BIT6_W {
        BIT6_W { w: self }
    }
    #[doc = "Bit 7 - Data interface gating acknowledge (RO)"]
    #[inline(always)]
    pub fn bit7(&mut self) -> BIT7_W {
        BIT7_W { w: self }
    }
    #[doc = "Bit 8 - Auto-increment"]
    #[inline(always)]
    pub fn bit8(&mut self) -> BIT8_W {
        BIT8_W { w: self }
    }
    #[doc = "Bit 31 - Security lockdown"]
    #[inline(always)]
    pub fn bit31(&mut self) -> BIT31_W {
        BIT31_W { w: self }
    }
}
