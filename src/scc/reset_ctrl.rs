#[doc = "Reader of register RESET_CTRL"]
pub type R = crate::R<u32, super::RESET_CTRL>;
#[doc = "Writer for register RESET_CTRL"]
pub type W = crate::W<u32, super::RESET_CTRL>;
#[doc = "Register RESET_CTRL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::RESET_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `GPTIMER_RESET`"]
pub type GPTIMER_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPTIMER_RESET`"]
pub struct GPTIMER_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPTIMER_RESET_W<'a> {
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
#[doc = "Reader of field `I2C0_RESET`"]
pub type I2C0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C0_RESET`"]
pub struct I2C0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C0_RESET_W<'a> {
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
#[doc = "Reader of field `I2C1_RESET`"]
pub type I2C1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_RESET`"]
pub struct I2C1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_RESET_W<'a> {
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
#[doc = "Reader of field `I2S_RESET`"]
pub type I2S_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2S_RESET`"]
pub struct I2S_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RESET_W<'a> {
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
#[doc = "Reader of field `SPI_RESET`"]
pub type SPI_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI_RESET`"]
pub struct SPI_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_RESET_W<'a> {
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
#[doc = "Reader of field `QSPI_RESET`"]
pub type QSPI_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QSPI_RESET`"]
pub struct QSPI_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPI_RESET_W<'a> {
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
#[doc = "Reader of field `UART0_RESET`"]
pub type UART0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART0_RESET`"]
pub struct UART0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_RESET_W<'a> {
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
#[doc = "Reader of field `UART1_RESET`"]
pub type UART1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UART1_RESET`"]
pub struct UART1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_RESET_W<'a> {
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
#[doc = "Reader of field `GPIO_RESET`"]
pub type GPIO_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_RESET`"]
pub struct GPIO_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PVT_RESET`"]
pub type PVT_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PVT_RESET`"]
pub struct PVT_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PVT_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `PWM0_RESET`"]
pub type PWM0_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM0_RESET`"]
pub struct PWM0_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM0_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `PWM1_RESET`"]
pub type PWM1_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM1_RESET`"]
pub struct PWM1_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM1_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PWM2_RESET`"]
pub type PWM2_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWM2_RESET`"]
pub struct PWM2_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM2_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RTC_RESET`"]
pub type RTC_RESET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC_RESET`"]
pub struct RTC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Reset Active low"]
    #[inline(always)]
    pub fn gptimer_reset(&self) -> GPTIMER_RESET_R {
        GPTIMER_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reset Active low"]
    #[inline(always)]
    pub fn i2c0_reset(&self) -> I2C0_RESET_R {
        I2C0_RESET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset Active low"]
    #[inline(always)]
    pub fn i2c1_reset(&self) -> I2C1_RESET_R {
        I2C1_RESET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Active low"]
    #[inline(always)]
    pub fn i2s_reset(&self) -> I2S_RESET_R {
        I2S_RESET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Reset Active low"]
    #[inline(always)]
    pub fn spi_reset(&self) -> SPI_RESET_R {
        SPI_RESET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Reset Active low"]
    #[inline(always)]
    pub fn qspi_reset(&self) -> QSPI_RESET_R {
        QSPI_RESET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Reset Active low"]
    #[inline(always)]
    pub fn uart0_reset(&self) -> UART0_RESET_R {
        UART0_RESET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Reset Active low"]
    #[inline(always)]
    pub fn uart1_reset(&self) -> UART1_RESET_R {
        UART1_RESET_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Reset Active low"]
    #[inline(always)]
    pub fn gpio_reset(&self) -> GPIO_RESET_R {
        GPIO_RESET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reset Active low"]
    #[inline(always)]
    pub fn pvt_reset(&self) -> PVT_RESET_R {
        PVT_RESET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Reset Active low"]
    #[inline(always)]
    pub fn pwm0_reset(&self) -> PWM0_RESET_R {
        PWM0_RESET_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reset Active low"]
    #[inline(always)]
    pub fn pwm1_reset(&self) -> PWM1_RESET_R {
        PWM1_RESET_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset Active low"]
    #[inline(always)]
    pub fn pwm2_reset(&self) -> PWM2_RESET_R {
        PWM2_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Reset Active low"]
    #[inline(always)]
    pub fn rtc_reset(&self) -> RTC_RESET_R {
        RTC_RESET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Reset Active low"]
    #[inline(always)]
    pub fn gptimer_reset(&mut self) -> GPTIMER_RESET_W {
        GPTIMER_RESET_W { w: self }
    }
    #[doc = "Bit 2 - Reset Active low"]
    #[inline(always)]
    pub fn i2c0_reset(&mut self) -> I2C0_RESET_W {
        I2C0_RESET_W { w: self }
    }
    #[doc = "Bit 3 - Reset Active low"]
    #[inline(always)]
    pub fn i2c1_reset(&mut self) -> I2C1_RESET_W {
        I2C1_RESET_W { w: self }
    }
    #[doc = "Bit 4 - Reset Active low"]
    #[inline(always)]
    pub fn i2s_reset(&mut self) -> I2S_RESET_W {
        I2S_RESET_W { w: self }
    }
    #[doc = "Bit 5 - Reset Active low"]
    #[inline(always)]
    pub fn spi_reset(&mut self) -> SPI_RESET_W {
        SPI_RESET_W { w: self }
    }
    #[doc = "Bit 6 - Reset Active low"]
    #[inline(always)]
    pub fn qspi_reset(&mut self) -> QSPI_RESET_W {
        QSPI_RESET_W { w: self }
    }
    #[doc = "Bit 7 - Reset Active low"]
    #[inline(always)]
    pub fn uart0_reset(&mut self) -> UART0_RESET_W {
        UART0_RESET_W { w: self }
    }
    #[doc = "Bit 8 - Reset Active low"]
    #[inline(always)]
    pub fn uart1_reset(&mut self) -> UART1_RESET_W {
        UART1_RESET_W { w: self }
    }
    #[doc = "Bit 9 - Reset Active low"]
    #[inline(always)]
    pub fn gpio_reset(&mut self) -> GPIO_RESET_W {
        GPIO_RESET_W { w: self }
    }
    #[doc = "Bit 10 - Reset Active low"]
    #[inline(always)]
    pub fn pvt_reset(&mut self) -> PVT_RESET_W {
        PVT_RESET_W { w: self }
    }
    #[doc = "Bit 11 - Reset Active low"]
    #[inline(always)]
    pub fn pwm0_reset(&mut self) -> PWM0_RESET_W {
        PWM0_RESET_W { w: self }
    }
    #[doc = "Bit 12 - Reset Active low"]
    #[inline(always)]
    pub fn pwm1_reset(&mut self) -> PWM1_RESET_W {
        PWM1_RESET_W { w: self }
    }
    #[doc = "Bit 13 - Reset Active low"]
    #[inline(always)]
    pub fn pwm2_reset(&mut self) -> PWM2_RESET_W {
        PWM2_RESET_W { w: self }
    }
    #[doc = "Bit 14 - Reset Active low"]
    #[inline(always)]
    pub fn rtc_reset(&mut self) -> RTC_RESET_W {
        RTC_RESET_W { w: self }
    }
}
