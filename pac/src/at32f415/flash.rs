#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    psr: Psr,
    unlock: Unlock,
    usd_unlock: UsdUnlock,
    sts: Sts,
    ctrl: Ctrl,
    addr: Addr,
    _reserved6: [u8; 0x04],
    usd: Usd,
    epps: Epps,
    _reserved8: [u8; 0x50],
    slib_sts0: SlibSts0,
    slib_sts1: SlibSts1,
    slib_pwd_clr: SlibPwdClr,
    slib_misc_sts: SlibMiscSts,
    crc_addr: CrcAddr,
    crc_ctrl: CrcCtrl,
    crc_chkr: CrcChkr,
    _reserved15: [u8; 0xd0],
    slib_set_pwd: SlibSetPwd,
    slib_set_range: SlibSetRange,
    em_slib_set: EmSlibSet,
    btm_mode_set: BtmModeSet,
    slib_unlock: SlibUnlock,
}
impl RegisterBlock {
    #[doc = "0x00 - Performance selection register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x04 - Unlock register"]
    #[inline(always)]
    pub const fn unlock(&self) -> &Unlock {
        &self.unlock
    }
    #[doc = "0x08 - USD unlock register"]
    #[inline(always)]
    pub const fn usd_unlock(&self) -> &UsdUnlock {
        &self.usd_unlock
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x10 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x14 - Address register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x1c - User system data register"]
    #[inline(always)]
    pub const fn usd(&self) -> &Usd {
        &self.usd
    }
    #[doc = "0x20 - Erase/program protection status register"]
    #[inline(always)]
    pub const fn epps(&self) -> &Epps {
        &self.epps
    }
    #[doc = "0x74 - sLib status 0 register"]
    #[inline(always)]
    pub const fn slib_sts0(&self) -> &SlibSts0 {
        &self.slib_sts0
    }
    #[doc = "0x78 - sLib status 1 register"]
    #[inline(always)]
    pub const fn slib_sts1(&self) -> &SlibSts1 {
        &self.slib_sts1
    }
    #[doc = "0x7c - SLIB password clear register"]
    #[inline(always)]
    pub const fn slib_pwd_clr(&self) -> &SlibPwdClr {
        &self.slib_pwd_clr
    }
    #[doc = "0x80 - sLib misc status register"]
    #[inline(always)]
    pub const fn slib_misc_sts(&self) -> &SlibMiscSts {
        &self.slib_misc_sts
    }
    #[doc = "0x84 - Flash CRC data start address register"]
    #[inline(always)]
    pub const fn crc_addr(&self) -> &CrcAddr {
        &self.crc_addr
    }
    #[doc = "0x88 - Flash CRC controll register"]
    #[inline(always)]
    pub const fn crc_ctrl(&self) -> &CrcCtrl {
        &self.crc_ctrl
    }
    #[doc = "0x8c - FLASH CRC check result register"]
    #[inline(always)]
    pub const fn crc_chkr(&self) -> &CrcChkr {
        &self.crc_chkr
    }
    #[doc = "0x160 - sLib password setting register"]
    #[inline(always)]
    pub const fn slib_set_pwd(&self) -> &SlibSetPwd {
        &self.slib_set_pwd
    }
    #[doc = "0x164 - Configure sLib range register"]
    #[inline(always)]
    pub const fn slib_set_range(&self) -> &SlibSetRange {
        &self.slib_set_range
    }
    #[doc = "0x168 - Extension momery slib set register"]
    #[inline(always)]
    pub const fn em_slib_set(&self) -> &EmSlibSet {
        &self.em_slib_set
    }
    #[doc = "0x16c - Boot memory mode setting register"]
    #[inline(always)]
    pub const fn btm_mode_set(&self) -> &BtmModeSet {
        &self.btm_mode_set
    }
    #[doc = "0x170 - sLib unlock register"]
    #[inline(always)]
    pub const fn slib_unlock(&self) -> &SlibUnlock {
        &self.slib_unlock
    }
}
#[doc = "PSR (rw) register accessor: Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Performance selection register"]
pub mod psr;
#[doc = "UNLOCK (w) register accessor: Unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock`]
module"]
#[doc(alias = "UNLOCK")]
pub type Unlock = crate::Reg<unlock::UnlockSpec>;
#[doc = "Unlock register"]
pub mod unlock;
#[doc = "USD_UNLOCK (w) register accessor: USD unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usd_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd_unlock`]
module"]
#[doc(alias = "USD_UNLOCK")]
pub type UsdUnlock = crate::Reg<usd_unlock::UsdUnlockSpec>;
#[doc = "USD unlock register"]
pub mod usd_unlock;
#[doc = "STS (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Status register"]
pub mod sts;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "ADDR (w) register accessor: Address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address register"]
pub mod addr;
#[doc = "USD (r) register accessor: User system data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usd`]
module"]
#[doc(alias = "USD")]
pub type Usd = crate::Reg<usd::UsdSpec>;
#[doc = "User system data register"]
pub mod usd;
#[doc = "EPPS (r) register accessor: Erase/program protection status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps`]
module"]
#[doc(alias = "EPPS")]
pub type Epps = crate::Reg<epps::EppsSpec>;
#[doc = "Erase/program protection status register"]
pub mod epps;
#[doc = "SLIB_STS0 (r) register accessor: sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts0`]
module"]
#[doc(alias = "SLIB_STS0")]
pub type SlibSts0 = crate::Reg<slib_sts0::SlibSts0Spec>;
#[doc = "sLib status 0 register"]
pub mod slib_sts0;
#[doc = "SLIB_STS1 (r) register accessor: sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts1`]
module"]
#[doc(alias = "SLIB_STS1")]
pub type SlibSts1 = crate::Reg<slib_sts1::SlibSts1Spec>;
#[doc = "sLib status 1 register"]
pub mod slib_sts1;
#[doc = "SLIB_PWD_CLR (w) register accessor: SLIB password clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_pwd_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_pwd_clr`]
module"]
#[doc(alias = "SLIB_PWD_CLR")]
pub type SlibPwdClr = crate::Reg<slib_pwd_clr::SlibPwdClrSpec>;
#[doc = "SLIB password clear register"]
pub mod slib_pwd_clr;
#[doc = "SLIB_MISC_STS (rw) register accessor: sLib misc status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_misc_sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_misc_sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_misc_sts`]
module"]
#[doc(alias = "SLIB_MISC_STS")]
pub type SlibMiscSts = crate::Reg<slib_misc_sts::SlibMiscStsSpec>;
#[doc = "sLib misc status register"]
pub mod slib_misc_sts;
#[doc = "CRC_ADDR (w) register accessor: Flash CRC data start address register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_addr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_addr`]
module"]
#[doc(alias = "CRC_ADDR")]
pub type CrcAddr = crate::Reg<crc_addr::CrcAddrSpec>;
#[doc = "Flash CRC data start address register"]
pub mod crc_addr;
#[doc = "CRC_CTRL (rw) register accessor: Flash CRC controll register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl`]
module"]
#[doc(alias = "CRC_CTRL")]
pub type CrcCtrl = crate::Reg<crc_ctrl::CrcCtrlSpec>;
#[doc = "Flash CRC controll register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: FLASH CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_chkr`]
module"]
#[doc(alias = "CRC_CHKR")]
pub type CrcChkr = crate::Reg<crc_chkr::CrcChkrSpec>;
#[doc = "FLASH CRC check result register"]
pub mod crc_chkr;
#[doc = "SLIB_SET_PWD (w) register accessor: sLib password setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_pwd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_pwd`]
module"]
#[doc(alias = "SLIB_SET_PWD")]
pub type SlibSetPwd = crate::Reg<slib_set_pwd::SlibSetPwdSpec>;
#[doc = "sLib password setting register"]
pub mod slib_set_pwd;
#[doc = "SLIB_SET_RANGE (w) register accessor: Configure sLib range register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range`]
module"]
#[doc(alias = "SLIB_SET_RANGE")]
pub type SlibSetRange = crate::Reg<slib_set_range::SlibSetRangeSpec>;
#[doc = "Configure sLib range register"]
pub mod slib_set_range;
#[doc = "EM_SLIB_SET (w) register accessor: Extension momery slib set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`em_slib_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em_slib_set`]
module"]
#[doc(alias = "EM_SLIB_SET")]
pub type EmSlibSet = crate::Reg<em_slib_set::EmSlibSetSpec>;
#[doc = "Extension momery slib set register"]
pub mod em_slib_set;
#[doc = "BTM_MODE_SET (w) register accessor: Boot memory mode setting register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btm_mode_set::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btm_mode_set`]
module"]
#[doc(alias = "BTM_MODE_SET")]
pub type BtmModeSet = crate::Reg<btm_mode_set::BtmModeSetSpec>;
#[doc = "Boot memory mode setting register"]
pub mod btm_mode_set;
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_unlock`]
module"]
#[doc(alias = "SLIB_UNLOCK")]
pub type SlibUnlock = crate::Reg<slib_unlock::SlibUnlockSpec>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
