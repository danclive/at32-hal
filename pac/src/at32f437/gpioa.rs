#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfgr: Cfgr,
    omode: Omode,
    odrvr: Odrvr,
    pull: Pull,
    idt: Idt,
    odt: Odt,
    scr: Scr,
    wpr: Wpr,
    muxl: Muxl,
    muxh: Muxh,
    clr: Clr,
    _reserved11: [u8; 0x10],
    hdrv: Hdrv,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x04 - GPIO output mode register"]
    #[inline(always)]
    pub const fn omode(&self) -> &Omode {
        &self.omode
    }
    #[doc = "0x08 - GPIO drive capability register"]
    #[inline(always)]
    pub const fn odrvr(&self) -> &Odrvr {
        &self.odrvr
    }
    #[doc = "0x0c - GPIO pull-up/pull-down register"]
    #[inline(always)]
    pub const fn pull(&self) -> &Pull {
        &self.pull
    }
    #[doc = "0x10 - GPIO input data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &Idt {
        &self.idt
    }
    #[doc = "0x14 - GPIO output data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &Odt {
        &self.odt
    }
    #[doc = "0x18 - Port bit set/clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x1c - Port write protect register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &Wpr {
        &self.wpr
    }
    #[doc = "0x20 - GPIO muxing function low register"]
    #[inline(always)]
    pub const fn muxl(&self) -> &Muxl {
        &self.muxl
    }
    #[doc = "0x24 - GPIO muxing function high register"]
    #[inline(always)]
    pub const fn muxh(&self) -> &Muxh {
        &self.muxh
    }
    #[doc = "0x28 - GPIO bit reset register"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x3c - Huge current driver"]
    #[inline(always)]
    pub const fn hdrv(&self) -> &Hdrv {
        &self.hdrv
    }
}
#[doc = "CFGR (rw) register accessor: GPIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "GPIO configuration register"]
pub mod cfgr;
#[doc = "OMODE (rw) register accessor: GPIO output mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`omode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`omode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omode`]
module"]
#[doc(alias = "OMODE")]
pub type Omode = crate::Reg<omode::OmodeSpec>;
#[doc = "GPIO output mode register"]
pub mod omode;
#[doc = "ODRVR (rw) register accessor: GPIO drive capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odrvr`]
module"]
#[doc(alias = "ODRVR")]
pub type Odrvr = crate::Reg<odrvr::OdrvrSpec>;
#[doc = "GPIO drive capability register"]
pub mod odrvr;
#[doc = "PULL (rw) register accessor: GPIO pull-up/pull-down register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pull::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pull::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pull`]
module"]
#[doc(alias = "PULL")]
pub type Pull = crate::Reg<pull::PullSpec>;
#[doc = "GPIO pull-up/pull-down register"]
pub mod pull;
#[doc = "IDT (r) register accessor: GPIO input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`]
module"]
#[doc(alias = "IDT")]
pub type Idt = crate::Reg<idt::IdtSpec>;
#[doc = "GPIO input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: GPIO output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`]
module"]
#[doc(alias = "ODT")]
pub type Odt = crate::Reg<odt::OdtSpec>;
#[doc = "GPIO output data register"]
pub mod odt;
#[doc = "SCR (w) register accessor: Port bit set/clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
#[doc(alias = "WPR")]
pub type Wpr = crate::Reg<wpr::WprSpec>;
#[doc = "Port write protect register"]
pub mod wpr;
#[doc = "MUXL (rw) register accessor: GPIO muxing function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxl`]
module"]
#[doc(alias = "MUXL")]
pub type Muxl = crate::Reg<muxl::MuxlSpec>;
#[doc = "GPIO muxing function low register"]
pub mod muxl;
#[doc = "MUXH (rw) register accessor: GPIO muxing function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxh`]
module"]
#[doc(alias = "MUXH")]
pub type Muxh = crate::Reg<muxh::MuxhSpec>;
#[doc = "GPIO muxing function high register"]
pub mod muxh;
#[doc = "CLR (w) register accessor: GPIO bit reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "GPIO bit reset register"]
pub mod clr;
#[doc = "HDRV (rw) register accessor: Huge current driver\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdrv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdrv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdrv`]
module"]
#[doc(alias = "HDRV")]
pub type Hdrv = crate::Reg<hdrv::HdrvSpec>;
#[doc = "Huge current driver"]
pub mod hdrv;
