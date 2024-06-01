#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dt: Dt,
    cdt: Cdt,
    ctrl: Ctrl,
    _reserved3: [u8; 0x04],
    idt: Idt,
    poly: Poly,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub const fn dt(&self) -> &Dt {
        &self.dt
    }
    #[doc = "0x04 - Common data register"]
    #[inline(always)]
    pub const fn cdt(&self) -> &Cdt {
        &self.cdt
    }
    #[doc = "0x08 - Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Initial data register"]
    #[inline(always)]
    pub const fn idt(&self) -> &Idt {
        &self.idt
    }
    #[doc = "0x14 - Polynomial coefficient register"]
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
}
#[doc = "DT (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt`]
module"]
#[doc(alias = "DT")]
pub type Dt = crate::Reg<dt::DtSpec>;
#[doc = "Data register"]
pub mod dt;
#[doc = "CDT (rw) register accessor: Common data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cdt`]
module"]
#[doc(alias = "CDT")]
pub type Cdt = crate::Reg<cdt::CdtSpec>;
#[doc = "Common data register"]
pub mod cdt;
#[doc = "CTRL (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control register"]
pub mod ctrl;
#[doc = "IDT (rw) register accessor: Initial data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idt`]
module"]
#[doc(alias = "IDT")]
pub type Idt = crate::Reg<idt::IdtSpec>;
#[doc = "Initial data register"]
pub mod idt;
#[doc = "POLY (rw) register accessor: Polynomial coefficient register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`poly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`]
module"]
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::PolySpec>;
#[doc = "Polynomial coefficient register"]
pub mod poly;
