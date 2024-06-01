#[doc = "Register `GINTSTS` reader"]
pub type R = crate::R<GintstsSpec>;
#[doc = "Register `GINTSTS` writer"]
pub type W = crate::W<GintstsSpec>;
#[doc = "Field `CURMOD` reader - Current mode of operation"]
pub type CurmodR = crate::BitReader;
#[doc = "Field `MODEMIS` reader - Mode mismatch interrupt"]
pub type ModemisR = crate::BitReader;
#[doc = "Field `MODEMIS` writer - Mode mismatch interrupt"]
pub type ModemisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINT` reader - OTG interrupt"]
pub type OtgintR = crate::BitReader;
#[doc = "Field `SOF` reader - Start of frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - Start of frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVL` reader - RxFIFO non-empty"]
pub type RxflvlR = crate::BitReader;
#[doc = "Field `NPTXFEMP` reader - Non-periodic TxFIFO empty"]
pub type NptxfempR = crate::BitReader;
#[doc = "Field `GINNAKEFF` reader - Global IN non-periodic NAK effective"]
pub type GinnakeffR = crate::BitReader;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK effective"]
pub type GoutnakeffR = crate::BitReader;
#[doc = "Field `ERLYSUSP` reader - Early suspend"]
pub type ErlysuspR = crate::BitReader;
#[doc = "Field `ERLYSUSP` writer - Early suspend"]
pub type ErlysuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSP` reader - USB suspend"]
pub type UsbsuspR = crate::BitReader;
#[doc = "Field `USBSUSP` writer - USB suspend"]
pub type UsbsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRST` reader - USB reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - USB reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONE` reader - Enumeration done"]
pub type EnumdoneR = crate::BitReader;
#[doc = "Field `ENUMDONE` writer - Enumeration done"]
pub type EnumdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROP` reader - Isochronous OUT packet dropped interrupt"]
pub type IsooutdropR = crate::BitReader;
#[doc = "Field `ISOOUTDROP` writer - Isochronous OUT packet dropped interrupt"]
pub type IsooutdropW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPF` reader - End of periodic frame interrupt"]
pub type EopfR = crate::BitReader;
#[doc = "Field `EOPF` writer - End of periodic frame interrupt"]
pub type EopfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPTINT` reader - IN endpoint interrupt"]
pub type IeptintR = crate::BitReader;
#[doc = "Field `OEPTINT` reader - OUT endpoint interrupt"]
pub type OeptintR = crate::BitReader;
#[doc = "Field `INCOMPISOIN` reader - Incomplete isochronous IN transfer"]
pub type IncompisoinR = crate::BitReader;
#[doc = "Field `INCOMPISOIN` writer - Incomplete isochronous IN transfer"]
pub type IncompisoinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPIP_INCOMPISOOUT` reader - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type IncompipIncompisooutR = crate::BitReader;
#[doc = "Field `INCOMPIP_INCOMPISOOUT` writer - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
pub type IncompipIncompisooutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINT` reader - Host port interrupt"]
pub type PrtintR = crate::BitReader;
#[doc = "Field `HCHINT` reader - Host channels interrupt"]
pub type HchintR = crate::BitReader;
#[doc = "Field `PTXFEMP` reader - Periodic TxFIFO empty"]
pub type PtxfempR = crate::BitReader;
#[doc = "Field `CONIDSCHG` reader - Connector ID status change"]
pub type ConidschgR = crate::BitReader;
#[doc = "Field `CONIDSCHG` writer - Connector ID status change"]
pub type ConidschgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONINT` reader - Disconnect detected interrupt"]
pub type DisconintR = crate::BitReader;
#[doc = "Field `DISCONINT` writer - Disconnect detected interrupt"]
pub type DisconintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINT` reader - Resume/remote wakeup detected interrupt"]
pub type WkupintR = crate::BitReader;
#[doc = "Field `WKUPINT` writer - Resume/remote wakeup detected interrupt"]
pub type WkupintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Current mode of operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CurmodR {
        CurmodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    pub fn modemis(&self) -> ModemisR {
        ModemisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt"]
    #[inline(always)]
    pub fn otgint(&self) -> OtgintR {
        OtgintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO non-empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RxflvlR {
        RxflvlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty"]
    #[inline(always)]
    pub fn nptxfemp(&self) -> NptxfempR {
        NptxfempR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN non-periodic NAK effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GinnakeffR {
        GinnakeffR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GoutnakeffR {
        GoutnakeffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ErlysuspR {
        ErlysuspR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> UsbsuspR {
        UsbsuspR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    pub fn enumdone(&self) -> EnumdoneR {
        EnumdoneR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> IsooutdropR {
        IsooutdropR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EopfR {
        EopfR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoint interrupt"]
    #[inline(always)]
    pub fn ieptint(&self) -> IeptintR {
        IeptintR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoint interrupt"]
    #[inline(always)]
    pub fn oeptint(&self) -> OeptintR {
        OeptintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> IncompisoinR {
        IncompisoinR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    pub fn incompip_incompisoout(&self) -> IncompipIncompisooutR {
        IncompipIncompisooutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt"]
    #[inline(always)]
    pub fn prtint(&self) -> PrtintR {
        PrtintR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt"]
    #[inline(always)]
    pub fn hchint(&self) -> HchintR {
        HchintR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty"]
    #[inline(always)]
    pub fn ptxfemp(&self) -> PtxfempR {
        PtxfempR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    pub fn conidschg(&self) -> ConidschgR {
        ConidschgR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    pub fn disconint(&self) -> DisconintR {
        DisconintR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WkupintR {
        WkupintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTSTS")
            .field("curmod", &self.curmod())
            .field("modemis", &self.modemis())
            .field("otgint", &self.otgint())
            .field("sof", &self.sof())
            .field("rxflvl", &self.rxflvl())
            .field("nptxfemp", &self.nptxfemp())
            .field("ginnakeff", &self.ginnakeff())
            .field("goutnakeff", &self.goutnakeff())
            .field("erlysusp", &self.erlysusp())
            .field("usbsusp", &self.usbsusp())
            .field("usbrst", &self.usbrst())
            .field("enumdone", &self.enumdone())
            .field("isooutdrop", &self.isooutdrop())
            .field("eopf", &self.eopf())
            .field("ieptint", &self.ieptint())
            .field("oeptint", &self.oeptint())
            .field("incompisoin", &self.incompisoin())
            .field("incompip_incompisoout", &self.incompip_incompisoout())
            .field("prtint", &self.prtint())
            .field("hchint", &self.hchint())
            .field("ptxfemp", &self.ptxfemp())
            .field("conidschg", &self.conidschg())
            .field("disconint", &self.disconint())
            .field("wkupint", &self.wkupint())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn modemis(&mut self) -> ModemisW<GintstsSpec> {
        ModemisW::new(self, 1)
    }
    #[doc = "Bit 3 - Start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<GintstsSpec> {
        SofW::new(self, 3)
    }
    #[doc = "Bit 10 - Early suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erlysusp(&mut self) -> ErlysuspW<GintstsSpec> {
        ErlysuspW::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend"]
    #[inline(always)]
    #[must_use]
    pub fn usbsusp(&mut self) -> UsbsuspW<GintstsSpec> {
        UsbsuspW::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<GintstsSpec> {
        UsbrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done"]
    #[inline(always)]
    #[must_use]
    pub fn enumdone(&mut self) -> EnumdoneW<GintstsSpec> {
        EnumdoneW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdrop(&mut self) -> IsooutdropW<GintstsSpec> {
        IsooutdropW::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eopf(&mut self) -> EopfW<GintstsSpec> {
        EopfW::new(self, 15)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer"]
    #[inline(always)]
    #[must_use]
    pub fn incompisoin(&mut self) -> IncompisoinW<GintstsSpec> {
        IncompisoinW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn incompip_incompisoout(&mut self) -> IncompipIncompisooutW<GintstsSpec> {
        IncompipIncompisooutW::new(self, 21)
    }
    #[doc = "Bit 28 - Connector ID status change"]
    #[inline(always)]
    #[must_use]
    pub fn conidschg(&mut self) -> ConidschgW<GintstsSpec> {
        ConidschgW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn disconint(&mut self) -> DisconintW<GintstsSpec> {
        DisconintW::new(self, 29)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wkupint(&mut self) -> WkupintW<GintstsSpec> {
        WkupintW::new(self, 31)
    }
}
#[doc = "OTGFS core interrupt register (OTGFS_GINTSTS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintstsSpec;
impl crate::RegisterSpec for GintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintsts::R`](R) reader structure"]
impl crate::Readable for GintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`gintsts::W`](W) writer structure"]
impl crate::Writable for GintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTSTS to value 0x0400_0020"]
impl crate::Resettable for GintstsSpec {
    const RESET_VALUE: u32 = 0x0400_0020;
}
