#[doc = "Writer for register PWMDI"]
pub type W = crate::W<u32, super::PWMDI>;
#[doc = "Register PWMDI `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMDI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `Disable_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DISABLE_BIT_AW {
    #[doc = "Disable the Interrupt generation"]
    DISABLED,
}
impl From<DISABLE_BIT_AW> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_BIT_AW) -> Self {
        match variant {
            DISABLE_BIT_AW::DISABLED => true,
        }
    }
}
#[doc = "Write proxy for field `Disable_BIT`"]
pub struct DISABLE_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DISABLE_BIT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable the Interrupt generation"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISABLE_BIT_AW::DISABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Determines whether the write accesses the Interrupt Disable register"]
    #[inline(always)]
    pub fn disable_bit(&mut self) -> DISABLE_BIT_W {
        DISABLE_BIT_W { w: self }
    }
}
