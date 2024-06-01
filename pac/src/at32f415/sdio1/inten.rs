#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `CMDFAIL` reader - CCRCFAILIE"]
pub type CmdfailR = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - CCRCFAILIE"]
pub type CmdfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAIL` reader - DCRCFAILIE"]
pub type DtfailR = crate::BitReader;
#[doc = "Field `DTFAIL` writer - DCRCFAILIE"]
pub type DtfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUTIE"]
pub type CmdtimeoutR = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - CTIMEOUTIE"]
pub type CmdtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUTIE"]
pub type DttimeoutR = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - DTIMEOUTIE"]
pub type DttimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRU` reader - TXUNDERRIE"]
pub type TxerruR = crate::BitReader;
#[doc = "Field `TXERRU` writer - TXUNDERRIE"]
pub type TxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRU` reader - RXOVERRIE"]
pub type RxerruR = crate::BitReader;
#[doc = "Field `RXERRU` writer - RXOVERRIE"]
pub type RxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPL` reader - CMDRENDIE"]
pub type CmdrspcmplR = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - CMDRENDIE"]
pub type CmdrspcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPL` reader - CMDSENTIE"]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - CMDSENTIE"]
pub type CmdcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPL` reader - DATAENDIE"]
pub type DtcmplR = crate::BitReader;
#[doc = "Field `DTCMPL` writer - DATAENDIE"]
pub type DtcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERR` reader - STBITERRIE"]
pub type SbiterrR = crate::BitReader;
#[doc = "Field `SBITERR` writer - STBITERRIE"]
pub type SbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPL` reader - DBACKENDIE"]
pub type DtblkcmplR = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - DBACKENDIE"]
pub type DtblkcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOCMD` reader - CMDACTIE"]
pub type DocmdR = crate::BitReader;
#[doc = "Field `DOCMD` writer - CMDACTIE"]
pub type DocmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOTX` reader - TXACTIE"]
pub type DotxR = crate::BitReader;
#[doc = "Field `DOTX` writer - TXACTIE"]
pub type DotxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DORX` reader - RXACTIE"]
pub type DorxR = crate::BitReader;
#[doc = "Field `DORX` writer - RXACTIE"]
pub type DorxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_H` reader - TXFIFOHEIE"]
pub type TxbufHR = crate::BitReader;
#[doc = "Field `TXBUF_H` writer - TXFIFOHEIE"]
pub type TxbufHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_H` reader - RXFIFOHFIE"]
pub type RxbufHR = crate::BitReader;
#[doc = "Field `RXBUF_H` writer - RXFIFOHFIE"]
pub type RxbufHW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_F` reader - TXFIFOFIE"]
pub type TxbufFR = crate::BitReader;
#[doc = "Field `TXBUF_F` writer - TXFIFOFIE"]
pub type TxbufFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_F` reader - RXFIFOFIE"]
pub type RxbufFR = crate::BitReader;
#[doc = "Field `RXBUF_F` writer - RXFIFOFIE"]
pub type RxbufFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF_E` reader - TXFIFOEIE"]
pub type TxbufER = crate::BitReader;
#[doc = "Field `TXBUF_E` writer - TXFIFOEIE"]
pub type TxbufEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF_E` reader - RXFIFOEIE"]
pub type RxbufER = crate::BitReader;
#[doc = "Field `RXBUF_E` writer - RXFIFOEIE"]
pub type RxbufEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUF` reader - TXDAVLIE"]
pub type TxbufR = crate::BitReader;
#[doc = "Field `TXBUF` writer - TXDAVLIE"]
pub type TxbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUF` reader - RXDAVLIE"]
pub type RxbufR = crate::BitReader;
#[doc = "Field `RXBUF` writer - RXDAVLIE"]
pub type RxbufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIF` reader - SDIOITIE"]
pub type SdioifR = crate::BitReader;
#[doc = "Field `SDIOIF` writer - SDIOITIE"]
pub type SdioifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CmdfailR {
        CmdfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    pub fn dtfail(&self) -> DtfailR {
        DtfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CmdtimeoutR {
        CmdtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DttimeoutR {
        DttimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    pub fn txerru(&self) -> TxerruR {
        TxerruR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    pub fn rxerru(&self) -> RxerruR {
        RxerruR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CmdrspcmplR {
        CmdrspcmplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DtcmplR {
        DtcmplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SbiterrR {
        SbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DtblkcmplR {
        DtblkcmplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    pub fn docmd(&self) -> DocmdR {
        DocmdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    pub fn dotx(&self) -> DotxR {
        DotxR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    pub fn dorx(&self) -> DorxR {
        DorxR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    pub fn txbuf_h(&self) -> TxbufHR {
        TxbufHR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    pub fn rxbuf_h(&self) -> RxbufHR {
        RxbufHR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    pub fn txbuf_f(&self) -> TxbufFR {
        TxbufFR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    pub fn rxbuf_f(&self) -> RxbufFR {
        RxbufFR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    pub fn txbuf_e(&self) -> TxbufER {
        TxbufER::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    pub fn rxbuf_e(&self) -> RxbufER {
        RxbufER::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    pub fn txbuf(&self) -> TxbufR {
        TxbufR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    pub fn rxbuf(&self) -> RxbufR {
        RxbufR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    pub fn sdioif(&self) -> SdioifR {
        SdioifR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("cmdfail", &self.cmdfail())
            .field("dtfail", &self.dtfail())
            .field("cmdtimeout", &self.cmdtimeout())
            .field("dttimeout", &self.dttimeout())
            .field("txerru", &self.txerru())
            .field("rxerru", &self.rxerru())
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
impl W {
    #[doc = "Bit 0 - CCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfail(&mut self) -> CmdfailW<IntenSpec> {
        CmdfailW::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtfail(&mut self) -> DtfailW<IntenSpec> {
        DtfailW::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeout(&mut self) -> CmdtimeoutW<IntenSpec> {
        CmdtimeoutW::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dttimeout(&mut self) -> DttimeoutW<IntenSpec> {
        DttimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn txerru(&mut self) -> TxerruW<IntenSpec> {
        TxerruW::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxerru(&mut self) -> RxerruW<IntenSpec> {
        RxerruW::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrspcmpl(&mut self) -> CmdrspcmplW<IntenSpec> {
        CmdrspcmplW::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTIE"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CmdcmplW<IntenSpec> {
        CmdcmplW::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmpl(&mut self) -> DtcmplW<IntenSpec> {
        DtcmplW::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRIE"]
    #[inline(always)]
    #[must_use]
    pub fn sbiterr(&mut self) -> SbiterrW<IntenSpec> {
        SbiterrW::new(self, 9)
    }
    #[doc = "Bit 10 - DBACKENDIE"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkcmpl(&mut self) -> DtblkcmplW<IntenSpec> {
        DtblkcmplW::new(self, 10)
    }
    #[doc = "Bit 11 - CMDACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn docmd(&mut self) -> DocmdW<IntenSpec> {
        DocmdW::new(self, 11)
    }
    #[doc = "Bit 12 - TXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dotx(&mut self) -> DotxW<IntenSpec> {
        DotxW::new(self, 12)
    }
    #[doc = "Bit 13 - RXACTIE"]
    #[inline(always)]
    #[must_use]
    pub fn dorx(&mut self) -> DorxW<IntenSpec> {
        DorxW::new(self, 13)
    }
    #[doc = "Bit 14 - TXFIFOHEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_h(&mut self) -> TxbufHW<IntenSpec> {
        TxbufHW::new(self, 14)
    }
    #[doc = "Bit 15 - RXFIFOHFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_h(&mut self) -> RxbufHW<IntenSpec> {
        RxbufHW::new(self, 15)
    }
    #[doc = "Bit 16 - TXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_f(&mut self) -> TxbufFW<IntenSpec> {
        TxbufFW::new(self, 16)
    }
    #[doc = "Bit 17 - RXFIFOFIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_f(&mut self) -> RxbufFW<IntenSpec> {
        RxbufFW::new(self, 17)
    }
    #[doc = "Bit 18 - TXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf_e(&mut self) -> TxbufEW<IntenSpec> {
        TxbufEW::new(self, 18)
    }
    #[doc = "Bit 19 - RXFIFOEIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf_e(&mut self) -> RxbufEW<IntenSpec> {
        RxbufEW::new(self, 19)
    }
    #[doc = "Bit 20 - TXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn txbuf(&mut self) -> TxbufW<IntenSpec> {
        TxbufW::new(self, 20)
    }
    #[doc = "Bit 21 - RXDAVLIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuf(&mut self) -> RxbufW<IntenSpec> {
        RxbufW::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOITIE"]
    #[inline(always)]
    #[must_use]
    pub fn sdioif(&mut self) -> SdioifW<IntenSpec> {
        SdioifW::new(self, 22)
    }
}
#[doc = "SDIO interrupt enable register (SDIO_INTEN)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
