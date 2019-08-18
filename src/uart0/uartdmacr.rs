#[doc = "Reader of register UARTDMACR"]
pub type R = crate::R<u32, super::UARTDMACR>;
#[doc = "Writer for register UARTDMACR"]
pub type W = crate::W<u32, super::UARTDMACR>;
#[doc = "Register UARTDMACR `reset()`'s with value 0"]
impl crate::ResetValue for super::UARTDMACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMAE_A {
    #[doc = "Receive DMA is disabled"]
    DISABLE,
    #[doc = "Receive DMA is enabled"]
    ENABLE,
}
impl From<RXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAE_A) -> Self {
        match variant {
            RXDMAE_A::DISABLE => false,
            RXDMAE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `RXDMAE`"]
pub type RXDMAE_R = crate::R<bool, RXDMAE_A>;
impl RXDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAE_A {
        match self.bits {
            false => RXDMAE_A::DISABLE,
            true => RXDMAE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RXDMAE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RXDMAE_A::ENABLE
    }
}
#[doc = "Write proxy for field `RXDMAE`"]
pub struct RXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receive DMA is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(RXDMAE_A::DISABLE)
    }
    #[doc = "Receive DMA is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(RXDMAE_A::ENABLE)
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
#[doc = "Possible values of the field `TXDMAE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMAE_A {
    #[doc = "Transmit DMA is disabled"]
    DISABLE,
    #[doc = "Transmit DMA is enabled"]
    ENABLE,
}
impl From<TXDMAE_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAE_A) -> Self {
        match variant {
            TXDMAE_A::DISABLE => false,
            TXDMAE_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `TXDMAE`"]
pub type TXDMAE_R = crate::R<bool, TXDMAE_A>;
impl TXDMAE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAE_A {
        match self.bits {
            false => TXDMAE_A::DISABLE,
            true => TXDMAE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TXDMAE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TXDMAE_A::ENABLE
    }
}
#[doc = "Write proxy for field `TXDMAE`"]
pub struct TXDMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMAE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMAE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Transmit DMA is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TXDMAE_A::DISABLE)
    }
    #[doc = "Transmit DMA is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TXDMAE_A::ENABLE)
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
#[doc = "Possible values of the field `DMAONERR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAONERR_A {
    #[doc = "DMA receive request outputs are\n                          enabled when the UART error interrupt is\n                          asserted"]
    DISABLE,
    #[doc = "DMA receive request outputs are\n                          disabled when the UART error interrupt is\n                          asserted"]
    ENABLE,
}
impl From<DMAONERR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAONERR_A) -> Self {
        match variant {
            DMAONERR_A::DISABLE => false,
            DMAONERR_A::ENABLE => true,
        }
    }
}
#[doc = "Reader of field `DMAONERR`"]
pub type DMAONERR_R = crate::R<bool, DMAONERR_A>;
impl DMAONERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAONERR_A {
        match self.bits {
            false => DMAONERR_A::DISABLE,
            true => DMAONERR_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DMAONERR_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DMAONERR_A::ENABLE
    }
}
#[doc = "Write proxy for field `DMAONERR`"]
pub struct DMAONERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAONERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMAONERR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "DMA receive request outputs are enabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DMAONERR_A::DISABLE)
    }
    #[doc = "DMA receive request outputs are disabled when the UART error interrupt is asserted"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DMAONERR_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RXDMAE_R {
        RXDMAE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TXDMAE_R {
        TXDMAE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&self) -> DMAONERR_R {
        DMAONERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&mut self) -> RXDMAE_W {
        RXDMAE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&mut self) -> TXDMAE_W {
        TXDMAE_W { w: self }
    }
    #[doc = "Bit 2 - DMA on error"]
    #[inline(always)]
    pub fn dmaonerr(&mut self) -> DMAONERR_W {
        DMAONERR_W { w: self }
    }
}
