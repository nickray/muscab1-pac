#[doc = "Reader of register CLOCK_FORCE"]
pub type R = crate::R<u32, super::CLOCK_FORCE>;
#[doc = "Writer for register CLOCK_FORCE"]
pub type W = crate::W<u32, super::CLOCK_FORCE>;
#[doc = "Register CLOCK_FORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::CLOCK_FORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAINCLK_FORCE`"]
pub type MAINCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MAINCLK_FORCE`"]
pub struct MAINCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAINCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `SYSSYSCLK_FORCE`"]
pub type SYSSYSCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSSYSCLK_FORCE`"]
pub struct SYSSYSCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSSYSCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `SYSFCLK_FORCE`"]
pub type SYSFCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSFCLK_FORCE`"]
pub struct SYSFCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `SRAMSYSCLK_FORCE`"]
pub type SRAMSYSCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMSYSCLK_FORCE`"]
pub struct SRAMSYSCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMSYSCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `SRAMFCLK_FORCE`"]
pub type SRAMFCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAMFCLK_FORCE`"]
pub struct SRAMFCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMFCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `CPUSYSCLK_FORCE`"]
pub type CPUSYSCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPUSYSCLK_FORCE`"]
pub struct CPUSYSCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUSYSCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `CPUFCLK_FORCE`"]
pub type CPUFCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CPUFCLK_FORCE`"]
pub struct CPUFCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUFCLK_FORCE_W<'a> {
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
#[doc = "Reader of field `CRYPTOSYSCLK_FORCE`"]
pub type CRYPTOSYSCLK_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYPTOSYSCLK_FORCE`"]
pub struct CRYPTOSYSCLK_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTOSYSCLK_FORCE_W<'a> {
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
#[doc = "Possible values of the field `FCLKHINTGATE_ENABLE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FCLKHINTGATE_ENABLE_A {
    #[doc = "Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    ENABLE,
    #[doc = "improve SRAM3 access latency at the cost of increased power consumption"]
    LATENCY,
}
impl From<FCLKHINTGATE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: FCLKHINTGATE_ENABLE_A) -> Self {
        match variant {
            FCLKHINTGATE_ENABLE_A::ENABLE => true,
            FCLKHINTGATE_ENABLE_A::LATENCY => false,
        }
    }
}
#[doc = "Reader of field `FCLKHINTGATE_ENABLE`"]
pub type FCLKHINTGATE_ENABLE_R = crate::R<bool, FCLKHINTGATE_ENABLE_A>;
impl FCLKHINTGATE_ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCLKHINTGATE_ENABLE_A {
        match self.bits {
            true => FCLKHINTGATE_ENABLE_A::ENABLE,
            false => FCLKHINTGATE_ENABLE_A::LATENCY,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FCLKHINTGATE_ENABLE_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `LATENCY`"]
    #[inline(always)]
    pub fn is_latency(&self) -> bool {
        *self == FCLKHINTGATE_ENABLE_A::LATENCY
    }
}
#[doc = "Write proxy for field `FCLKHINTGATE_ENABLE`"]
pub struct FCLKHINTGATE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCLKHINTGATE_ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FCLKHINTGATE_ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FCLKHINTGATE_ENABLE_A::ENABLE)
    }
    #[doc = "improve SRAM3 access latency at the cost of increased power consumption"]
    #[inline(always)]
    pub fn latency(self) -> &'a mut W {
        self.variant(FCLKHINTGATE_ENABLE_A::LATENCY)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force MAINCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn mainclk_force(&self) -> MAINCLK_FORCE_R {
        MAINCLK_FORCE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Force Base element Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn syssysclk_force(&self) -> SYSSYSCLK_FORCE_R {
        SYSSYSCLK_FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force Base element Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sysfclk_force(&self) -> SYSFCLK_FORCE_R {
        SYSFCLK_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Force SRAM Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramsysclk_force(&self) -> SRAMSYSCLK_FORCE_R {
        SRAMSYSCLK_FORCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Force SRAM Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramfclk_force(&self) -> SRAMFCLK_FORCE_R {
        SRAMFCLK_FORCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force all CPU SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpusysclk_force(&self) -> CPUSYSCLK_FORCE_R {
        CPUSYSCLK_FORCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Force all CPU FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpufclk_force(&self) -> CPUFCLK_FORCE_R {
        CPUFCLK_FORCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Force all CryptoCell clocks to run when set to HIGH"]
    #[inline(always)]
    pub fn cryptosysclk_force(&self) -> CRYPTOSYSCLK_FORCE_R {
        CRYPTOSYSCLK_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn fclkhintgate_enable(&self) -> FCLKHINTGATE_ENABLE_R {
        FCLKHINTGATE_ENABLE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force MAINCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn mainclk_force(&mut self) -> MAINCLK_FORCE_W {
        MAINCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 1 - Force Base element Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn syssysclk_force(&mut self) -> SYSSYSCLK_FORCE_W {
        SYSSYSCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 2 - Force Base element Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sysfclk_force(&mut self) -> SYSFCLK_FORCE_W {
        SYSFCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 3 - Force SRAM Local SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramsysclk_force(&mut self) -> SRAMSYSCLK_FORCE_W {
        SRAMSYSCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 4 - Force SRAM Local FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn sramfclk_force(&mut self) -> SRAMFCLK_FORCE_W {
        SRAMFCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 5 - Force all CPU SYSCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpusysclk_force(&mut self) -> CPUSYSCLK_FORCE_W {
        CPUSYSCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 6 - Force all CPU FCLK to run when set to HIGH"]
    #[inline(always)]
    pub fn cpufclk_force(&mut self) -> CPUFCLK_FORCE_W {
        CPUFCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 7 - Force all CryptoCell clocks to run when set to HIGH"]
    #[inline(always)]
    pub fn cryptosysclk_force(&mut self) -> CRYPTOSYSCLK_FORCE_W {
        CRYPTOSYSCLK_FORCE_W { w: self }
    }
    #[doc = "Bit 8 - Enable FCLK gating by HINTSYSCLKEN when CPU 1 is OFF"]
    #[inline(always)]
    pub fn fclkhintgate_enable(&mut self) -> FCLKHINTGATE_ENABLE_W {
        FCLKHINTGATE_ENABLE_W { w: self }
    }
}
