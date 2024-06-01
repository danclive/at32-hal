#[doc = "Register `GINTMSK` reader"]
pub type R = crate::R<GintmskSpec>;
#[doc = "Register `GINTMSK` writer"]
pub type W = crate::W<GintmskSpec>;
#[doc = "Field `MODEMISMSK` reader - Mode mismatch interrupt mask"]
pub type ModemismskR = crate::BitReader;
#[doc = "Field `MODEMISMSK` writer - Mode mismatch interrupt mask"]
pub type ModemismskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGINTMSK` reader - OTG interrupt mask"]
pub type OtgintmskR = crate::BitReader;
#[doc = "Field `OTGINTMSK` writer - OTG interrupt mask"]
pub type OtgintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFMSK` reader - Start of frame mask"]
pub type SofmskR = crate::BitReader;
#[doc = "Field `SOFMSK` writer - Start of frame mask"]
pub type SofmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO non-empty mask"]
pub type RxflvlmskR = crate::BitReader;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO non-empty mask"]
pub type RxflvlmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NPTXFEMPMSK` reader - Non-periodic TxFIFO empty mask"]
pub type NptxfempmskR = crate::BitReader;
#[doc = "Field `NPTXFEMPMSK` writer - Non-periodic TxFIFO empty mask"]
pub type NptxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINNAKEFFMSK` reader - Global non-periodic IN NAK effective mask"]
pub type GinnakeffmskR = crate::BitReader;
#[doc = "Field `GINNAKEFFMSK` writer - Global non-periodic IN NAK effective mask"]
pub type GinnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK effective mask"]
pub type GoutnakeffmskR = crate::BitReader;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK effective mask"]
pub type GoutnakeffmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERLYSUSPMSK` reader - Early suspend mask"]
pub type ErlysuspmskR = crate::BitReader;
#[doc = "Field `ERLYSUSPMSK` writer - Early suspend mask"]
pub type ErlysuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBSUSPMSK` reader - USB suspend mask"]
pub type UsbsuspmskR = crate::BitReader;
#[doc = "Field `USBSUSPMSK` writer - USB suspend mask"]
pub type UsbsuspmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBRSTMSK` reader - USB reset mask"]
pub type UsbrstmskR = crate::BitReader;
#[doc = "Field `USBRSTMSK` writer - USB reset mask"]
pub type UsbrstmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration done mask"]
pub type EnumdonemskR = crate::BitReader;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration done mask"]
pub type EnumdonemskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT packet dropped interrupt mask"]
pub type IsooutdropmskR = crate::BitReader;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT packet dropped interrupt mask"]
pub type IsooutdropmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPFMSK` reader - End of periodic frame interrupt mask"]
pub type EopfmskR = crate::BitReader;
#[doc = "Field `EOPFMSK` writer - End of periodic frame interrupt mask"]
pub type EopfmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEPTINTMSK` reader - IN endpoints interrupt mask"]
pub type IeptintmskR = crate::BitReader;
#[doc = "Field `IEPTINTMSK` writer - IN endpoints interrupt mask"]
pub type IeptintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OEPTINTMSK` reader - OUT endpoints interrupt mask"]
pub type OeptintmskR = crate::BitReader;
#[doc = "Field `OEPTINTMSK` writer - OUT endpoints interrupt mask"]
pub type OeptintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMISOINMSK` reader - Incomplete isochronous IN transfer mask"]
pub type IncomisoinmskR = crate::BitReader;
#[doc = "Field `INCOMISOINMSK` writer - Incomplete isochronous IN transfer mask"]
pub type IncomisoinmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCOMPIP_INCOMPISOOUTMSK` reader - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IncompipIncompisooutmskR = crate::BitReader;
#[doc = "Field `INCOMPIP_INCOMPISOOUTMSK` writer - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
pub type IncompipIncompisooutmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTINTMSK` reader - Host port interrupt mask"]
pub type PrtintmskR = crate::BitReader;
#[doc = "Field `PRTINTMSK` writer - Host port interrupt mask"]
pub type PrtintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCHINTMSK` reader - Host channels interrupt mask"]
pub type HchintmskR = crate::BitReader;
#[doc = "Field `HCHINTMSK` writer - Host channels interrupt mask"]
pub type HchintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTXFEMPMSK` reader - Periodic TxFIFO empty mask"]
pub type PtxfempmskR = crate::BitReader;
#[doc = "Field `PTXFEMPMSK` writer - Periodic TxFIFO empty mask"]
pub type PtxfempmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONIDSCHGMSK` reader - Connector ID status change mask"]
pub type ConidschgmskR = crate::BitReader;
#[doc = "Field `CONIDSCHGMSK` writer - Connector ID status change mask"]
pub type ConidschgmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONINTMSK` reader - Disconnect detected interrupt mask"]
pub type DisconintmskR = crate::BitReader;
#[doc = "Field `DISCONINTMSK` writer - Disconnect detected interrupt mask"]
pub type DisconintmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPINTMSK` reader - Resume/remote wakeup detected interrupt mask"]
pub type WkupintmskR = crate::BitReader;
#[doc = "Field `WKUPINTMSK` writer - Resume/remote wakeup detected interrupt mask"]
pub type WkupintmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> ModemismskR {
        ModemismskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    pub fn otgintmsk(&self) -> OtgintmskR {
        OtgintmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SofmskR {
        SofmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RxflvlmskR {
        RxflvlmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn nptxfempmsk(&self) -> NptxfempmskR {
        NptxfempmskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GinnakeffmskR {
        GinnakeffmskR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GoutnakeffmskR {
        GoutnakeffmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ErlysuspmskR {
        ErlysuspmskR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> UsbsuspmskR {
        UsbsuspmskR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> UsbrstmskR {
        UsbrstmskR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> EnumdonemskR {
        EnumdonemskR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> IsooutdropmskR {
        IsooutdropmskR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EopfmskR {
        EopfmskR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    pub fn ieptintmsk(&self) -> IeptintmskR {
        IeptintmskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    pub fn oeptintmsk(&self) -> OeptintmskR {
        OeptintmskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    pub fn incomisoinmsk(&self) -> IncomisoinmskR {
        IncomisoinmskR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    pub fn incompip_incompisooutmsk(&self) -> IncompipIncompisooutmskR {
        IncompipIncompisooutmskR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    pub fn prtintmsk(&self) -> PrtintmskR {
        PrtintmskR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    pub fn hchintmsk(&self) -> HchintmskR {
        HchintmskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    pub fn ptxfempmsk(&self) -> PtxfempmskR {
        PtxfempmskR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    pub fn conidschgmsk(&self) -> ConidschgmskR {
        ConidschgmskR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    pub fn disconintmsk(&self) -> DisconintmskR {
        DisconintmskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WkupintmskR {
        WkupintmskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GINTMSK")
            .field("modemismsk", &self.modemismsk())
            .field("otgintmsk", &self.otgintmsk())
            .field("sofmsk", &self.sofmsk())
            .field("rxflvlmsk", &self.rxflvlmsk())
            .field("nptxfempmsk", &self.nptxfempmsk())
            .field("ginnakeffmsk", &self.ginnakeffmsk())
            .field("goutnakeffmsk", &self.goutnakeffmsk())
            .field("erlysuspmsk", &self.erlysuspmsk())
            .field("usbsuspmsk", &self.usbsuspmsk())
            .field("usbrstmsk", &self.usbrstmsk())
            .field("enumdonemsk", &self.enumdonemsk())
            .field("isooutdropmsk", &self.isooutdropmsk())
            .field("eopfmsk", &self.eopfmsk())
            .field("ieptintmsk", &self.ieptintmsk())
            .field("oeptintmsk", &self.oeptintmsk())
            .field("incomisoinmsk", &self.incomisoinmsk())
            .field("incompip_incompisooutmsk", &self.incompip_incompisooutmsk())
            .field("prtintmsk", &self.prtintmsk())
            .field("hchintmsk", &self.hchintmsk())
            .field("ptxfempmsk", &self.ptxfempmsk())
            .field("conidschgmsk", &self.conidschgmsk())
            .field("disconintmsk", &self.disconintmsk())
            .field("wkupintmsk", &self.wkupintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Mode mismatch interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn modemismsk(&mut self) -> ModemismskW<GintmskSpec> {
        ModemismskW::new(self, 1)
    }
    #[doc = "Bit 2 - OTG interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn otgintmsk(&mut self) -> OtgintmskW<GintmskSpec> {
        OtgintmskW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of frame mask"]
    #[inline(always)]
    #[must_use]
    pub fn sofmsk(&mut self) -> SofmskW<GintmskSpec> {
        SofmskW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO non-empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxflvlmsk(&mut self) -> RxflvlmskW<GintmskSpec> {
        RxflvlmskW::new(self, 4)
    }
    #[doc = "Bit 5 - Non-periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn nptxfempmsk(&mut self) -> NptxfempmskW<GintmskSpec> {
        NptxfempmskW::new(self, 5)
    }
    #[doc = "Bit 6 - Global non-periodic IN NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn ginnakeffmsk(&mut self) -> GinnakeffmskW<GintmskSpec> {
        GinnakeffmskW::new(self, 6)
    }
    #[doc = "Bit 7 - Global OUT NAK effective mask"]
    #[inline(always)]
    #[must_use]
    pub fn goutnakeffmsk(&mut self) -> GoutnakeffmskW<GintmskSpec> {
        GoutnakeffmskW::new(self, 7)
    }
    #[doc = "Bit 10 - Early suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn erlysuspmsk(&mut self) -> ErlysuspmskW<GintmskSpec> {
        ErlysuspmskW::new(self, 10)
    }
    #[doc = "Bit 11 - USB suspend mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbsuspmsk(&mut self) -> UsbsuspmskW<GintmskSpec> {
        UsbsuspmskW::new(self, 11)
    }
    #[doc = "Bit 12 - USB reset mask"]
    #[inline(always)]
    #[must_use]
    pub fn usbrstmsk(&mut self) -> UsbrstmskW<GintmskSpec> {
        UsbrstmskW::new(self, 12)
    }
    #[doc = "Bit 13 - Enumeration done mask"]
    #[inline(always)]
    #[must_use]
    pub fn enumdonemsk(&mut self) -> EnumdonemskW<GintmskSpec> {
        EnumdonemskW::new(self, 13)
    }
    #[doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn isooutdropmsk(&mut self) -> IsooutdropmskW<GintmskSpec> {
        IsooutdropmskW::new(self, 14)
    }
    #[doc = "Bit 15 - End of periodic frame interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn eopfmsk(&mut self) -> EopfmskW<GintmskSpec> {
        EopfmskW::new(self, 15)
    }
    #[doc = "Bit 18 - IN endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ieptintmsk(&mut self) -> IeptintmskW<GintmskSpec> {
        IeptintmskW::new(self, 18)
    }
    #[doc = "Bit 19 - OUT endpoints interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn oeptintmsk(&mut self) -> OeptintmskW<GintmskSpec> {
        OeptintmskW::new(self, 19)
    }
    #[doc = "Bit 20 - Incomplete isochronous IN transfer mask"]
    #[inline(always)]
    #[must_use]
    pub fn incomisoinmsk(&mut self) -> IncomisoinmskW<GintmskSpec> {
        IncomisoinmskW::new(self, 20)
    }
    #[doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)"]
    #[inline(always)]
    #[must_use]
    pub fn incompip_incompisooutmsk(&mut self) -> IncompipIncompisooutmskW<GintmskSpec> {
        IncompipIncompisooutmskW::new(self, 21)
    }
    #[doc = "Bit 24 - Host port interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn prtintmsk(&mut self) -> PrtintmskW<GintmskSpec> {
        PrtintmskW::new(self, 24)
    }
    #[doc = "Bit 25 - Host channels interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn hchintmsk(&mut self) -> HchintmskW<GintmskSpec> {
        HchintmskW::new(self, 25)
    }
    #[doc = "Bit 26 - Periodic TxFIFO empty mask"]
    #[inline(always)]
    #[must_use]
    pub fn ptxfempmsk(&mut self) -> PtxfempmskW<GintmskSpec> {
        PtxfempmskW::new(self, 26)
    }
    #[doc = "Bit 28 - Connector ID status change mask"]
    #[inline(always)]
    #[must_use]
    pub fn conidschgmsk(&mut self) -> ConidschgmskW<GintmskSpec> {
        ConidschgmskW::new(self, 28)
    }
    #[doc = "Bit 29 - Disconnect detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn disconintmsk(&mut self) -> DisconintmskW<GintmskSpec> {
        DisconintmskW::new(self, 29)
    }
    #[doc = "Bit 31 - Resume/remote wakeup detected interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn wkupintmsk(&mut self) -> WkupintmskW<GintmskSpec> {
        WkupintmskW::new(self, 31)
    }
}
#[doc = "OTG_FS interrupt mask register (OTG_FS_GINTMSK)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gintmsk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gintmsk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GintmskSpec;
impl crate::RegisterSpec for GintmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gintmsk::R`](R) reader structure"]
impl crate::Readable for GintmskSpec {}
#[doc = "`write(|w| ..)` method takes [`gintmsk::W`](W) writer structure"]
impl crate::Writable for GintmskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GintmskSpec {
    const RESET_VALUE: u32 = 0;
}
