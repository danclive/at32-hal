#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmd: Cmd,
    div: Div,
    rld: Rld,
    sts: Sts,
    win: Win,
}
impl RegisterBlock {
    #[doc = "0x00 - Command register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x04 - Division register"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0x08 - Reload register"]
    #[inline(always)]
    pub const fn rld(&self) -> &Rld {
        &self.rld
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x10 - Window register"]
    #[inline(always)]
    pub const fn win(&self) -> &Win {
        &self.win
    }
}
#[doc = "CMD (w) register accessor: Command register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command register"]
pub mod cmd;
#[doc = "DIV (rw) register accessor: Division register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`]
module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "Division register"]
pub mod div;
#[doc = "RLD (rw) register accessor: Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rld::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rld::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rld`]
module"]
#[doc(alias = "RLD")]
pub type Rld = crate::Reg<rld::RldSpec>;
#[doc = "Reload register"]
pub mod rld;
#[doc = "STS (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Status register"]
pub mod sts;
#[doc = "WIN (rw) register accessor: Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@win`]
module"]
#[doc(alias = "WIN")]
pub type Win = crate::Reg<win::WinSpec>;
#[doc = "Window register"]
pub mod win;
