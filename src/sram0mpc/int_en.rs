#[doc = "Reader of register INT_EN"]
pub type R = crate::R<u32, super::INT_EN>;
#[doc = "Writer for register INT_EN"]
pub type W = crate::W<u32, super::INT_EN>;
#[doc = "Register INT_EN `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_EN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `bit[0]`"]
pub type BIT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `bit[0]`"]
pub struct BIT0_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT0_W<'a> {
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
    #[doc = "Bit 0 - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
    #[inline(always)]
    pub fn bit0(&self) -> BIT0_R {
        BIT0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - mpc_irq enable. Bits are valid when mpc_irq triggered is set"]
    #[inline(always)]
    pub fn bit0(&mut self) -> BIT0_W {
        BIT0_W { w: self }
    }
}
