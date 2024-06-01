#[repr(C)]
#[derive(Debug)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrctrl: Pwrctrl,
    clkctrl: Clkctrl,
    argu: Argu,
    cmdctrl: Cmdctrl,
    rspcmd: Rspcmd,
    rsp1: Rsp1,
    rsp2: Rsp2,
    rsp3: Rsp3,
    rsp4: Rsp4,
    dttmr: Dttmr,
    dtlen: Dtlen,
    dtctrl: Dtctrl,
    dtcnt: Dtcnt,
    sts: Sts,
    intclr: Intclr,
    inten: Inten,
    _reserved16: [u8; 0x08],
    bufcnt: Bufcnt,
    _reserved17: [u8; 0x34],
    buf: Buf,
}
impl RegisterBlock {
    #[doc = "0x00 - Bits 1:0 = PWRCTRL: Power supply control bits"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    #[doc = "0x04 - SD clock control register (SDIO_CLKCTRL)"]
    #[inline(always)]
    pub const fn clkctrl(&self) -> &Clkctrl {
        &self.clkctrl
    }
    #[doc = "0x08 - Bits 31:0 = : Command argument"]
    #[inline(always)]
    pub const fn argu(&self) -> &Argu {
        &self.argu
    }
    #[doc = "0x0c - SDIO command control register (SDIO_CMDCTRL)"]
    #[inline(always)]
    pub const fn cmdctrl(&self) -> &Cmdctrl {
        &self.cmdctrl
    }
    #[doc = "0x10 - SDIO command register"]
    #[inline(always)]
    pub const fn rspcmd(&self) -> &Rspcmd {
        &self.rspcmd
    }
    #[doc = "0x14 - Bits 31:0 = CARDSTATUS1"]
    #[inline(always)]
    pub const fn rsp1(&self) -> &Rsp1 {
        &self.rsp1
    }
    #[doc = "0x18 - Bits 31:0 = CARDSTATUS2"]
    #[inline(always)]
    pub const fn rsp2(&self) -> &Rsp2 {
        &self.rsp2
    }
    #[doc = "0x1c - Bits 31:0 = CARDSTATUS3"]
    #[inline(always)]
    pub const fn rsp3(&self) -> &Rsp3 {
        &self.rsp3
    }
    #[doc = "0x20 - Bits 31:0 = CARDSTATUS4"]
    #[inline(always)]
    pub const fn rsp4(&self) -> &Rsp4 {
        &self.rsp4
    }
    #[doc = "0x24 - Bits 31:0 = TIMEOUT: Data timeout period"]
    #[inline(always)]
    pub const fn dttmr(&self) -> &Dttmr {
        &self.dttmr
    }
    #[doc = "0x28 - Bits 24:0 = DATALENGTH: Data length value"]
    #[inline(always)]
    pub const fn dtlen(&self) -> &Dtlen {
        &self.dtlen
    }
    #[doc = "0x2c - SDIO data control register (SDIO_DCTRL)"]
    #[inline(always)]
    pub const fn dtctrl(&self) -> &Dtctrl {
        &self.dtctrl
    }
    #[doc = "0x30 - Bits 24:0 = DATACOUNT: Data count value"]
    #[inline(always)]
    pub const fn dtcnt(&self) -> &Dtcnt {
        &self.dtcnt
    }
    #[doc = "0x34 - SDIO status register (SDIO_STS)"]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
    #[doc = "0x38 - SDIO interrupt clear register (SDIO_INTCLR)"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x3c - SDIO mask register (SDIO_INTEN)"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x48 - Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO"]
    #[inline(always)]
    pub const fn bufcnt(&self) -> &Bufcnt {
        &self.bufcnt
    }
    #[doc = "0x80 - bits 31:0 = Buffer Data: Receive and transmit buffer data"]
    #[inline(always)]
    pub const fn buf(&self) -> &Buf {
        &self.buf
    }
}
#[doc = "PWRCTRL (rw) register accessor: Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`]
module"]
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PwrctrlSpec>;
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits"]
pub mod pwrctrl;
#[doc = "CLKCTRL (rw) register accessor: SD clock control register (SDIO_CLKCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctrl`]
module"]
#[doc(alias = "CLKCTRL")]
pub type Clkctrl = crate::Reg<clkctrl::ClkctrlSpec>;
#[doc = "SD clock control register (SDIO_CLKCTRL)"]
pub mod clkctrl;
#[doc = "ARGU (rw) register accessor: Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argu`]
module"]
#[doc(alias = "ARGU")]
pub type Argu = crate::Reg<argu::ArguSpec>;
#[doc = "Bits 31:0 = : Command argument"]
pub mod argu;
#[doc = "CMDCTRL (rw) register accessor: SDIO command control register (SDIO_CMDCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdctrl`]
module"]
#[doc(alias = "CMDCTRL")]
pub type Cmdctrl = crate::Reg<cmdctrl::CmdctrlSpec>;
#[doc = "SDIO command control register (SDIO_CMDCTRL)"]
pub mod cmdctrl;
#[doc = "RSPCMD (r) register accessor: SDIO command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rspcmd::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rspcmd`]
module"]
#[doc(alias = "RSPCMD")]
pub type Rspcmd = crate::Reg<rspcmd::RspcmdSpec>;
#[doc = "SDIO command register"]
pub mod rspcmd;
#[doc = "RSP1 (r) register accessor: Bits 31:0 = CARDSTATUS1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp1`]
module"]
#[doc(alias = "RSP1")]
pub type Rsp1 = crate::Reg<rsp1::Rsp1Spec>;
#[doc = "Bits 31:0 = CARDSTATUS1"]
pub mod rsp1;
#[doc = "RSP2 (r) register accessor: Bits 31:0 = CARDSTATUS2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp2`]
module"]
#[doc(alias = "RSP2")]
pub type Rsp2 = crate::Reg<rsp2::Rsp2Spec>;
#[doc = "Bits 31:0 = CARDSTATUS2"]
pub mod rsp2;
#[doc = "RSP3 (r) register accessor: Bits 31:0 = CARDSTATUS3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp3`]
module"]
#[doc(alias = "RSP3")]
pub type Rsp3 = crate::Reg<rsp3::Rsp3Spec>;
#[doc = "Bits 31:0 = CARDSTATUS3"]
pub mod rsp3;
#[doc = "RSP4 (r) register accessor: Bits 31:0 = CARDSTATUS4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsp4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsp4`]
module"]
#[doc(alias = "RSP4")]
pub type Rsp4 = crate::Reg<rsp4::Rsp4Spec>;
#[doc = "Bits 31:0 = CARDSTATUS4"]
pub mod rsp4;
#[doc = "DTTMR (rw) register accessor: Bits 31:0 = TIMEOUT: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dttmr`]
module"]
#[doc(alias = "DTTMR")]
pub type Dttmr = crate::Reg<dttmr::DttmrSpec>;
#[doc = "Bits 31:0 = TIMEOUT: Data timeout period"]
pub mod dttmr;
#[doc = "DTLEN (rw) register accessor: Bits 24:0 = DATALENGTH: Data length value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtlen`]
module"]
#[doc(alias = "DTLEN")]
pub type Dtlen = crate::Reg<dtlen::DtlenSpec>;
#[doc = "Bits 24:0 = DATALENGTH: Data length value"]
pub mod dtlen;
#[doc = "DTCTRL (rw) register accessor: SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctrl`]
module"]
#[doc(alias = "DTCTRL")]
pub type Dtctrl = crate::Reg<dtctrl::DtctrlSpec>;
#[doc = "SDIO data control register (SDIO_DCTRL)"]
pub mod dtctrl;
#[doc = "DTCNT (r) register accessor: Bits 24:0 = DATACOUNT: Data count value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcnt`]
module"]
#[doc(alias = "DTCNT")]
pub type Dtcnt = crate::Reg<dtcnt::DtcntSpec>;
#[doc = "Bits 24:0 = DATACOUNT: Data count value"]
pub mod dtcnt;
#[doc = "STS (r) register accessor: SDIO status register (SDIO_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "STS")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "SDIO status register (SDIO_STS)"]
pub mod sts;
#[doc = "INTCLR (rw) register accessor: SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`]
module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)"]
pub mod intclr;
#[doc = "INTEN (rw) register accessor: SDIO mask register (SDIO_INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "SDIO mask register (SDIO_INTEN)"]
pub mod inten;
#[doc = "BUFCNT (r) register accessor: Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bufcnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufcnt`]
module"]
#[doc(alias = "BUFCNT")]
pub type Bufcnt = crate::Reg<bufcnt::BufcntSpec>;
#[doc = "Bits 23:0 = BUFCOUNT: Remaining number of words to be written to or read from the FIFO"]
pub mod bufcnt;
#[doc = "BUF (rw) register accessor: bits 31:0 = Buffer Data: Receive and transmit buffer data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buf`]
module"]
#[doc(alias = "BUF")]
pub type Buf = crate::Reg<buf::BufSpec>;
#[doc = "bits 31:0 = Buffer Data: Receive and transmit buffer data"]
pub mod buf;
