#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CSRST` reader - Core soft reset"]
pub type CsrstR = crate::BitReader;
#[doc = "Field `CSRST` writer - Core soft reset"]
pub type CsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUSB` reader - Disable usb phy"]
pub type DisusbR = crate::BitReader;
#[doc = "Field `DISUSB` writer - Disable usb phy"]
pub type DisusbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM` reader - Low power mode"]
pub type LpmR = crate::BitReader;
#[doc = "Field `LPM` writer - Low power mode"]
pub type LpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSP` reader - Soft suspend config"]
pub type SspR = crate::BitReader;
#[doc = "Field `SSP` writer - Soft suspend config"]
pub type SspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GRESUME` reader - Generate resume request"]
pub type GresumeR = crate::BitReader;
#[doc = "Field `GRESUME` writer - Generate resume request"]
pub type GresumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSOFIEN` reader - Lost start of frame interrupt enable"]
pub type LsofienR = crate::BitReader;
#[doc = "Field `LSOFIEN` writer - Lost start of frame interrupt enable"]
pub type LsofienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFIEN` reader - Start of frame interrupt enable"]
pub type SofienR = crate::BitReader;
#[doc = "Field `SOFIEN` writer - Start of frame interrupt enable"]
pub type SofienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTIEN` reader - Bus reset interrupt enable"]
pub type RstienR = crate::BitReader;
#[doc = "Field `RSTIEN` writer - Bus reset interrupt enable"]
pub type RstienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN` reader - Bus suspend mode interrupt enable"]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - Bus suspend mode interrupt enable"]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKIEN` reader - Wakeup/Remote wakeup interrupt enable"]
pub type WkienR = crate::BitReader;
#[doc = "Field `WKIEN` writer - Wakeup/Remote wakeup interrupt enable"]
pub type WkienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIEN` reader - Bus error interrupt enable"]
pub type BeienR = crate::BitReader;
#[doc = "Field `BEIEN` writer - Bus error interrupt enable"]
pub type BeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCFORIEN` reader - USB Core fifo overrun interrupt enable"]
pub type UcforienR = crate::BitReader;
#[doc = "Field `UCFORIEN` writer - USB Core fifo overrun interrupt enable"]
pub type UcforienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIEN` reader - transmission completed interrupt enable"]
pub type TcienR = crate::BitReader;
#[doc = "Field `TCIEN` writer - transmission completed interrupt enable"]
pub type TcienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csrst(&self) -> CsrstR {
        CsrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable usb phy"]
    #[inline(always)]
    pub fn disusb(&self) -> DisusbR {
        DisusbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low power mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LpmR {
        LpmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Soft suspend config"]
    #[inline(always)]
    pub fn ssp(&self) -> SspR {
        SspR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generate resume request"]
    #[inline(always)]
    pub fn gresume(&self) -> GresumeR {
        GresumeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost start of frame interrupt enable"]
    #[inline(always)]
    pub fn lsofien(&self) -> LsofienR {
        LsofienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    pub fn sofien(&self) -> SofienR {
        SofienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus reset interrupt enable"]
    #[inline(always)]
    pub fn rstien(&self) -> RstienR {
        RstienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus suspend mode interrupt enable"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup/Remote wakeup interrupt enable"]
    #[inline(always)]
    pub fn wkien(&self) -> WkienR {
        WkienR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus error interrupt enable"]
    #[inline(always)]
    pub fn beien(&self) -> BeienR {
        BeienR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB Core fifo overrun interrupt enable"]
    #[inline(always)]
    pub fn ucforien(&self) -> UcforienR {
        UcforienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transmission completed interrupt enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TcienR {
        TcienR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("csrst", &self.csrst())
            .field("disusb", &self.disusb())
            .field("lpm", &self.lpm())
            .field("ssp", &self.ssp())
            .field("gresume", &self.gresume())
            .field("lsofien", &self.lsofien())
            .field("sofien", &self.sofien())
            .field("rstien", &self.rstien())
            .field("spien", &self.spien())
            .field("wkien", &self.wkien())
            .field("beien", &self.beien())
            .field("ucforien", &self.ucforien())
            .field("tcien", &self.tcien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csrst(&mut self) -> CsrstW<CtrlSpec> {
        CsrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable usb phy"]
    #[inline(always)]
    #[must_use]
    pub fn disusb(&mut self) -> DisusbW<CtrlSpec> {
        DisusbW::new(self, 1)
    }
    #[doc = "Bit 2 - Low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LpmW<CtrlSpec> {
        LpmW::new(self, 2)
    }
    #[doc = "Bit 3 - Soft suspend config"]
    #[inline(always)]
    #[must_use]
    pub fn ssp(&mut self) -> SspW<CtrlSpec> {
        SspW::new(self, 3)
    }
    #[doc = "Bit 4 - Generate resume request"]
    #[inline(always)]
    #[must_use]
    pub fn gresume(&mut self) -> GresumeW<CtrlSpec> {
        GresumeW::new(self, 4)
    }
    #[doc = "Bit 8 - Lost start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn lsofien(&mut self) -> LsofienW<CtrlSpec> {
        LsofienW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn sofien(&mut self) -> SofienW<CtrlSpec> {
        SofienW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus reset interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rstien(&mut self) -> RstienW<CtrlSpec> {
        RstienW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus suspend mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SpienW<CtrlSpec> {
        SpienW::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup/Remote wakeup interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wkien(&mut self) -> WkienW<CtrlSpec> {
        WkienW::new(self, 12)
    }
    #[doc = "Bit 13 - Bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn beien(&mut self) -> BeienW<CtrlSpec> {
        BeienW::new(self, 13)
    }
    #[doc = "Bit 14 - USB Core fifo overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ucforien(&mut self) -> UcforienW<CtrlSpec> {
        UcforienW::new(self, 14)
    }
    #[doc = "Bit 15 - transmission completed interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TcienW<CtrlSpec> {
        TcienW::new(self, 15)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x03"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
