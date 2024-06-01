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
    _reserved8: [u8; 0x60],
    unlock3: Unlock3,
    select: Select,
    sts3: Sts3,
    ctrl3: Ctrl3,
    addr3: Addr3,
    da: Da,
    _reserved14: [u8; 0x30],
    slib_sts0: SlibSts0,
    slib_sts1: SlibSts1,
    slib_pwd_clr: SlibPwdClr,
    slib_misc_sts: SlibMiscSts,
    slib_set_pwd: SlibSetPwd,
    slib_set_range: SlibSetRange,
    _reserved20: [u8; 0x0c],
    slib_unlock: SlibUnlock,
    crc_ctrl: CrcCtrl,
    crc_chkr: CrcChkr,
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
    #[doc = "0x84 - Unlock 3 register"]
    #[inline(always)]
    pub const fn unlock3(&self) -> &Unlock3 {
        &self.unlock3
    }
    #[doc = "0x88 - Select register"]
    #[inline(always)]
    pub const fn select(&self) -> &Select {
        &self.select
    }
    #[doc = "0x8c - Status 3 register"]
    #[inline(always)]
    pub const fn sts3(&self) -> &Sts3 {
        &self.sts3
    }
    #[doc = "0x90 - Control 3 register"]
    #[inline(always)]
    pub const fn ctrl3(&self) -> &Ctrl3 {
        &self.ctrl3
    }
    #[doc = "0x94 - Address 3 register"]
    #[inline(always)]
    pub const fn addr3(&self) -> &Addr3 {
        &self.addr3
    }
    #[doc = "0x98 - Spim decryption address"]
    #[inline(always)]
    pub const fn da(&self) -> &Da {
        &self.da
    }
    #[doc = "0xcc - sLib status 0 register"]
    #[inline(always)]
    pub const fn slib_sts0(&self) -> &SlibSts0 {
        &self.slib_sts0
    }
    #[doc = "0xd0 - sLib status 1 register"]
    #[inline(always)]
    pub const fn slib_sts1(&self) -> &SlibSts1 {
        &self.slib_sts1
    }
    #[doc = "0xd4 - SLIB password clear register"]
    #[inline(always)]
    pub const fn slib_pwd_clr(&self) -> &SlibPwdClr {
        &self.slib_pwd_clr
    }
    #[doc = "0xd8 - sLib misc status register"]
    #[inline(always)]
    pub const fn slib_misc_sts(&self) -> &SlibMiscSts {
        &self.slib_misc_sts
    }
    #[doc = "0xdc - sLib password setting register"]
    #[inline(always)]
    pub const fn slib_set_pwd(&self) -> &SlibSetPwd {
        &self.slib_set_pwd
    }
    #[doc = "0xe0 - Configure sLib range register"]
    #[inline(always)]
    pub const fn slib_set_range(&self) -> &SlibSetRange {
        &self.slib_set_range
    }
    #[doc = "0xf0 - sLib unlock register"]
    #[inline(always)]
    pub const fn slib_unlock(&self) -> &SlibUnlock {
        &self.slib_unlock
    }
    #[doc = "0xf4 - Flash CRC controler register"]
    #[inline(always)]
    pub const fn crc_ctrl(&self) -> &CrcCtrl {
        &self.crc_ctrl
    }
    #[doc = "0xf8 - FLASH CRC check result register"]
    #[inline(always)]
    pub const fn crc_chkr(&self) -> &CrcChkr {
        &self.crc_chkr
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
#[doc = "UNLOCK3 (w) register accessor: Unlock 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock3`]
module"]
#[doc(alias = "UNLOCK3")]
pub type Unlock3 = crate::Reg<unlock3::Unlock3Spec>;
#[doc = "Unlock 3 register"]
pub mod unlock3;
#[doc = "SELECT (w) register accessor: Select register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`select::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@select`]
module"]
#[doc(alias = "SELECT")]
pub type Select = crate::Reg<select::SelectSpec>;
#[doc = "Select register"]
pub mod select;
#[doc = "STS3 (rw) register accessor: Status 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts3`]
module"]
#[doc(alias = "STS3")]
pub type Sts3 = crate::Reg<sts3::Sts3Spec>;
#[doc = "Status 3 register"]
pub mod sts3;
#[doc = "CTRL3 (rw) register accessor: Control 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl3`]
module"]
#[doc(alias = "CTRL3")]
pub type Ctrl3 = crate::Reg<ctrl3::Ctrl3Spec>;
#[doc = "Control 3 register"]
pub mod ctrl3;
#[doc = "ADDR3 (w) register accessor: Address 3 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr3`]
module"]
#[doc(alias = "ADDR3")]
pub type Addr3 = crate::Reg<addr3::Addr3Spec>;
#[doc = "Address 3 register"]
pub mod addr3;
#[doc = "DA (w) register accessor: Spim decryption address\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`da::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@da`]
module"]
#[doc(alias = "DA")]
pub type Da = crate::Reg<da::DaSpec>;
#[doc = "Spim decryption address"]
pub mod da;
#[doc = "SLIB_STS0 (rw) register accessor: sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts0`]
module"]
#[doc(alias = "SLIB_STS0")]
pub type SlibSts0 = crate::Reg<slib_sts0::SlibSts0Spec>;
#[doc = "sLib status 0 register"]
pub mod slib_sts0;
#[doc = "SLIB_STS1 (rw) register accessor: sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts1`]
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
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_unlock`]
module"]
#[doc(alias = "SLIB_UNLOCK")]
pub type SlibUnlock = crate::Reg<slib_unlock::SlibUnlockSpec>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
#[doc = "CRC_CTRL (w) register accessor: Flash CRC controler register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl`]
module"]
#[doc(alias = "CRC_CTRL")]
pub type CrcCtrl = crate::Reg<crc_ctrl::CrcCtrlSpec>;
#[doc = "Flash CRC controler register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: FLASH CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_chkr`]
module"]
#[doc(alias = "CRC_CHKR")]
pub type CrcChkr = crate::Reg<crc_chkr::CrcChkrSpec>;
#[doc = "FLASH CRC check result register"]
pub mod crc_chkr;
