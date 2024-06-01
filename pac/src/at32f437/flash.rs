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
    epps0: Epps0,
    _reserved8: [u8; 0x08],
    epps1: Epps1,
    _reserved9: [u8; 0x14],
    unlock2: Unlock2,
    _reserved10: [u8; 0x04],
    sts2: Sts2,
    ctrl2: Ctrl2,
    addr2: Addr2,
    contr: Contr,
    _reserved14: [u8; 0x04],
    divr: Divr,
    _reserved15: [u8; 0x64],
    slib_sts2: SlibSts2,
    slib_sts0: SlibSts0,
    slib_sts1: SlibSts1,
    slib_pwd_clr: SlibPwdClr,
    slib_misc_sts: SlibMiscSts,
    slib_set_pwd: SlibSetPwd,
    slib_set_range0: SlibSetRange0,
    slib_set_range1: SlibSetRange1,
    _reserved23: [u8; 0x08],
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
    #[doc = "0x20 - Erase/program protection status register 0"]
    #[inline(always)]
    pub const fn epps0(&self) -> &Epps0 {
        &self.epps0
    }
    #[doc = "0x2c - Erase/program protection status register 1"]
    #[inline(always)]
    pub const fn epps1(&self) -> &Epps1 {
        &self.epps1
    }
    #[doc = "0x44 - Unlock 2 register"]
    #[inline(always)]
    pub const fn unlock2(&self) -> &Unlock2 {
        &self.unlock2
    }
    #[doc = "0x4c - Status 2 register"]
    #[inline(always)]
    pub const fn sts2(&self) -> &Sts2 {
        &self.sts2
    }
    #[doc = "0x50 - Control 2 register"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x54 - Address 2 register"]
    #[inline(always)]
    pub const fn addr2(&self) -> &Addr2 {
        &self.addr2
    }
    #[doc = "0x58 - Flash continue read register"]
    #[inline(always)]
    pub const fn contr(&self) -> &Contr {
        &self.contr
    }
    #[doc = "0x60 - Flash divider register"]
    #[inline(always)]
    pub const fn divr(&self) -> &Divr {
        &self.divr
    }
    #[doc = "0xc8 - sLib status 2 register"]
    #[inline(always)]
    pub const fn slib_sts2(&self) -> &SlibSts2 {
        &self.slib_sts2
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
    #[doc = "0xe0 - Configure sLib range register 0"]
    #[inline(always)]
    pub const fn slib_set_range0(&self) -> &SlibSetRange0 {
        &self.slib_set_range0
    }
    #[doc = "0xe4 - Configure sLib range register 1"]
    #[inline(always)]
    pub const fn slib_set_range1(&self) -> &SlibSetRange1 {
        &self.slib_set_range1
    }
    #[doc = "0xf0 - sLib unlock register"]
    #[inline(always)]
    pub const fn slib_unlock(&self) -> &SlibUnlock {
        &self.slib_unlock
    }
    #[doc = "0xf4 - CRC controler register"]
    #[inline(always)]
    pub const fn crc_ctrl(&self) -> &CrcCtrl {
        &self.crc_ctrl
    }
    #[doc = "0xf8 - CRC check result register"]
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
#[doc = "EPPS0 (r) register accessor: Erase/program protection status register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps0`]
module"]
#[doc(alias = "EPPS0")]
pub type Epps0 = crate::Reg<epps0::Epps0Spec>;
#[doc = "Erase/program protection status register 0"]
pub mod epps0;
#[doc = "EPPS1 (r) register accessor: Erase/program protection status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epps1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epps1`]
module"]
#[doc(alias = "EPPS1")]
pub type Epps1 = crate::Reg<epps1::Epps1Spec>;
#[doc = "Erase/program protection status register 1"]
pub mod epps1;
#[doc = "UNLOCK2 (w) register accessor: Unlock 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unlock2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@unlock2`]
module"]
#[doc(alias = "UNLOCK2")]
pub type Unlock2 = crate::Reg<unlock2::Unlock2Spec>;
#[doc = "Unlock 2 register"]
pub mod unlock2;
#[doc = "STS2 (rw) register accessor: Status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`]
module"]
#[doc(alias = "STS2")]
pub type Sts2 = crate::Reg<sts2::Sts2Spec>;
#[doc = "Status 2 register"]
pub mod sts2;
#[doc = "CTRL2 (rw) register accessor: Control 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "Control 2 register"]
pub mod ctrl2;
#[doc = "ADDR2 (w) register accessor: Address 2 register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr2`]
module"]
#[doc(alias = "ADDR2")]
pub type Addr2 = crate::Reg<addr2::Addr2Spec>;
#[doc = "Address 2 register"]
pub mod addr2;
#[doc = "CONTR (rw) register accessor: Flash continue read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`contr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`contr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@contr`]
module"]
#[doc(alias = "CONTR")]
pub type Contr = crate::Reg<contr::ContrSpec>;
#[doc = "Flash continue read register"]
pub mod contr;
#[doc = "DIVR (rw) register accessor: Flash divider register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divr`]
module"]
#[doc(alias = "DIVR")]
pub type Divr = crate::Reg<divr::DivrSpec>;
#[doc = "Flash divider register"]
pub mod divr;
#[doc = "SLIB_STS2 (rw) register accessor: sLib status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_sts2`]
module"]
#[doc(alias = "SLIB_STS2")]
pub type SlibSts2 = crate::Reg<slib_sts2::SlibSts2Spec>;
#[doc = "sLib status 2 register"]
pub mod slib_sts2;
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
#[doc = "SLIB_SET_RANGE0 (w) register accessor: Configure sLib range register 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range0`]
module"]
#[doc(alias = "SLIB_SET_RANGE0")]
pub type SlibSetRange0 = crate::Reg<slib_set_range0::SlibSetRange0Spec>;
#[doc = "Configure sLib range register 0"]
pub mod slib_set_range0;
#[doc = "SLIB_SET_RANGE1 (w) register accessor: Configure sLib range register 1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_set_range1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_set_range1`]
module"]
#[doc(alias = "SLIB_SET_RANGE1")]
pub type SlibSetRange1 = crate::Reg<slib_set_range1::SlibSetRange1Spec>;
#[doc = "Configure sLib range register 1"]
pub mod slib_set_range1;
#[doc = "SLIB_UNLOCK (w) register accessor: sLib unlock register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_unlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slib_unlock`]
module"]
#[doc(alias = "SLIB_UNLOCK")]
pub type SlibUnlock = crate::Reg<slib_unlock::SlibUnlockSpec>;
#[doc = "sLib unlock register"]
pub mod slib_unlock;
#[doc = "CRC_CTRL (w) register accessor: CRC controler register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`crc_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_ctrl`]
module"]
#[doc(alias = "CRC_CTRL")]
pub type CrcCtrl = crate::Reg<crc_ctrl::CrcCtrlSpec>;
#[doc = "CRC controler register"]
pub mod crc_ctrl;
#[doc = "CRC_CHKR (r) register accessor: CRC check result register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`crc_chkr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crc_chkr`]
module"]
#[doc(alias = "CRC_CHKR")]
pub type CrcChkr = crate::Reg<crc_chkr::CrcChkrSpec>;
#[doc = "CRC check result register"]
pub mod crc_chkr;
