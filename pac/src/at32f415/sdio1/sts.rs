#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `CMDFAIL` reader - CCRCFAIL"]
pub type CmdfailR = crate::BitReader;
#[doc = "Field `DTFAIL` reader - DCRCFAIL"]
pub type DtfailR = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUT"]
pub type CmdtimeoutR = crate::BitReader;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUT"]
pub type DttimeoutR = crate::BitReader;
#[doc = "Field `TXERRU` reader - TXUNDERR"]
pub type TxerruR = crate::BitReader;
#[doc = "Field `RXERRO` reader - RXOVERR"]
pub type RxerroR = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` reader - CMDREND"]
pub type CmdrspcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` reader - CMDSENT"]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `DTCMPL` reader - DATAEND"]
pub type DtcmplR = crate::BitReader;
#[doc = "Field `SBITERR` reader - STBITERR"]
pub type SbiterrR = crate::BitReader;
#[doc = "Field `DTBLKCMPL` reader - DBCKEND"]
pub type DtblkcmplR = crate::BitReader;
#[doc = "Field `DOCMD` reader - CMDACT"]
pub type DocmdR = crate::BitReader;
#[doc = "Field `DOTX` reader - TXACT"]
pub type DotxR = crate::BitReader;
#[doc = "Field `DORX` reader - RXACT"]
pub type DorxR = crate::BitReader;
#[doc = "Field `TXBUF_H` reader - TXFIFOHE"]
pub type TxbufHR = crate::BitReader;
#[doc = "Field `RXBUF_H` reader - RXFIFOHF"]
pub type RxbufHR = crate::BitReader;
#[doc = "Field `TXBUF_F` reader - TXFIFOF"]
pub type TxbufFR = crate::BitReader;
#[doc = "Field `RXBUF_F` reader - RXFIFOF"]
pub type RxbufFR = crate::BitReader;
#[doc = "Field `TXBUF_E` reader - TXFIFOE"]
pub type TxbufER = crate::BitReader;
#[doc = "Field `RXBUF_E` reader - RXFIFOE"]
pub type RxbufER = crate::BitReader;
#[doc = "Field `TXBUF` reader - TXDAVL"]
pub type TxbufR = crate::BitReader;
#[doc = "Field `RXBUF` reader - RXDAVL"]
pub type RxbufR = crate::BitReader;
#[doc = "Field `SDIOIF` reader - SDIOIT"]
pub type SdioifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CCRCFAIL"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CmdfailR {
        CmdfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL"]
    #[inline(always)]
    pub fn dtfail(&self) -> DtfailR {
        DtfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CmdtimeoutR {
        CmdtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DttimeoutR {
        DttimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR"]
    #[inline(always)]
    pub fn txerru(&self) -> TxerruR {
        TxerruR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR"]
    #[inline(always)]
    pub fn rxerro(&self) -> RxerroR {
        RxerroR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CmdrspcmplR {
        CmdrspcmplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DtcmplR {
        DtcmplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERR"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SbiterrR {
        SbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DtblkcmplR {
        DtblkcmplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACT"]
    #[inline(always)]
    pub fn docmd(&self) -> DocmdR {
        DocmdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACT"]
    #[inline(always)]
    pub fn dotx(&self) -> DotxR {
        DotxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACT"]
    #[inline(always)]
    pub fn dorx(&self) -> DorxR {
        DorxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHE"]
    #[inline(always)]
    pub fn txbuf_h(&self) -> TxbufHR {
        TxbufHR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHF"]
    #[inline(always)]
    pub fn rxbuf_h(&self) -> RxbufHR {
        RxbufHR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOF"]
    #[inline(always)]
    pub fn txbuf_f(&self) -> TxbufFR {
        TxbufFR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOF"]
    #[inline(always)]
    pub fn rxbuf_f(&self) -> RxbufFR {
        RxbufFR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOE"]
    #[inline(always)]
    pub fn txbuf_e(&self) -> TxbufER {
        TxbufER::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOE"]
    #[inline(always)]
    pub fn rxbuf_e(&self) -> RxbufER {
        RxbufER::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVL"]
    #[inline(always)]
    pub fn txbuf(&self) -> TxbufR {
        TxbufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVL"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RxbufR {
        RxbufR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT"]
    #[inline(always)]
    pub fn sdioif(&self) -> SdioifR {
        SdioifR::new(((self.bits >> 22) & 1) != 0)
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
            .field("txbuf_h", &self.txbuf_h())
            .field("rxbuf_h", &self.rxbuf_h())
            .field("txbuf_f", &self.txbuf_f())
            .field("rxbuf_f", &self.rxbuf_f())
            .field("txbuf_e", &self.txbuf_e())
            .field("rxbuf_e", &self.rxbuf_e())
            .field("txbuf", &self.txbuf())
            .field("rxbuf", &self.rxbuf())
            .field("sdioif", &self.sdioif())
            .finish()
    }
}
#[doc = "SDIO status register (SDIO_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
