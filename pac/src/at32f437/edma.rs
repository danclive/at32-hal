#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts1: Sts1,
    sts2: Sts2,
    clr1: Clr1,
    clr2: Clr2,
    s1ctrl: S1ctrl,
    s1dtcnt: S1dtcnt,
    s1paddr: S1paddr,
    s1m0addr: S1m0addr,
    s1m1addr: S1m1addr,
    s1fctrl: S1fctrl,
    s2ctrl: S2ctrl,
    s2dtcnt: S2dtcnt,
    s2paddr: S2paddr,
    s2m0addr: S2m0addr,
    s2m1addr: S2m1addr,
    s2fctrl: S2fctrl,
    s3ctrl: S3ctrl,
    s3dtcnt: S3dtcnt,
    s3paddr: S3paddr,
    s3m0addr: S3m0addr,
    s3m1addr: S3m1addr,
    s3fctrl: S3fctrl,
    s4ctrl: S4ctrl,
    s4dtcnt: S4dtcnt,
    s4paddr: S4paddr,
    s4m0addr: S4m0addr,
    s4m1addr: S4m1addr,
    s4fctrl: S4fctrl,
    s5ctrl: S5ctrl,
    s5dtcnt: S5dtcnt,
    s5paddr: S5paddr,
    s5m0addr: S5m0addr,
    s5m1addr: S5m1addr,
    s5fctrl: S5fctrl,
    s6ctrl: S6ctrl,
    s6dtcnt: S6dtcnt,
    s6paddr: S6paddr,
    s6m0addr: S6m0addr,
    s6m1addr: S6m1addr,
    s6fctrl: S6fctrl,
    s7ctrl: S7ctrl,
    s7dtcnt: S7dtcnt,
    s7paddr: S7paddr,
    s7m0addr: S7m0addr,
    s7m1addr: S7m1addr,
    s7fctrl: S7fctrl,
    s8ctrl: S8ctrl,
    s8dtcnt: S8dtcnt,
    s8paddr: S8paddr,
    s8m0addr: S8m0addr,
    s8m1addr: S8m1addr,
    s8fctrl: S8fctrl,
    llctrl: Llctrl,
    s1llp: S1llp,
    s2llp: S2llp,
    s3llp: S3llp,
    s4llp: S4llp,
    s5llp: S5llp,
    s6llp: S6llp,
    s7llp: S7llp,
    s8llp: S8llp,
    s2dctrl: S2dctrl,
    s1_2dcnt: S1_2dcnt,
    s1_stride: S1Stride,
    s2_2dcnt: S2_2dcnt,
    s2_stride: S2Stride,
    s3_2dcnt: S3_2dcnt,
    s3_stride: S3Stride,
    s4_2dcnt: S4_2dcnt,
    s4_stride: S4Stride,
    s5_2dcnt: S5_2dcnt,
    s5_stride: S5Stride,
    s6_2dcnt: S6_2dcnt,
    s6_stride: S6Stride,
    s7_2dcnt: S7_2dcnt,
    s7_stride: S7Stride,
    s8_2dcnt: S8_2dcnt,
    s8_stride: S8Stride,
    syncen: Syncen,
    muxsel: Muxsel,
    muxs1ctrl: Muxs1ctrl,
    muxs2ctrl: Muxs2ctrl,
    muxs3ctrl: Muxs3ctrl,
    muxs4ctrl: Muxs4ctrl,
    muxs5ctrl: Muxs5ctrl,
    muxs6ctrl: Muxs6ctrl,
    muxs7ctrl: Muxs7ctrl,
    muxs8ctrl: Muxs8ctrl,
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
    #[doc = "0x00 - Interrupt status register1"]
    #[inline(always)]
    pub const fn sts1(&self) -> &Sts1 {
        &self.sts1
    }
    #[doc = "0x04 - Interrupt status register2"]
    #[inline(always)]
    pub const fn sts2(&self) -> &Sts2 {
        &self.sts2
    }
    #[doc = "0x08 - Interrupt flag clear register1"]
    #[inline(always)]
    pub const fn clr1(&self) -> &Clr1 {
        &self.clr1
    }
    #[doc = "0x0c - Interrupt flag clear register2"]
    #[inline(always)]
    pub const fn clr2(&self) -> &Clr2 {
        &self.clr2
    }
    #[doc = "0x10 - stream 1 control register"]
    #[inline(always)]
    pub const fn s1ctrl(&self) -> &S1ctrl {
        &self.s1ctrl
    }
    #[doc = "0x14 - stream 1 number of data register"]
    #[inline(always)]
    pub const fn s1dtcnt(&self) -> &S1dtcnt {
        &self.s1dtcnt
    }
    #[doc = "0x18 - stream 1 peripheral address register"]
    #[inline(always)]
    pub const fn s1paddr(&self) -> &S1paddr {
        &self.s1paddr
    }
    #[doc = "0x1c - stream 1 memory 0 address register"]
    #[inline(always)]
    pub const fn s1m0addr(&self) -> &S1m0addr {
        &self.s1m0addr
    }
    #[doc = "0x20 - stream 1 memory 1 address register"]
    #[inline(always)]
    pub const fn s1m1addr(&self) -> &S1m1addr {
        &self.s1m1addr
    }
    #[doc = "0x24 - stream 1 FIFO control register"]
    #[inline(always)]
    pub const fn s1fctrl(&self) -> &S1fctrl {
        &self.s1fctrl
    }
    #[doc = "0x28 - stream 2 control register"]
    #[inline(always)]
    pub const fn s2ctrl(&self) -> &S2ctrl {
        &self.s2ctrl
    }
    #[doc = "0x2c - stream 2 number of data register"]
    #[inline(always)]
    pub const fn s2dtcnt(&self) -> &S2dtcnt {
        &self.s2dtcnt
    }
    #[doc = "0x30 - stream 2 peripheral address register"]
    #[inline(always)]
    pub const fn s2paddr(&self) -> &S2paddr {
        &self.s2paddr
    }
    #[doc = "0x34 - stream 2 memory 0 address register"]
    #[inline(always)]
    pub const fn s2m0addr(&self) -> &S2m0addr {
        &self.s2m0addr
    }
    #[doc = "0x38 - stream 2 memory 1 address register"]
    #[inline(always)]
    pub const fn s2m1addr(&self) -> &S2m1addr {
        &self.s2m1addr
    }
    #[doc = "0x3c - stream 2 FIFO control register"]
    #[inline(always)]
    pub const fn s2fctrl(&self) -> &S2fctrl {
        &self.s2fctrl
    }
    #[doc = "0x40 - stream 3 control register"]
    #[inline(always)]
    pub const fn s3ctrl(&self) -> &S3ctrl {
        &self.s3ctrl
    }
    #[doc = "0x44 - stream 3 number of data register"]
    #[inline(always)]
    pub const fn s3dtcnt(&self) -> &S3dtcnt {
        &self.s3dtcnt
    }
    #[doc = "0x48 - stream 3 peripheral address register"]
    #[inline(always)]
    pub const fn s3paddr(&self) -> &S3paddr {
        &self.s3paddr
    }
    #[doc = "0x4c - stream 3 memory 0 address register"]
    #[inline(always)]
    pub const fn s3m0addr(&self) -> &S3m0addr {
        &self.s3m0addr
    }
    #[doc = "0x50 - stream 3 memory 1 address register"]
    #[inline(always)]
    pub const fn s3m1addr(&self) -> &S3m1addr {
        &self.s3m1addr
    }
    #[doc = "0x54 - stream 3 FIFO control register"]
    #[inline(always)]
    pub const fn s3fctrl(&self) -> &S3fctrl {
        &self.s3fctrl
    }
    #[doc = "0x58 - stream 4 control register"]
    #[inline(always)]
    pub const fn s4ctrl(&self) -> &S4ctrl {
        &self.s4ctrl
    }
    #[doc = "0x5c - stream 4 number of data register"]
    #[inline(always)]
    pub const fn s4dtcnt(&self) -> &S4dtcnt {
        &self.s4dtcnt
    }
    #[doc = "0x60 - stream 4 peripheral address register"]
    #[inline(always)]
    pub const fn s4paddr(&self) -> &S4paddr {
        &self.s4paddr
    }
    #[doc = "0x64 - stream 4 memory 0 address register"]
    #[inline(always)]
    pub const fn s4m0addr(&self) -> &S4m0addr {
        &self.s4m0addr
    }
    #[doc = "0x68 - stream 4 memory 1 address register"]
    #[inline(always)]
    pub const fn s4m1addr(&self) -> &S4m1addr {
        &self.s4m1addr
    }
    #[doc = "0x6c - stream 4 FIFO control register"]
    #[inline(always)]
    pub const fn s4fctrl(&self) -> &S4fctrl {
        &self.s4fctrl
    }
    #[doc = "0x70 - stream 5 control register"]
    #[inline(always)]
    pub const fn s5ctrl(&self) -> &S5ctrl {
        &self.s5ctrl
    }
    #[doc = "0x74 - stream 5 number of data register"]
    #[inline(always)]
    pub const fn s5dtcnt(&self) -> &S5dtcnt {
        &self.s5dtcnt
    }
    #[doc = "0x78 - stream 5 peripheral address register"]
    #[inline(always)]
    pub const fn s5paddr(&self) -> &S5paddr {
        &self.s5paddr
    }
    #[doc = "0x7c - stream 5 memory 0 address register"]
    #[inline(always)]
    pub const fn s5m0addr(&self) -> &S5m0addr {
        &self.s5m0addr
    }
    #[doc = "0x80 - stream 5 memory 1 address register"]
    #[inline(always)]
    pub const fn s5m1addr(&self) -> &S5m1addr {
        &self.s5m1addr
    }
    #[doc = "0x84 - stream 5 FIFO control register"]
    #[inline(always)]
    pub const fn s5fctrl(&self) -> &S5fctrl {
        &self.s5fctrl
    }
    #[doc = "0x88 - stream 6 control register"]
    #[inline(always)]
    pub const fn s6ctrl(&self) -> &S6ctrl {
        &self.s6ctrl
    }
    #[doc = "0x8c - stream 6 number of data register"]
    #[inline(always)]
    pub const fn s6dtcnt(&self) -> &S6dtcnt {
        &self.s6dtcnt
    }
    #[doc = "0x90 - stream 6 peripheral address register"]
    #[inline(always)]
    pub const fn s6paddr(&self) -> &S6paddr {
        &self.s6paddr
    }
    #[doc = "0x94 - stream 6 memory 0 address register"]
    #[inline(always)]
    pub const fn s6m0addr(&self) -> &S6m0addr {
        &self.s6m0addr
    }
    #[doc = "0x98 - stream 6 memory 1 address register"]
    #[inline(always)]
    pub const fn s6m1addr(&self) -> &S6m1addr {
        &self.s6m1addr
    }
    #[doc = "0x9c - stream 6 FIFO control register"]
    #[inline(always)]
    pub const fn s6fctrl(&self) -> &S6fctrl {
        &self.s6fctrl
    }
    #[doc = "0xa0 - stream 7 control register"]
    #[inline(always)]
    pub const fn s7ctrl(&self) -> &S7ctrl {
        &self.s7ctrl
    }
    #[doc = "0xa4 - stream 7 number of data register"]
    #[inline(always)]
    pub const fn s7dtcnt(&self) -> &S7dtcnt {
        &self.s7dtcnt
    }
    #[doc = "0xa8 - stream 7 peripheral address register"]
    #[inline(always)]
    pub const fn s7paddr(&self) -> &S7paddr {
        &self.s7paddr
    }
    #[doc = "0xac - stream 7 memory 0 address register"]
    #[inline(always)]
    pub const fn s7m0addr(&self) -> &S7m0addr {
        &self.s7m0addr
    }
    #[doc = "0xb0 - stream 7 memory 1 address register"]
    #[inline(always)]
    pub const fn s7m1addr(&self) -> &S7m1addr {
        &self.s7m1addr
    }
    #[doc = "0xb4 - stream 7 FIFO control register"]
    #[inline(always)]
    pub const fn s7fctrl(&self) -> &S7fctrl {
        &self.s7fctrl
    }
    #[doc = "0xb8 - stream 8 control register"]
    #[inline(always)]
    pub const fn s8ctrl(&self) -> &S8ctrl {
        &self.s8ctrl
    }
    #[doc = "0xbc - stream 8 number of data register"]
    #[inline(always)]
    pub const fn s8dtcnt(&self) -> &S8dtcnt {
        &self.s8dtcnt
    }
    #[doc = "0xc0 - stream 8 peripheral address register"]
    #[inline(always)]
    pub const fn s8paddr(&self) -> &S8paddr {
        &self.s8paddr
    }
    #[doc = "0xc4 - stream 8 memory 0 address register"]
    #[inline(always)]
    pub const fn s8m0addr(&self) -> &S8m0addr {
        &self.s8m0addr
    }
    #[doc = "0xc8 - stream 8 memory 1 address register"]
    #[inline(always)]
    pub const fn s8m1addr(&self) -> &S8m1addr {
        &self.s8m1addr
    }
    #[doc = "0xcc - stream 8 FIFO control register"]
    #[inline(always)]
    pub const fn s8fctrl(&self) -> &S8fctrl {
        &self.s8fctrl
    }
    #[doc = "0xd0 - DMA Link List Control Register"]
    #[inline(always)]
    pub const fn llctrl(&self) -> &Llctrl {
        &self.llctrl
    }
    #[doc = "0xd4 - Stream 1 Link List Pointer"]
    #[inline(always)]
    pub const fn s1llp(&self) -> &S1llp {
        &self.s1llp
    }
    #[doc = "0xd8 - Stream 2 Link List Pointer"]
    #[inline(always)]
    pub const fn s2llp(&self) -> &S2llp {
        &self.s2llp
    }
    #[doc = "0xdc - Stream 3 Link List Pointer"]
    #[inline(always)]
    pub const fn s3llp(&self) -> &S3llp {
        &self.s3llp
    }
    #[doc = "0xe0 - Stream 4 Link List Pointer"]
    #[inline(always)]
    pub const fn s4llp(&self) -> &S4llp {
        &self.s4llp
    }
    #[doc = "0xe4 - Stream 5 Link List Pointer"]
    #[inline(always)]
    pub const fn s5llp(&self) -> &S5llp {
        &self.s5llp
    }
    #[doc = "0xe8 - Stream 6 Link List Pointer"]
    #[inline(always)]
    pub const fn s6llp(&self) -> &S6llp {
        &self.s6llp
    }
    #[doc = "0xec - Stream 7 Link List Pointer"]
    #[inline(always)]
    pub const fn s7llp(&self) -> &S7llp {
        &self.s7llp
    }
    #[doc = "0xf0 - Stream 8 Link List Pointer"]
    #[inline(always)]
    pub const fn s8llp(&self) -> &S8llp {
        &self.s8llp
    }
    #[doc = "0xf4 - EDMA 2D Transfer Control Register"]
    #[inline(always)]
    pub const fn s2dctrl(&self) -> &S2dctrl {
        &self.s2dctrl
    }
    #[doc = "0xf8 - Stream 1 2D Transfer Count"]
    #[inline(always)]
    pub const fn s1_2dcnt(&self) -> &S1_2dcnt {
        &self.s1_2dcnt
    }
    #[doc = "0xfc - Stream 1 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s1_stride(&self) -> &S1Stride {
        &self.s1_stride
    }
    #[doc = "0x100 - Stream 2 2D Transfer Count"]
    #[inline(always)]
    pub const fn s2_2dcnt(&self) -> &S2_2dcnt {
        &self.s2_2dcnt
    }
    #[doc = "0x104 - Stream 2 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s2_stride(&self) -> &S2Stride {
        &self.s2_stride
    }
    #[doc = "0x108 - Stream 3 2D Transfer Count"]
    #[inline(always)]
    pub const fn s3_2dcnt(&self) -> &S3_2dcnt {
        &self.s3_2dcnt
    }
    #[doc = "0x10c - Stream 3 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s3_stride(&self) -> &S3Stride {
        &self.s3_stride
    }
    #[doc = "0x110 - Stream 4 2D Transfer Count"]
    #[inline(always)]
    pub const fn s4_2dcnt(&self) -> &S4_2dcnt {
        &self.s4_2dcnt
    }
    #[doc = "0x114 - Stream 4 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s4_stride(&self) -> &S4Stride {
        &self.s4_stride
    }
    #[doc = "0x118 - Stream 5 2D Transfer Count"]
    #[inline(always)]
    pub const fn s5_2dcnt(&self) -> &S5_2dcnt {
        &self.s5_2dcnt
    }
    #[doc = "0x11c - Stream 5 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s5_stride(&self) -> &S5Stride {
        &self.s5_stride
    }
    #[doc = "0x120 - Stream 6 2D Transfer Count"]
    #[inline(always)]
    pub const fn s6_2dcnt(&self) -> &S6_2dcnt {
        &self.s6_2dcnt
    }
    #[doc = "0x124 - Stream 6 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s6_stride(&self) -> &S6Stride {
        &self.s6_stride
    }
    #[doc = "0x128 - Stream 7 2D Transfer Count"]
    #[inline(always)]
    pub const fn s7_2dcnt(&self) -> &S7_2dcnt {
        &self.s7_2dcnt
    }
    #[doc = "0x12c - Stream 7 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s7_stride(&self) -> &S7Stride {
        &self.s7_stride
    }
    #[doc = "0x130 - Stream 8 2D Transfer Count"]
    #[inline(always)]
    pub const fn s8_2dcnt(&self) -> &S8_2dcnt {
        &self.s8_2dcnt
    }
    #[doc = "0x134 - Stream 8 2D Transfer Stride"]
    #[inline(always)]
    pub const fn s8_stride(&self) -> &S8Stride {
        &self.s8_stride
    }
    #[doc = "0x138 - Sync Enable"]
    #[inline(always)]
    pub const fn syncen(&self) -> &Syncen {
        &self.syncen
    }
    #[doc = "0x13c - EDMA MUX Table Selection"]
    #[inline(always)]
    pub const fn muxsel(&self) -> &Muxsel {
        &self.muxsel
    }
    #[doc = "0x140 - Stream 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxs1ctrl(&self) -> &Muxs1ctrl {
        &self.muxs1ctrl
    }
    #[doc = "0x144 - Stream 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxs2ctrl(&self) -> &Muxs2ctrl {
        &self.muxs2ctrl
    }
    #[doc = "0x148 - Stream 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxs3ctrl(&self) -> &Muxs3ctrl {
        &self.muxs3ctrl
    }
    #[doc = "0x14c - Stream 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxs4ctrl(&self) -> &Muxs4ctrl {
        &self.muxs4ctrl
    }
    #[doc = "0x150 - Stream x Configuration Register"]
    #[inline(always)]
    pub const fn muxs5ctrl(&self) -> &Muxs5ctrl {
        &self.muxs5ctrl
    }
    #[doc = "0x154 - Stream 6 Configuration Register"]
    #[inline(always)]
    pub const fn muxs6ctrl(&self) -> &Muxs6ctrl {
        &self.muxs6ctrl
    }
    #[doc = "0x158 - Stream 7 Configuration Register"]
    #[inline(always)]
    pub const fn muxs7ctrl(&self) -> &Muxs7ctrl {
        &self.muxs7ctrl
    }
    #[doc = "0x15c - Stream 8 Configuration Register"]
    #[inline(always)]
    pub const fn muxs8ctrl(&self) -> &Muxs8ctrl {
        &self.muxs8ctrl
    }
    #[doc = "0x160 - Generator 1 Configuration Register"]
    #[inline(always)]
    pub const fn muxg1ctrl(&self) -> &Muxg1ctrl {
        &self.muxg1ctrl
    }
    #[doc = "0x164 - Generator 2 Configuration Register"]
    #[inline(always)]
    pub const fn muxg2ctrl(&self) -> &Muxg2ctrl {
        &self.muxg2ctrl
    }
    #[doc = "0x168 - Generator 3 Configuration Register"]
    #[inline(always)]
    pub const fn muxg3ctrl(&self) -> &Muxg3ctrl {
        &self.muxg3ctrl
    }
    #[doc = "0x16c - Generator 4 Configuration Register"]
    #[inline(always)]
    pub const fn muxg4ctrl(&self) -> &Muxg4ctrl {
        &self.muxg4ctrl
    }
    #[doc = "0x170 - Channel Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxsyncsts(&self) -> &Muxsyncsts {
        &self.muxsyncsts
    }
    #[doc = "0x174 - Channel Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxsyncclr(&self) -> &Muxsyncclr {
        &self.muxsyncclr
    }
    #[doc = "0x178 - Generator Interrupt Status Register"]
    #[inline(always)]
    pub const fn muxgsts(&self) -> &Muxgsts {
        &self.muxgsts
    }
    #[doc = "0x17c - Generator Interrupt Clear Flag Register"]
    #[inline(always)]
    pub const fn muxgclr(&self) -> &Muxgclr {
        &self.muxgclr
    }
}
#[doc = "STS1 (r) register accessor: Interrupt status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts1`]
module"]
#[doc(alias = "STS1")]
pub type Sts1 = crate::Reg<sts1::Sts1Spec>;
#[doc = "Interrupt status register1"]
pub mod sts1;
#[doc = "STS2 (r) register accessor: Interrupt status register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts2`]
module"]
#[doc(alias = "STS2")]
pub type Sts2 = crate::Reg<sts2::Sts2Spec>;
#[doc = "Interrupt status register2"]
pub mod sts2;
#[doc = "CLR1 (rw) register accessor: Interrupt flag clear register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr1`]
module"]
#[doc(alias = "CLR1")]
pub type Clr1 = crate::Reg<clr1::Clr1Spec>;
#[doc = "Interrupt flag clear register1"]
pub mod clr1;
#[doc = "CLR2 (rw) register accessor: Interrupt flag clear register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr2`]
module"]
#[doc(alias = "CLR2")]
pub type Clr2 = crate::Reg<clr2::Clr2Spec>;
#[doc = "Interrupt flag clear register2"]
pub mod clr2;
#[doc = "S1CTRL (rw) register accessor: stream 1 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1ctrl`]
module"]
#[doc(alias = "S1CTRL")]
pub type S1ctrl = crate::Reg<s1ctrl::S1ctrlSpec>;
#[doc = "stream 1 control register"]
pub mod s1ctrl;
#[doc = "S1DTCNT (rw) register accessor: stream 1 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1dtcnt`]
module"]
#[doc(alias = "S1DTCNT")]
pub type S1dtcnt = crate::Reg<s1dtcnt::S1dtcntSpec>;
#[doc = "stream 1 number of data register"]
pub mod s1dtcnt;
#[doc = "S1PADDR (rw) register accessor: stream 1 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1paddr`]
module"]
#[doc(alias = "S1PADDR")]
pub type S1paddr = crate::Reg<s1paddr::S1paddrSpec>;
#[doc = "stream 1 peripheral address register"]
pub mod s1paddr;
#[doc = "S1M0ADDR (rw) register accessor: stream 1 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m0addr`]
module"]
#[doc(alias = "S1M0ADDR")]
pub type S1m0addr = crate::Reg<s1m0addr::S1m0addrSpec>;
#[doc = "stream 1 memory 0 address register"]
pub mod s1m0addr;
#[doc = "S1M1ADDR (rw) register accessor: stream 1 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1m1addr`]
module"]
#[doc(alias = "S1M1ADDR")]
pub type S1m1addr = crate::Reg<s1m1addr::S1m1addrSpec>;
#[doc = "stream 1 memory 1 address register"]
pub mod s1m1addr;
#[doc = "S1FCTRL (rw) register accessor: stream 1 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1fctrl`]
module"]
#[doc(alias = "S1FCTRL")]
pub type S1fctrl = crate::Reg<s1fctrl::S1fctrlSpec>;
#[doc = "stream 1 FIFO control register"]
pub mod s1fctrl;
#[doc = "S2CTRL (rw) register accessor: stream 2 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2ctrl`]
module"]
#[doc(alias = "S2CTRL")]
pub type S2ctrl = crate::Reg<s2ctrl::S2ctrlSpec>;
#[doc = "stream 2 control register"]
pub mod s2ctrl;
#[doc = "S2DTCNT (rw) register accessor: stream 2 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2dtcnt`]
module"]
#[doc(alias = "S2DTCNT")]
pub type S2dtcnt = crate::Reg<s2dtcnt::S2dtcntSpec>;
#[doc = "stream 2 number of data register"]
pub mod s2dtcnt;
#[doc = "S2PADDR (rw) register accessor: stream 2 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2paddr`]
module"]
#[doc(alias = "S2PADDR")]
pub type S2paddr = crate::Reg<s2paddr::S2paddrSpec>;
#[doc = "stream 2 peripheral address register"]
pub mod s2paddr;
#[doc = "S2M0ADDR (rw) register accessor: stream 2 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m0addr`]
module"]
#[doc(alias = "S2M0ADDR")]
pub type S2m0addr = crate::Reg<s2m0addr::S2m0addrSpec>;
#[doc = "stream 2 memory 0 address register"]
pub mod s2m0addr;
#[doc = "S2M1ADDR (rw) register accessor: stream 2 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2m1addr`]
module"]
#[doc(alias = "S2M1ADDR")]
pub type S2m1addr = crate::Reg<s2m1addr::S2m1addrSpec>;
#[doc = "stream 2 memory 1 address register"]
pub mod s2m1addr;
#[doc = "S2FCTRL (rw) register accessor: stream 2 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2fctrl`]
module"]
#[doc(alias = "S2FCTRL")]
pub type S2fctrl = crate::Reg<s2fctrl::S2fctrlSpec>;
#[doc = "stream 2 FIFO control register"]
pub mod s2fctrl;
#[doc = "S3CTRL (rw) register accessor: stream 3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3ctrl`]
module"]
#[doc(alias = "S3CTRL")]
pub type S3ctrl = crate::Reg<s3ctrl::S3ctrlSpec>;
#[doc = "stream 3 control register"]
pub mod s3ctrl;
#[doc = "S3DTCNT (rw) register accessor: stream 3 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3dtcnt`]
module"]
#[doc(alias = "S3DTCNT")]
pub type S3dtcnt = crate::Reg<s3dtcnt::S3dtcntSpec>;
#[doc = "stream 3 number of data register"]
pub mod s3dtcnt;
#[doc = "S3PADDR (rw) register accessor: stream 3 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3paddr`]
module"]
#[doc(alias = "S3PADDR")]
pub type S3paddr = crate::Reg<s3paddr::S3paddrSpec>;
#[doc = "stream 3 peripheral address register"]
pub mod s3paddr;
#[doc = "S3M0ADDR (rw) register accessor: stream 3 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m0addr`]
module"]
#[doc(alias = "S3M0ADDR")]
pub type S3m0addr = crate::Reg<s3m0addr::S3m0addrSpec>;
#[doc = "stream 3 memory 0 address register"]
pub mod s3m0addr;
#[doc = "S3M1ADDR (rw) register accessor: stream 3 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3m1addr`]
module"]
#[doc(alias = "S3M1ADDR")]
pub type S3m1addr = crate::Reg<s3m1addr::S3m1addrSpec>;
#[doc = "stream 3 memory 1 address register"]
pub mod s3m1addr;
#[doc = "S3FCTRL (rw) register accessor: stream 3 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3fctrl`]
module"]
#[doc(alias = "S3FCTRL")]
pub type S3fctrl = crate::Reg<s3fctrl::S3fctrlSpec>;
#[doc = "stream 3 FIFO control register"]
pub mod s3fctrl;
#[doc = "S4CTRL (rw) register accessor: stream 4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4ctrl`]
module"]
#[doc(alias = "S4CTRL")]
pub type S4ctrl = crate::Reg<s4ctrl::S4ctrlSpec>;
#[doc = "stream 4 control register"]
pub mod s4ctrl;
#[doc = "S4DTCNT (rw) register accessor: stream 4 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4dtcnt`]
module"]
#[doc(alias = "S4DTCNT")]
pub type S4dtcnt = crate::Reg<s4dtcnt::S4dtcntSpec>;
#[doc = "stream 4 number of data register"]
pub mod s4dtcnt;
#[doc = "S4PADDR (rw) register accessor: stream 4 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4paddr`]
module"]
#[doc(alias = "S4PADDR")]
pub type S4paddr = crate::Reg<s4paddr::S4paddrSpec>;
#[doc = "stream 4 peripheral address register"]
pub mod s4paddr;
#[doc = "S4M0ADDR (rw) register accessor: stream 4 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m0addr`]
module"]
#[doc(alias = "S4M0ADDR")]
pub type S4m0addr = crate::Reg<s4m0addr::S4m0addrSpec>;
#[doc = "stream 4 memory 0 address register"]
pub mod s4m0addr;
#[doc = "S4M1ADDR (rw) register accessor: stream 4 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4m1addr`]
module"]
#[doc(alias = "S4M1ADDR")]
pub type S4m1addr = crate::Reg<s4m1addr::S4m1addrSpec>;
#[doc = "stream 4 memory 1 address register"]
pub mod s4m1addr;
#[doc = "S4FCTRL (rw) register accessor: stream 4 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4fctrl`]
module"]
#[doc(alias = "S4FCTRL")]
pub type S4fctrl = crate::Reg<s4fctrl::S4fctrlSpec>;
#[doc = "stream 4 FIFO control register"]
pub mod s4fctrl;
#[doc = "S5CTRL (rw) register accessor: stream 5 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5ctrl`]
module"]
#[doc(alias = "S5CTRL")]
pub type S5ctrl = crate::Reg<s5ctrl::S5ctrlSpec>;
#[doc = "stream 5 control register"]
pub mod s5ctrl;
#[doc = "S5DTCNT (rw) register accessor: stream 5 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5dtcnt`]
module"]
#[doc(alias = "S5DTCNT")]
pub type S5dtcnt = crate::Reg<s5dtcnt::S5dtcntSpec>;
#[doc = "stream 5 number of data register"]
pub mod s5dtcnt;
#[doc = "S5PADDR (rw) register accessor: stream 5 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5paddr`]
module"]
#[doc(alias = "S5PADDR")]
pub type S5paddr = crate::Reg<s5paddr::S5paddrSpec>;
#[doc = "stream 5 peripheral address register"]
pub mod s5paddr;
#[doc = "S5M0ADDR (rw) register accessor: stream 5 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m0addr`]
module"]
#[doc(alias = "S5M0ADDR")]
pub type S5m0addr = crate::Reg<s5m0addr::S5m0addrSpec>;
#[doc = "stream 5 memory 0 address register"]
pub mod s5m0addr;
#[doc = "S5M1ADDR (rw) register accessor: stream 5 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5m1addr`]
module"]
#[doc(alias = "S5M1ADDR")]
pub type S5m1addr = crate::Reg<s5m1addr::S5m1addrSpec>;
#[doc = "stream 5 memory 1 address register"]
pub mod s5m1addr;
#[doc = "S5FCTRL (rw) register accessor: stream 5 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5fctrl`]
module"]
#[doc(alias = "S5FCTRL")]
pub type S5fctrl = crate::Reg<s5fctrl::S5fctrlSpec>;
#[doc = "stream 5 FIFO control register"]
pub mod s5fctrl;
#[doc = "S6CTRL (rw) register accessor: stream 6 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6ctrl`]
module"]
#[doc(alias = "S6CTRL")]
pub type S6ctrl = crate::Reg<s6ctrl::S6ctrlSpec>;
#[doc = "stream 6 control register"]
pub mod s6ctrl;
#[doc = "S6DTCNT (rw) register accessor: stream 6 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6dtcnt`]
module"]
#[doc(alias = "S6DTCNT")]
pub type S6dtcnt = crate::Reg<s6dtcnt::S6dtcntSpec>;
#[doc = "stream 6 number of data register"]
pub mod s6dtcnt;
#[doc = "S6PADDR (rw) register accessor: stream 6 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6paddr`]
module"]
#[doc(alias = "S6PADDR")]
pub type S6paddr = crate::Reg<s6paddr::S6paddrSpec>;
#[doc = "stream 6 peripheral address register"]
pub mod s6paddr;
#[doc = "S6M0ADDR (rw) register accessor: stream 6 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m0addr`]
module"]
#[doc(alias = "S6M0ADDR")]
pub type S6m0addr = crate::Reg<s6m0addr::S6m0addrSpec>;
#[doc = "stream 6 memory 0 address register"]
pub mod s6m0addr;
#[doc = "S6M1ADDR (rw) register accessor: stream 6 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6m1addr`]
module"]
#[doc(alias = "S6M1ADDR")]
pub type S6m1addr = crate::Reg<s6m1addr::S6m1addrSpec>;
#[doc = "stream 6 memory 1 address register"]
pub mod s6m1addr;
#[doc = "S6FCTRL (rw) register accessor: stream 6 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6fctrl`]
module"]
#[doc(alias = "S6FCTRL")]
pub type S6fctrl = crate::Reg<s6fctrl::S6fctrlSpec>;
#[doc = "stream 6 FIFO control register"]
pub mod s6fctrl;
#[doc = "S7CTRL (rw) register accessor: stream 7 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7ctrl`]
module"]
#[doc(alias = "S7CTRL")]
pub type S7ctrl = crate::Reg<s7ctrl::S7ctrlSpec>;
#[doc = "stream 7 control register"]
pub mod s7ctrl;
#[doc = "S7DTCNT (rw) register accessor: stream 7 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7dtcnt`]
module"]
#[doc(alias = "S7DTCNT")]
pub type S7dtcnt = crate::Reg<s7dtcnt::S7dtcntSpec>;
#[doc = "stream 7 number of data register"]
pub mod s7dtcnt;
#[doc = "S7PADDR (rw) register accessor: stream 7 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7paddr`]
module"]
#[doc(alias = "S7PADDR")]
pub type S7paddr = crate::Reg<s7paddr::S7paddrSpec>;
#[doc = "stream 7 peripheral address register"]
pub mod s7paddr;
#[doc = "S7M0ADDR (rw) register accessor: stream 7 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m0addr`]
module"]
#[doc(alias = "S7M0ADDR")]
pub type S7m0addr = crate::Reg<s7m0addr::S7m0addrSpec>;
#[doc = "stream 7 memory 0 address register"]
pub mod s7m0addr;
#[doc = "S7M1ADDR (rw) register accessor: stream 7 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7m1addr`]
module"]
#[doc(alias = "S7M1ADDR")]
pub type S7m1addr = crate::Reg<s7m1addr::S7m1addrSpec>;
#[doc = "stream 7 memory 1 address register"]
pub mod s7m1addr;
#[doc = "S7FCTRL (rw) register accessor: stream 7 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7fctrl`]
module"]
#[doc(alias = "S7FCTRL")]
pub type S7fctrl = crate::Reg<s7fctrl::S7fctrlSpec>;
#[doc = "stream 7 FIFO control register"]
pub mod s7fctrl;
#[doc = "S8CTRL (rw) register accessor: stream 8 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8ctrl`]
module"]
#[doc(alias = "S8CTRL")]
pub type S8ctrl = crate::Reg<s8ctrl::S8ctrlSpec>;
#[doc = "stream 8 control register"]
pub mod s8ctrl;
#[doc = "S8DTCNT (rw) register accessor: stream 8 number of data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8dtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8dtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8dtcnt`]
module"]
#[doc(alias = "S8DTCNT")]
pub type S8dtcnt = crate::Reg<s8dtcnt::S8dtcntSpec>;
#[doc = "stream 8 number of data register"]
pub mod s8dtcnt;
#[doc = "S8PADDR (rw) register accessor: stream 8 peripheral address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8paddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8paddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8paddr`]
module"]
#[doc(alias = "S8PADDR")]
pub type S8paddr = crate::Reg<s8paddr::S8paddrSpec>;
#[doc = "stream 8 peripheral address register"]
pub mod s8paddr;
#[doc = "S8M0ADDR (rw) register accessor: stream 8 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8m0addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8m0addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8m0addr`]
module"]
#[doc(alias = "S8M0ADDR")]
pub type S8m0addr = crate::Reg<s8m0addr::S8m0addrSpec>;
#[doc = "stream 8 memory 0 address register"]
pub mod s8m0addr;
#[doc = "S8M1ADDR (rw) register accessor: stream 8 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8m1addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8m1addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8m1addr`]
module"]
#[doc(alias = "S8M1ADDR")]
pub type S8m1addr = crate::Reg<s8m1addr::S8m1addrSpec>;
#[doc = "stream 8 memory 1 address register"]
pub mod s8m1addr;
#[doc = "S8FCTRL (rw) register accessor: stream 8 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8fctrl`]
module"]
#[doc(alias = "S8FCTRL")]
pub type S8fctrl = crate::Reg<s8fctrl::S8fctrlSpec>;
#[doc = "stream 8 FIFO control register"]
pub mod s8fctrl;
#[doc = "LLCTRL (rw) register accessor: DMA Link List Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llctrl`]
module"]
#[doc(alias = "LLCTRL")]
pub type Llctrl = crate::Reg<llctrl::LlctrlSpec>;
#[doc = "DMA Link List Control Register"]
pub mod llctrl;
#[doc = "S1LLP (rw) register accessor: Stream 1 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1llp`]
module"]
#[doc(alias = "S1LLP")]
pub type S1llp = crate::Reg<s1llp::S1llpSpec>;
#[doc = "Stream 1 Link List Pointer"]
pub mod s1llp;
#[doc = "S2LLP (rw) register accessor: Stream 2 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2llp`]
module"]
#[doc(alias = "S2LLP")]
pub type S2llp = crate::Reg<s2llp::S2llpSpec>;
#[doc = "Stream 2 Link List Pointer"]
pub mod s2llp;
#[doc = "S3LLP (rw) register accessor: Stream 3 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3llp`]
module"]
#[doc(alias = "S3LLP")]
pub type S3llp = crate::Reg<s3llp::S3llpSpec>;
#[doc = "Stream 3 Link List Pointer"]
pub mod s3llp;
#[doc = "S4LLP (rw) register accessor: Stream 4 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4llp`]
module"]
#[doc(alias = "S4LLP")]
pub type S4llp = crate::Reg<s4llp::S4llpSpec>;
#[doc = "Stream 4 Link List Pointer"]
pub mod s4llp;
#[doc = "S5LLP (rw) register accessor: Stream 5 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5llp`]
module"]
#[doc(alias = "S5LLP")]
pub type S5llp = crate::Reg<s5llp::S5llpSpec>;
#[doc = "Stream 5 Link List Pointer"]
pub mod s5llp;
#[doc = "S6LLP (rw) register accessor: Stream 6 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6llp`]
module"]
#[doc(alias = "S6LLP")]
pub type S6llp = crate::Reg<s6llp::S6llpSpec>;
#[doc = "Stream 6 Link List Pointer"]
pub mod s6llp;
#[doc = "S7LLP (rw) register accessor: Stream 7 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7llp`]
module"]
#[doc(alias = "S7LLP")]
pub type S7llp = crate::Reg<s7llp::S7llpSpec>;
#[doc = "Stream 7 Link List Pointer"]
pub mod s7llp;
#[doc = "S8LLP (rw) register accessor: Stream 8 Link List Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8llp`]
module"]
#[doc(alias = "S8LLP")]
pub type S8llp = crate::Reg<s8llp::S8llpSpec>;
#[doc = "Stream 8 Link List Pointer"]
pub mod s8llp;
#[doc = "S2DCTRL (rw) register accessor: EDMA 2D Transfer Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2dctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2dctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2dctrl`]
module"]
#[doc(alias = "S2DCTRL")]
pub type S2dctrl = crate::Reg<s2dctrl::S2dctrlSpec>;
#[doc = "EDMA 2D Transfer Control Register"]
pub mod s2dctrl;
#[doc = "S1_2DCNT (rw) register accessor: Stream 1 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1_2dcnt`]
module"]
#[doc(alias = "S1_2DCNT")]
pub type S1_2dcnt = crate::Reg<s1_2dcnt::S1_2dcntSpec>;
#[doc = "Stream 1 2D Transfer Count"]
pub mod s1_2dcnt;
#[doc = "S1_STRIDE (rw) register accessor: Stream 1 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s1_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s1_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1_stride`]
module"]
#[doc(alias = "S1_STRIDE")]
pub type S1Stride = crate::Reg<s1_stride::S1StrideSpec>;
#[doc = "Stream 1 2D Transfer Stride"]
pub mod s1_stride;
#[doc = "S2_2DCNT (rw) register accessor: Stream 2 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2_2dcnt`]
module"]
#[doc(alias = "S2_2DCNT")]
pub type S2_2dcnt = crate::Reg<s2_2dcnt::S2_2dcntSpec>;
#[doc = "Stream 2 2D Transfer Count"]
pub mod s2_2dcnt;
#[doc = "S2_STRIDE (rw) register accessor: Stream 2 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2_stride`]
module"]
#[doc(alias = "S2_STRIDE")]
pub type S2Stride = crate::Reg<s2_stride::S2StrideSpec>;
#[doc = "Stream 2 2D Transfer Stride"]
pub mod s2_stride;
#[doc = "S3_2DCNT (rw) register accessor: Stream 3 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3_2dcnt`]
module"]
#[doc(alias = "S3_2DCNT")]
pub type S3_2dcnt = crate::Reg<s3_2dcnt::S3_2dcntSpec>;
#[doc = "Stream 3 2D Transfer Count"]
pub mod s3_2dcnt;
#[doc = "S3_STRIDE (rw) register accessor: Stream 3 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s3_stride`]
module"]
#[doc(alias = "S3_STRIDE")]
pub type S3Stride = crate::Reg<s3_stride::S3StrideSpec>;
#[doc = "Stream 3 2D Transfer Stride"]
pub mod s3_stride;
#[doc = "S4_2DCNT (rw) register accessor: Stream 4 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4_2dcnt`]
module"]
#[doc(alias = "S4_2DCNT")]
pub type S4_2dcnt = crate::Reg<s4_2dcnt::S4_2dcntSpec>;
#[doc = "Stream 4 2D Transfer Count"]
pub mod s4_2dcnt;
#[doc = "S4_STRIDE (rw) register accessor: Stream 4 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s4_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s4_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s4_stride`]
module"]
#[doc(alias = "S4_STRIDE")]
pub type S4Stride = crate::Reg<s4_stride::S4StrideSpec>;
#[doc = "Stream 4 2D Transfer Stride"]
pub mod s4_stride;
#[doc = "S5_2DCNT (rw) register accessor: Stream 5 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5_2dcnt`]
module"]
#[doc(alias = "S5_2DCNT")]
pub type S5_2dcnt = crate::Reg<s5_2dcnt::S5_2dcntSpec>;
#[doc = "Stream 5 2D Transfer Count"]
pub mod s5_2dcnt;
#[doc = "S5_STRIDE (rw) register accessor: Stream 5 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s5_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s5_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s5_stride`]
module"]
#[doc(alias = "S5_STRIDE")]
pub type S5Stride = crate::Reg<s5_stride::S5StrideSpec>;
#[doc = "Stream 5 2D Transfer Stride"]
pub mod s5_stride;
#[doc = "S6_2DCNT (rw) register accessor: Stream 6 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6_2dcnt`]
module"]
#[doc(alias = "S6_2DCNT")]
pub type S6_2dcnt = crate::Reg<s6_2dcnt::S6_2dcntSpec>;
#[doc = "Stream 6 2D Transfer Count"]
pub mod s6_2dcnt;
#[doc = "S6_STRIDE (rw) register accessor: Stream 6 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s6_stride`]
module"]
#[doc(alias = "S6_STRIDE")]
pub type S6Stride = crate::Reg<s6_stride::S6StrideSpec>;
#[doc = "Stream 6 2D Transfer Stride"]
pub mod s6_stride;
#[doc = "S7_2DCNT (rw) register accessor: Stream 7 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7_2dcnt`]
module"]
#[doc(alias = "S7_2DCNT")]
pub type S7_2dcnt = crate::Reg<s7_2dcnt::S7_2dcntSpec>;
#[doc = "Stream 7 2D Transfer Count"]
pub mod s7_2dcnt;
#[doc = "S7_STRIDE (rw) register accessor: Stream 7 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s7_stride`]
module"]
#[doc(alias = "S7_STRIDE")]
pub type S7Stride = crate::Reg<s7_stride::S7StrideSpec>;
#[doc = "Stream 7 2D Transfer Stride"]
pub mod s7_stride;
#[doc = "S8_2DCNT (rw) register accessor: Stream 8 2D Transfer Count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8_2dcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8_2dcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8_2dcnt`]
module"]
#[doc(alias = "S8_2DCNT")]
pub type S8_2dcnt = crate::Reg<s8_2dcnt::S8_2dcntSpec>;
#[doc = "Stream 8 2D Transfer Count"]
pub mod s8_2dcnt;
#[doc = "S8_STRIDE (rw) register accessor: Stream 8 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8_stride::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8_stride::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s8_stride`]
module"]
#[doc(alias = "S8_STRIDE")]
pub type S8Stride = crate::Reg<s8_stride::S8StrideSpec>;
#[doc = "Stream 8 2D Transfer Stride"]
pub mod s8_stride;
#[doc = "SYNCEN (rw) register accessor: Sync Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syncen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syncen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncen`]
module"]
#[doc(alias = "SYNCEN")]
pub type Syncen = crate::Reg<syncen::SyncenSpec>;
#[doc = "Sync Enable"]
pub mod syncen;
#[doc = "MUXSEL (rw) register accessor: EDMA MUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxsel`]
module"]
#[doc(alias = "MUXSEL")]
pub type Muxsel = crate::Reg<muxsel::MuxselSpec>;
#[doc = "EDMA MUX Table Selection"]
pub mod muxsel;
#[doc = "MUXS1CTRL (rw) register accessor: Stream 1 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs1ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs1ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs1ctrl`]
module"]
#[doc(alias = "MUXS1CTRL")]
pub type Muxs1ctrl = crate::Reg<muxs1ctrl::Muxs1ctrlSpec>;
#[doc = "Stream 1 Configuration Register"]
pub mod muxs1ctrl;
#[doc = "MUXS2CTRL (rw) register accessor: Stream 2 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs2ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs2ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs2ctrl`]
module"]
#[doc(alias = "MUXS2CTRL")]
pub type Muxs2ctrl = crate::Reg<muxs2ctrl::Muxs2ctrlSpec>;
#[doc = "Stream 2 Configuration Register"]
pub mod muxs2ctrl;
#[doc = "MUXS3CTRL (rw) register accessor: Stream 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs3ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs3ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs3ctrl`]
module"]
#[doc(alias = "MUXS3CTRL")]
pub type Muxs3ctrl = crate::Reg<muxs3ctrl::Muxs3ctrlSpec>;
#[doc = "Stream 3 Configuration Register"]
pub mod muxs3ctrl;
#[doc = "MUXS4CTRL (rw) register accessor: Stream 4 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs4ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs4ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs4ctrl`]
module"]
#[doc(alias = "MUXS4CTRL")]
pub type Muxs4ctrl = crate::Reg<muxs4ctrl::Muxs4ctrlSpec>;
#[doc = "Stream 4 Configuration Register"]
pub mod muxs4ctrl;
#[doc = "MUXS5CTRL (rw) register accessor: Stream x Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs5ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs5ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs5ctrl`]
module"]
#[doc(alias = "MUXS5CTRL")]
pub type Muxs5ctrl = crate::Reg<muxs5ctrl::Muxs5ctrlSpec>;
#[doc = "Stream x Configuration Register"]
pub mod muxs5ctrl;
#[doc = "MUXS6CTRL (rw) register accessor: Stream 6 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs6ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs6ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs6ctrl`]
module"]
#[doc(alias = "MUXS6CTRL")]
pub type Muxs6ctrl = crate::Reg<muxs6ctrl::Muxs6ctrlSpec>;
#[doc = "Stream 6 Configuration Register"]
pub mod muxs6ctrl;
#[doc = "MUXS7CTRL (rw) register accessor: Stream 7 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs7ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs7ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs7ctrl`]
module"]
#[doc(alias = "MUXS7CTRL")]
pub type Muxs7ctrl = crate::Reg<muxs7ctrl::Muxs7ctrlSpec>;
#[doc = "Stream 7 Configuration Register"]
pub mod muxs7ctrl;
#[doc = "MUXS8CTRL (rw) register accessor: Stream 8 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs8ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs8ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxs8ctrl`]
module"]
#[doc(alias = "MUXS8CTRL")]
pub type Muxs8ctrl = crate::Reg<muxs8ctrl::Muxs8ctrlSpec>;
#[doc = "Stream 8 Configuration Register"]
pub mod muxs8ctrl;
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
