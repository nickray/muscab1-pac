#[doc = "Reader of register BLK_MAX"]
pub type R = crate::R<u32, super::BLK_MAX>;
#[doc = "Reader of field `bit[3_0]`"]
pub type BIT3_0_R = crate::R<u8, u8>;
#[doc = "Reader of field `bit[31]`"]
pub type BIT31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Block size"]
    #[inline(always)]
    pub fn bit3_0(&self) -> BIT3_0_R {
        BIT3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Initialization in progress"]
    #[inline(always)]
    pub fn bit31(&self) -> BIT31_R {
        BIT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
