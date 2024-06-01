#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mmcctrl: Mmcctrl,
    mmcri: Mmcri,
    mmcti: Mmcti,
    mmcrim: Mmcrim,
    mmctim: Mmctim,
    _reserved5: [u8; 0x38],
    mmctfscc: Mmctfscc,
    mmctfmscc: Mmctfmscc,
    _reserved7: [u8; 0x14],
    mmctfcnt: Mmctfcnt,
    _reserved8: [u8; 0x28],
    mmcrfcecnt: Mmcrfcecnt,
    mmcrfaecnt: Mmcrfaecnt,
    _reserved10: [u8; 0x28],
    mmcrgufcnt: Mmcrgufcnt,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MMC control register"]
    #[inline(always)]
    pub const fn mmcctrl(&self) -> &Mmcctrl {
        &self.mmcctrl
    }
    #[doc = "0x04 - Ethernet MMC receive interrupt register"]
    #[inline(always)]
    pub const fn mmcri(&self) -> &Mmcri {
        &self.mmcri
    }
    #[doc = "0x08 - Ethernet MMC transmit interrupt register"]
    #[inline(always)]
    pub const fn mmcti(&self) -> &Mmcti {
        &self.mmcti
    }
    #[doc = "0x0c - Ethernet MMC receive interrupt mask register"]
    #[inline(always)]
    pub const fn mmcrim(&self) -> &Mmcrim {
        &self.mmcrim
    }
    #[doc = "0x10 - Ethernet MMC transmit interrupt mask register"]
    #[inline(always)]
    pub const fn mmctim(&self) -> &Mmctim {
        &self.mmctim
    }
    #[doc = "0x4c - Ethernet MMC transmitted good frames after a single collision counter"]
    #[inline(always)]
    pub const fn mmctfscc(&self) -> &Mmctfscc {
        &self.mmctfscc
    }
    #[doc = "0x50 - Ethernet MMC transmitted good frames after more than a single collision"]
    #[inline(always)]
    pub const fn mmctfmscc(&self) -> &Mmctfmscc {
        &self.mmctfmscc
    }
    #[doc = "0x68 - Ethernet MMC transmitted good frames counter register"]
    #[inline(always)]
    pub const fn mmctfcnt(&self) -> &Mmctfcnt {
        &self.mmctfcnt
    }
    #[doc = "0x94 - Ethernet MMC received frames with CRC error counter register"]
    #[inline(always)]
    pub const fn mmcrfcecnt(&self) -> &Mmcrfcecnt {
        &self.mmcrfcecnt
    }
    #[doc = "0x98 - Ethernet MMC received frames with alignment error counter register"]
    #[inline(always)]
    pub const fn mmcrfaecnt(&self) -> &Mmcrfaecnt {
        &self.mmcrfaecnt
    }
    #[doc = "0xc4 - MMC received good unicast frames counter register"]
    #[inline(always)]
    pub const fn mmcrgufcnt(&self) -> &Mmcrgufcnt {
        &self.mmcrgufcnt
    }
}
#[doc = "MMCCTRL (rw) register accessor: Ethernet MMC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcctrl`]
module"]
#[doc(alias = "MMCCTRL")]
pub type Mmcctrl = crate::Reg<mmcctrl::MmcctrlSpec>;
#[doc = "Ethernet MMC control register"]
pub mod mmcctrl;
#[doc = "MMCRI (rw) register accessor: Ethernet MMC receive interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcri::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcri::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcri`]
module"]
#[doc(alias = "MMCRI")]
pub type Mmcri = crate::Reg<mmcri::MmcriSpec>;
#[doc = "Ethernet MMC receive interrupt register"]
pub mod mmcri;
#[doc = "MMCTI (rw) register accessor: Ethernet MMC transmit interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcti::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcti::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcti`]
module"]
#[doc(alias = "MMCTI")]
pub type Mmcti = crate::Reg<mmcti::MmctiSpec>;
#[doc = "Ethernet MMC transmit interrupt register"]
pub mod mmcti;
#[doc = "MMCRIM (rw) register accessor: Ethernet MMC receive interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmcrim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrim`]
module"]
#[doc(alias = "MMCRIM")]
pub type Mmcrim = crate::Reg<mmcrim::MmcrimSpec>;
#[doc = "Ethernet MMC receive interrupt mask register"]
pub mod mmcrim;
#[doc = "MMCTIM (rw) register accessor: Ethernet MMC transmit interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmctim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctim`]
module"]
#[doc(alias = "MMCTIM")]
pub type Mmctim = crate::Reg<mmctim::MmctimSpec>;
#[doc = "Ethernet MMC transmit interrupt mask register"]
pub mod mmctim;
#[doc = "MMCTFSCC (r) register accessor: Ethernet MMC transmitted good frames after a single collision counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfscc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctfscc`]
module"]
#[doc(alias = "MMCTFSCC")]
pub type Mmctfscc = crate::Reg<mmctfscc::MmctfsccSpec>;
#[doc = "Ethernet MMC transmitted good frames after a single collision counter"]
pub mod mmctfscc;
#[doc = "MMCTFMSCC (r) register accessor: Ethernet MMC transmitted good frames after more than a single collision\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfmscc::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctfmscc`]
module"]
#[doc(alias = "MMCTFMSCC")]
pub type Mmctfmscc = crate::Reg<mmctfmscc::MmctfmsccSpec>;
#[doc = "Ethernet MMC transmitted good frames after more than a single collision"]
pub mod mmctfmscc;
#[doc = "MMCTFCNT (r) register accessor: Ethernet MMC transmitted good frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmctfcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmctfcnt`]
module"]
#[doc(alias = "MMCTFCNT")]
pub type Mmctfcnt = crate::Reg<mmctfcnt::MmctfcntSpec>;
#[doc = "Ethernet MMC transmitted good frames counter register"]
pub mod mmctfcnt;
#[doc = "MMCRFCECNT (r) register accessor: Ethernet MMC received frames with CRC error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfcecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfcecnt`]
module"]
#[doc(alias = "MMCRFCECNT")]
pub type Mmcrfcecnt = crate::Reg<mmcrfcecnt::MmcrfcecntSpec>;
#[doc = "Ethernet MMC received frames with CRC error counter register"]
pub mod mmcrfcecnt;
#[doc = "MMCRFAECNT (r) register accessor: Ethernet MMC received frames with alignment error counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrfaecnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrfaecnt`]
module"]
#[doc(alias = "MMCRFAECNT")]
pub type Mmcrfaecnt = crate::Reg<mmcrfaecnt::MmcrfaecntSpec>;
#[doc = "Ethernet MMC received frames with alignment error counter register"]
pub mod mmcrfaecnt;
#[doc = "MMCRGUFCNT (r) register accessor: MMC received good unicast frames counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmcrgufcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mmcrgufcnt`]
module"]
#[doc(alias = "MMCRGUFCNT")]
pub type Mmcrgufcnt = crate::Reg<mmcrgufcnt::MmcrgufcntSpec>;
#[doc = "MMC received good unicast frames counter register"]
pub mod mmcrgufcnt;
