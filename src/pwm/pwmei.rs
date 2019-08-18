#[doc = "Writer for register PWMEI"]
pub type W = crate::W<u32, super::PWMEI>;
#[doc = "Register PWMEI `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMEI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `Enable_BIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_BIT_AW {
    #[doc = "Enable the Interrupt generation"]
    ENABLED,
}
impl From<ENABLE_BIT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_BIT_AW) -> Self {
        match variant {
            ENABLE_BIT_AW::ENABLED => true,
        }
    }
}
#[doc = "Write proxy for field `Enable_BIT`"]
pub struct ENABLE_BIT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_BIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_BIT_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable the Interrupt generation"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_BIT_AW::ENABLED)
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
    #[doc = "Bit 0 - Determines whether the write accesses the Interrupt Enable register"]
    #[inline(always)]
    pub fn enable_bit(&mut self) -> ENABLE_BIT_W {
        ENABLE_BIT_W { w: self }
    }
}
