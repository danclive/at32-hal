#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfglr: Cfglr,
    cfghr: Cfghr,
    idt: Idt,
    odt: Odt,
    scr: Scr,
    clr: Clr,
    wpr: Wpr,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO function configurate low register"]
    #[inline(always)]
    pub const fn cfglr(&self) -> &Cfglr {
        &self.cfglr
    }
    #[doc = "0x04 - GPIO function configurate high register"]
    #[inline(always)]
    pub const fn cfghr(&self) -> &Cfghr {
        &self.cfghr
    }
    #[doc = "0x08 - Port input data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &Idt {
        &self.idt
    }
    #[doc = "0x0c - Port output data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &Odt {
        &self.odt
    }
    #[doc = "0x10 - Port bit set/clear register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x14 - Port bit reset register"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x18 - Port write protect register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &Wpr {
        &self.wpr
    }
}
#[doc = "CFGLR (rw) register accessor: GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfglr`]
module"]
#[doc(alias = "CFGLR")]
pub type Cfglr = crate::Reg<cfglr::CfglrSpec>;
#[doc = "GPIO function configurate low register"]
pub mod cfglr;
#[doc = "CFGHR (rw) register accessor: GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfghr`]
module"]
#[doc(alias = "CFGHR")]
pub type Cfghr = crate::Reg<cfghr::CfghrSpec>;
#[doc = "GPIO function configurate high register"]
pub mod cfghr;
#[doc = "IDT (r) register accessor: Port input data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`]
module"]
#[doc(alias = "IDT")]
pub type Idt = crate::Reg<idt::IdtSpec>;
#[doc = "Port input data register"]
pub mod idt;
#[doc = "ODT (rw) register accessor: Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`]
module"]
#[doc(alias = "ODT")]
pub type Odt = crate::Reg<odt::OdtSpec>;
#[doc = "Port output data register"]
pub mod odt;
#[doc = "SCR (rw) register accessor: Port bit set/clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Port bit set/clear register"]
pub mod scr;
#[doc = "CLR (rw) register accessor: Port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "Port bit reset register"]
pub mod clr;
#[doc = "WPR (rw) register accessor: Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
#[doc(alias = "WPR")]
pub type Wpr = crate::Reg<wpr::WprSpec>;
#[doc = "Port write protect register"]
pub mod wpr;
