#[doc = "Register `DMAOPM` reader"]
pub type R = crate::R<DmaopmSpec>;
#[doc = "Register `DMAOPM` writer"]
pub type W = crate::W<DmaopmSpec>;
#[doc = "Field `SSR` reader - Start or stop receive"]
pub type SsrR = crate::BitReader;
#[doc = "Field `SSR` writer - Start or stop receive"]
pub type SsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSF` reader - Operate on second frame"]
pub type OsfR = crate::BitReader;
#[doc = "Field `OSF` writer - Operate on second frame"]
pub type OsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC` reader - Receive threshold control"]
pub type RtcR = crate::FieldReader;
#[doc = "Field `RTC` writer - Receive threshold control"]
pub type RtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUGF` reader - Forward undersized good frames"]
pub type FugfR = crate::BitReader;
#[doc = "Field `FUGF` writer - Forward undersized good frames"]
pub type FugfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEF` reader - Forward error frames"]
pub type FefR = crate::BitReader;
#[doc = "Field `FEF` writer - Forward error frames"]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSTC` reader - Start of stop transmission command"]
pub type SstcR = crate::BitReader;
#[doc = "Field `SSTC` writer - Start of stop transmission command"]
pub type SstcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTC` reader - Transmit threshold control"]
pub type TtcR = crate::FieldReader;
#[doc = "Field `TTC` writer - Transmit threshold control"]
pub type TtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FTF` reader - Flush transmit FIFO"]
pub type FtfR = crate::BitReader;
#[doc = "Field `FTF` writer - Flush transmit FIFO"]
pub type FtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF` reader - Transmit store and forward"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Transmit store and forward"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFRF` reader - Disable flushing of received frames"]
pub type DfrfR = crate::BitReader;
#[doc = "Field `DFRF` writer - Disable flushing of received frames"]
pub type DfrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSF` reader - Receive store and forward"]
pub type RsfR = crate::BitReader;
#[doc = "Field `RSF` writer - Receive store and forward"]
pub type RsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Disable dropping of TCP/IP checksum error frames"]
pub type DtR = crate::BitReader;
#[doc = "Field `DT` writer - Disable dropping of TCP/IP checksum error frames"]
pub type DtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    pub fn ssr(&self) -> SsrR {
        SsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    pub fn osf(&self) -> OsfR {
        OsfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    pub fn fugf(&self) -> FugfR {
        FugfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    pub fn sstc(&self) -> SstcR {
        SstcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    pub fn ttc(&self) -> TtcR {
        TtcR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    pub fn dfrf(&self) -> DfrfR {
        DfrfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    pub fn rsf(&self) -> RsfR {
        RsfR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAOPM")
            .field("ssr", &self.ssr())
            .field("osf", &self.osf())
            .field("rtc", &self.rtc())
            .field("fugf", &self.fugf())
            .field("fef", &self.fef())
            .field("sstc", &self.sstc())
            .field("ttc", &self.ttc())
            .field("ftf", &self.ftf())
            .field("tsf", &self.tsf())
            .field("dfrf", &self.dfrf())
            .field("rsf", &self.rsf())
            .field("dt", &self.dt())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Start or stop receive"]
    #[inline(always)]
    #[must_use]
    pub fn ssr(&mut self) -> SsrW<DmaopmSpec> {
        SsrW::new(self, 1)
    }
    #[doc = "Bit 2 - Operate on second frame"]
    #[inline(always)]
    #[must_use]
    pub fn osf(&mut self) -> OsfW<DmaopmSpec> {
        OsfW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Receive threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<DmaopmSpec> {
        RtcW::new(self, 3)
    }
    #[doc = "Bit 6 - Forward undersized good frames"]
    #[inline(always)]
    #[must_use]
    pub fn fugf(&mut self) -> FugfW<DmaopmSpec> {
        FugfW::new(self, 6)
    }
    #[doc = "Bit 7 - Forward error frames"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FefW<DmaopmSpec> {
        FefW::new(self, 7)
    }
    #[doc = "Bit 13 - Start of stop transmission command"]
    #[inline(always)]
    #[must_use]
    pub fn sstc(&mut self) -> SstcW<DmaopmSpec> {
        SstcW::new(self, 13)
    }
    #[doc = "Bits 14:16 - Transmit threshold control"]
    #[inline(always)]
    #[must_use]
    pub fn ttc(&mut self) -> TtcW<DmaopmSpec> {
        TtcW::new(self, 14)
    }
    #[doc = "Bit 20 - Flush transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FtfW<DmaopmSpec> {
        FtfW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<DmaopmSpec> {
        TsfW::new(self, 21)
    }
    #[doc = "Bit 24 - Disable flushing of received frames"]
    #[inline(always)]
    #[must_use]
    pub fn dfrf(&mut self) -> DfrfW<DmaopmSpec> {
        DfrfW::new(self, 24)
    }
    #[doc = "Bit 25 - Receive store and forward"]
    #[inline(always)]
    #[must_use]
    pub fn rsf(&mut self) -> RsfW<DmaopmSpec> {
        RsfW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable dropping of TCP/IP checksum error frames"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<DmaopmSpec> {
        DtW::new(self, 26)
    }
}
#[doc = "Ethernet DMA operation mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaopm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaopm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaopmSpec;
impl crate::RegisterSpec for DmaopmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaopm::R`](R) reader structure"]
impl crate::Readable for DmaopmSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaopm::W`](W) writer structure"]
impl crate::Writable for DmaopmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAOPM to value 0"]
impl crate::Resettable for DmaopmSpec {
    const RESET_VALUE: u32 = 0;
}
