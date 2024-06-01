#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `CMDFAIL` reader - Command crc fail"]
pub type CmdfailR = crate::BitReader;
#[doc = "Field `DTFAIL` reader - Data crc fail"]
pub type DtfailR = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` reader - Command timeout"]
pub type CmdtimeoutR = crate::BitReader;
#[doc = "Field `DTTIMEOUT` reader - Data timeout"]
pub type DttimeoutR = crate::BitReader;
#[doc = "Field `TXERRU` reader - Tx under run error"]
pub type TxerruR = crate::BitReader;
#[doc = "Field `RXERRO` reader - Rx over run error"]
pub type RxerroR = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` reader - Command response complete"]
pub type CmdrspcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` reader - Command sent"]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `DTCMPL` reader - Data sent"]
pub type DtcmplR = crate::BitReader;
#[doc = "Field `SBITERR` reader - Start bit error"]
pub type SbiterrR = crate::BitReader;
#[doc = "Field `DTBLKCMPL` reader - Data block sent"]
pub type DtblkcmplR = crate::BitReader;
#[doc = "Field `DOCMD` reader - Command transfer in progress"]
pub type DocmdR = crate::BitReader;
#[doc = "Field `DOTX` reader - Data transmit in progress"]
pub type DotxR = crate::BitReader;
#[doc = "Field `DORX` reader - Data receive in progress"]
pub type DorxR = crate::BitReader;
#[doc = "Field `TXBUFH` reader - Tx buffer half empty"]
pub type TxbufhR = crate::BitReader;
#[doc = "Field `RXBUFH` reader - Rx buffer half empty"]
pub type RxbufhR = crate::BitReader;
#[doc = "Field `TXBUFF` reader - Tx buffer full"]
pub type TxbuffR = crate::BitReader;
#[doc = "Field `RXBUFF` reader - Rx buffer full"]
pub type RxbuffR = crate::BitReader;
#[doc = "Field `TXBUFE` reader - Tx buffer empty"]
pub type TxbufeR = crate::BitReader;
#[doc = "Field `RXBUFE` reader - Rx buffer empty"]
pub type RxbufeR = crate::BitReader;
#[doc = "Field `TXBUF` reader - Tx data vaild"]
pub type TxbufR = crate::BitReader;
#[doc = "Field `RXBUF` reader - Rx data vaild"]
pub type RxbufR = crate::BitReader;
#[doc = "Field `IOIF` reader - SD I/O interrupt"]
pub type IoifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command crc fail"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CmdfailR {
        CmdfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data crc fail"]
    #[inline(always)]
    pub fn dtfail(&self) -> DtfailR {
        DtfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CmdtimeoutR {
        CmdtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DttimeoutR {
        DttimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx under run error"]
    #[inline(always)]
    pub fn txerru(&self) -> TxerruR {
        TxerruR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx over run error"]
    #[inline(always)]
    pub fn rxerro(&self) -> RxerroR {
        RxerroR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response complete"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CmdrspcmplR {
        CmdrspcmplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data sent"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DtcmplR {
        DtcmplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start bit error"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SbiterrR {
        SbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block sent"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DtblkcmplR {
        DtblkcmplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Command transfer in progress"]
    #[inline(always)]
    pub fn docmd(&self) -> DocmdR {
        DocmdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Data transmit in progress"]
    #[inline(always)]
    pub fn dotx(&self) -> DotxR {
        DotxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Data receive in progress"]
    #[inline(always)]
    pub fn dorx(&self) -> DorxR {
        DorxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx buffer half empty"]
    #[inline(always)]
    pub fn txbufh(&self) -> TxbufhR {
        TxbufhR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx buffer half empty"]
    #[inline(always)]
    pub fn rxbufh(&self) -> RxbufhR {
        RxbufhR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tx buffer full"]
    #[inline(always)]
    pub fn txbuff(&self) -> TxbuffR {
        TxbuffR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx buffer full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RxbuffR {
        RxbuffR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx buffer empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TxbufeR {
        TxbufeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Rx buffer empty"]
    #[inline(always)]
    pub fn rxbufe(&self) -> RxbufeR {
        RxbufeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tx data vaild"]
    #[inline(always)]
    pub fn txbuf(&self) -> TxbufR {
        TxbufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rx data vaild"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RxbufR {
        RxbufR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SD I/O interrupt"]
    #[inline(always)]
    pub fn ioif(&self) -> IoifR {
        IoifR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("cmdfail", &self.cmdfail())
            .field("dtfail", &self.dtfail())
            .field("cmdtimeout", &self.cmdtimeout())
            .field("dttimeout", &self.dttimeout())
            .field("txerru", &self.txerru())
            .field("rxerro", &self.rxerro())
            .field("cmdrspcmpl", &self.cmdrspcmpl())
            .field("cmdcmpl", &self.cmdcmpl())
            .field("dtcmpl", &self.dtcmpl())
            .field("sbiterr", &self.sbiterr())
            .field("dtblkcmpl", &self.dtblkcmpl())
            .field("docmd", &self.docmd())
            .field("dotx", &self.dotx())
            .field("dorx", &self.dorx())
            .field("txbufh", &self.txbufh())
            .field("rxbufh", &self.rxbufh())
            .field("txbuff", &self.txbuff())
            .field("rxbuff", &self.rxbuff())
            .field("txbufe", &self.txbufe())
            .field("rxbufe", &self.rxbufe())
            .field("txbuf", &self.txbuf())
            .field("rxbuf", &self.rxbuf())
            .field("ioif", &self.ioif())
            .finish()
    }
}
#[doc = "SDIO status register (SDIO_STA)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
