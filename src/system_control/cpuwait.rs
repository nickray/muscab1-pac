#[doc = "Reader of register CPUWAIT"]
pub type R = crate::R<u32, super::CPUWAIT>;
#[doc = "Writer for register CPUWAIT"]
pub type W = crate::W<u32, super::CPUWAIT>;
#[doc = "Register CPUWAIT `reset()`'s with value 0"]
impl crate::ResetValue for super::CPUWAIT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CPU0WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0WAIT_A {
    #[doc = "CPU0 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 powers up"]
    NORMALLYORPOWERUP,
    #[doc = "CPU0 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 do not power up"]
    WAITORNOPOWERUP,
}
impl From<CPU0WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0WAIT_A) -> Self {
        match variant {
            CPU0WAIT_A::NORMALLYORPOWERUP => false,
            CPU0WAIT_A::WAITORNOPOWERUP => true,
        }
    }
}
#[doc = "Reader of field `CPU0WAIT`"]
pub type CPU0WAIT_R = crate::R<bool, CPU0WAIT_A>;
impl CPU0WAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0WAIT_A {
        match self.bits {
            false => CPU0WAIT_A::NORMALLYORPOWERUP,
            true => CPU0WAIT_A::WAITORNOPOWERUP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMALLYORPOWERUP`"]
    #[inline(always)]
    pub fn is_normallyorpowerup(&self) -> bool {
        *self == CPU0WAIT_A::NORMALLYORPOWERUP
    }
    #[doc = "Checks if the value of the field is `WAITORNOPOWERUP`"]
    #[inline(always)]
    pub fn is_waitornopowerup(&self) -> bool {
        *self == CPU0WAIT_A::WAITORNOPOWERUP
    }
}
#[doc = "Write proxy for field `CPU0WAIT`"]
pub struct CPU0WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU0WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU0WAIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU0 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 powers up"]
    #[inline(always)]
    pub fn normallyorpowerup(self) -> &'a mut W {
        self.variant(CPU0WAIT_A::NORMALLYORPOWERUP)
    }
    #[doc = "CPU0 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 1 do not power up"]
    #[inline(always)]
    pub fn waitornopowerup(self) -> &'a mut W {
        self.variant(CPU0WAIT_A::WAITORNOPOWERUP)
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
#[doc = "Possible values of the field `CPU1WAIT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1WAIT_A {
    #[doc = "CPU1 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 powers up"]
    NORMALLYORPOWERUP,
    #[doc = "CPU1 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 do not power up"]
    WAITORNOPOWERUP,
}
impl From<CPU1WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1WAIT_A) -> Self {
        match variant {
            CPU1WAIT_A::NORMALLYORPOWERUP => false,
            CPU1WAIT_A::WAITORNOPOWERUP => true,
        }
    }
}
#[doc = "Reader of field `CPU1WAIT`"]
pub type CPU1WAIT_R = crate::R<bool, CPU1WAIT_A>;
impl CPU1WAIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1WAIT_A {
        match self.bits {
            false => CPU1WAIT_A::NORMALLYORPOWERUP,
            true => CPU1WAIT_A::WAITORNOPOWERUP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMALLYORPOWERUP`"]
    #[inline(always)]
    pub fn is_normallyorpowerup(&self) -> bool {
        *self == CPU1WAIT_A::NORMALLYORPOWERUP
    }
    #[doc = "Checks if the value of the field is `WAITORNOPOWERUP`"]
    #[inline(always)]
    pub fn is_waitornopowerup(&self) -> bool {
        *self == CPU1WAIT_A::WAITORNOPOWERUP
    }
}
#[doc = "Write proxy for field `CPU1WAIT`"]
pub struct CPU1WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU1WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPU1WAIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CPU1 boot normally. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 powers up"]
    #[inline(always)]
    pub fn normallyorpowerup(self) -> &'a mut W {
        self.variant(CPU1WAIT_A::NORMALLYORPOWERUP)
    }
    #[doc = "CPU1 wait. From Power ON reset, nSRST reset or Watchdog Reset, CPU 0 do not power up"]
    #[inline(always)]
    pub fn waitornopowerup(self) -> &'a mut W {
        self.variant(CPU1WAIT_A::WAITORNOPOWERUP)
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
impl R {
    #[doc = "Bit 0 - CPU 0 waits at boot and whether CPU1 powers up"]
    #[inline(always)]
    pub fn cpu0wait(&self) -> CPU0WAIT_R {
        CPU0WAIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CPU 1 waits at boot and whether CPU0 powers up"]
    #[inline(always)]
    pub fn cpu1wait(&self) -> CPU1WAIT_R {
        CPU1WAIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU 0 waits at boot and whether CPU1 powers up"]
    #[inline(always)]
    pub fn cpu0wait(&mut self) -> CPU0WAIT_W {
        CPU0WAIT_W { w: self }
    }
    #[doc = "Bit 1 - CPU 1 waits at boot and whether CPU0 powers up"]
    #[inline(always)]
    pub fn cpu1wait(&mut self) -> CPU1WAIT_W {
        CPU1WAIT_W { w: self }
    }
}
