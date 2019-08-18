#[doc = "Reader of register TIMER1CONTROL"]
pub type R = crate::R<u32, super::TIMER1CONTROL>;
#[doc = "Writer for register TIMER1CONTROL"]
pub type W = crate::W<u32, super::TIMER1CONTROL>;
#[doc = "Register TIMER1CONTROL `reset()`'s with value 0x20"]
impl crate::ResetValue for super::TIMER1CONTROL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x20
    }
}
#[doc = "Possible values of the field `OneShotCount`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOTCOUNT_A {
    #[doc = "Wrapping counter mode"]
    WRAPPING,
    #[doc = "One-shot counter mode"]
    ONESHOT,
}
impl From<ONESHOTCOUNT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOTCOUNT_A) -> Self {
        match variant {
            ONESHOTCOUNT_A::WRAPPING => false,
            ONESHOTCOUNT_A::ONESHOT => true,
        }
    }
}
#[doc = "Reader of field `OneShotCount`"]
pub type ONESHOTCOUNT_R = crate::R<bool, ONESHOTCOUNT_A>;
impl ONESHOTCOUNT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOTCOUNT_A {
        match self.bits {
            false => ONESHOTCOUNT_A::WRAPPING,
            true => ONESHOTCOUNT_A::ONESHOT,
        }
    }
    #[doc = "Checks if the value of the field is `WRAPPING`"]
    #[inline(always)]
    pub fn is_wrapping(&self) -> bool {
        *self == ONESHOTCOUNT_A::WRAPPING
    }
    #[doc = "Checks if the value of the field is `ONESHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == ONESHOTCOUNT_A::ONESHOT
    }
}
#[doc = "Write proxy for field `OneShotCount`"]
pub struct ONESHOTCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOTCOUNT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOTCOUNT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Wrapping counter mode"]
    #[inline(always)]
    pub fn wrapping(self) -> &'a mut W {
        self.variant(ONESHOTCOUNT_A::WRAPPING)
    }
    #[doc = "One-shot counter mode"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(ONESHOTCOUNT_A::ONESHOT)
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
#[doc = "Possible values of the field `TimerSize`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERSIZE_A {
    #[doc = "16-bit counter mode"]
    _16BIT,
    #[doc = "32-bit counter mode"]
    _32BIT,
}
impl From<TIMERSIZE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMERSIZE_A) -> Self {
        match variant {
            TIMERSIZE_A::_16BIT => false,
            TIMERSIZE_A::_32BIT => true,
        }
    }
}
#[doc = "Reader of field `TimerSize`"]
pub type TIMERSIZE_R = crate::R<bool, TIMERSIZE_A>;
impl TIMERSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMERSIZE_A {
        match self.bits {
            false => TIMERSIZE_A::_16BIT,
            true => TIMERSIZE_A::_32BIT,
        }
    }
    #[doc = "Checks if the value of the field is `_16BIT`"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == TIMERSIZE_A::_16BIT
    }
    #[doc = "Checks if the value of the field is `_32BIT`"]
    #[inline(always)]
    pub fn is_32bit(&self) -> bool {
        *self == TIMERSIZE_A::_32BIT
    }
}
#[doc = "Write proxy for field `TimerSize`"]
pub struct TIMERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMERSIZE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16-bit counter mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut W {
        self.variant(TIMERSIZE_A::_16BIT)
    }
    #[doc = "32-bit counter mode"]
    #[inline(always)]
    pub fn _32bit(self) -> &'a mut W {
        self.variant(TIMERSIZE_A::_32BIT)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `TimerPre`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERPRE_A {
    #[doc = "clock is divided by 1"]
    DIVIDEDBY1,
    #[doc = "clock is divided by 16"]
    DIVIDEDBY16,
    #[doc = "clock is divided by 256"]
    DIVIDEDBY256,
}
impl From<TIMERPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMERPRE_A) -> Self {
        match variant {
            TIMERPRE_A::DIVIDEDBY1 => 0,
            TIMERPRE_A::DIVIDEDBY16 => 1,
            TIMERPRE_A::DIVIDEDBY256 => 2,
        }
    }
}
#[doc = "Reader of field `TimerPre`"]
pub type TIMERPRE_R = crate::R<u8, TIMERPRE_A>;
impl TIMERPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TIMERPRE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TIMERPRE_A::DIVIDEDBY1),
            1 => Val(TIMERPRE_A::DIVIDEDBY16),
            2 => Val(TIMERPRE_A::DIVIDEDBY256),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDEDBY1`"]
    #[inline(always)]
    pub fn is_dividedby1(&self) -> bool {
        *self == TIMERPRE_A::DIVIDEDBY1
    }
    #[doc = "Checks if the value of the field is `DIVIDEDBY16`"]
    #[inline(always)]
    pub fn is_dividedby16(&self) -> bool {
        *self == TIMERPRE_A::DIVIDEDBY16
    }
    #[doc = "Checks if the value of the field is `DIVIDEDBY256`"]
    #[inline(always)]
    pub fn is_dividedby256(&self) -> bool {
        *self == TIMERPRE_A::DIVIDEDBY256
    }
}
#[doc = "Write proxy for field `TimerPre`"]
pub struct TIMERPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMERPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "clock is divided by 1"]
    #[inline(always)]
    pub fn dividedby1(self) -> &'a mut W {
        self.variant(TIMERPRE_A::DIVIDEDBY1)
    }
    #[doc = "clock is divided by 16"]
    #[inline(always)]
    pub fn dividedby16(self) -> &'a mut W {
        self.variant(TIMERPRE_A::DIVIDEDBY16)
    }
    #[doc = "clock is divided by 256"]
    #[inline(always)]
    pub fn dividedby256(self) -> &'a mut W {
        self.variant(TIMERPRE_A::DIVIDEDBY256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Possible values of the field `InterruptEnable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTERRUPTENABLE_A {
    #[doc = "Interrupt is disabled."]
    DISABLE,
    #[doc = "Interrupt is enabled."]
    ENABLE,
}
impl From<INTERRUPTENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: INTERRUPTENABLE_A) -> Self {
        match variant {
            INTERRUPTENABLE_A::DISABLE => false,
            INTERRUPTENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `InterruptEnable`"]
pub type INTERRUPTENABLE_R = crate::R<bool, INTERRUPTENABLE_A>;
impl INTERRUPTENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTERRUPTENABLE_A {
        match self.bits {
            false => INTERRUPTENABLE_A::DISABLE,
            true => INTERRUPTENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == INTERRUPTENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == INTERRUPTENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `InterruptEnable`"]
pub struct INTERRUPTENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPTENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTERRUPTENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTERRUPTENABLE_A::DISABLE)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(INTERRUPTENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Possible values of the field `TimerMode`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERMODE_A {
    #[doc = "Free-Running timer mode."]
    FREERUNNING,
    #[doc = "Periodic timer mode."]
    PERIODIC,
}
impl From<TIMERMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMERMODE_A) -> Self {
        match variant {
            TIMERMODE_A::FREERUNNING => false,
            TIMERMODE_A::PERIODIC => true,
        }
    }
}
#[doc = "Reader of field `TimerMode`"]
pub type TIMERMODE_R = crate::R<bool, TIMERMODE_A>;
impl TIMERMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMERMODE_A {
        match self.bits {
            false => TIMERMODE_A::FREERUNNING,
            true => TIMERMODE_A::PERIODIC,
        }
    }
    #[doc = "Checks if the value of the field is `FREERUNNING`"]
    #[inline(always)]
    pub fn is_free_running(&self) -> bool {
        *self == TIMERMODE_A::FREERUNNING
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        *self == TIMERMODE_A::PERIODIC
    }
}
#[doc = "Write proxy for field `TimerMode`"]
pub struct TIMERMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMERMODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Free-Running timer mode."]
    #[inline(always)]
    pub fn free_running(self) -> &'a mut W {
        self.variant(TIMERMODE_A::FREERUNNING)
    }
    #[doc = "Periodic timer mode."]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TIMERMODE_A::PERIODIC)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `TimerEnable`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMERENABLE_A {
    #[doc = "Timer is disabled."]
    DISABLE,
    #[doc = "Timer is enabled."]
    ENABLE,
}
impl From<TIMERENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMERENABLE_A) -> Self {
        match variant {
            TIMERENABLE_A::DISABLE => false,
            TIMERENABLE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TimerEnable`"]
pub type TIMERENABLE_R = crate::R<bool, TIMERENABLE_A>;
impl TIMERENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMERENABLE_A {
        match self.bits {
            false => TIMERENABLE_A::DISABLE,
            true => TIMERENABLE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMERENABLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TIMERENABLE_A::ENABLE
    }
}
#[doc = "Write proxy for field `TimerEnable`"]
pub struct TIMERENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMERENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMERENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMERENABLE_A::DISABLE)
    }
    #[doc = "Timer is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TIMERENABLE_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode."]
    #[inline(always)]
    pub fn one_shot_count(&self) -> ONESHOTCOUNT_R {
        ONESHOTCOUNT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects 16-bit or 32- bit counter operation."]
    #[inline(always)]
    pub fn timer_size(&self) -> TIMERSIZE_R {
        TIMERSIZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Timer prescale bits."]
    #[inline(always)]
    pub fn timer_pre(&self) -> TIMERPRE_R {
        TIMERPRE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Interrupt Enable bit."]
    #[inline(always)]
    pub fn interrupt_enable(&self) -> INTERRUPTENABLE_R {
        INTERRUPTENABLE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer Mode bit."]
    #[inline(always)]
    pub fn timer_mode(&self) -> TIMERMODE_R {
        TIMERMODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Timer Enable Enable bit."]
    #[inline(always)]
    pub fn timer_enable(&self) -> TIMERENABLE_R {
        TIMERENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode."]
    #[inline(always)]
    pub fn one_shot_count(&mut self) -> ONESHOTCOUNT_W {
        ONESHOTCOUNT_W { w: self }
    }
    #[doc = "Bit 1 - Selects 16-bit or 32- bit counter operation."]
    #[inline(always)]
    pub fn timer_size(&mut self) -> TIMERSIZE_W {
        TIMERSIZE_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer prescale bits."]
    #[inline(always)]
    pub fn timer_pre(&mut self) -> TIMERPRE_W {
        TIMERPRE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt Enable bit."]
    #[inline(always)]
    pub fn interrupt_enable(&mut self) -> INTERRUPTENABLE_W {
        INTERRUPTENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Timer Mode bit."]
    #[inline(always)]
    pub fn timer_mode(&mut self) -> TIMERMODE_W {
        TIMERMODE_W { w: self }
    }
    #[doc = "Bit 7 - Timer Enable Enable bit."]
    #[inline(always)]
    pub fn timer_enable(&mut self) -> TIMERENABLE_W {
        TIMERENABLE_W { w: self }
    }
}
