#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Version Register"]
    pub sys_version: SYS_VERSION,
    #[doc = "0x04 - System Hardware Configuration register"]
    pub sys_config: SYS_CONFIG,
    _reserved2: [u8; 4040usize],
    #[doc = "0xfd0 - Peripheral ID 4"]
    pub pidr4: PIDR4,
    _reserved3: [u8; 12usize],
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
#[doc = "System Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sys_version](sys_version) module"]
pub type SYS_VERSION = crate::Reg<u32, _SYS_VERSION>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_VERSION;
#[doc = "`read()` method returns [sys_version::R](sys_version::R) reader structure"]
impl crate::Readable for SYS_VERSION {}
#[doc = "System Version Register"]
pub mod sys_version;
#[doc = "System Hardware Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sys_config](sys_config) module"]
pub type SYS_CONFIG = crate::Reg<u32, _SYS_CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYS_CONFIG;
#[doc = "`read()` method returns [sys_config::R](sys_config::R) reader structure"]
impl crate::Readable for SYS_CONFIG {}
#[doc = "System Hardware Configuration register"]
pub mod sys_config;
#[doc = "Peripheral ID 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pidr4](pidr4) module"]
pub type PIDR4 = crate::Reg<u32, _PIDR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIDR4;
#[doc = "`read()` method returns [pidr4::R](pidr4::R) reader structure"]
impl crate::Readable for PIDR4 {}
#[doc = "Peripheral ID 4"]
pub mod pidr4;
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
