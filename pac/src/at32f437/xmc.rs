#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bk1ctrl1: Bk1ctrl1,
    bk1tmg1: Bk1tmg1,
    bk1ctrl2: Bk1ctrl2,
    bk1tmg2: Bk1tmg2,
    bk1ctrl3: Bk1ctrl3,
    bk1tmg3: Bk1tmg3,
    bk1ctrl4: Bk1ctrl4,
    bk1tmg4: Bk1tmg4,
    _reserved8: [u8; 0x40],
    bk2ctrl: Bk2ctrl,
    bk2is: Bk2is,
    bk2tmgrg: Bk2tmgrg,
    bk2tmgsp: Bk2tmgsp,
    _reserved12: [u8; 0x04],
    bk2ecc: Bk2ecc,
    _reserved13: [u8; 0x08],
    bk3ctrl: Bk3ctrl,
    bk3is: Bk3is,
    bk3tmgrg: Bk3tmgrg,
    bk3tmgsp: Bk3tmgsp,
    _reserved17: [u8; 0x04],
    bk3ecc: Bk3ecc,
    _reserved18: [u8; 0x08],
    bk4ctrl: Bk4ctrl,
    bk4is: Bk4is,
    bk4tmgcm: Bk4tmgcm,
    bk4tmgat: Bk4tmgat,
    bk4tmgio: Bk4tmgio,
    _reserved23: [u8; 0x50],
    bk1tmgwr1: Bk1tmgwr1,
    _reserved24: [u8; 0x04],
    bk1tmgwr2: Bk1tmgwr2,
    _reserved25: [u8; 0x04],
    bk1tmgwr3: Bk1tmgwr3,
    _reserved26: [u8; 0x04],
    bk1tmgwr4: Bk1tmgwr4,
    _reserved27: [u8; 0x20],
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    tm1: Tm1,
    tm2: Tm2,
    cmd: Cmd,
    rcnt: Rcnt,
    sts: Sts,
    _reserved34: [u8; 0xc4],
    ext1: Ext1,
    ext2: Ext2,
    ext3: Ext3,
    ext4: Ext4,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM/NOR-Flash chip-select control register 1"]
    #[inline(always)]
    pub const fn bk1ctrl1(&self) -> &Bk1ctrl1 {
        &self.bk1ctrl1
    }
    #[doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1"]
    #[inline(always)]
    pub const fn bk1tmg1(&self) -> &Bk1tmg1 {
        &self.bk1tmg1
    }
    #[doc = "0x08 - SRAM/NOR-Flash chip-select control register 2"]
    #[inline(always)]
    pub const fn bk1ctrl2(&self) -> &Bk1ctrl2 {
        &self.bk1ctrl2
    }
    #[doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2"]
    #[inline(always)]
    pub const fn bk1tmg2(&self) -> &Bk1tmg2 {
        &self.bk1tmg2
    }
    #[doc = "0x10 - SRAM/NOR-Flash chip-select control register 3"]
    #[inline(always)]
    pub const fn bk1ctrl3(&self) -> &Bk1ctrl3 {
        &self.bk1ctrl3
    }
    #[doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3"]
    #[inline(always)]
    pub const fn bk1tmg3(&self) -> &Bk1tmg3 {
        &self.bk1tmg3
    }
    #[doc = "0x18 - SRAM/NOR-Flash chip-select control register 4"]
    #[inline(always)]
    pub const fn bk1ctrl4(&self) -> &Bk1ctrl4 {
        &self.bk1ctrl4
    }
    #[doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4"]
    #[inline(always)]
    pub const fn bk1tmg4(&self) -> &Bk1tmg4 {
        &self.bk1tmg4
    }
    #[doc = "0x60 - PC Card/NAND Flash control register 2"]
    #[inline(always)]
    pub const fn bk2ctrl(&self) -> &Bk2ctrl {
        &self.bk2ctrl
    }
    #[doc = "0x64 - FIFO status and interrupt register 2"]
    #[inline(always)]
    pub const fn bk2is(&self) -> &Bk2is {
        &self.bk2is
    }
    #[doc = "0x68 - Regular memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgrg(&self) -> &Bk2tmgrg {
        &self.bk2tmgrg
    }
    #[doc = "0x6c - special memory space timing register 2"]
    #[inline(always)]
    pub const fn bk2tmgsp(&self) -> &Bk2tmgsp {
        &self.bk2tmgsp
    }
    #[doc = "0x74 - ECC result register 2"]
    #[inline(always)]
    pub const fn bk2ecc(&self) -> &Bk2ecc {
        &self.bk2ecc
    }
    #[doc = "0x80 - PC Card/NAND Flash control register 3"]
    #[inline(always)]
    pub const fn bk3ctrl(&self) -> &Bk3ctrl {
        &self.bk3ctrl
    }
    #[doc = "0x84 - FIFO status and interrupt register 3"]
    #[inline(always)]
    pub const fn bk3is(&self) -> &Bk3is {
        &self.bk3is
    }
    #[doc = "0x88 - Regular memory space timing register 3"]
    #[inline(always)]
    pub const fn bk3tmgrg(&self) -> &Bk3tmgrg {
        &self.bk3tmgrg
    }
    #[doc = "0x8c - special memory space timing register 3"]
    #[inline(always)]
    pub const fn bk3tmgsp(&self) -> &Bk3tmgsp {
        &self.bk3tmgsp
    }
    #[doc = "0x94 - ECC result register 3"]
    #[inline(always)]
    pub const fn bk3ecc(&self) -> &Bk3ecc {
        &self.bk3ecc
    }
    #[doc = "0xa0 - PC Card/NAND Flash control register 4"]
    #[inline(always)]
    pub const fn bk4ctrl(&self) -> &Bk4ctrl {
        &self.bk4ctrl
    }
    #[doc = "0xa4 - FIFO status and interrupt register 4"]
    #[inline(always)]
    pub const fn bk4is(&self) -> &Bk4is {
        &self.bk4is
    }
    #[doc = "0xa8 - Regular memory space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgcm(&self) -> &Bk4tmgcm {
        &self.bk4tmgcm
    }
    #[doc = "0xac - special memory space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgat(&self) -> &Bk4tmgat {
        &self.bk4tmgat
    }
    #[doc = "0xb0 - I/O space timing register 4"]
    #[inline(always)]
    pub const fn bk4tmgio(&self) -> &Bk4tmgio {
        &self.bk4tmgio
    }
    #[doc = "0x104 - SRAM/NOR-Flash write timing registers 1"]
    #[inline(always)]
    pub const fn bk1tmgwr1(&self) -> &Bk1tmgwr1 {
        &self.bk1tmgwr1
    }
    #[doc = "0x10c - SRAM/NOR-Flash write timing registers 2"]
    #[inline(always)]
    pub const fn bk1tmgwr2(&self) -> &Bk1tmgwr2 {
        &self.bk1tmgwr2
    }
    #[doc = "0x114 - SRAM/NOR-Flash write timing registers 3"]
    #[inline(always)]
    pub const fn bk1tmgwr3(&self) -> &Bk1tmgwr3 {
        &self.bk1tmgwr3
    }
    #[doc = "0x11c - SRAM/NOR-Flash write timing registers 4"]
    #[inline(always)]
    pub const fn bk1tmgwr4(&self) -> &Bk1tmgwr4 {
        &self.bk1tmgwr4
    }
    #[doc = "0x140 - SDRAM Control Register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x144 - SDRAM Control Register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x148 - SDRAM Timing register 1"]
    #[inline(always)]
    pub const fn tm1(&self) -> &Tm1 {
        &self.tm1
    }
    #[doc = "0x14c - SDRAM Timing register 2"]
    #[inline(always)]
    pub const fn tm2(&self) -> &Tm2 {
        &self.tm2
    }
    #[doc = "0x150 - SDRAM Command Mode register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x154 - SDRAM Refresh Timer register"]
    #[inline(always)]
    pub const fn rcnt(&self) -> &Rcnt {
        &self.rcnt
    }
    #[doc = "0x158 - SDRAM Status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x220 - externl timeing register 1"]
    #[inline(always)]
    pub const fn ext1(&self) -> &Ext1 {
        &self.ext1
    }
    #[doc = "0x224 - externl timeing register 2"]
    #[inline(always)]
    pub const fn ext2(&self) -> &Ext2 {
        &self.ext2
    }
    #[doc = "0x228 - externl timeing register 3"]
    #[inline(always)]
    pub const fn ext3(&self) -> &Ext3 {
        &self.ext3
    }
    #[doc = "0x22c - externl timeing register 4"]
    #[inline(always)]
    pub const fn ext4(&self) -> &Ext4 {
        &self.ext4
    }
}
#[doc = "BK1CTRL1 (rw) register accessor: SRAM/NOR-Flash chip-select control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl1`]
module"]
#[doc(alias = "BK1CTRL1")]
pub type Bk1ctrl1 = crate::Reg<bk1ctrl1::Bk1ctrl1Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 1"]
pub mod bk1ctrl1;
#[doc = "BK1TMG1 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg1`]
module"]
#[doc(alias = "BK1TMG1")]
pub type Bk1tmg1 = crate::Reg<bk1tmg1::Bk1tmg1Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 1"]
pub mod bk1tmg1;
#[doc = "BK1CTRL2 (rw) register accessor: SRAM/NOR-Flash chip-select control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl2`]
module"]
#[doc(alias = "BK1CTRL2")]
pub type Bk1ctrl2 = crate::Reg<bk1ctrl2::Bk1ctrl2Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 2"]
pub mod bk1ctrl2;
#[doc = "BK1TMG2 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg2`]
module"]
#[doc(alias = "BK1TMG2")]
pub type Bk1tmg2 = crate::Reg<bk1tmg2::Bk1tmg2Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 2"]
pub mod bk1tmg2;
#[doc = "BK1CTRL3 (rw) register accessor: SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl3`]
module"]
#[doc(alias = "BK1CTRL3")]
pub type Bk1ctrl3 = crate::Reg<bk1ctrl3::Bk1ctrl3Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 3"]
pub mod bk1ctrl3;
#[doc = "BK1TMG3 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg3`]
module"]
#[doc(alias = "BK1TMG3")]
pub type Bk1tmg3 = crate::Reg<bk1tmg3::Bk1tmg3Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 3"]
pub mod bk1tmg3;
#[doc = "BK1CTRL4 (rw) register accessor: SRAM/NOR-Flash chip-select control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1ctrl4`]
module"]
#[doc(alias = "BK1CTRL4")]
pub type Bk1ctrl4 = crate::Reg<bk1ctrl4::Bk1ctrl4Spec>;
#[doc = "SRAM/NOR-Flash chip-select control register 4"]
pub mod bk1ctrl4;
#[doc = "BK1TMG4 (rw) register accessor: SRAM/NOR-Flash chip-select timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmg4`]
module"]
#[doc(alias = "BK1TMG4")]
pub type Bk1tmg4 = crate::Reg<bk1tmg4::Bk1tmg4Spec>;
#[doc = "SRAM/NOR-Flash chip-select timing register 4"]
pub mod bk1tmg4;
#[doc = "BK2CTRL (rw) register accessor: PC Card/NAND Flash control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ctrl`]
module"]
#[doc(alias = "BK2CTRL")]
pub type Bk2ctrl = crate::Reg<bk2ctrl::Bk2ctrlSpec>;
#[doc = "PC Card/NAND Flash control register 2"]
pub mod bk2ctrl;
#[doc = "BK2IS (rw) register accessor: FIFO status and interrupt register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2is`]
module"]
#[doc(alias = "BK2IS")]
pub type Bk2is = crate::Reg<bk2is::Bk2isSpec>;
#[doc = "FIFO status and interrupt register 2"]
pub mod bk2is;
#[doc = "BK2TMGRG (rw) register accessor: Regular memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgrg`]
module"]
#[doc(alias = "BK2TMGRG")]
pub type Bk2tmgrg = crate::Reg<bk2tmgrg::Bk2tmgrgSpec>;
#[doc = "Regular memory space timing register 2"]
pub mod bk2tmgrg;
#[doc = "BK2TMGSP (rw) register accessor: special memory space timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2tmgsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2tmgsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2tmgsp`]
module"]
#[doc(alias = "BK2TMGSP")]
pub type Bk2tmgsp = crate::Reg<bk2tmgsp::Bk2tmgspSpec>;
#[doc = "special memory space timing register 2"]
pub mod bk2tmgsp;
#[doc = "BK2ECC (rw) register accessor: ECC result register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk2ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk2ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk2ecc`]
module"]
#[doc(alias = "BK2ECC")]
pub type Bk2ecc = crate::Reg<bk2ecc::Bk2eccSpec>;
#[doc = "ECC result register 2"]
pub mod bk2ecc;
#[doc = "BK3CTRL (rw) register accessor: PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3ctrl`]
module"]
#[doc(alias = "BK3CTRL")]
pub type Bk3ctrl = crate::Reg<bk3ctrl::Bk3ctrlSpec>;
#[doc = "PC Card/NAND Flash control register 3"]
pub mod bk3ctrl;
#[doc = "BK3IS (rw) register accessor: FIFO status and interrupt register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3is`]
module"]
#[doc(alias = "BK3IS")]
pub type Bk3is = crate::Reg<bk3is::Bk3isSpec>;
#[doc = "FIFO status and interrupt register 3"]
pub mod bk3is;
#[doc = "BK3TMGRG (rw) register accessor: Regular memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgrg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgrg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3tmgrg`]
module"]
#[doc(alias = "BK3TMGRG")]
pub type Bk3tmgrg = crate::Reg<bk3tmgrg::Bk3tmgrgSpec>;
#[doc = "Regular memory space timing register 3"]
pub mod bk3tmgrg;
#[doc = "BK3TMGSP (rw) register accessor: special memory space timing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3tmgsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3tmgsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3tmgsp`]
module"]
#[doc(alias = "BK3TMGSP")]
pub type Bk3tmgsp = crate::Reg<bk3tmgsp::Bk3tmgspSpec>;
#[doc = "special memory space timing register 3"]
pub mod bk3tmgsp;
#[doc = "BK3ECC (rw) register accessor: ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ecc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ecc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk3ecc`]
module"]
#[doc(alias = "BK3ECC")]
pub type Bk3ecc = crate::Reg<bk3ecc::Bk3eccSpec>;
#[doc = "ECC result register 3"]
pub mod bk3ecc;
#[doc = "BK4CTRL (rw) register accessor: PC Card/NAND Flash control register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4ctrl`]
module"]
#[doc(alias = "BK4CTRL")]
pub type Bk4ctrl = crate::Reg<bk4ctrl::Bk4ctrlSpec>;
#[doc = "PC Card/NAND Flash control register 4"]
pub mod bk4ctrl;
#[doc = "BK4IS (rw) register accessor: FIFO status and interrupt register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4is::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4is::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4is`]
module"]
#[doc(alias = "BK4IS")]
pub type Bk4is = crate::Reg<bk4is::Bk4isSpec>;
#[doc = "FIFO status and interrupt register 4"]
pub mod bk4is;
#[doc = "BK4TMGCM (rw) register accessor: Regular memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgcm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgcm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgcm`]
module"]
#[doc(alias = "BK4TMGCM")]
pub type Bk4tmgcm = crate::Reg<bk4tmgcm::Bk4tmgcmSpec>;
#[doc = "Regular memory space timing register 4"]
pub mod bk4tmgcm;
#[doc = "BK4TMGAT (rw) register accessor: special memory space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgat`]
module"]
#[doc(alias = "BK4TMGAT")]
pub type Bk4tmgat = crate::Reg<bk4tmgat::Bk4tmgatSpec>;
#[doc = "special memory space timing register 4"]
pub mod bk4tmgat;
#[doc = "BK4TMGIO (rw) register accessor: I/O space timing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk4tmgio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk4tmgio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk4tmgio`]
module"]
#[doc(alias = "BK4TMGIO")]
pub type Bk4tmgio = crate::Reg<bk4tmgio::Bk4tmgioSpec>;
#[doc = "I/O space timing register 4"]
pub mod bk4tmgio;
#[doc = "BK1TMGWR1 (rw) register accessor: SRAM/NOR-Flash write timing registers 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr1`]
module"]
#[doc(alias = "BK1TMGWR1")]
pub type Bk1tmgwr1 = crate::Reg<bk1tmgwr1::Bk1tmgwr1Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 1"]
pub mod bk1tmgwr1;
#[doc = "BK1TMGWR2 (rw) register accessor: SRAM/NOR-Flash write timing registers 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr2`]
module"]
#[doc(alias = "BK1TMGWR2")]
pub type Bk1tmgwr2 = crate::Reg<bk1tmgwr2::Bk1tmgwr2Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 2"]
pub mod bk1tmgwr2;
#[doc = "BK1TMGWR3 (rw) register accessor: SRAM/NOR-Flash write timing registers 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr3`]
module"]
#[doc(alias = "BK1TMGWR3")]
pub type Bk1tmgwr3 = crate::Reg<bk1tmgwr3::Bk1tmgwr3Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 3"]
pub mod bk1tmgwr3;
#[doc = "BK1TMGWR4 (rw) register accessor: SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bk1tmgwr4`]
module"]
#[doc(alias = "BK1TMGWR4")]
pub type Bk1tmgwr4 = crate::Reg<bk1tmgwr4::Bk1tmgwr4Spec>;
#[doc = "SRAM/NOR-Flash write timing registers 4"]
pub mod bk1tmgwr4;
#[doc = "CTRL1 (rw) register accessor: SDRAM Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "SDRAM Control Register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: SDRAM Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "SDRAM Control Register 2"]
pub mod ctrl2;
#[doc = "TM1 (rw) register accessor: SDRAM Timing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tm1`]
module"]
#[doc(alias = "TM1")]
pub type Tm1 = crate::Reg<tm1::Tm1Spec>;
#[doc = "SDRAM Timing register 1"]
pub mod tm1;
#[doc = "TM2 (rw) register accessor: SDRAM Timing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tm2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tm2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tm2`]
module"]
#[doc(alias = "TM2")]
pub type Tm2 = crate::Reg<tm2::Tm2Spec>;
#[doc = "SDRAM Timing register 2"]
pub mod tm2;
#[doc = "CMD (rw) register accessor: SDRAM Command Mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "SDRAM Command Mode register"]
pub mod cmd;
#[doc = "RCNT (rw) register accessor: SDRAM Refresh Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcnt`]
module"]
#[doc(alias = "RCNT")]
pub type Rcnt = crate::Reg<rcnt::RcntSpec>;
#[doc = "SDRAM Refresh Timer register"]
pub mod rcnt;
#[doc = "STS (r) register accessor: SDRAM Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "SDRAM Status register"]
pub mod sts;
#[doc = "EXT1 (rw) register accessor: externl timeing register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext1`]
module"]
#[doc(alias = "EXT1")]
pub type Ext1 = crate::Reg<ext1::Ext1Spec>;
#[doc = "externl timeing register 1"]
pub mod ext1;
#[doc = "EXT2 (rw) register accessor: externl timeing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext2`]
module"]
#[doc(alias = "EXT2")]
pub type Ext2 = crate::Reg<ext2::Ext2Spec>;
#[doc = "externl timeing register 2"]
pub mod ext2;
#[doc = "EXT3 (rw) register accessor: externl timeing register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext3`]
module"]
#[doc(alias = "EXT3")]
pub type Ext3 = crate::Reg<ext3::Ext3Spec>;
#[doc = "externl timeing register 3"]
pub mod ext3;
#[doc = "EXT4 (rw) register accessor: externl timeing register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext4`]
module"]
#[doc(alias = "EXT4")]
pub type Ext4 = crate::Reg<ext4::Ext4Spec>;
#[doc = "externl timeing register 4"]
pub mod ext4;
