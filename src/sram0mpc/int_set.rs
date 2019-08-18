#[doc = "Writer for register INT_SET"]
pub type W = crate::W<u32, super::INT_SET>;
#[doc = "Register INT_SET `reset()`'s with value 0"]
impl crate::ResetValue for super::INT_SET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
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
impl W {
    #[doc = "Bit 0 - mpc_irq set. Debug purpose only"]
    #[inline(always)]
    pub fn bit0(&mut self) -> BIT0_W {
        BIT0_W { w: self }
    }
}
