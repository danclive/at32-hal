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
    _reserved22: [u8; 0x04],
    c6ctrl: C6ctrl,
    c6dtcnt: C6dtcnt,
    c6paddr: C6paddr,
    c6maddr: C6maddr,
    _reserved26: [u8; 0x04],
    c7ctrl: C7ctrl,
    c7dtcnt: C7dtcnt,
    c7paddr: C7paddr,
    c7maddr: C7maddr,
    _reserved30: [u8; 0x70],
    dma_muxsel: DmaMuxsel,
    muxc1ctrl: Muxc1ctrl,
    muxc2ctrl: Muxc2ctrl,
    muxc3ctrl: Muxc3ctrl,
    muxc4ctrl: Muxc4ctrl,
    muxc5ctrl: Muxc5ctrl,
    muxc6ctrl: Muxc6ctrl,
    muxc7ctrl: Muxc7ctrl,
    muxg1ctrl: Muxg1ctrl,
    muxg2ctrl: Muxg2ctrl,
    muxg3ctrl: Muxg3ctrl,
    muxg4ctrl: Muxg4ctrl,
    muxsyncsts: Muxsyncsts,
    muxsyncclr: Muxsyncclr,
    muxgsts: Muxgsts,
    muxgclr: Muxgclr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_STS)"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_CLR)"]
    #[inline(always)]
    pub const fn clr(&self) -> &Clr {
        &self.clr
    }
    #[doc = "0x08 - DMA channel configuration register(DMA_C1CTRL)"]
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
    #[doc = "0x1c - DMA channel configuration register (DMA_C2CTRL)"]
    #[inline(always)]
    pub const fn c2ctrl(&self) -> &C2ctrl {
        &self.c2ctrl
    }
    #[doc = "0x20 - DMA channel 2 number of data to transferregister"]
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
    #[doc = "0x30 - DMA channel configuration register (DMA_C3CTRL)"]
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
    #[doc = "0x44 - DMA channel configuration register (DMA_C4CTRL)"]
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
    #[doc = "0x58 - DMA channel configuration register (DMA_C5CTRL)"]
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
    #[doc = "0x6c - DMA channel configuration register(DMA_C6CTRL)"]
    #[inline(always)]
    pub const fn c6ctrl(&self) -> &C6ctrl {
        &self.c6ctrl
    }
    #[doc = "0x70 - DMA channel 6 number of data to transfer register"]
    #[inline(always)]
    pub const fn c6dtcnt(&self) -> &C6dtcnt {
        &self.c6dtcnt
    }
    #[doc = "0x74 - DMA channel 6 peripheral address base register"]
    #[inline(always)]
    pub const fn c6paddr(&self) -> &C6paddr {
        &self.c6paddr
    }
    #[doc = "0x78 - DMA channel 6 memory address base register"]
    #[inline(always)]
    pub const fn c6maddr(&self) -> &C6maddr {
        &self.c6maddr
    }
    #[doc = "0x80 - DMA channel configuration register(DMA_C7CTRL)"]
    #[inline(always)]
    pub const fn c7ctrl(&self) -> &C7ctrl {
        &self.c7ctrl
    }
    #[doc = "0x84 - DMA channel 7 number of data to transfer register"]
    #[inline(always)]
    pub const fn c7dtcnt(&self) -> &C7dtcnt {
        &self.c7dtcnt
    }
    #[doc = "0x88 - DMA channel 7 peripheral base address register"]
    #[inline(always)]
    pub const fn c7paddr(&self) -> &C7paddr {
        &self.c7paddr
    }
    #[doc = "0x8c - DMA channel 7 memory base address register"]
    #[inline(always)]
    pub const fn c7maddr(&self) -> &C7maddr {
        &self.c7maddr
    }
    #[doc = "0x100 - DMAMUX Table Selection"]
    #[inline(always)]
    pub const fn dma_muxsel(&self) -> &DmaMuxsel {
        &self.dma_muxsel
    }
    #[doc = "0x104 - Channel 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxc1ctrl(&self) -> &Muxc1ctrl {
        &self.muxc1ctrl
    }
    #[doc = "0x108 - Channel 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxc2ctrl(&self) -> &Muxc2ctrl {
        &self.muxc2ctrl
    }
    #[doc = "0x10c - Channel 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxc3ctrl(&self) -> &Muxc3ctrl {
        &self.muxc3ctrl
    }
    #[doc = "0x110 - Channel 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxc4ctrl(&self) -> &Muxc4ctrl {
        &self.muxc4ctrl
    }
    #[doc = "0x114 - Channel 5 Configuration Register"]
    #[inline(always)]
    pub const fn muxc5ctrl(&self) -> &Muxc5ctrl {
        &self.muxc5ctrl
    }
    #[doc = "0x118 - Channel 6 Configuration Register"]
    #[inline(always)]
    pub const fn muxc6ctrl(&self) -> &Muxc6ctrl {
        &self.muxc6ctrl
    }
    #[doc = "0x11c - Channel 7 Configuration Register"]
    #[inline(always)]
    pub const fn muxc7ctrl(&self) -> &Muxc7ctrl {
        &self.muxc7ctrl
    }
    #[doc = "0x120 - Generator 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxg1ctrl(&self) -> &Muxg1ctrl {
        &self.muxg1ctrl
    }
    #[doc = "0x124 - Generator 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxg2ctrl(&self) -> &Muxg2ctrl {
        &self.muxg2ctrl
    }
    #[doc = "0x128 - Generator 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxg3ctrl(&self) -> &Muxg3ctrl {
        &self.muxg3ctrl
    }
    #[doc = "0x12c - Generator 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxg4ctrl(&self) -> &Muxg4ctrl {
        &self.muxg4ctrl
    }
    #[doc = "0x130 - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxsyncsts(&self) -> &Muxsyncsts {
        &self.muxsyncsts
    }
    #[doc = "0x134 - Channel Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxsyncclr(&self) -> &Muxsyncclr {
        &self.muxsyncclr
    }
    #[doc = "0x138 - Generator Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxgsts(&self) -> &Muxgsts {
        &self.muxgsts
    }
    #[doc = "0x13c - Generator Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxgclr(&self) -> &Muxgclr {
        &self.muxgclr
    }
}
#[doc = "STS (r) register accessor: DMA interrupt status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "DMA interrupt status register (DMA_STS)"]
pub mod sts;
#[doc = "CLR (rw) register accessor: DMA interrupt flag clear register (DMA_CLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr`]
module"]
#[doc(alias = "CLR")]
pub type Clr = crate::Reg<clr::ClrSpec>;
#[doc = "DMA interrupt flag clear register (DMA_CLR)"]
pub mod clr;
#[doc = "C1CTRL (rw) register accessor: DMA channel configuration register(DMA_C1CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1ctrl`]
module"]
#[doc(alias = "C1CTRL")]
pub type C1ctrl = crate::Reg<c1ctrl::C1ctrlSpec>;
#[doc = "DMA channel configuration register(DMA_C1CTRL)"]
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
#[doc = "C2CTRL (rw) register accessor: DMA channel configuration register (DMA_C2CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2ctrl`]
module"]
#[doc(alias = "C2CTRL")]
pub type C2ctrl = crate::Reg<c2ctrl::C2ctrlSpec>;
#[doc = "DMA channel configuration register (DMA_C2CTRL)"]
pub mod c2ctrl;
#[doc = "C2DTCNT (rw) register accessor: DMA channel 2 number of data to transferregister\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2dtcnt`]
module"]
#[doc(alias = "C2DTCNT")]
pub type C2dtcnt = crate::Reg<c2dtcnt::C2dtcntSpec>;
#[doc = "DMA channel 2 number of data to transferregister"]
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
#[doc = "C3CTRL (rw) register accessor: DMA channel configuration register (DMA_C3CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3ctrl`]
module"]
#[doc(alias = "C3CTRL")]
pub type C3ctrl = crate::Reg<c3ctrl::C3ctrlSpec>;
#[doc = "DMA channel configuration register (DMA_C3CTRL)"]
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
#[doc = "C4CTRL (rw) register accessor: DMA channel configuration register (DMA_C4CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4ctrl`]
module"]
#[doc(alias = "C4CTRL")]
pub type C4ctrl = crate::Reg<c4ctrl::C4ctrlSpec>;
#[doc = "DMA channel configuration register (DMA_C4CTRL)"]
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
#[doc = "C5CTRL (rw) register accessor: DMA channel configuration register (DMA_C5CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5ctrl`]
module"]
#[doc(alias = "C5CTRL")]
pub type C5ctrl = crate::Reg<c5ctrl::C5ctrlSpec>;
#[doc = "DMA channel configuration register (DMA_C5CTRL)"]
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
#[doc = "C6CTRL (rw) register accessor: DMA channel configuration register(DMA_C6CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6ctrl`]
module"]
#[doc(alias = "C6CTRL")]
pub type C6ctrl = crate::Reg<c6ctrl::C6ctrlSpec>;
#[doc = "DMA channel configuration register(DMA_C6CTRL)"]
pub mod c6ctrl;
#[doc = "C6DTCNT (rw) register accessor: DMA channel 6 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6dtcnt`]
module"]
#[doc(alias = "C6DTCNT")]
pub type C6dtcnt = crate::Reg<c6dtcnt::C6dtcntSpec>;
#[doc = "DMA channel 6 number of data to transfer register"]
pub mod c6dtcnt;
#[doc = "C6PADDR (rw) register accessor: DMA channel 6 peripheral address base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6paddr`]
module"]
#[doc(alias = "C6PADDR")]
pub type C6paddr = crate::Reg<c6paddr::C6paddrSpec>;
#[doc = "DMA channel 6 peripheral address base register"]
pub mod c6paddr;
#[doc = "C6MADDR (rw) register accessor: DMA channel 6 memory address base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6maddr`]
module"]
#[doc(alias = "C6MADDR")]
pub type C6maddr = crate::Reg<c6maddr::C6maddrSpec>;
#[doc = "DMA channel 6 memory address base register"]
pub mod c6maddr;
#[doc = "C7CTRL (rw) register accessor: DMA channel configuration register(DMA_C7CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7ctrl`]
module"]
#[doc(alias = "C7CTRL")]
pub type C7ctrl = crate::Reg<c7ctrl::C7ctrlSpec>;
#[doc = "DMA channel configuration register(DMA_C7CTRL)"]
pub mod c7ctrl;
#[doc = "C7DTCNT (rw) register accessor: DMA channel 7 number of data to transfer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7dtcnt`]
module"]
#[doc(alias = "C7DTCNT")]
pub type C7dtcnt = crate::Reg<c7dtcnt::C7dtcntSpec>;
#[doc = "DMA channel 7 number of data to transfer register"]
pub mod c7dtcnt;
#[doc = "C7PADDR (rw) register accessor: DMA channel 7 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7paddr`]
module"]
#[doc(alias = "C7PADDR")]
pub type C7paddr = crate::Reg<c7paddr::C7paddrSpec>;
#[doc = "DMA channel 7 peripheral base address register"]
pub mod c7paddr;
#[doc = "C7MADDR (rw) register accessor: DMA channel 7 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7maddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7maddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7maddr`]
module"]
#[doc(alias = "C7MADDR")]
pub type C7maddr = crate::Reg<c7maddr::C7maddrSpec>;
#[doc = "DMA channel 7 memory base address register"]
pub mod c7maddr;
#[doc = "DMA_MUXSEL (rw) register accessor: DMAMUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_muxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_muxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_muxsel`]
module"]
#[doc(alias = "DMA_MUXSEL")]
pub type DmaMuxsel = crate::Reg<dma_muxsel::DmaMuxselSpec>;
#[doc = "DMAMUX Table Selection"]
pub mod dma_muxsel;
#[doc = "MUXC1CTRL (rw) register accessor: Channel 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc1ctrl`]
module"]
#[doc(alias = "MUXC1CTRL")]
pub type Muxc1ctrl = crate::Reg<muxc1ctrl::Muxc1ctrlSpec>;
#[doc = "Channel 1 Configuration Register"]
pub mod muxc1ctrl;
#[doc = "MUXC2CTRL (rw) register accessor: Channel 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc2ctrl`]
module"]
#[doc(alias = "MUXC2CTRL")]
pub type Muxc2ctrl = crate::Reg<muxc2ctrl::Muxc2ctrlSpec>;
#[doc = "Channel 2 Configuration Register"]
pub mod muxc2ctrl;
#[doc = "MUXC3CTRL (rw) register accessor: Channel 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc3ctrl`]
module"]
#[doc(alias = "MUXC3CTRL")]
pub type Muxc3ctrl = crate::Reg<muxc3ctrl::Muxc3ctrlSpec>;
#[doc = "Channel 3 Configuration Register"]
pub mod muxc3ctrl;
#[doc = "MUXC4CTRL (rw) register accessor: Channel 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc4ctrl`]
module"]
#[doc(alias = "MUXC4CTRL")]
pub type Muxc4ctrl = crate::Reg<muxc4ctrl::Muxc4ctrlSpec>;
#[doc = "Channel 4 Configuration Register"]
pub mod muxc4ctrl;
#[doc = "MUXC5CTRL (rw) register accessor: Channel 5 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc5ctrl`]
module"]
#[doc(alias = "MUXC5CTRL")]
pub type Muxc5ctrl = crate::Reg<muxc5ctrl::Muxc5ctrlSpec>;
#[doc = "Channel 5 Configuration Register"]
pub mod muxc5ctrl;
#[doc = "MUXC6CTRL (rw) register accessor: Channel 6 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc6ctrl`]
module"]
#[doc(alias = "MUXC6CTRL")]
pub type Muxc6ctrl = crate::Reg<muxc6ctrl::Muxc6ctrlSpec>;
#[doc = "Channel 6 Configuration Register"]
pub mod muxc6ctrl;
#[doc = "MUXC7CTRL (rw) register accessor: Channel 7 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxc7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxc7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxc7ctrl`]
module"]
#[doc(alias = "MUXC7CTRL")]
pub type Muxc7ctrl = crate::Reg<muxc7ctrl::Muxc7ctrlSpec>;
#[doc = "Channel 7 Configuration Register"]
pub mod muxc7ctrl;
#[doc = "MUXG1CTRL (rw) register accessor: Generator 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg1ctrl`]
module"]
#[doc(alias = "MUXG1CTRL")]
pub type Muxg1ctrl = crate::Reg<muxg1ctrl::Muxg1ctrlSpec>;
#[doc = "Generator 1 Configuration Register"]
pub mod muxg1ctrl;
#[doc = "MUXG2CTRL (rw) register accessor: Generator 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg2ctrl`]
module"]
#[doc(alias = "MUXG2CTRL")]
pub type Muxg2ctrl = crate::Reg<muxg2ctrl::Muxg2ctrlSpec>;
#[doc = "Generator 2 Configuration Register"]
pub mod muxg2ctrl;
#[doc = "MUXG3CTRL (rw) register accessor: Generator 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg3ctrl`]
module"]
#[doc(alias = "MUXG3CTRL")]
pub type Muxg3ctrl = crate::Reg<muxg3ctrl::Muxg3ctrlSpec>;
#[doc = "Generator 3 Configuration Register"]
pub mod muxg3ctrl;
#[doc = "MUXG4CTRL (rw) register accessor: Generator 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxg4ctrl`]
module"]
#[doc(alias = "MUXG4CTRL")]
pub type Muxg4ctrl = crate::Reg<muxg4ctrl::Muxg4ctrlSpec>;
#[doc = "Generator 4 Configuration Register"]
pub mod muxg4ctrl;
#[doc = "MUXSYNCSTS (rw) register accessor: Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncsts`]
module"]
#[doc(alias = "MUXSYNCSTS")]
pub type Muxsyncsts = crate::Reg<muxsyncsts::MuxsyncstsSpec>;
#[doc = "Channel Interrupt Status Register"]
pub mod muxsyncsts;
#[doc = "MUXSYNCCLR (rw) register accessor: Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsyncclr`]
module"]
#[doc(alias = "MUXSYNCCLR")]
pub type Muxsyncclr = crate::Reg<muxsyncclr::MuxsyncclrSpec>;
#[doc = "Channel Interrupt Clear Flag Register"]
pub mod muxsyncclr;
#[doc = "MUXGSTS (rw) register accessor: Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgsts`]
module"]
#[doc(alias = "MUXGSTS")]
pub type Muxgsts = crate::Reg<muxgsts::MuxgstsSpec>;
#[doc = "Generator Interrupt Status Register"]
pub mod muxgsts;
#[doc = "MUXGCLR (rw) register accessor: Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxgclr`]
module"]
#[doc(alias = "MUXGCLR")]
pub type Muxgclr = crate::Reg<muxgclr::MuxgclrSpec>;
#[doc = "Generator Interrupt Clear Flag Register"]
pub mod muxgclr;
