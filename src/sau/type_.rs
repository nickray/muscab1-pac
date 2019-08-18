#[doc = "Reader of register TYPE"]
pub type R = crate::R<u32, super::TYPE>;
#[doc = "Reader of field `SREGION`"]
pub type SREGION_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of implemented SAU regions"]
    #[inline(always)]
    pub fn sregion(&self) -> SREGION_R {
        SREGION_R::new((self.bits & 0xff) as u8)
    }
}
