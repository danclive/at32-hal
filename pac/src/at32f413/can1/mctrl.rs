#[doc = "Register `MCTRL` reader"]
pub type R = crate::R<MctrlSpec>;
#[doc = "Register `MCTRL` writer"]
pub type W = crate::W<MctrlSpec>;
#[doc = "Field `FZEN` reader - Freeze mode enable"]
pub type FzenR = crate::BitReader;
#[doc = "Field `FZEN` writer - Freeze mode enable"]
pub type FzenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DZEN` reader - Doze mode enable"]
pub type DzenR = crate::BitReader;
#[doc = "Field `DZEN` writer - Doze mode enable"]
pub type DzenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMSSR` reader - Multiple message sending sequence rule"]
pub type MmssrR = crate::BitReader;
#[doc = "Field `MMSSR` writer - Multiple message sending sequence rule"]
pub type MmssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDRSEL` reader - Message discarding rule select when overflow"]
pub type MdrselR = crate::BitReader;
#[doc = "Field `MDRSEL` writer - Message discarding rule select when overflow"]
pub type MdrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRSFEN` reader - Prohibit retransmission when sending fails enable"]
pub type PrsfenR = crate::BitReader;
#[doc = "Field `PRSFEN` writer - Prohibit retransmission when sending fails enable"]
pub type PrsfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEDEN` reader - Automatic exit doze mode enable"]
pub type AedenR = crate::BitReader;
#[doc = "Field `AEDEN` writer - Automatic exit doze mode enable"]
pub type AedenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AEBOEN` reader - Automatic exit bus-off enable"]
pub type AeboenR = crate::BitReader;
#[doc = "Field `AEBOEN` writer - Automatic exit bus-off enable"]
pub type AeboenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTCEN` reader - Time triggered communication mode enable"]
pub type TtcenR = crate::BitReader;
#[doc = "Field `TTCEN` writer - Time triggered communication mode enable"]
pub type TtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPRST` reader - Software partial reset"]
pub type SprstR = crate::BitReader;
#[doc = "Field `SPRST` writer - Software partial reset"]
pub type SprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTD` reader - Prohibit transmission when debug"]
pub type PtdR = crate::BitReader;
#[doc = "Field `PTD` writer - Prohibit transmission when debug"]
pub type PtdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    pub fn fzen(&self) -> FzenR {
        FzenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    pub fn dzen(&self) -> DzenR {
        DzenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    pub fn mmssr(&self) -> MmssrR {
        MmssrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    pub fn mdrsel(&self) -> MdrselR {
        MdrselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    pub fn prsfen(&self) -> PrsfenR {
        PrsfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    pub fn aeden(&self) -> AedenR {
        AedenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    pub fn aeboen(&self) -> AeboenR {
        AeboenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    pub fn ttcen(&self) -> TtcenR {
        TtcenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    pub fn sprst(&self) -> SprstR {
        SprstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    pub fn ptd(&self) -> PtdR {
        PtdR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MCTRL")
            .field("ptd", &self.ptd())
            .field("sprst", &self.sprst())
            .field("ttcen", &self.ttcen())
            .field("aeboen", &self.aeboen())
            .field("aeden", &self.aeden())
            .field("prsfen", &self.prsfen())
            .field("mdrsel", &self.mdrsel())
            .field("mmssr", &self.mmssr())
            .field("dzen", &self.dzen())
            .field("fzen", &self.fzen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Freeze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fzen(&mut self) -> FzenW<MctrlSpec> {
        FzenW::new(self, 0)
    }
    #[doc = "Bit 1 - Doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dzen(&mut self) -> DzenW<MctrlSpec> {
        DzenW::new(self, 1)
    }
    #[doc = "Bit 2 - Multiple message sending sequence rule"]
    #[inline(always)]
    #[must_use]
    pub fn mmssr(&mut self) -> MmssrW<MctrlSpec> {
        MmssrW::new(self, 2)
    }
    #[doc = "Bit 3 - Message discarding rule select when overflow"]
    #[inline(always)]
    #[must_use]
    pub fn mdrsel(&mut self) -> MdrselW<MctrlSpec> {
        MdrselW::new(self, 3)
    }
    #[doc = "Bit 4 - Prohibit retransmission when sending fails enable"]
    #[inline(always)]
    #[must_use]
    pub fn prsfen(&mut self) -> PrsfenW<MctrlSpec> {
        PrsfenW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic exit doze mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeden(&mut self) -> AedenW<MctrlSpec> {
        AedenW::new(self, 5)
    }
    #[doc = "Bit 6 - Automatic exit bus-off enable"]
    #[inline(always)]
    #[must_use]
    pub fn aeboen(&mut self) -> AeboenW<MctrlSpec> {
        AeboenW::new(self, 6)
    }
    #[doc = "Bit 7 - Time triggered communication mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttcen(&mut self) -> TtcenW<MctrlSpec> {
        TtcenW::new(self, 7)
    }
    #[doc = "Bit 15 - Software partial reset"]
    #[inline(always)]
    #[must_use]
    pub fn sprst(&mut self) -> SprstW<MctrlSpec> {
        SprstW::new(self, 15)
    }
    #[doc = "Bit 16 - Prohibit transmission when debug"]
    #[inline(always)]
    #[must_use]
    pub fn ptd(&mut self) -> PtdW<MctrlSpec> {
        PtdW::new(self, 16)
    }
}
#[doc = "Main control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MctrlSpec;
impl crate::RegisterSpec for MctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mctrl::R`](R) reader structure"]
impl crate::Readable for MctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mctrl::W`](W) writer structure"]
impl crate::Writable for MctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCTRL to value 0x0001_0002"]
impl crate::Resettable for MctrlSpec {
    const RESET_VALUE: u32 = 0x0001_0002;
}
