#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: Sts,
    clr: Clr,
    c1ctrl: C1ctrl,
    c1dtcnt: C1dtcnt,
    c1paddr: C1paddr,
    c1maddr: C1maddr,
    _reserved6: [u8; 0x04],
    c2ctrl: C2ctrl,
    c2dtcnt: C2dtcnt,
    c2paddr: C2paddr,
    c2maddr: C2maddr,
    _reserved10: [u8; 0x04],
    c3ctrl: C3ctrl,
    c3dtcnt: C3dtcnt,
    c3paddr: C3paddr,
    c3maddr: C3maddr,
    _reserved14: [u8; 0x04],
    c4ctrl: C4ctrl,
    c4dtcnt: C4dtcnt,
    c4paddr: C4paddr,
    c4maddr: C4maddr,
    _reserved18: [u8; 0x04],
    c5ctrl: C5ctrl,
    c5dtcnt: C5dtcnt,
    c5paddr: C5paddr,
    c5maddr: C5maddr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA status register (DMA_STS)"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - DMA flag clear register (DMA_CLR)"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x08 - DMA channel configuration register"]
    #[inline(always)]
    pub const fn c1ctrl(&self) -> &C1ctrl {
        &self.c1ctrl
    }
    #[doc = "0x0c - DMA channel 1 number of data to transfer register"]
    #[inline(always)]
    pub const fn c1dtcnt(&self) -> &C1dtcnt {
        &self.c1dtcnt
    }
    #[doc = "0x10 - DMA channel 1 peripheral base address register"]
    #[inline(always)]
    pub const fn c1paddr(&self) -> &C1paddr {
        &self.c1paddr
    }
    #[doc = "0x14 - DMA channel 1 memory base address register"]
    #[inline(always)]
    pub const fn c1maddr(&self) -> &C1maddr {
        &self.c1maddr
    }
    #[doc = "0x1c - DMA channel configuration register"]
    #[inline(always)]
    pub const fn c2ctrl(&self) -> &C2ctrl {
        &self.c2ctrl
    }
    #[doc = "0x20 - DMA channel 2 number of data to transfer register"]
    #[inline(always)]
    pub const fn c2dtcnt(&self) -> &C2dtcnt {
        &self.c2dtcnt
    }
    #[doc = "0x24 - DMA channel 2 peripheral base address register"]
    #[inline(always)]
    pub const fn c2paddr(&self) -> &C2paddr {
        &self.c2paddr
    }
    #[doc = "0x28 - DMA channel 2 memory base address register"]
    #[inline(always)]
    pub const fn c2maddr(&self) -> &C2maddr {
        &self.c2maddr
    }
    #[doc = "0x30 - DMA channel configuration register"]
    #[inline(always)]
    pub const fn c3ctrl(&self) -> &C3ctrl {
        &self.c3ctrl
    }
    #[doc = "0x34 - DMA channel 3 number of data to transfer register"]
    #[inline(always)]
    pub const fn c3dtcnt(&self) -> &C3dtcnt {
        &self.c3dtcnt
    }
    #[doc = "0x38 - DMA channel 3 peripheral base address register"]
    #[inline(always)]
    pub const fn c3paddr(&self) -> &C3paddr {
        &self.c3paddr
    }
    #[doc = "0x3c - DMA channel 3 memory base address register"]
    #[inline(always)]
    pub const fn c3maddr(&self) -> &C3maddr {
        &self.c3maddr
    }
    #[doc = "0x44 - DMA channel configuration register"]
    #[inline(always)]
    pub const fn c4ctrl(&self) -> &C4ctrl {
        &self.c4ctrl
    }
    #[doc = "0x48 - DMA channel 4 number of data to transfer register"]
    #[inline(always)]
    pub const fn c4dtcnt(&self) -> &C4dtcnt {
        &self.c4dtcnt
    }
    #[doc = "0x4c - DMA channel 4 peripheral base address register"]
    #[inline(always)]
    pub const fn c4paddr(&self) -> &C4paddr {
        &self.c4paddr
    }
    #[doc = "0x50 - DMA channel 4 memory base address register"]
    #[inline(always)]
    pub const fn c4maddr(&self) -> &C4maddr {
        &self.c4maddr
    }
    #[doc = "0x58 - DMA channel configuration register"]
    #[inline(always)]
    pub const fn c5ctrl(&self) -> &C5ctrl {
        &self.c5ctrl
    }
    #[doc = "0x5c - DMA channel 5 number of data to transfer register"]
    #[inline(always)]
    pub const fn c5dtcnt(&self) -> &C5dtcnt {
        &self.c5dtcnt
    }
    #[doc = "0x60 - DMA channel 5 peripheral base address register"]
    #[inline(always)]
    pub const fn c5paddr(&self) -> &C5paddr {
        &self.c5paddr
    }
    #[doc = "0x64 - DMA channel 5 memory base address register"]
    #[inline(always)]
    pub const fn c5maddr(&self) -> &C5maddr {
        &self.c5maddr
    }
}
#[doc = "STS (r) register accessor: DMA status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "DMA status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA flag clear register (DMA_CLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "DMA flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "C1CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1ctrl`]
module"]
#[doc(alias = "C1CTRL")]
pub type C1ctrl = crate::Reg<c1ctrl::C1ctrlSpec>;
#[doc = "DMA channel configuration register"]
pub mod c1ctrl;
#[doc = "C1DTCNT (rw) register accessor: DMA channel 1 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1dtcnt`]
module"]
#[doc(alias = "C1DTCNT")]
pub type C1dtcnt = crate::Reg<c1dtcnt::C1dtcntSpec>;
#[doc = "DMA channel 1 number of data to transfer register"]
pub mod c1dtcnt;
#[doc = "C1PADDR (rw) register accessor: DMA channel 1 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1paddr`]
module"]
#[doc(alias = "C1PADDR")]
pub type C1paddr = crate::Reg<c1paddr::C1paddrSpec>;
#[doc = "DMA channel 1 peripheral base address register"]
pub mod c1paddr;
#[doc = "C1MADDR (rw) register accessor: DMA channel 1 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1maddr`]
module"]
#[doc(alias = "C1MADDR")]
pub type C1maddr = crate::Reg<c1maddr::C1maddrSpec>;
#[doc = "DMA channel 1 memory base address register"]
pub mod c1maddr;
#[doc = "C2CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2ctrl`]
module"]
#[doc(alias = "C2CTRL")]
pub type C2ctrl = crate::Reg<c2ctrl::C2ctrlSpec>;
#[doc = "DMA channel configuration register"]
pub mod c2ctrl;
#[doc = "C2DTCNT (rw) register accessor: DMA channel 2 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2dtcnt`]
module"]
#[doc(alias = "C2DTCNT")]
pub type C2dtcnt = crate::Reg<c2dtcnt::C2dtcntSpec>;
#[doc = "DMA channel 2 number of data to transfer register"]
pub mod c2dtcnt;
#[doc = "C2PADDR (rw) register accessor: DMA channel 2 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2paddr`]
module"]
#[doc(alias = "C2PADDR")]
pub type C2paddr = crate::Reg<c2paddr::C2paddrSpec>;
#[doc = "DMA channel 2 peripheral base address register"]
pub mod c2paddr;
#[doc = "C2MADDR (rw) register accessor: DMA channel 2 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2maddr`]
module"]
#[doc(alias = "C2MADDR")]
pub type C2maddr = crate::Reg<c2maddr::C2maddrSpec>;
#[doc = "DMA channel 2 memory base address register"]
pub mod c2maddr;
#[doc = "C3CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3ctrl`]
module"]
#[doc(alias = "C3CTRL")]
pub type C3ctrl = crate::Reg<c3ctrl::C3ctrlSpec>;
#[doc = "DMA channel configuration register"]
pub mod c3ctrl;
#[doc = "C3DTCNT (rw) register accessor: DMA channel 3 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3dtcnt`]
module"]
#[doc(alias = "C3DTCNT")]
pub type C3dtcnt = crate::Reg<c3dtcnt::C3dtcntSpec>;
#[doc = "DMA channel 3 number of data to transfer register"]
pub mod c3dtcnt;
#[doc = "C3PADDR (rw) register accessor: DMA channel 3 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3paddr`]
module"]
#[doc(alias = "C3PADDR")]
pub type C3paddr = crate::Reg<c3paddr::C3paddrSpec>;
#[doc = "DMA channel 3 peripheral base address register"]
pub mod c3paddr;
#[doc = "C3MADDR (rw) register accessor: DMA channel 3 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3maddr`]
module"]
#[doc(alias = "C3MADDR")]
pub type C3maddr = crate::Reg<c3maddr::C3maddrSpec>;
#[doc = "DMA channel 3 memory base address register"]
pub mod c3maddr;
#[doc = "C4CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4ctrl`]
module"]
#[doc(alias = "C4CTRL")]
pub type C4ctrl = crate::Reg<c4ctrl::C4ctrlSpec>;
#[doc = "DMA channel configuration register"]
pub mod c4ctrl;
#[doc = "C4DTCNT (rw) register accessor: DMA channel 4 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4dtcnt`]
module"]
#[doc(alias = "C4DTCNT")]
pub type C4dtcnt = crate::Reg<c4dtcnt::C4dtcntSpec>;
#[doc = "DMA channel 4 number of data to transfer register"]
pub mod c4dtcnt;
#[doc = "C4PADDR (rw) register accessor: DMA channel 4 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4paddr`]
module"]
#[doc(alias = "C4PADDR")]
pub type C4paddr = crate::Reg<c4paddr::C4paddrSpec>;
#[doc = "DMA channel 4 peripheral base address register"]
pub mod c4paddr;
#[doc = "C4MADDR (rw) register accessor: DMA channel 4 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4maddr`]
module"]
#[doc(alias = "C4MADDR")]
pub type C4maddr = crate::Reg<c4maddr::C4maddrSpec>;
#[doc = "DMA channel 4 memory base address register"]
pub mod c4maddr;
#[doc = "C5CTRL (rw) register accessor: DMA channel configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5ctrl`]
module"]
#[doc(alias = "C5CTRL")]
pub type C5ctrl = crate::Reg<c5ctrl::C5ctrlSpec>;
#[doc = "DMA channel configuration register"]
pub mod c5ctrl;
#[doc = "C5DTCNT (rw) register accessor: DMA channel 5 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5dtcnt`]
module"]
#[doc(alias = "C5DTCNT")]
pub type C5dtcnt = crate::Reg<c5dtcnt::C5dtcntSpec>;
#[doc = "DMA channel 5 number of data to transfer register"]
pub mod c5dtcnt;
#[doc = "C5PADDR (rw) register accessor: DMA channel 5 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5paddr`]
module"]
#[doc(alias = "C5PADDR")]
pub type C5paddr = crate::Reg<c5paddr::C5paddrSpec>;
#[doc = "DMA channel 5 peripheral base address register"]
pub mod c5paddr;
#[doc = "C5MADDR (rw) register accessor: DMA channel 5 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5maddr`]
module"]
#[doc(alias = "C5MADDR")]
pub type C5maddr = crate::Reg<c5maddr::C5maddrSpec>;
#[doc = "DMA channel 5 memory base address register"]
pub mod c5maddr;
