#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MPC Control register"]
    pub ctrl: CTRL,
    _reserved1: [u8; 12usize],
    #[doc = "0x10 - Maximum value of block based index register"]
    pub blk_max: BLK_MAX,
    #[doc = "0x14 - Block Configuration"]
    pub blk_cfg: BLK_CFG,
    #[doc = "0x18 - Index value for accessing block based look up table"]
    pub blk_idx: BLK_IDX,
    #[doc = "0x1c - Block based gating Look Up Table"]
    pub blk_lut: BLK_LUT,
    #[doc = "0x20 - Interrupt state"]
    pub int_stat: INT_STAT,
    #[doc = "0x24 - Interrupt clear"]
    pub int_clear: INT_CLEAR,
    #[doc = "0x28 - Interrupt enable"]
    pub int_en: INT_EN,
    #[doc = "0x2c - Interrupt information 1"]
    pub int_info1: INT_INFO1,
    #[doc = "0x30 - Interrupt information 2"]
    pub int_info2: INT_INFO2,
    #[doc = "0x34 - Interrupt set. Debug purpose only"]
    pub int_set: INT_SET,
    _reserved11: [u8; 3992usize],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub pidr4: PIDR4,
    #[doc = "0xfd4 - Peripheral ID 5"]
    pub pidr5: PIDR5,
    #[doc = "0xfd8 - Peripheral ID 6"]
    pub pidr6: PIDR6,
    #[doc = "0xfdc - Peripheral ID 7"]
    pub pidr7: PIDR7,
    #[doc = "0xfe0 - Peripheral ID 0"]
    pub pidr0: PIDR0,
    #[doc = "0xfe4 - Peripheral ID 1"]
    pub pidr1: PIDR1,
    #[doc = "0xfe8 - Peripheral ID 2"]
    pub pidr2: PIDR2,
    #[doc = "0xfec - Peripheral ID 3"]
    pub pidr3: PIDR3,
    #[doc = "0xff0 - Component ID 0"]
    pub cidr0: CIDR0,
    #[doc = "0xff4 - Component ID 1"]
    pub cidr1: CIDR1,
    #[doc = "0xff8 - Component ID 2"]
    pub cidr2: CIDR2,
    #[doc = "0xffc - Component ID 3"]
    pub cidr3: CIDR3,
}
#[doc = "MPC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "MPC Control register"]
pub mod ctrl;
#[doc = "Maximum value of block based index register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk_max](blk_max) module"]
pub type BLK_MAX = crate::Reg<u32, _BLK_MAX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_MAX;
#[doc = "`read()` method returns [blk_max::R](blk_max::R) reader structure"]
impl crate::Readable for BLK_MAX {}
#[doc = "Maximum value of block based index register"]
pub mod blk_max;
#[doc = "Block Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk_cfg](blk_cfg) module"]
pub type BLK_CFG = crate::Reg<u32, _BLK_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_CFG;
#[doc = "`read()` method returns [blk_cfg::R](blk_cfg::R) reader structure"]
impl crate::Readable for BLK_CFG {}
#[doc = "Block Configuration"]
pub mod blk_cfg;
#[doc = "Index value for accessing block based look up table\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk_idx](blk_idx) module"]
pub type BLK_IDX = crate::Reg<u32, _BLK_IDX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_IDX;
#[doc = "`read()` method returns [blk_idx::R](blk_idx::R) reader structure"]
impl crate::Readable for BLK_IDX {}
#[doc = "`write(|w| ..)` method takes [blk_idx::W](blk_idx::W) writer structure"]
impl crate::Writable for BLK_IDX {}
#[doc = "Index value for accessing block based look up table"]
pub mod blk_idx;
#[doc = "Block based gating Look Up Table\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blk_lut](blk_lut) module"]
pub type BLK_LUT = crate::Reg<u32, _BLK_LUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLK_LUT;
#[doc = "`read()` method returns [blk_lut::R](blk_lut::R) reader structure"]
impl crate::Readable for BLK_LUT {}
#[doc = "`write(|w| ..)` method takes [blk_lut::W](blk_lut::W) writer structure"]
impl crate::Writable for BLK_LUT {}
#[doc = "Block based gating Look Up Table"]
pub mod blk_lut;
#[doc = "Interrupt state\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_stat](int_stat) module"]
pub type INT_STAT = crate::Reg<u32, _INT_STAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_STAT;
#[doc = "`read()` method returns [int_stat::R](int_stat::R) reader structure"]
impl crate::Readable for INT_STAT {}
#[doc = "Interrupt state"]
pub mod int_stat;
#[doc = "Interrupt clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_clear](int_clear) module"]
pub type INT_CLEAR = crate::Reg<u32, _INT_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_CLEAR;
#[doc = "`write(|w| ..)` method takes [int_clear::W](int_clear::W) writer structure"]
impl crate::Writable for INT_CLEAR {}
#[doc = "Interrupt clear"]
pub mod int_clear;
#[doc = "Interrupt enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_en](int_en) module"]
pub type INT_EN = crate::Reg<u32, _INT_EN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_EN;
#[doc = "`read()` method returns [int_en::R](int_en::R) reader structure"]
impl crate::Readable for INT_EN {}
#[doc = "`write(|w| ..)` method takes [int_en::W](int_en::W) writer structure"]
impl crate::Writable for INT_EN {}
#[doc = "Interrupt enable"]
pub mod int_en;
#[doc = "Interrupt information 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_info1](int_info1) module"]
pub type INT_INFO1 = crate::Reg<u32, _INT_INFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_INFO1;
#[doc = "`read()` method returns [int_info1::R](int_info1::R) reader structure"]
impl crate::Readable for INT_INFO1 {}
#[doc = "Interrupt information 1"]
pub mod int_info1;
#[doc = "Interrupt information 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_info2](int_info2) module"]
pub type INT_INFO2 = crate::Reg<u32, _INT_INFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_INFO2;
#[doc = "`read()` method returns [int_info2::R](int_info2::R) reader structure"]
impl crate::Readable for INT_INFO2 {}
#[doc = "Interrupt information 2"]
pub mod int_info2;
#[doc = "Interrupt set. Debug purpose only\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [int_set](int_set) module"]
pub type INT_SET = crate::Reg<u32, _INT_SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INT_SET;
#[doc = "`write(|w| ..)` method takes [int_set::W](int_set::W) writer structure"]
impl crate::Writable for INT_SET {}
#[doc = "Interrupt set. Debug purpose only"]
pub mod int_set;
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral ID 4"]
pub mod pidr4;
#[doc = "Peripheral ID 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr5](pidr5) module"]
pub type PIDR5 = crate::Reg<u32, _PIDR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR5;
#[doc = "`read()` method returns [pidr5::R](pidr5::R) reader structure"]
impl crate::Readable for PIDR5 {}
#[doc = "Peripheral ID 5"]
pub mod pidr5;
#[doc = "Peripheral ID 6\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr6](pidr6) module"]
pub type PIDR6 = crate::Reg<u32, _PIDR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR6;
#[doc = "`read()` method returns [pidr6::R](pidr6::R) reader structure"]
impl crate::Readable for PIDR6 {}
#[doc = "Peripheral ID 6"]
pub mod pidr6;
#[doc = "Peripheral ID 7\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr7](pidr7) module"]
pub type PIDR7 = crate::Reg<u32, _PIDR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR7;
#[doc = "`read()` method returns [pidr7::R](pidr7::R) reader structure"]
impl crate::Readable for PIDR7 {}
#[doc = "Peripheral ID 7"]
pub mod pidr7;
#[doc = "Peripheral ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr0](pidr0) module"]
pub type PIDR0 = crate::Reg<u32, _PIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR0;
#[doc = "`read()` method returns [pidr0::R](pidr0::R) reader structure"]
impl crate::Readable for PIDR0 {}
#[doc = "Peripheral ID 0"]
pub mod pidr0;
#[doc = "Peripheral ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr1](pidr1) module"]
pub type PIDR1 = crate::Reg<u32, _PIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR1;
#[doc = "`read()` method returns [pidr1::R](pidr1::R) reader structure"]
impl crate::Readable for PIDR1 {}
#[doc = "Peripheral ID 1"]
pub mod pidr1;
#[doc = "Peripheral ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr2](pidr2) module"]
pub type PIDR2 = crate::Reg<u32, _PIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR2;
#[doc = "`read()` method returns [pidr2::R](pidr2::R) reader structure"]
impl crate::Readable for PIDR2 {}
#[doc = "Peripheral ID 2"]
pub mod pidr2;
#[doc = "Peripheral ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr3](pidr3) module"]
pub type PIDR3 = crate::Reg<u32, _PIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR3;
#[doc = "`read()` method returns [pidr3::R](pidr3::R) reader structure"]
impl crate::Readable for PIDR3 {}
#[doc = "Peripheral ID 3"]
pub mod pidr3;
#[doc = "Component ID 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr0](cidr0) module"]
pub type CIDR0 = crate::Reg<u32, _CIDR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR0;
#[doc = "`read()` method returns [cidr0::R](cidr0::R) reader structure"]
impl crate::Readable for CIDR0 {}
#[doc = "Component ID 0"]
pub mod cidr0;
#[doc = "Component ID 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr1](cidr1) module"]
pub type CIDR1 = crate::Reg<u32, _CIDR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR1;
#[doc = "`read()` method returns [cidr1::R](cidr1::R) reader structure"]
impl crate::Readable for CIDR1 {}
#[doc = "Component ID 1"]
pub mod cidr1;
#[doc = "Component ID 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr2](cidr2) module"]
pub type CIDR2 = crate::Reg<u32, _CIDR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR2;
#[doc = "`read()` method returns [cidr2::R](cidr2::R) reader structure"]
impl crate::Readable for CIDR2 {}
#[doc = "Component ID 2"]
pub mod cidr2;
#[doc = "Component ID 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cidr3](cidr3) module"]
pub type CIDR3 = crate::Reg<u32, _CIDR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIDR3;
#[doc = "`read()` method returns [cidr3::R](cidr3::R) reader structure"]
impl crate::Readable for CIDR3 {}
#[doc = "Component ID 3"]
pub mod cidr3;
