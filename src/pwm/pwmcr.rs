#[doc = "Reader of register PWMCR"]
pub type R = crate::R<u32, super::PWMCR>;
#[doc = "Writer for register PWMCR"]
pub type W = crate::W<u32, super::PWMCR>;
#[doc = "Register PWMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `OUTPUT_SET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTPUT_SET_A {
    #[doc = "Generate programmed waveform on\n                          pwm_output"]
    ENABLED,
    #[doc = "Set pwm_output continually high"]
    DISABLED,
}
impl From<OUTPUT_SET_A> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_SET_A) -> Self {
        match variant {
            OUTPUT_SET_A::ENABLED => true,
            OUTPUT_SET_A::DISABLED => false,
        }
    }
}
#[doc = "Reader of field `OUTPUT_SET`"]
pub type OUTPUT_SET_R = crate::R<bool, OUTPUT_SET_A>;
impl OUTPUT_SET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTPUT_SET_A {
        match self.bits {
            true => OUTPUT_SET_A::ENABLED,
            false => OUTPUT_SET_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OUTPUT_SET_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OUTPUT_SET_A::DISABLED
    }
}
#[doc = "Write proxy for field `OUTPUT_SET`"]
pub struct OUTPUT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTPUT_SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTPUT_SET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Generate programmed waveform on pwm_output"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OUTPUT_SET_A::ENABLED)
    }
    #[doc = "Set pwm_output continually high"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OUTPUT_SET_A::DISABLED)
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
impl R {
    #[doc = "Bit 0 - Start stop bit for the pwm_output"]
    #[inline(always)]
    pub fn output_set(&self) -> OUTPUT_SET_R {
        OUTPUT_SET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start stop bit for the pwm_output"]
    #[inline(always)]
    pub fn output_set(&mut self) -> OUTPUT_SET_W {
        OUTPUT_SET_W { w: self }
    }
}
