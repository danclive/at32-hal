#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sts: Sts,
    ctrl1: Ctrl1,
    ctrl2: Ctrl2,
    spt1: Spt1,
    spt2: Spt2,
    pcdto1: Pcdto1,
    pcdto2: Pcdto2,
    pcdto3: Pcdto3,
    pcdto4: Pcdto4,
    vmhb: Vmhb,
    vmlb: Vmlb,
    osq1: Osq1,
    osq2: Osq2,
    osq3: Osq3,
    psq: Psq,
    pdt1: Pdt1,
    pdt2: Pdt2,
    pdt3: Pdt3,
    pdt4: Pdt4,
    odt: Odt,
    _reserved20: [u8; 0x30],
    ovsp: Ovsp,
    _reserved21: [u8; 0x30],
    calval: Calval,
}
impl RegisterBlock {
    #[doc = "0x00 - status register"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x04 - control register 1"]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
    #[doc = "0x08 - control register 2"]
    #[inline(always)]
    pub const fn ctrl2(&self) -> &Ctrl2 {
        &self.ctrl2
    }
    #[doc = "0x0c - sample time register 1"]
    #[inline(always)]
    pub const fn spt1(&self) -> &Spt1 {
        &self.spt1
    }
    #[doc = "0x10 - sample time register 2"]
    #[inline(always)]
    pub const fn spt2(&self) -> &Spt2 {
        &self.spt2
    }
    #[doc = "0x14 - Preempted channel 1 data offset register"]
    #[inline(always)]
    pub const fn pcdto1(&self) -> &Pcdto1 {
        &self.pcdto1
    }
    #[doc = "0x18 - Preempted channel 2 data offset register"]
    #[inline(always)]
    pub const fn pcdto2(&self) -> &Pcdto2 {
        &self.pcdto2
    }
    #[doc = "0x1c - Preempted channel 3 data offset register"]
    #[inline(always)]
    pub const fn pcdto3(&self) -> &Pcdto3 {
        &self.pcdto3
    }
    #[doc = "0x20 - Preempted channel 4 data offset register"]
    #[inline(always)]
    pub const fn pcdto4(&self) -> &Pcdto4 {
        &self.pcdto4
    }
    #[doc = "0x24 - Voltage monitoring high boundary register"]
    #[inline(always)]
    pub const fn vmhb(&self) -> &Vmhb {
        &self.vmhb
    }
    #[doc = "0x28 - Voltage monitoring low boundary register"]
    #[inline(always)]
    pub const fn vmlb(&self) -> &Vmlb {
        &self.vmlb
    }
    #[doc = "0x2c - Ordinary sequence register 1"]
    #[inline(always)]
    pub const fn osq1(&self) -> &Osq1 {
        &self.osq1
    }
    #[doc = "0x30 - Ordinary sequence register 2"]
    #[inline(always)]
    pub const fn osq2(&self) -> &Osq2 {
        &self.osq2
    }
    #[doc = "0x34 - Ordinary sequence register 3"]
    #[inline(always)]
    pub const fn osq3(&self) -> &Osq3 {
        &self.osq3
    }
    #[doc = "0x38 - Preempted sequence register"]
    #[inline(always)]
    pub const fn psq(&self) -> &Psq {
        &self.psq
    }
    #[doc = "0x3c - Preempted data register 1"]
    #[inline(always)]
    pub const fn pdt1(&self) -> &Pdt1 {
        &self.pdt1
    }
    #[doc = "0x40 - Preempted data register 2"]
    #[inline(always)]
    pub const fn pdt2(&self) -> &Pdt2 {
        &self.pdt2
    }
    #[doc = "0x44 - Preempted data register 3"]
    #[inline(always)]
    pub const fn pdt3(&self) -> &Pdt3 {
        &self.pdt3
    }
    #[doc = "0x48 - Preempted data register 4"]
    #[inline(always)]
    pub const fn pdt4(&self) -> &Pdt4 {
        &self.pdt4
    }
    #[doc = "0x4c - Ordinary data register"]
    #[inline(always)]
    pub const fn odt(&self) -> &Odt {
        &self.odt
    }
    #[doc = "0x80 - oversampling register"]
    #[inline(always)]
    pub const fn ovsp(&self) -> &Ovsp {
        &self.ovsp
    }
    #[doc = "0xb4 - Calibration value register"]
    #[inline(always)]
    pub const fn calval(&self) -> &Calval {
        &self.calval
    }
}
#[doc = "STS (rw) register accessor: status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "status register"]
pub mod sts;
#[doc = "CTRL1 (rw) register accessor: control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "CTRL1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "control register 1"]
pub mod ctrl1;
#[doc = "CTRL2 (rw) register accessor: control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl2`]
module"]
#[doc(alias = "CTRL2")]
pub type Ctrl2 = crate::Reg<ctrl2::Ctrl2Spec>;
#[doc = "control register 2"]
pub mod ctrl2;
#[doc = "SPT1 (rw) register accessor: sample time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spt1`]
module"]
#[doc(alias = "SPT1")]
pub type Spt1 = crate::Reg<spt1::Spt1Spec>;
#[doc = "sample time register 1"]
pub mod spt1;
#[doc = "SPT2 (rw) register accessor: sample time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spt2`]
module"]
#[doc(alias = "SPT2")]
pub type Spt2 = crate::Reg<spt2::Spt2Spec>;
#[doc = "sample time register 2"]
pub mod spt2;
#[doc = "PCDTO1 (rw) register accessor: Preempted channel 1 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdto1`]
module"]
#[doc(alias = "PCDTO1")]
pub type Pcdto1 = crate::Reg<pcdto1::Pcdto1Spec>;
#[doc = "Preempted channel 1 data offset register"]
pub mod pcdto1;
#[doc = "PCDTO2 (rw) register accessor: Preempted channel 2 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdto2`]
module"]
#[doc(alias = "PCDTO2")]
pub type Pcdto2 = crate::Reg<pcdto2::Pcdto2Spec>;
#[doc = "Preempted channel 2 data offset register"]
pub mod pcdto2;
#[doc = "PCDTO3 (rw) register accessor: Preempted channel 3 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdto3`]
module"]
#[doc(alias = "PCDTO3")]
pub type Pcdto3 = crate::Reg<pcdto3::Pcdto3Spec>;
#[doc = "Preempted channel 3 data offset register"]
pub mod pcdto3;
#[doc = "PCDTO4 (rw) register accessor: Preempted channel 4 data offset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcdto4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcdto4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcdto4`]
module"]
#[doc(alias = "PCDTO4")]
pub type Pcdto4 = crate::Reg<pcdto4::Pcdto4Spec>;
#[doc = "Preempted channel 4 data offset register"]
pub mod pcdto4;
#[doc = "VMHB (rw) register accessor: Voltage monitoring high boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmhb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmhb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmhb`]
module"]
#[doc(alias = "VMHB")]
pub type Vmhb = crate::Reg<vmhb::VmhbSpec>;
#[doc = "Voltage monitoring high boundary register"]
pub mod vmhb;
#[doc = "VMLB (rw) register accessor: Voltage monitoring low boundary register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vmlb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vmlb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vmlb`]
module"]
#[doc(alias = "VMLB")]
pub type Vmlb = crate::Reg<vmlb::VmlbSpec>;
#[doc = "Voltage monitoring low boundary register"]
pub mod vmlb;
#[doc = "OSQ1 (rw) register accessor: Ordinary sequence register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq1`]
module"]
#[doc(alias = "OSQ1")]
pub type Osq1 = crate::Reg<osq1::Osq1Spec>;
#[doc = "Ordinary sequence register 1"]
pub mod osq1;
#[doc = "OSQ2 (rw) register accessor: Ordinary sequence register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq2`]
module"]
#[doc(alias = "OSQ2")]
pub type Osq2 = crate::Reg<osq2::Osq2Spec>;
#[doc = "Ordinary sequence register 2"]
pub mod osq2;
#[doc = "OSQ3 (rw) register accessor: Ordinary sequence register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osq3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osq3`]
module"]
#[doc(alias = "OSQ3")]
pub type Osq3 = crate::Reg<osq3::Osq3Spec>;
#[doc = "Ordinary sequence register 3"]
pub mod osq3;
#[doc = "PSQ (rw) register accessor: Preempted sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psq`]
module"]
#[doc(alias = "PSQ")]
pub type Psq = crate::Reg<psq::PsqSpec>;
#[doc = "Preempted sequence register"]
pub mod psq;
#[doc = "PDT1 (r) register accessor: Preempted data register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdt1`]
module"]
#[doc(alias = "PDT1")]
pub type Pdt1 = crate::Reg<pdt1::Pdt1Spec>;
#[doc = "Preempted data register 1"]
pub mod pdt1;
#[doc = "PDT2 (r) register accessor: Preempted data register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdt2`]
module"]
#[doc(alias = "PDT2")]
pub type Pdt2 = crate::Reg<pdt2::Pdt2Spec>;
#[doc = "Preempted data register 2"]
pub mod pdt2;
#[doc = "PDT3 (r) register accessor: Preempted data register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdt3`]
module"]
#[doc(alias = "PDT3")]
pub type Pdt3 = crate::Reg<pdt3::Pdt3Spec>;
#[doc = "Preempted data register 3"]
pub mod pdt3;
#[doc = "PDT4 (r) register accessor: Preempted data register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdt4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdt4`]
module"]
#[doc(alias = "PDT4")]
pub type Pdt4 = crate::Reg<pdt4::Pdt4Spec>;
#[doc = "Preempted data register 4"]
pub mod pdt4;
#[doc = "ODT (r) register accessor: Ordinary data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odt`]
module"]
#[doc(alias = "ODT")]
pub type Odt = crate::Reg<odt::OdtSpec>;
#[doc = "Ordinary data register"]
pub mod odt;
#[doc = "OVSP (rw) register accessor: oversampling register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ovsp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ovsp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ovsp`]
module"]
#[doc(alias = "OVSP")]
pub type Ovsp = crate::Reg<ovsp::OvspSpec>;
#[doc = "oversampling register"]
pub mod ovsp;
#[doc = "CALVAL (rw) register accessor: Calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calval::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calval::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calval`]
module"]
#[doc(alias = "CALVAL")]
pub type Calval = crate::Reg<calval::CalvalSpec>;
#[doc = "Calibration value register"]
pub mod calval;
