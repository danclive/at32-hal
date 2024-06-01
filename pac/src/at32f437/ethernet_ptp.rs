#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ptptsctrl: Ptptsctrl,
    ptpssinc: Ptpssinc,
    ptptsh: Ptptsh,
    ptptsl: Ptptsl,
    ptptshud: Ptptshud,
    ptptslud: Ptptslud,
    ptptsad: Ptptsad,
    ptptth: Ptptth,
    ptpttl: Ptpttl,
    _reserved9: [u8; 0x04],
    ptptssr: Ptptssr,
    ptpppscr: Ptpppscr,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet PTP time stamp control register"]
    #[inline(always)]
    pub const fn ptptsctrl(&self) -> &Ptptsctrl {
        &self.ptptsctrl
    }
    #[doc = "0x04 - Ethernet PTP subsecond increment register"]
    #[inline(always)]
    pub const fn ptpssinc(&self) -> &Ptpssinc {
        &self.ptpssinc
    }
    #[doc = "0x08 - Ethernet PTP time stamp high register"]
    #[inline(always)]
    pub const fn ptptsh(&self) -> &Ptptsh {
        &self.ptptsh
    }
    #[doc = "0x0c - Ethernet PTP time stamp low register"]
    #[inline(always)]
    pub const fn ptptsl(&self) -> &Ptptsl {
        &self.ptptsl
    }
    #[doc = "0x10 - Ethernet PTP time stamp high update register"]
    #[inline(always)]
    pub const fn ptptshud(&self) -> &Ptptshud {
        &self.ptptshud
    }
    #[doc = "0x14 - Ethernet PTP time stamp low update register"]
    #[inline(always)]
    pub const fn ptptslud(&self) -> &Ptptslud {
        &self.ptptslud
    }
    #[doc = "0x18 - Ethernet PTP time stamp addend register"]
    #[inline(always)]
    pub const fn ptptsad(&self) -> &Ptptsad {
        &self.ptptsad
    }
    #[doc = "0x1c - Ethernet PTP target time high register"]
    #[inline(always)]
    pub const fn ptptth(&self) -> &Ptptth {
        &self.ptptth
    }
    #[doc = "0x20 - Ethernet PTP target time low register"]
    #[inline(always)]
    pub const fn ptpttl(&self) -> &Ptpttl {
        &self.ptpttl
    }
    #[doc = "0x28 - Ethernet PTP time stamp status register"]
    #[inline(always)]
    pub const fn ptptssr(&self) -> &Ptptssr {
        &self.ptptssr
    }
    #[doc = "0x2c - Ethernet PTP PPS control register"]
    #[inline(always)]
    pub const fn ptpppscr(&self) -> &Ptpppscr {
        &self.ptpppscr
    }
}
#[doc = "PTPTSCTRL (rw) register accessor: Ethernet PTP time stamp control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsctrl`]
module"]
#[doc(alias = "PTPTSCTRL")]
pub type Ptptsctrl = crate::Reg<ptptsctrl::PtptsctrlSpec>;
#[doc = "Ethernet PTP time stamp control register"]
pub mod ptptsctrl;
#[doc = "PTPSSINC (rw) register accessor: Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpssinc`]
module"]
#[doc(alias = "PTPSSINC")]
pub type Ptpssinc = crate::Reg<ptpssinc::PtpssincSpec>;
#[doc = "Ethernet PTP subsecond increment register"]
pub mod ptpssinc;
#[doc = "PTPTSH (r) register accessor: Ethernet PTP time stamp high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsh::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsh`]
module"]
#[doc(alias = "PTPTSH")]
pub type Ptptsh = crate::Reg<ptptsh::PtptshSpec>;
#[doc = "Ethernet PTP time stamp high register"]
pub mod ptptsh;
#[doc = "PTPTSL (r) register accessor: Ethernet PTP time stamp low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsl`]
module"]
#[doc(alias = "PTPTSL")]
pub type Ptptsl = crate::Reg<ptptsl::PtptslSpec>;
#[doc = "Ethernet PTP time stamp low register"]
pub mod ptptsl;
#[doc = "PTPTSHUD (rw) register accessor: Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptshud`]
module"]
#[doc(alias = "PTPTSHUD")]
pub type Ptptshud = crate::Reg<ptptshud::PtptshudSpec>;
#[doc = "Ethernet PTP time stamp high update register"]
pub mod ptptshud;
#[doc = "PTPTSLUD (rw) register accessor: Ethernet PTP time stamp low update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptslud::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptslud::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptslud`]
module"]
#[doc(alias = "PTPTSLUD")]
pub type Ptptslud = crate::Reg<ptptslud::PtptsludSpec>;
#[doc = "Ethernet PTP time stamp low update register"]
pub mod ptptslud;
#[doc = "PTPTSAD (rw) register accessor: Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsad::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsad::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptsad`]
module"]
#[doc(alias = "PTPTSAD")]
pub type Ptptsad = crate::Reg<ptptsad::PtptsadSpec>;
#[doc = "Ethernet PTP time stamp addend register"]
pub mod ptptsad;
#[doc = "PTPTTH (rw) register accessor: Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptth`]
module"]
#[doc(alias = "PTPTTH")]
pub type Ptptth = crate::Reg<ptptth::PtptthSpec>;
#[doc = "Ethernet PTP target time high register"]
pub mod ptptth;
#[doc = "PTPTTL (rw) register accessor: Ethernet PTP target time low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpttl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpttl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpttl`]
module"]
#[doc(alias = "PTPTTL")]
pub type Ptpttl = crate::Reg<ptpttl::PtpttlSpec>;
#[doc = "Ethernet PTP target time low register"]
pub mod ptpttl;
#[doc = "PTPTSSR (r) register accessor: Ethernet PTP time stamp status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptptssr`]
module"]
#[doc(alias = "PTPTSSR")]
pub type Ptptssr = crate::Reg<ptptssr::PtptssrSpec>;
#[doc = "Ethernet PTP time stamp status register"]
pub mod ptptssr;
#[doc = "PTPPPSCR (r) register accessor: Ethernet PTP PPS control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpppscr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptpppscr`]
module"]
#[doc(alias = "PTPPPSCR")]
pub type Ptpppscr = crate::Reg<ptpppscr::PtpppscrSpec>;
#[doc = "Ethernet PTP PPS control register"]
pub mod ptpppscr;
