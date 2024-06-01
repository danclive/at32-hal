#[doc = "Register `INTCLR` reader"]
pub type R = crate::R<IntclrSpec>;
#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `CMDFAIL` reader - CCRCFAILC"]
pub type CmdfailR = crate::BitReader;
#[doc = "Field `CMDFAIL` writer - CCRCFAILC"]
pub type CmdfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFAIL` reader - DCRCFAILC"]
pub type DtfailR = crate::BitReader;
#[doc = "Field `DTFAIL` writer - DCRCFAILC"]
pub type DtfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDTIMEOUT` reader - CTIMEOUTC"]
pub type CmdtimeoutR = crate::BitReader;
#[doc = "Field `CMDTIMEOUT` writer - CTIMEOUTC"]
pub type CmdtimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTTIMEOUT` reader - DTIMEOUTC"]
pub type DttimeoutR = crate::BitReader;
#[doc = "Field `DTTIMEOUT` writer - DTIMEOUTC"]
pub type DttimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRU` reader - TXUNDERRC"]
pub type TxerruR = crate::BitReader;
#[doc = "Field `TXERRU` writer - TXUNDERRC"]
pub type TxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERRU` reader - RXOVERRC"]
pub type RxerruR = crate::BitReader;
#[doc = "Field `RXERRU` writer - RXOVERRC"]
pub type RxerruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRSPCMPL` reader - CMDRENDC"]
pub type CmdrspcmplR = crate::BitReader;
#[doc = "Field `CMDRSPCMPL` writer - CMDRENDC"]
pub type CmdrspcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDCMPL` reader - CMDSENTC"]
pub type CmdcmplR = crate::BitReader;
#[doc = "Field `CMDCMPL` writer - CMDSENTC"]
pub type CmdcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCMPL` reader - DATAENDC"]
pub type DtcmplR = crate::BitReader;
#[doc = "Field `DTCMPL` writer - DATAENDC"]
pub type DtcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBITERR` reader - STBITERRC"]
pub type SbiterrR = crate::BitReader;
#[doc = "Field `SBITERR` writer - STBITERRC"]
pub type SbiterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTBLKCMPL` reader - DBCKENDC"]
pub type DtblkcmplR = crate::BitReader;
#[doc = "Field `DTBLKCMPL` writer - DBCKENDC"]
pub type DtblkcmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOIF` reader - SDIOITC"]
pub type SdioifR = crate::BitReader;
#[doc = "Field `SDIOIF` writer - SDIOITC"]
pub type SdioifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    pub fn cmdfail(&self) -> CmdfailR {
        CmdfailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    pub fn dtfail(&self) -> DtfailR {
        DtfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    pub fn cmdtimeout(&self) -> CmdtimeoutR {
        CmdtimeoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    pub fn dttimeout(&self) -> DttimeoutR {
        DttimeoutR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    pub fn txerru(&self) -> TxerruR {
        TxerruR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    pub fn rxerru(&self) -> RxerruR {
        RxerruR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    pub fn cmdrspcmpl(&self) -> CmdrspcmplR {
        CmdrspcmplR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    pub fn cmdcmpl(&self) -> CmdcmplR {
        CmdcmplR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    pub fn dtcmpl(&self) -> DtcmplR {
        DtcmplR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    pub fn sbiterr(&self) -> SbiterrR {
        SbiterrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    pub fn dtblkcmpl(&self) -> DtblkcmplR {
        DtblkcmplR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    pub fn sdioif(&self) -> SdioifR {
        SdioifR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTCLR")
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
            .field("sdioif", &self.sdioif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdfail(&mut self) -> CmdfailW<IntclrSpec> {
        CmdfailW::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAILC"]
    #[inline(always)]
    #[must_use]
    pub fn dtfail(&mut self) -> DtfailW<IntclrSpec> {
        DtfailW::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtimeout(&mut self) -> CmdtimeoutW<IntclrSpec> {
        CmdtimeoutW::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUTC"]
    #[inline(always)]
    #[must_use]
    pub fn dttimeout(&mut self) -> DttimeoutW<IntclrSpec> {
        DttimeoutW::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERRC"]
    #[inline(always)]
    #[must_use]
    pub fn txerru(&mut self) -> TxerruW<IntclrSpec> {
        TxerruW::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERRC"]
    #[inline(always)]
    #[must_use]
    pub fn rxerru(&mut self) -> RxerruW<IntclrSpec> {
        RxerruW::new(self, 5)
    }
    #[doc = "Bit 6 - CMDRENDC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdrspcmpl(&mut self) -> CmdrspcmplW<IntclrSpec> {
        CmdrspcmplW::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENTC"]
    #[inline(always)]
    #[must_use]
    pub fn cmdcmpl(&mut self) -> CmdcmplW<IntclrSpec> {
        CmdcmplW::new(self, 7)
    }
    #[doc = "Bit 8 - DATAENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dtcmpl(&mut self) -> DtcmplW<IntclrSpec> {
        DtcmplW::new(self, 8)
    }
    #[doc = "Bit 9 - STBITERRC"]
    #[inline(always)]
    #[must_use]
    pub fn sbiterr(&mut self) -> SbiterrW<IntclrSpec> {
        SbiterrW::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKENDC"]
    #[inline(always)]
    #[must_use]
    pub fn dtblkcmpl(&mut self) -> DtblkcmplW<IntclrSpec> {
        DtblkcmplW::new(self, 10)
    }
    #[doc = "Bit 22 - SDIOITC"]
    #[inline(always)]
    #[must_use]
    pub fn sdioif(&mut self) -> SdioifW<IntclrSpec> {
        SdioifW::new(self, 22)
    }
}
#[doc = "SDIO interrupt clear register (SDIO_INTCLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intclr::R`](R) reader structure"]
impl crate::Readable for IntclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
