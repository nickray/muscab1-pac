#[doc = "Reader of register SYS_CONFIG"]
pub type R = crate::R<u32, super::SYS_CONFIG>;
#[doc = "Reader of field `SRAM_NUM_BANK`"]
pub type SRAM_NUM_BANK_R = crate::R<u8, u8>;
#[doc = "Reader of field `SRAM_ADDR_WIDTH`"]
pub type SRAM_ADDR_WIDTH_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `CPU0_HAS_TCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0_HAS_TCM_A {
    #[doc = "CPU 0 does not have Data TCM"]
    NO,
    #[doc = "CPU 0 has Data TCM"]
    YES,
}
impl From<CPU0_HAS_TCM_A> for bool {
    #[inline(always)]
    fn from(variant: CPU0_HAS_TCM_A) -> Self {
        match variant {
            CPU0_HAS_TCM_A::NO => false,
            CPU0_HAS_TCM_A::YES => true,
        }
    }
}
#[doc = "Reader of field `CPU0_HAS_TCM`"]
pub type CPU0_HAS_TCM_R = crate::R<bool, CPU0_HAS_TCM_A>;
impl CPU0_HAS_TCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU0_HAS_TCM_A {
        match self.bits {
            false => CPU0_HAS_TCM_A::NO,
            true => CPU0_HAS_TCM_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CPU0_HAS_TCM_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CPU0_HAS_TCM_A::YES
    }
}
#[doc = "Possible values of the field `CPU1_HAS_TCM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_HAS_TCM_A {
    #[doc = "CPU 1 does not have Data TCM"]
    NO,
    #[doc = "CPU 1 has Data TCM"]
    YES,
}
impl From<CPU1_HAS_TCM_A> for bool {
    #[inline(always)]
    fn from(variant: CPU1_HAS_TCM_A) -> Self {
        match variant {
            CPU1_HAS_TCM_A::NO => false,
            CPU1_HAS_TCM_A::YES => true,
        }
    }
}
#[doc = "Reader of field `CPU1_HAS_TCM`"]
pub type CPU1_HAS_TCM_R = crate::R<bool, CPU1_HAS_TCM_A>;
impl CPU1_HAS_TCM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPU1_HAS_TCM_A {
        match self.bits {
            false => CPU1_HAS_TCM_A::NO,
            true => CPU1_HAS_TCM_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == CPU1_HAS_TCM_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == CPU1_HAS_TCM_A::YES
    }
}
#[doc = "Possible values of the field `HAS_CRYPTO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HAS_CRYPTO_A {
    #[doc = "CryptoCell Not Included"]
    NO,
    #[doc = "CryptoCell Included"]
    YES,
}
impl From<HAS_CRYPTO_A> for bool {
    #[inline(always)]
    fn from(variant: HAS_CRYPTO_A) -> Self {
        match variant {
            HAS_CRYPTO_A::NO => false,
            HAS_CRYPTO_A::YES => true,
        }
    }
}
#[doc = "Reader of field `HAS_CRYPTO`"]
pub type HAS_CRYPTO_R = crate::R<bool, HAS_CRYPTO_A>;
impl HAS_CRYPTO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HAS_CRYPTO_A {
        match self.bits {
            false => HAS_CRYPTO_A::NO,
            true => HAS_CRYPTO_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == HAS_CRYPTO_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == HAS_CRYPTO_A::YES
    }
}
#[doc = "Reader of field `CPU0_TCM_BANK_NUM`"]
pub type CPU0_TCM_BANK_NUM_R = crate::R<u8, u8>;
#[doc = "Possible values of the field `CPU1_TCM_BANK_NUM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_TCM_BANK_NUM_A {
    #[doc = "4 SRAM Banks"]
    FOUR,
    #[doc = "3 SRAM Banks"]
    THREE,
    #[doc = "2 SRAM Banks"]
    TWO,
    #[doc = "Otherwise"]
    OTHERWISE,
}
impl From<CPU1_TCM_BANK_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1_TCM_BANK_NUM_A) -> Self {
        match variant {
            CPU1_TCM_BANK_NUM_A::FOUR => 3,
            CPU1_TCM_BANK_NUM_A::THREE => 2,
            CPU1_TCM_BANK_NUM_A::TWO => 1,
            CPU1_TCM_BANK_NUM_A::OTHERWISE => 0,
        }
    }
}
#[doc = "Reader of field `CPU1_TCM_BANK_NUM`"]
pub type CPU1_TCM_BANK_NUM_R = crate::R<u8, CPU1_TCM_BANK_NUM_A>;
impl CPU1_TCM_BANK_NUM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU1_TCM_BANK_NUM_A> {
        use crate::Variant::*;
        match self.bits {
            3 => Val(CPU1_TCM_BANK_NUM_A::FOUR),
            2 => Val(CPU1_TCM_BANK_NUM_A::THREE),
            1 => Val(CPU1_TCM_BANK_NUM_A::TWO),
            0 => Val(CPU1_TCM_BANK_NUM_A::OTHERWISE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == CPU1_TCM_BANK_NUM_A::FOUR
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == CPU1_TCM_BANK_NUM_A::THREE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CPU1_TCM_BANK_NUM_A::TWO
    }
    #[doc = "Checks if the value of the field is `OTHERWISE`"]
    #[inline(always)]
    pub fn is_otherwise(&self) -> bool {
        *self == CPU1_TCM_BANK_NUM_A::OTHERWISE
    }
}
#[doc = "Possible values of the field `CPU0_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU0_TYPE_A {
    #[doc = "Does Not Exist"]
    NOTEXIST,
    #[doc = "Cortex-M33 Core"]
    CM33,
}
impl From<CPU0_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU0_TYPE_A) -> Self {
        match variant {
            CPU0_TYPE_A::NOTEXIST => 0,
            CPU0_TYPE_A::CM33 => 2,
        }
    }
}
#[doc = "Reader of field `CPU0_TYPE`"]
pub type CPU0_TYPE_R = crate::R<u8, CPU0_TYPE_A>;
impl CPU0_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU0_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPU0_TYPE_A::NOTEXIST),
            2 => Val(CPU0_TYPE_A::CM33),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTEXIST`"]
    #[inline(always)]
    pub fn is_not_exist(&self) -> bool {
        *self == CPU0_TYPE_A::NOTEXIST
    }
    #[doc = "Checks if the value of the field is `CM33`"]
    #[inline(always)]
    pub fn is_cm33(&self) -> bool {
        *self == CPU0_TYPE_A::CM33
    }
}
#[doc = "Possible values of the field `CPU1_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPU1_TYPE_A {
    #[doc = "Does Not Exist"]
    NOTEXIST,
    #[doc = "Cortex-M33 Core"]
    CM33,
}
impl From<CPU1_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: CPU1_TYPE_A) -> Self {
        match variant {
            CPU1_TYPE_A::NOTEXIST => 0,
            CPU1_TYPE_A::CM33 => 2,
        }
    }
}
#[doc = "Reader of field `CPU1_TYPE`"]
pub type CPU1_TYPE_R = crate::R<u8, CPU1_TYPE_A>;
impl CPU1_TYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CPU1_TYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CPU1_TYPE_A::NOTEXIST),
            2 => Val(CPU1_TYPE_A::CM33),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTEXIST`"]
    #[inline(always)]
    pub fn is_not_exist(&self) -> bool {
        *self == CPU1_TYPE_A::NOTEXIST
    }
    #[doc = "Checks if the value of the field is `CM33`"]
    #[inline(always)]
    pub fn is_cm33(&self) -> bool {
        *self == CPU1_TYPE_A::CM33
    }
}
impl R {
    #[doc = "Bits 0:3 - SRAM Number of Banks"]
    #[inline(always)]
    pub fn sram_num_bank(&self) -> SRAM_NUM_BANK_R {
        SRAM_NUM_BANK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - SRAM Bank Address Width"]
    #[inline(always)]
    pub fn sram_addr_width(&self) -> SRAM_ADDR_WIDTH_R {
        SRAM_ADDR_WIDTH_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bit 9 - CPU 0 has Data TCM:"]
    #[inline(always)]
    pub fn cpu0_has_tcm(&self) -> CPU0_HAS_TCM_R {
        CPU0_HAS_TCM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CPU 1 has Data TCM:"]
    #[inline(always)]
    pub fn cpu1_has_tcm(&self) -> CPU1_HAS_TCM_R {
        CPU1_HAS_TCM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Whether CryptoCell Included:"]
    #[inline(always)]
    pub fn has_crypto(&self) -> HAS_CRYPTO_R {
        HAS_CRYPTO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - The SRAM Bank that maps CPU0 Data TCM"]
    #[inline(always)]
    pub fn cpu0_tcm_bank_num(&self) -> CPU0_TCM_BANK_NUM_R {
        CPU0_TCM_BANK_NUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Number of SRAM banks"]
    #[inline(always)]
    pub fn cpu1_tcm_bank_num(&self) -> CPU1_TCM_BANK_NUM_R {
        CPU1_TCM_BANK_NUM_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CPU 0 Core Type"]
    #[inline(always)]
    pub fn cpu0_type(&self) -> CPU0_TYPE_R {
        CPU0_TYPE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CPU 1 Core Type"]
    #[inline(always)]
    pub fn cpu1_type(&self) -> CPU1_TYPE_R {
        CPU1_TYPE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
