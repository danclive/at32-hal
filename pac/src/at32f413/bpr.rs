#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dt1: Dt1,
    dt2: Dt2,
    dt3: Dt3,
    dt4: Dt4,
    dt5: Dt5,
    dt6: Dt6,
    dt7: Dt7,
    dt8: Dt8,
    dt9: Dt9,
    dt10: Dt10,
    rtccal: Rtccal,
    ctrl: Ctrl,
    ctrlsts: Ctrlsts,
    _reserved13: [u8; 0x08],
    dt11: Dt11,
    dt12: Dt12,
    dt13: Dt13,
    dt14: Dt14,
    dt15: Dt15,
    dt16: Dt16,
    dt17: Dt17,
    dt18: Dt18,
    dt19: Dt19,
    dt20: Dt20,
    dt21: Dt21,
    dt22: Dt22,
    dt23: Dt23,
    dt24: Dt24,
    dt25: Dt25,
    dt26: Dt26,
    dt27: Dt27,
    dt28: Dt28,
    dt29: Dt29,
    dt30: Dt30,
    dt31: Dt31,
    dt32: Dt32,
    dt33: Dt33,
    dt34: Dt34,
    dt35: Dt35,
    dt36: Dt36,
    dt37: Dt37,
    dt38: Dt38,
    dt39: Dt39,
    dt40: Dt40,
    dt41: Dt41,
    dt42: Dt42,
}
impl RegisterBlock {
    #[doc = "0x00 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt1(&self) -> &Dt1 {
        &self.dt1
    }
    #[doc = "0x04 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt2(&self) -> &Dt2 {
        &self.dt2
    }
    #[doc = "0x08 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt3(&self) -> &Dt3 {
        &self.dt3
    }
    #[doc = "0x0c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt4(&self) -> &Dt4 {
        &self.dt4
    }
    #[doc = "0x10 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt5(&self) -> &Dt5 {
        &self.dt5
    }
    #[doc = "0x14 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt6(&self) -> &Dt6 {
        &self.dt6
    }
    #[doc = "0x18 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt7(&self) -> &Dt7 {
        &self.dt7
    }
    #[doc = "0x1c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt8(&self) -> &Dt8 {
        &self.dt8
    }
    #[doc = "0x20 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt9(&self) -> &Dt9 {
        &self.dt9
    }
    #[doc = "0x24 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt10(&self) -> &Dt10 {
        &self.dt10
    }
    #[doc = "0x28 - RTC clock calibration register (BPR_RTCCAL)"]
    #[inline(always)]
    pub const fn rtccal(&self) -> &Rtccal {
        &self.rtccal
    }
    #[doc = "0x2c - BPR control register (BPR_CTRL)"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x30 - BPR control/status register (BPR_CTRLSTS)"]
    #[inline(always)]
    pub const fn ctrlsts(&self) -> &Ctrlsts {
        &self.ctrlsts
    }
    #[doc = "0x3c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt11(&self) -> &Dt11 {
        &self.dt11
    }
    #[doc = "0x40 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt12(&self) -> &Dt12 {
        &self.dt12
    }
    #[doc = "0x44 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt13(&self) -> &Dt13 {
        &self.dt13
    }
    #[doc = "0x48 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt14(&self) -> &Dt14 {
        &self.dt14
    }
    #[doc = "0x4c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt15(&self) -> &Dt15 {
        &self.dt15
    }
    #[doc = "0x50 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt16(&self) -> &Dt16 {
        &self.dt16
    }
    #[doc = "0x54 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt17(&self) -> &Dt17 {
        &self.dt17
    }
    #[doc = "0x58 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt18(&self) -> &Dt18 {
        &self.dt18
    }
    #[doc = "0x5c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt19(&self) -> &Dt19 {
        &self.dt19
    }
    #[doc = "0x60 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt20(&self) -> &Dt20 {
        &self.dt20
    }
    #[doc = "0x64 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt21(&self) -> &Dt21 {
        &self.dt21
    }
    #[doc = "0x68 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt22(&self) -> &Dt22 {
        &self.dt22
    }
    #[doc = "0x6c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt23(&self) -> &Dt23 {
        &self.dt23
    }
    #[doc = "0x70 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt24(&self) -> &Dt24 {
        &self.dt24
    }
    #[doc = "0x74 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt25(&self) -> &Dt25 {
        &self.dt25
    }
    #[doc = "0x78 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt26(&self) -> &Dt26 {
        &self.dt26
    }
    #[doc = "0x7c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt27(&self) -> &Dt27 {
        &self.dt27
    }
    #[doc = "0x80 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt28(&self) -> &Dt28 {
        &self.dt28
    }
    #[doc = "0x84 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt29(&self) -> &Dt29 {
        &self.dt29
    }
    #[doc = "0x88 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt30(&self) -> &Dt30 {
        &self.dt30
    }
    #[doc = "0x8c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt31(&self) -> &Dt31 {
        &self.dt31
    }
    #[doc = "0x90 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt32(&self) -> &Dt32 {
        &self.dt32
    }
    #[doc = "0x94 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt33(&self) -> &Dt33 {
        &self.dt33
    }
    #[doc = "0x98 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt34(&self) -> &Dt34 {
        &self.dt34
    }
    #[doc = "0x9c - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt35(&self) -> &Dt35 {
        &self.dt35
    }
    #[doc = "0xa0 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt36(&self) -> &Dt36 {
        &self.dt36
    }
    #[doc = "0xa4 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt37(&self) -> &Dt37 {
        &self.dt37
    }
    #[doc = "0xa8 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt38(&self) -> &Dt38 {
        &self.dt38
    }
    #[doc = "0xac - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt39(&self) -> &Dt39 {
        &self.dt39
    }
    #[doc = "0xb0 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt40(&self) -> &Dt40 {
        &self.dt40
    }
    #[doc = "0xb4 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt41(&self) -> &Dt41 {
        &self.dt41
    }
    #[doc = "0xb8 - Battery powered domain data register (BPR_DTx)"]
    #[inline(always)]
    pub const fn dt42(&self) -> &Dt42 {
        &self.dt42
    }
}
#[doc = "DT1 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1`]
module"]
#[doc(alias = "DT1")]
pub type Dt1 = crate::Reg<dt1::Dt1Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt1;
#[doc = "DT2 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt2`]
module"]
#[doc(alias = "DT2")]
pub type Dt2 = crate::Reg<dt2::Dt2Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt2;
#[doc = "DT3 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt3`]
module"]
#[doc(alias = "DT3")]
pub type Dt3 = crate::Reg<dt3::Dt3Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt3;
#[doc = "DT4 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt4`]
module"]
#[doc(alias = "DT4")]
pub type Dt4 = crate::Reg<dt4::Dt4Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt4;
#[doc = "DT5 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt5`]
module"]
#[doc(alias = "DT5")]
pub type Dt5 = crate::Reg<dt5::Dt5Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt5;
#[doc = "DT6 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt6`]
module"]
#[doc(alias = "DT6")]
pub type Dt6 = crate::Reg<dt6::Dt6Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt6;
#[doc = "DT7 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt7`]
module"]
#[doc(alias = "DT7")]
pub type Dt7 = crate::Reg<dt7::Dt7Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt7;
#[doc = "DT8 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt8`]
module"]
#[doc(alias = "DT8")]
pub type Dt8 = crate::Reg<dt8::Dt8Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt8;
#[doc = "DT9 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt9`]
module"]
#[doc(alias = "DT9")]
pub type Dt9 = crate::Reg<dt9::Dt9Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt9;
#[doc = "DT10 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt10`]
module"]
#[doc(alias = "DT10")]
pub type Dt10 = crate::Reg<dt10::Dt10Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt10;
#[doc = "DT11 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt11`]
module"]
#[doc(alias = "DT11")]
pub type Dt11 = crate::Reg<dt11::Dt11Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt11;
#[doc = "DT12 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt12`]
module"]
#[doc(alias = "DT12")]
pub type Dt12 = crate::Reg<dt12::Dt12Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt12;
#[doc = "DT13 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt13`]
module"]
#[doc(alias = "DT13")]
pub type Dt13 = crate::Reg<dt13::Dt13Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt13;
#[doc = "DT14 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt14`]
module"]
#[doc(alias = "DT14")]
pub type Dt14 = crate::Reg<dt14::Dt14Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt14;
#[doc = "DT15 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt15`]
module"]
#[doc(alias = "DT15")]
pub type Dt15 = crate::Reg<dt15::Dt15Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt15;
#[doc = "DT16 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt16`]
module"]
#[doc(alias = "DT16")]
pub type Dt16 = crate::Reg<dt16::Dt16Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt16;
#[doc = "DT17 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt17`]
module"]
#[doc(alias = "DT17")]
pub type Dt17 = crate::Reg<dt17::Dt17Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt17;
#[doc = "DT18 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt18`]
module"]
#[doc(alias = "DT18")]
pub type Dt18 = crate::Reg<dt18::Dt18Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt18;
#[doc = "DT19 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt19`]
module"]
#[doc(alias = "DT19")]
pub type Dt19 = crate::Reg<dt19::Dt19Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt19;
#[doc = "DT20 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt20`]
module"]
#[doc(alias = "DT20")]
pub type Dt20 = crate::Reg<dt20::Dt20Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt20;
#[doc = "DT21 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt21`]
module"]
#[doc(alias = "DT21")]
pub type Dt21 = crate::Reg<dt21::Dt21Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt21;
#[doc = "DT22 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt22`]
module"]
#[doc(alias = "DT22")]
pub type Dt22 = crate::Reg<dt22::Dt22Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt22;
#[doc = "DT23 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt23`]
module"]
#[doc(alias = "DT23")]
pub type Dt23 = crate::Reg<dt23::Dt23Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt23;
#[doc = "DT24 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt24`]
module"]
#[doc(alias = "DT24")]
pub type Dt24 = crate::Reg<dt24::Dt24Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt24;
#[doc = "DT25 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt25`]
module"]
#[doc(alias = "DT25")]
pub type Dt25 = crate::Reg<dt25::Dt25Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt25;
#[doc = "DT26 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt26`]
module"]
#[doc(alias = "DT26")]
pub type Dt26 = crate::Reg<dt26::Dt26Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt26;
#[doc = "DT27 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt27`]
module"]
#[doc(alias = "DT27")]
pub type Dt27 = crate::Reg<dt27::Dt27Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt27;
#[doc = "DT28 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt28`]
module"]
#[doc(alias = "DT28")]
pub type Dt28 = crate::Reg<dt28::Dt28Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt28;
#[doc = "DT29 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt29`]
module"]
#[doc(alias = "DT29")]
pub type Dt29 = crate::Reg<dt29::Dt29Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt29;
#[doc = "DT30 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt30`]
module"]
#[doc(alias = "DT30")]
pub type Dt30 = crate::Reg<dt30::Dt30Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt30;
#[doc = "DT31 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt31`]
module"]
#[doc(alias = "DT31")]
pub type Dt31 = crate::Reg<dt31::Dt31Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt31;
#[doc = "DT32 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt32`]
module"]
#[doc(alias = "DT32")]
pub type Dt32 = crate::Reg<dt32::Dt32Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt32;
#[doc = "DT33 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt33`]
module"]
#[doc(alias = "DT33")]
pub type Dt33 = crate::Reg<dt33::Dt33Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt33;
#[doc = "DT34 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt34`]
module"]
#[doc(alias = "DT34")]
pub type Dt34 = crate::Reg<dt34::Dt34Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt34;
#[doc = "DT35 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt35`]
module"]
#[doc(alias = "DT35")]
pub type Dt35 = crate::Reg<dt35::Dt35Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt35;
#[doc = "DT36 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt36`]
module"]
#[doc(alias = "DT36")]
pub type Dt36 = crate::Reg<dt36::Dt36Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt36;
#[doc = "DT37 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt37`]
module"]
#[doc(alias = "DT37")]
pub type Dt37 = crate::Reg<dt37::Dt37Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt37;
#[doc = "DT38 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt38::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt38::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt38`]
module"]
#[doc(alias = "DT38")]
pub type Dt38 = crate::Reg<dt38::Dt38Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt38;
#[doc = "DT39 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt39`]
module"]
#[doc(alias = "DT39")]
pub type Dt39 = crate::Reg<dt39::Dt39Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt39;
#[doc = "DT40 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt40`]
module"]
#[doc(alias = "DT40")]
pub type Dt40 = crate::Reg<dt40::Dt40Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt40;
#[doc = "DT41 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt41::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt41::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt41`]
module"]
#[doc(alias = "DT41")]
pub type Dt41 = crate::Reg<dt41::Dt41Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt41;
#[doc = "DT42 (rw) register accessor: Battery powered domain data register (BPR_DTx)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt42::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt42::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt42`]
module"]
#[doc(alias = "DT42")]
pub type Dt42 = crate::Reg<dt42::Dt42Spec>;
#[doc = "Battery powered domain data register (BPR_DTx)"]
pub mod dt42;
#[doc = "RTCCAL (rw) register accessor: RTC clock calibration register (BPR_RTCCAL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccal`]
module"]
#[doc(alias = "RTCCAL")]
pub type Rtccal = crate::Reg<rtccal::RtccalSpec>;
#[doc = "RTC clock calibration register (BPR_RTCCAL)"]
pub mod rtccal;
#[doc = "CTRL (rw) register accessor: BPR control register (BPR_CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "BPR control register (BPR_CTRL)"]
pub mod ctrl;
#[doc = "CTRLSTS (rw) register accessor: BPR control/status register (BPR_CTRLSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlsts`]
module"]
#[doc(alias = "CTRLSTS")]
pub type Ctrlsts = crate::Reg<ctrlsts::CtrlstsSpec>;
#[doc = "BPR control/status register (BPR_CTRLSTS)"]
pub mod ctrlsts;
