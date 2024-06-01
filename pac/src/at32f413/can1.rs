#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mctrl: Mctrl,
    msts: Msts,
    tsts: Tsts,
    rf0: Rf0,
    rf1: Rf1,
    inten: Inten,
    ests: Ests,
    btmg: Btmg,
    _reserved8: [u8; 0x0160],
    tmi0: Tmi0,
    tmc0: Tmc0,
    tmdtl0: Tmdtl0,
    tmdth0: Tmdth0,
    tmi1: Tmi1,
    tmc1: Tmc1,
    tmdtl1: Tmdtl1,
    tmdth1: Tmdth1,
    tmi2: Tmi2,
    tmc2: Tmc2,
    tmdtl2: Tmdtl2,
    tmdth2: Tmdth2,
    rfi0: Rfi0,
    rfc0: Rfc0,
    rfdtl0: Rfdtl0,
    rfdth0: Rfdth0,
    rfi1: Rfi1,
    rfc1: Rfc1,
    rfdtl1: Rfdtl1,
    rfdth1: Rfdth1,
    _reserved28: [u8; 0x30],
    fctrl: Fctrl,
    fmcfg: Fmcfg,
    _reserved30: [u8; 0x04],
    fbwcfg: Fbwcfg,
    _reserved31: [u8; 0x04],
    frf: Frf,
    _reserved32: [u8; 0x04],
    facfg: Facfg,
    _reserved33: [u8; 0x20],
    f0fb1: F0fb1,
    f0fb2: F0fb2,
    f1fb1: F1fb1,
    f1fb2: F1fb2,
    f2fb1: F2fb1,
    f2fb2: F2fb2,
    f3fb1: F3fb1,
    f3fb2: F3fb2,
    f4fb1: F4fb1,
    f4fb2: F4fb2,
    f5fb1: F5fb1,
    f5fb2: F5fb2,
    f6fb1: F6fb1,
    f6fb2: F6fb2,
    f7fb1: F7fb1,
    f7fb2: F7fb2,
    f8fb1: F8fb1,
    f8fb2: F8fb2,
    f9fb1: F9fb1,
    f9fb2: F9fb2,
    f10fb1: F10fb1,
    f10fb2: F10fb2,
    f11fb1: F11fb1,
    f11fb2: F11fb2,
    f12fb1: F12fb1,
    f12fb2: F12fb2,
    f13fb1: F13fb1,
    f13fb2: F13fb2,
}
impl RegisterBlock {
    #[doc = "0x00 - Main control register"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x04 - Main status register"]
    #[inline(always)]
    pub const fn msts(&self) -> &Msts {
        &self.msts
    }
    #[doc = "0x08 - Transmit status register"]
    #[inline(always)]
    pub const fn tsts(&self) -> &Tsts {
        &self.tsts
    }
    #[doc = "0x0c - Receive FIFO 0 register"]
    #[inline(always)]
    pub const fn rf0(&self) -> &Rf0 {
        &self.rf0
    }
    #[doc = "0x10 - Receive FIFO 1 register"]
    #[inline(always)]
    pub const fn rf1(&self) -> &Rf1 {
        &self.rf1
    }
    #[doc = "0x14 - Interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x18 - Error status register"]
    #[inline(always)]
    pub const fn ests(&self) -> &Ests {
        &self.ests
    }
    #[doc = "0x1c - Bit timing register"]
    #[inline(always)]
    pub const fn btmg(&self) -> &Btmg {
        &self.btmg
    }
    #[doc = "0x180 - Transmit mailbox 0 identifier register"]
    #[inline(always)]
    pub const fn tmi0(&self) -> &Tmi0 {
        &self.tmi0
    }
    #[doc = "0x184 - Transmit mailbox 0 data length and time stamp register"]
    #[inline(always)]
    pub const fn tmc0(&self) -> &Tmc0 {
        &self.tmc0
    }
    #[doc = "0x188 - Transmit mailbox 0 low byte data register"]
    #[inline(always)]
    pub const fn tmdtl0(&self) -> &Tmdtl0 {
        &self.tmdtl0
    }
    #[doc = "0x18c - Transmit mailbox 0 high byte data register"]
    #[inline(always)]
    pub const fn tmdth0(&self) -> &Tmdth0 {
        &self.tmdth0
    }
    #[doc = "0x190 - Transmit mailbox 1 identifier register"]
    #[inline(always)]
    pub const fn tmi1(&self) -> &Tmi1 {
        &self.tmi1
    }
    #[doc = "0x194 - Transmit mailbox 1 data length and time stamp register"]
    #[inline(always)]
    pub const fn tmc1(&self) -> &Tmc1 {
        &self.tmc1
    }
    #[doc = "0x198 - Transmit mailbox 1 low byte data register"]
    #[inline(always)]
    pub const fn tmdtl1(&self) -> &Tmdtl1 {
        &self.tmdtl1
    }
    #[doc = "0x19c - Transmit mailbox 1 high byte data register"]
    #[inline(always)]
    pub const fn tmdth1(&self) -> &Tmdth1 {
        &self.tmdth1
    }
    #[doc = "0x1a0 - Transmit mailbox 2 identifier register"]
    #[inline(always)]
    pub const fn tmi2(&self) -> &Tmi2 {
        &self.tmi2
    }
    #[doc = "0x1a4 - Transmit mailbox 2 data length and time stamp register"]
    #[inline(always)]
    pub const fn tmc2(&self) -> &Tmc2 {
        &self.tmc2
    }
    #[doc = "0x1a8 - Transmit mailbox 2 low byte data register"]
    #[inline(always)]
    pub const fn tmdtl2(&self) -> &Tmdtl2 {
        &self.tmdtl2
    }
    #[doc = "0x1ac - Transmit mailbox 2 high byte data register"]
    #[inline(always)]
    pub const fn tmdth2(&self) -> &Tmdth2 {
        &self.tmdth2
    }
    #[doc = "0x1b0 - Receive FIFO 0 register"]
    #[inline(always)]
    pub const fn rfi0(&self) -> &Rfi0 {
        &self.rfi0
    }
    #[doc = "0x1b4 - Receive FIFO 0 data length and time stamp register"]
    #[inline(always)]
    pub const fn rfc0(&self) -> &Rfc0 {
        &self.rfc0
    }
    #[doc = "0x1b8 - Receive FIFO 0 low byte data register"]
    #[inline(always)]
    pub const fn rfdtl0(&self) -> &Rfdtl0 {
        &self.rfdtl0
    }
    #[doc = "0x1bc - Receive FIFO 0 high byte data register"]
    #[inline(always)]
    pub const fn rfdth0(&self) -> &Rfdth0 {
        &self.rfdth0
    }
    #[doc = "0x1c0 - Receive FIFO 1 register"]
    #[inline(always)]
    pub const fn rfi1(&self) -> &Rfi1 {
        &self.rfi1
    }
    #[doc = "0x1c4 - Receive FIFO 1 data length and time stamp register"]
    #[inline(always)]
    pub const fn rfc1(&self) -> &Rfc1 {
        &self.rfc1
    }
    #[doc = "0x1c8 - Receive FIFO 1 low byte data register"]
    #[inline(always)]
    pub const fn rfdtl1(&self) -> &Rfdtl1 {
        &self.rfdtl1
    }
    #[doc = "0x1cc - Receive FIFO 1 high byte data register"]
    #[inline(always)]
    pub const fn rfdth1(&self) -> &Rfdth1 {
        &self.rfdth1
    }
    #[doc = "0x200 - Filter control register"]
    #[inline(always)]
    pub const fn fctrl(&self) -> &Fctrl {
        &self.fctrl
    }
    #[doc = "0x204 - Filter mode config register"]
    #[inline(always)]
    pub const fn fmcfg(&self) -> &Fmcfg {
        &self.fmcfg
    }
    #[doc = "0x20c - Filter bit width config register"]
    #[inline(always)]
    pub const fn fbwcfg(&self) -> &Fbwcfg {
        &self.fbwcfg
    }
    #[doc = "0x214 - Filter related FIFO register"]
    #[inline(always)]
    pub const fn frf(&self) -> &Frf {
        &self.frf
    }
    #[doc = "0x21c - Filter activate configuration register"]
    #[inline(always)]
    pub const fn facfg(&self) -> &Facfg {
        &self.facfg
    }
    #[doc = "0x240 - Filter bank 0 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f0fb1(&self) -> &F0fb1 {
        &self.f0fb1
    }
    #[doc = "0x244 - Filter bank 0 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f0fb2(&self) -> &F0fb2 {
        &self.f0fb2
    }
    #[doc = "0x248 - Filter bank 1 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f1fb1(&self) -> &F1fb1 {
        &self.f1fb1
    }
    #[doc = "0x24c - Filter bank 1 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f1fb2(&self) -> &F1fb2 {
        &self.f1fb2
    }
    #[doc = "0x250 - Filter bank 2 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f2fb1(&self) -> &F2fb1 {
        &self.f2fb1
    }
    #[doc = "0x254 - Filter bank 2 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f2fb2(&self) -> &F2fb2 {
        &self.f2fb2
    }
    #[doc = "0x258 - Filter bank 3 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f3fb1(&self) -> &F3fb1 {
        &self.f3fb1
    }
    #[doc = "0x25c - Filter bank 3 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f3fb2(&self) -> &F3fb2 {
        &self.f3fb2
    }
    #[doc = "0x260 - Filter bank 4 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f4fb1(&self) -> &F4fb1 {
        &self.f4fb1
    }
    #[doc = "0x264 - Filter bank 4 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f4fb2(&self) -> &F4fb2 {
        &self.f4fb2
    }
    #[doc = "0x268 - Filter bank 5 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f5fb1(&self) -> &F5fb1 {
        &self.f5fb1
    }
    #[doc = "0x26c - Filter bank 5 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f5fb2(&self) -> &F5fb2 {
        &self.f5fb2
    }
    #[doc = "0x270 - Filter bank 6 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f6fb1(&self) -> &F6fb1 {
        &self.f6fb1
    }
    #[doc = "0x274 - Filter bank 6 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f6fb2(&self) -> &F6fb2 {
        &self.f6fb2
    }
    #[doc = "0x278 - Filter bank 7 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f7fb1(&self) -> &F7fb1 {
        &self.f7fb1
    }
    #[doc = "0x27c - Filter bank 7 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f7fb2(&self) -> &F7fb2 {
        &self.f7fb2
    }
    #[doc = "0x280 - Filter bank 8 filtrate bit filtrate bit register 1"]
    #[inline(always)]
    pub const fn f8fb1(&self) -> &F8fb1 {
        &self.f8fb1
    }
    #[doc = "0x284 - Filter bank 8 filtrate bit filtrate bit register 2"]
    #[inline(always)]
    pub const fn f8fb2(&self) -> &F8fb2 {
        &self.f8fb2
    }
    #[doc = "0x288 - Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1"]
    #[inline(always)]
    pub const fn f9fb1(&self) -> &F9fb1 {
        &self.f9fb1
    }
    #[doc = "0x28c - Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2"]
    #[inline(always)]
    pub const fn f9fb2(&self) -> &F9fb2 {
        &self.f9fb2
    }
    #[doc = "0x290 - Filter bank 10 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f10fb1(&self) -> &F10fb1 {
        &self.f10fb1
    }
    #[doc = "0x294 - Filter bank 10 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f10fb2(&self) -> &F10fb2 {
        &self.f10fb2
    }
    #[doc = "0x298 - Filter bank 11 filtrate bit register 1"]
    #[inline(always)]
    pub const fn f11fb1(&self) -> &F11fb1 {
        &self.f11fb1
    }
    #[doc = "0x29c - Filter bank 11 filtrate bit register 2"]
    #[inline(always)]
    pub const fn f11fb2(&self) -> &F11fb2 {
        &self.f11fb2
    }
    #[doc = "0x2a0 - Filter bank 12 filtrate bit filtrate bit register 1"]
    #[inline(always)]
    pub const fn f12fb1(&self) -> &F12fb1 {
        &self.f12fb1
    }
    #[doc = "0x2a4 - Filter bank 12 filtrate bit filtrate bit register 2"]
    #[inline(always)]
    pub const fn f12fb2(&self) -> &F12fb2 {
        &self.f12fb2
    }
    #[doc = "0x2a8 - Filter bank 13 filtrate bit filtrate bit register 1"]
    #[inline(always)]
    pub const fn f13fb1(&self) -> &F13fb1 {
        &self.f13fb1
    }
    #[doc = "0x2ac - Filter bank 13 filtrate bit filtrate bit register 2"]
    #[inline(always)]
    pub const fn f13fb2(&self) -> &F13fb2 {
        &self.f13fb2
    }
}
#[doc = "MCTRL (rw) register accessor: Main control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Main control register"]
pub mod mctrl;
#[doc = "MSTS (rw) register accessor: Main status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msts`]
module"]
#[doc(alias = "MSTS")]
pub type Msts = crate::Reg<msts::MstsSpec>;
#[doc = "Main status register"]
pub mod msts;
#[doc = "TSTS (rw) register accessor: Transmit status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsts`]
module"]
#[doc(alias = "TSTS")]
pub type Tsts = crate::Reg<tsts::TstsSpec>;
#[doc = "Transmit status register"]
pub mod tsts;
#[doc = "RF0 (rw) register accessor: Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf0`]
module"]
#[doc(alias = "RF0")]
pub type Rf0 = crate::Reg<rf0::Rf0Spec>;
#[doc = "Receive FIFO 0 register"]
pub mod rf0;
#[doc = "RF1 (rw) register accessor: Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rf1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rf1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rf1`]
module"]
#[doc(alias = "RF1")]
pub type Rf1 = crate::Reg<rf1::Rf1Spec>;
#[doc = "Receive FIFO 1 register"]
pub mod rf1;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "ESTS (rw) register accessor: Error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ests::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ests`]
module"]
#[doc(alias = "ESTS")]
pub type Ests = crate::Reg<ests::EstsSpec>;
#[doc = "Error status register"]
pub mod ests;
#[doc = "BTMG (rw) register accessor: Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btmg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btmg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@btmg`]
module"]
#[doc(alias = "BTMG")]
pub type Btmg = crate::Reg<btmg::BtmgSpec>;
#[doc = "Bit timing register"]
pub mod btmg;
#[doc = "TMI0 (rw) register accessor: Transmit mailbox 0 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi0`]
module"]
#[doc(alias = "TMI0")]
pub type Tmi0 = crate::Reg<tmi0::Tmi0Spec>;
#[doc = "Transmit mailbox 0 identifier register"]
pub mod tmi0;
#[doc = "TMC0 (rw) register accessor: Transmit mailbox 0 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmc0`]
module"]
#[doc(alias = "TMC0")]
pub type Tmc0 = crate::Reg<tmc0::Tmc0Spec>;
#[doc = "Transmit mailbox 0 data length and time stamp register"]
pub mod tmc0;
#[doc = "TMDTL0 (rw) register accessor: Transmit mailbox 0 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdtl0`]
module"]
#[doc(alias = "TMDTL0")]
pub type Tmdtl0 = crate::Reg<tmdtl0::Tmdtl0Spec>;
#[doc = "Transmit mailbox 0 low byte data register"]
pub mod tmdtl0;
#[doc = "TMDTH0 (rw) register accessor: Transmit mailbox 0 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdth0`]
module"]
#[doc(alias = "TMDTH0")]
pub type Tmdth0 = crate::Reg<tmdth0::Tmdth0Spec>;
#[doc = "Transmit mailbox 0 high byte data register"]
pub mod tmdth0;
#[doc = "TMI1 (rw) register accessor: Transmit mailbox 1 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi1`]
module"]
#[doc(alias = "TMI1")]
pub type Tmi1 = crate::Reg<tmi1::Tmi1Spec>;
#[doc = "Transmit mailbox 1 identifier register"]
pub mod tmi1;
#[doc = "TMC1 (rw) register accessor: Transmit mailbox 1 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmc1`]
module"]
#[doc(alias = "TMC1")]
pub type Tmc1 = crate::Reg<tmc1::Tmc1Spec>;
#[doc = "Transmit mailbox 1 data length and time stamp register"]
pub mod tmc1;
#[doc = "TMDTL1 (rw) register accessor: Transmit mailbox 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdtl1`]
module"]
#[doc(alias = "TMDTL1")]
pub type Tmdtl1 = crate::Reg<tmdtl1::Tmdtl1Spec>;
#[doc = "Transmit mailbox 1 low byte data register"]
pub mod tmdtl1;
#[doc = "TMDTH1 (rw) register accessor: Transmit mailbox 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdth1`]
module"]
#[doc(alias = "TMDTH1")]
pub type Tmdth1 = crate::Reg<tmdth1::Tmdth1Spec>;
#[doc = "Transmit mailbox 1 high byte data register"]
pub mod tmdth1;
#[doc = "TMI2 (rw) register accessor: Transmit mailbox 2 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmi2`]
module"]
#[doc(alias = "TMI2")]
pub type Tmi2 = crate::Reg<tmi2::Tmi2Spec>;
#[doc = "Transmit mailbox 2 identifier register"]
pub mod tmi2;
#[doc = "TMC2 (rw) register accessor: Transmit mailbox 2 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmc2`]
module"]
#[doc(alias = "TMC2")]
pub type Tmc2 = crate::Reg<tmc2::Tmc2Spec>;
#[doc = "Transmit mailbox 2 data length and time stamp register"]
pub mod tmc2;
#[doc = "TMDTL2 (rw) register accessor: Transmit mailbox 2 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdtl2`]
module"]
#[doc(alias = "TMDTL2")]
pub type Tmdtl2 = crate::Reg<tmdtl2::Tmdtl2Spec>;
#[doc = "Transmit mailbox 2 low byte data register"]
pub mod tmdtl2;
#[doc = "TMDTH2 (rw) register accessor: Transmit mailbox 2 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmdth2`]
module"]
#[doc(alias = "TMDTH2")]
pub type Tmdth2 = crate::Reg<tmdth2::Tmdth2Spec>;
#[doc = "Transmit mailbox 2 high byte data register"]
pub mod tmdth2;
#[doc = "RFI0 (r) register accessor: Receive FIFO 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfi0`]
module"]
#[doc(alias = "RFI0")]
pub type Rfi0 = crate::Reg<rfi0::Rfi0Spec>;
#[doc = "Receive FIFO 0 register"]
pub mod rfi0;
#[doc = "RFC0 (r) register accessor: Receive FIFO 0 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc0`]
module"]
#[doc(alias = "RFC0")]
pub type Rfc0 = crate::Reg<rfc0::Rfc0Spec>;
#[doc = "Receive FIFO 0 data length and time stamp register"]
pub mod rfc0;
#[doc = "RFDTL0 (r) register accessor: Receive FIFO 0 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdtl0`]
module"]
#[doc(alias = "RFDTL0")]
pub type Rfdtl0 = crate::Reg<rfdtl0::Rfdtl0Spec>;
#[doc = "Receive FIFO 0 low byte data register"]
pub mod rfdtl0;
#[doc = "RFDTH0 (r) register accessor: Receive FIFO 0 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdth0`]
module"]
#[doc(alias = "RFDTH0")]
pub type Rfdth0 = crate::Reg<rfdth0::Rfdth0Spec>;
#[doc = "Receive FIFO 0 high byte data register"]
pub mod rfdth0;
#[doc = "RFI1 (r) register accessor: Receive FIFO 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfi1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfi1`]
module"]
#[doc(alias = "RFI1")]
pub type Rfi1 = crate::Reg<rfi1::Rfi1Spec>;
#[doc = "Receive FIFO 1 register"]
pub mod rfi1;
#[doc = "RFC1 (r) register accessor: Receive FIFO 1 data length and time stamp register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfc1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfc1`]
module"]
#[doc(alias = "RFC1")]
pub type Rfc1 = crate::Reg<rfc1::Rfc1Spec>;
#[doc = "Receive FIFO 1 data length and time stamp register"]
pub mod rfc1;
#[doc = "RFDTL1 (r) register accessor: Receive FIFO 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdtl1`]
module"]
#[doc(alias = "RFDTL1")]
pub type Rfdtl1 = crate::Reg<rfdtl1::Rfdtl1Spec>;
#[doc = "Receive FIFO 1 low byte data register"]
pub mod rfdtl1;
#[doc = "RFDTH1 (r) register accessor: Receive FIFO 1 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfdth1`]
module"]
#[doc(alias = "RFDTH1")]
pub type Rfdth1 = crate::Reg<rfdth1::Rfdth1Spec>;
#[doc = "Receive FIFO 1 high byte data register"]
pub mod rfdth1;
#[doc = "FCTRL (rw) register accessor: Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fctrl`]
module"]
#[doc(alias = "FCTRL")]
pub type Fctrl = crate::Reg<fctrl::FctrlSpec>;
#[doc = "Filter control register"]
pub mod fctrl;
#[doc = "FMCFG (rw) register accessor: Filter mode config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcfg`]
module"]
#[doc(alias = "FMCFG")]
pub type Fmcfg = crate::Reg<fmcfg::FmcfgSpec>;
#[doc = "Filter mode config register"]
pub mod fmcfg;
#[doc = "FBWCFG (rw) register accessor: Filter bit width config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbwcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbwcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbwcfg`]
module"]
#[doc(alias = "FBWCFG")]
pub type Fbwcfg = crate::Reg<fbwcfg::FbwcfgSpec>;
#[doc = "Filter bit width config register"]
pub mod fbwcfg;
#[doc = "FRF (rw) register accessor: Filter related FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frf`]
module"]
#[doc(alias = "FRF")]
pub type Frf = crate::Reg<frf::FrfSpec>;
#[doc = "Filter related FIFO register"]
pub mod frf;
#[doc = "FACFG (rw) register accessor: Filter activate configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`facfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`facfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@facfg`]
module"]
#[doc(alias = "FACFG")]
pub type Facfg = crate::Reg<facfg::FacfgSpec>;
#[doc = "Filter activate configuration register"]
pub mod facfg;
#[doc = "F0FB1 (rw) register accessor: Filter bank 0 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0fb1`]
module"]
#[doc(alias = "F0FB1")]
pub type F0fb1 = crate::Reg<f0fb1::F0fb1Spec>;
#[doc = "Filter bank 0 filtrate bit register 1"]
pub mod f0fb1;
#[doc = "F0FB2 (rw) register accessor: Filter bank 0 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f0fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f0fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f0fb2`]
module"]
#[doc(alias = "F0FB2")]
pub type F0fb2 = crate::Reg<f0fb2::F0fb2Spec>;
#[doc = "Filter bank 0 filtrate bit register 2"]
pub mod f0fb2;
#[doc = "F1FB1 (rw) register accessor: Filter bank 1 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1fb1`]
module"]
#[doc(alias = "F1FB1")]
pub type F1fb1 = crate::Reg<f1fb1::F1fb1Spec>;
#[doc = "Filter bank 1 filtrate bit register 1"]
pub mod f1fb1;
#[doc = "F1FB2 (rw) register accessor: Filter bank 1 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f1fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f1fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f1fb2`]
module"]
#[doc(alias = "F1FB2")]
pub type F1fb2 = crate::Reg<f1fb2::F1fb2Spec>;
#[doc = "Filter bank 1 filtrate bit register 2"]
pub mod f1fb2;
#[doc = "F2FB1 (rw) register accessor: Filter bank 2 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2fb1`]
module"]
#[doc(alias = "F2FB1")]
pub type F2fb1 = crate::Reg<f2fb1::F2fb1Spec>;
#[doc = "Filter bank 2 filtrate bit register 1"]
pub mod f2fb1;
#[doc = "F2FB2 (rw) register accessor: Filter bank 2 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f2fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f2fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f2fb2`]
module"]
#[doc(alias = "F2FB2")]
pub type F2fb2 = crate::Reg<f2fb2::F2fb2Spec>;
#[doc = "Filter bank 2 filtrate bit register 2"]
pub mod f2fb2;
#[doc = "F3FB1 (rw) register accessor: Filter bank 3 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3fb1`]
module"]
#[doc(alias = "F3FB1")]
pub type F3fb1 = crate::Reg<f3fb1::F3fb1Spec>;
#[doc = "Filter bank 3 filtrate bit register 1"]
pub mod f3fb1;
#[doc = "F3FB2 (rw) register accessor: Filter bank 3 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f3fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f3fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f3fb2`]
module"]
#[doc(alias = "F3FB2")]
pub type F3fb2 = crate::Reg<f3fb2::F3fb2Spec>;
#[doc = "Filter bank 3 filtrate bit register 2"]
pub mod f3fb2;
#[doc = "F4FB1 (rw) register accessor: Filter bank 4 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4fb1`]
module"]
#[doc(alias = "F4FB1")]
pub type F4fb1 = crate::Reg<f4fb1::F4fb1Spec>;
#[doc = "Filter bank 4 filtrate bit register 1"]
pub mod f4fb1;
#[doc = "F4FB2 (rw) register accessor: Filter bank 4 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f4fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f4fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f4fb2`]
module"]
#[doc(alias = "F4FB2")]
pub type F4fb2 = crate::Reg<f4fb2::F4fb2Spec>;
#[doc = "Filter bank 4 filtrate bit register 2"]
pub mod f4fb2;
#[doc = "F5FB1 (rw) register accessor: Filter bank 5 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5fb1`]
module"]
#[doc(alias = "F5FB1")]
pub type F5fb1 = crate::Reg<f5fb1::F5fb1Spec>;
#[doc = "Filter bank 5 filtrate bit register 1"]
pub mod f5fb1;
#[doc = "F5FB2 (rw) register accessor: Filter bank 5 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f5fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f5fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f5fb2`]
module"]
#[doc(alias = "F5FB2")]
pub type F5fb2 = crate::Reg<f5fb2::F5fb2Spec>;
#[doc = "Filter bank 5 filtrate bit register 2"]
pub mod f5fb2;
#[doc = "F6FB1 (rw) register accessor: Filter bank 6 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6fb1`]
module"]
#[doc(alias = "F6FB1")]
pub type F6fb1 = crate::Reg<f6fb1::F6fb1Spec>;
#[doc = "Filter bank 6 filtrate bit register 1"]
pub mod f6fb1;
#[doc = "F6FB2 (rw) register accessor: Filter bank 6 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f6fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f6fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f6fb2`]
module"]
#[doc(alias = "F6FB2")]
pub type F6fb2 = crate::Reg<f6fb2::F6fb2Spec>;
#[doc = "Filter bank 6 filtrate bit register 2"]
pub mod f6fb2;
#[doc = "F7FB1 (rw) register accessor: Filter bank 7 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7fb1`]
module"]
#[doc(alias = "F7FB1")]
pub type F7fb1 = crate::Reg<f7fb1::F7fb1Spec>;
#[doc = "Filter bank 7 filtrate bit register 1"]
pub mod f7fb1;
#[doc = "F7FB2 (rw) register accessor: Filter bank 7 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f7fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f7fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f7fb2`]
module"]
#[doc(alias = "F7FB2")]
pub type F7fb2 = crate::Reg<f7fb2::F7fb2Spec>;
#[doc = "Filter bank 7 filtrate bit register 2"]
pub mod f7fb2;
#[doc = "F8FB1 (rw) register accessor: Filter bank 8 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8fb1`]
module"]
#[doc(alias = "F8FB1")]
pub type F8fb1 = crate::Reg<f8fb1::F8fb1Spec>;
#[doc = "Filter bank 8 filtrate bit filtrate bit register 1"]
pub mod f8fb1;
#[doc = "F8FB2 (rw) register accessor: Filter bank 8 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f8fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f8fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f8fb2`]
module"]
#[doc(alias = "F8FB2")]
pub type F8fb2 = crate::Reg<f8fb2::F8fb2Spec>;
#[doc = "Filter bank 8 filtrate bit filtrate bit register 2"]
pub mod f8fb2;
#[doc = "F9FB1 (rw) register accessor: Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9fb1`]
module"]
#[doc(alias = "F9FB1")]
pub type F9fb1 = crate::Reg<f9fb1::F9fb1Spec>;
#[doc = "Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 1"]
pub mod f9fb1;
#[doc = "F9FB2 (rw) register accessor: Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f9fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f9fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f9fb2`]
module"]
#[doc(alias = "F9FB2")]
pub type F9fb2 = crate::Reg<f9fb2::F9fb2Spec>;
#[doc = "Filter bank 9 filtrate bit filtrate bit filtrate bit filtrate bit filtrate bit register 2"]
pub mod f9fb2;
#[doc = "F10FB1 (rw) register accessor: Filter bank 10 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10fb1`]
module"]
#[doc(alias = "F10FB1")]
pub type F10fb1 = crate::Reg<f10fb1::F10fb1Spec>;
#[doc = "Filter bank 10 filtrate bit register 1"]
pub mod f10fb1;
#[doc = "F10FB2 (rw) register accessor: Filter bank 10 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f10fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f10fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f10fb2`]
module"]
#[doc(alias = "F10FB2")]
pub type F10fb2 = crate::Reg<f10fb2::F10fb2Spec>;
#[doc = "Filter bank 10 filtrate bit register 2"]
pub mod f10fb2;
#[doc = "F11FB1 (rw) register accessor: Filter bank 11 filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11fb1`]
module"]
#[doc(alias = "F11FB1")]
pub type F11fb1 = crate::Reg<f11fb1::F11fb1Spec>;
#[doc = "Filter bank 11 filtrate bit register 1"]
pub mod f11fb1;
#[doc = "F11FB2 (rw) register accessor: Filter bank 11 filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f11fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f11fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f11fb2`]
module"]
#[doc(alias = "F11FB2")]
pub type F11fb2 = crate::Reg<f11fb2::F11fb2Spec>;
#[doc = "Filter bank 11 filtrate bit register 2"]
pub mod f11fb2;
#[doc = "F12FB1 (rw) register accessor: Filter bank 12 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12fb1`]
module"]
#[doc(alias = "F12FB1")]
pub type F12fb1 = crate::Reg<f12fb1::F12fb1Spec>;
#[doc = "Filter bank 12 filtrate bit filtrate bit register 1"]
pub mod f12fb1;
#[doc = "F12FB2 (rw) register accessor: Filter bank 12 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f12fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f12fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f12fb2`]
module"]
#[doc(alias = "F12FB2")]
pub type F12fb2 = crate::Reg<f12fb2::F12fb2Spec>;
#[doc = "Filter bank 12 filtrate bit filtrate bit register 2"]
pub mod f12fb2;
#[doc = "F13FB1 (rw) register accessor: Filter bank 13 filtrate bit filtrate bit register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13fb1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13fb1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13fb1`]
module"]
#[doc(alias = "F13FB1")]
pub type F13fb1 = crate::Reg<f13fb1::F13fb1Spec>;
#[doc = "Filter bank 13 filtrate bit filtrate bit register 1"]
pub mod f13fb1;
#[doc = "F13FB2 (rw) register accessor: Filter bank 13 filtrate bit filtrate bit register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`f13fb2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`f13fb2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@f13fb2`]
module"]
#[doc(alias = "F13FB2")]
pub type F13fb2 = crate::Reg<f13fb2::F13fb2Spec>;
#[doc = "Filter bank 13 filtrate bit filtrate bit register 2"]
pub mod f13fb2;
