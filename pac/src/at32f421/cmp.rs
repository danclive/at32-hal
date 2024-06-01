#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrlsts: Ctrlsts,
    _reserved1: [u8; 0x04],
    g_filter_en: GFilterEn,
    high_pulse: HighPulse,
    low_pulse: LowPulse,
}
impl RegisterBlock {
    #[doc = "0x00 - CMP control/status register"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0x08 - G_FILTER_EN"]
    #[inline(always)]
    pub const fn g_filter_en(&self) -> &GFilterEn {
        &self.g_filter_en
    }
    #[doc = "0x0c - HIGH_PULSE"]
    #[inline(always)]
    pub const fn high_pulse(&self) -> &HighPulse {
        &self.high_pulse
    }
    #[doc = "0x10 - LOW_PULSE"]
    #[inline(always)]
    pub const fn low_pulse(&self) -> &LowPulse {
        &self.low_pulse
    }
}
#[doc = "CTRLSTS (rw) register accessor: CMP control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "CMP control/status register"]
pub mod ctrlsts;
#[doc = "G_FILTER_EN (rw) register accessor: G_FILTER_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g_filter_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g_filter_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@g_filter_en`]
module"]
#[doc(alias = "G_FILTER_EN")]
pub type GFilterEn = crate::Reg<g_filter_en::GFilterEnSpec>;
#[doc = "G_FILTER_EN"]
pub mod g_filter_en;
#[doc = "HIGH_PULSE (rw) register accessor: HIGH_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`high_pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`high_pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@high_pulse`]
module"]
#[doc(alias = "HIGH_PULSE")]
pub type HighPulse = crate::Reg<high_pulse::HighPulseSpec>;
#[doc = "HIGH_PULSE"]
pub mod high_pulse;
#[doc = "LOW_PULSE (rw) register accessor: LOW_PULSE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`low_pulse::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`low_pulse::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@low_pulse`]
module"]
#[doc(alias = "LOW_PULSE")]
pub type LowPulse = crate::Reg<low_pulse::LowPulseSpec>;
#[doc = "LOW_PULSE"]
pub mod low_pulse;
