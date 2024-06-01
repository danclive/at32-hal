#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    macctrl: Macctrl,
    macfrmf: Macfrmf,
    machth: Machth,
    machtl: Machtl,
    macmiiaddr: Macmiiaddr,
    macmiidt: Macmiidt,
    macfctrl: Macfctrl,
    macvlt: Macvlt,
    _reserved8: [u8; 0x08],
    macrwff: Macrwff,
    macpmtctrlsts: Macpmtctrlsts,
    _reserved10: [u8; 0x08],
    macists: Macists,
    macimr: Macimr,
    maca0h: Maca0h,
    maca0l: Maca0l,
    maca1h: Maca1h,
    maca1l: Maca1l,
    maca2h: Maca2h,
    maca2l: Maca2l,
    maca3h: Maca3h,
    maca3l: Maca3l,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MAC configuration register"]
    #[inline(always)]
    pub const fn macctrl(&self) -> &Macctrl {
        &self.macctrl
    }
    #[doc = "0x04 - Ethernet MAC frame filter register"]
    #[inline(always)]
    pub const fn macfrmf(&self) -> &Macfrmf {
        &self.macfrmf
    }
    #[doc = "0x08 - Ethernet MAC hash table high register"]
    #[inline(always)]
    pub const fn machth(&self) -> &Machth {
        &self.machth
    }
    #[doc = "0x0c - Ethernet MAC hash table low register"]
    #[inline(always)]
    pub const fn machtl(&self) -> &Machtl {
        &self.machtl
    }
    #[doc = "0x10 - Ethernet MAC MII address register"]
    #[inline(always)]
    pub const fn macmiiaddr(&self) -> &Macmiiaddr {
        &self.macmiiaddr
    }
    #[doc = "0x14 - Ethernet MAC MII data register"]
    #[inline(always)]
    pub const fn macmiidt(&self) -> &Macmiidt {
        &self.macmiidt
    }
    #[doc = "0x18 - Ethernet MAC flow control register"]
    #[inline(always)]
    pub const fn macfctrl(&self) -> &Macfctrl {
        &self.macfctrl
    }
    #[doc = "0x1c - Ethernet MAC VLAN tag register"]
    #[inline(always)]
    pub const fn macvlt(&self) -> &Macvlt {
        &self.macvlt
    }
    #[doc = "0x28 - Ethernet MAC remote wakeup frame filter register"]
    #[inline(always)]
    pub const fn macrwff(&self) -> &Macrwff {
        &self.macrwff
    }
    #[doc = "0x2c - Ethernet MAC PMT control and status register"]
    #[inline(always)]
    pub const fn macpmtctrlsts(&self) -> &Macpmtctrlsts {
        &self.macpmtctrlsts
    }
    #[doc = "0x38 - Ethernet MAC interrupt status register"]
    #[inline(always)]
    pub const fn macists(&self) -> &Macists {
        &self.macists
    }
    #[doc = "0x3c - Ethernet MAC interrupt mask register"]
    #[inline(always)]
    pub const fn macimr(&self) -> &Macimr {
        &self.macimr
    }
    #[doc = "0x40 - Ethernet MAC address 0 high register"]
    #[inline(always)]
    pub const fn maca0h(&self) -> &Maca0h {
        &self.maca0h
    }
    #[doc = "0x44 - Ethernet MAC address 0 low register"]
    #[inline(always)]
    pub const fn maca0l(&self) -> &Maca0l {
        &self.maca0l
    }
    #[doc = "0x48 - Ethernet MAC address 1 high register"]
    #[inline(always)]
    pub const fn maca1h(&self) -> &Maca1h {
        &self.maca1h
    }
    #[doc = "0x4c - Ethernet MAC address1 low register"]
    #[inline(always)]
    pub const fn maca1l(&self) -> &Maca1l {
        &self.maca1l
    }
    #[doc = "0x50 - Ethernet MAC address 2 high register"]
    #[inline(always)]
    pub const fn maca2h(&self) -> &Maca2h {
        &self.maca2h
    }
    #[doc = "0x54 - Ethernet MAC address 2 low register"]
    #[inline(always)]
    pub const fn maca2l(&self) -> &Maca2l {
        &self.maca2l
    }
    #[doc = "0x58 - Ethernet MAC address 3 high register"]
    #[inline(always)]
    pub const fn maca3h(&self) -> &Maca3h {
        &self.maca3h
    }
    #[doc = "0x5c - Ethernet MAC address 3 low register"]
    #[inline(always)]
    pub const fn maca3l(&self) -> &Maca3l {
        &self.maca3l
    }
}
#[doc = "MACCTRL (rw) register accessor: Ethernet MAC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macctrl`]
module"]
#[doc(alias = "MACCTRL")]
pub type Macctrl = crate::Reg<macctrl::MacctrlSpec>;
#[doc = "Ethernet MAC configuration register"]
pub mod macctrl;
#[doc = "MACFRMF (rw) register accessor: Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfrmf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfrmf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macfrmf`]
module"]
#[doc(alias = "MACFRMF")]
pub type Macfrmf = crate::Reg<macfrmf::MacfrmfSpec>;
#[doc = "Ethernet MAC frame filter register"]
pub mod macfrmf;
#[doc = "MACHTH (rw) register accessor: Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machth`]
module"]
#[doc(alias = "MACHTH")]
pub type Machth = crate::Reg<machth::MachthSpec>;
#[doc = "Ethernet MAC hash table high register"]
pub mod machth;
#[doc = "MACHTL (rw) register accessor: Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@machtl`]
module"]
#[doc(alias = "MACHTL")]
pub type Machtl = crate::Reg<machtl::MachtlSpec>;
#[doc = "Ethernet MAC hash table low register"]
pub mod machtl;
#[doc = "MACMIIADDR (rw) register accessor: Ethernet MAC MII address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiiaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiiaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiiaddr`]
module"]
#[doc(alias = "MACMIIADDR")]
pub type Macmiiaddr = crate::Reg<macmiiaddr::MacmiiaddrSpec>;
#[doc = "Ethernet MAC MII address register"]
pub mod macmiiaddr;
#[doc = "MACMIIDT (rw) register accessor: Ethernet MAC MII data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macmiidt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macmiidt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmiidt`]
module"]
#[doc(alias = "MACMIIDT")]
pub type Macmiidt = crate::Reg<macmiidt::MacmiidtSpec>;
#[doc = "Ethernet MAC MII data register"]
pub mod macmiidt;
#[doc = "MACFCTRL (rw) register accessor: Ethernet MAC flow control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macfctrl`]
module"]
#[doc(alias = "MACFCTRL")]
pub type Macfctrl = crate::Reg<macfctrl::MacfctrlSpec>;
#[doc = "Ethernet MAC flow control register"]
pub mod macfctrl;
#[doc = "MACVLT (rw) register accessor: Ethernet MAC VLAN tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macvlt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macvlt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macvlt`]
module"]
#[doc(alias = "MACVLT")]
pub type Macvlt = crate::Reg<macvlt::MacvltSpec>;
#[doc = "Ethernet MAC VLAN tag register"]
pub mod macvlt;
#[doc = "MACRWFF (rw) register accessor: Ethernet MAC remote wakeup frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macrwff::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macrwff::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macrwff`]
module"]
#[doc(alias = "MACRWFF")]
pub type Macrwff = crate::Reg<macrwff::MacrwffSpec>;
#[doc = "Ethernet MAC remote wakeup frame filter register"]
pub mod macrwff;
#[doc = "MACPMTCTRLSTS (rw) register accessor: Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macpmtctrlsts`]
module"]
#[doc(alias = "MACPMTCTRLSTS")]
pub type Macpmtctrlsts = crate::Reg<macpmtctrlsts::MacpmtctrlstsSpec>;
#[doc = "Ethernet MAC PMT control and status register"]
pub mod macpmtctrlsts;
#[doc = "MACISTS (rw) register accessor: Ethernet MAC interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macists::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macists::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macists`]
module"]
#[doc(alias = "MACISTS")]
pub type Macists = crate::Reg<macists::MacistsSpec>;
#[doc = "Ethernet MAC interrupt status register"]
pub mod macists;
#[doc = "MACIMR (rw) register accessor: Ethernet MAC interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macimr`]
module"]
#[doc(alias = "MACIMR")]
pub type Macimr = crate::Reg<macimr::MacimrSpec>;
#[doc = "Ethernet MAC interrupt mask register"]
pub mod macimr;
#[doc = "MACA0H (rw) register accessor: Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0h`]
module"]
#[doc(alias = "MACA0H")]
pub type Maca0h = crate::Reg<maca0h::Maca0hSpec>;
#[doc = "Ethernet MAC address 0 high register"]
pub mod maca0h;
#[doc = "MACA0L (rw) register accessor: Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca0l`]
module"]
#[doc(alias = "MACA0L")]
pub type Maca0l = crate::Reg<maca0l::Maca0lSpec>;
#[doc = "Ethernet MAC address 0 low register"]
pub mod maca0l;
#[doc = "MACA1H (rw) register accessor: Ethernet MAC address 1 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1h`]
module"]
#[doc(alias = "MACA1H")]
pub type Maca1h = crate::Reg<maca1h::Maca1hSpec>;
#[doc = "Ethernet MAC address 1 high register"]
pub mod maca1h;
#[doc = "MACA1L (rw) register accessor: Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca1l`]
module"]
#[doc(alias = "MACA1L")]
pub type Maca1l = crate::Reg<maca1l::Maca1lSpec>;
#[doc = "Ethernet MAC address1 low register"]
pub mod maca1l;
#[doc = "MACA2H (rw) register accessor: Ethernet MAC address 2 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2h`]
module"]
#[doc(alias = "MACA2H")]
pub type Maca2h = crate::Reg<maca2h::Maca2hSpec>;
#[doc = "Ethernet MAC address 2 high register"]
pub mod maca2h;
#[doc = "MACA2L (rw) register accessor: Ethernet MAC address 2 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca2l`]
module"]
#[doc(alias = "MACA2L")]
pub type Maca2l = crate::Reg<maca2l::Maca2lSpec>;
#[doc = "Ethernet MAC address 2 low register"]
pub mod maca2l;
#[doc = "MACA3H (rw) register accessor: Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3h`]
module"]
#[doc(alias = "MACA3H")]
pub type Maca3h = crate::Reg<maca3h::Maca3hSpec>;
#[doc = "Ethernet MAC address 3 high register"]
pub mod maca3h;
#[doc = "MACA3L (rw) register accessor: Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maca3l`]
module"]
#[doc(alias = "MACA3L")]
pub type Maca3l = crate::Reg<maca3l::Maca3lSpec>;
#[doc = "Ethernet MAC address 3 low register"]
pub mod maca3l;
