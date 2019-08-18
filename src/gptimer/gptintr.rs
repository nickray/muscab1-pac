#[doc = "Reader of register GPTINTR"]
pub type R = crate::R<u32, super::GPTINTR>;
#[doc = "Reader of field `GPTINTR`"]
pub type GPTINTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Raw interrupt state, before masking of GPTINTR interrupt"]
    #[inline(always)]
    pub fn gptintr(&self) -> GPTINTR_R {
        GPTINTR_R::new((self.bits & 0x07) as u8)
    }
}
