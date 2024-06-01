#[doc = "Register `MACFRMF` reader"]
pub type R = crate::R<MacfrmfSpec>;
#[doc = "Register `MACFRMF` writer"]
pub type W = crate::W<MacfrmfSpec>;
#[doc = "Field `PR` reader - Promiscuous mode"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Promiscuous mode"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - Hash unicast"]
pub type HucR = crate::BitReader;
#[doc = "Field `HUC` writer - Hash unicast"]
pub type HucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - Hash multicast"]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - Hash multicast"]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - Destination address inverse filtering"]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - Destination address inverse filtering"]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMC` reader - Pass multicast"]
pub type PmcR = crate::BitReader;
#[doc = "Field `PMC` writer - Pass multicast"]
pub type PmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - Disable broadcast frames"]
pub type DbfR = crate::BitReader;
#[doc = "Field `DBF` writer - Disable broadcast frames"]
pub type DbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - Pass control frames"]
pub type PcfR = crate::FieldReader;
#[doc = "Field `PCF` writer - Pass control frames"]
pub type PcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - Source address inverse filtering"]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - Source address inverse filtering"]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - Source address filter"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - Source address filter"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - Hash or perfect filter"]
pub type HpfR = crate::BitReader;
#[doc = "Field `HPF` writer - Hash or perfect filter"]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - Receive all"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - Receive all"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    pub fn huc(&self) -> HucR {
        HucR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    pub fn pmc(&self) -> PmcR {
        PmcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACFRMF")
            .field("pr", &self.pr())
            .field("huc", &self.huc())
            .field("hmc", &self.hmc())
            .field("daif", &self.daif())
            .field("pmc", &self.pmc())
            .field("dbf", &self.dbf())
            .field("pcf", &self.pcf())
            .field("saif", &self.saif())
            .field("saf", &self.saf())
            .field("hpf", &self.hpf())
            .field("ra", &self.ra())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Promiscuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<MacfrmfSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 1 - Hash unicast"]
    #[inline(always)]
    #[must_use]
    pub fn huc(&mut self) -> HucW<MacfrmfSpec> {
        HucW::new(self, 1)
    }
    #[doc = "Bit 2 - Hash multicast"]
    #[inline(always)]
    #[must_use]
    pub fn hmc(&mut self) -> HmcW<MacfrmfSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 3 - Destination address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn daif(&mut self) -> DaifW<MacfrmfSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - Pass multicast"]
    #[inline(always)]
    #[must_use]
    pub fn pmc(&mut self) -> PmcW<MacfrmfSpec> {
        PmcW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable broadcast frames"]
    #[inline(always)]
    #[must_use]
    pub fn dbf(&mut self) -> DbfW<MacfrmfSpec> {
        DbfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Pass control frames"]
    #[inline(always)]
    #[must_use]
    pub fn pcf(&mut self) -> PcfW<MacfrmfSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - Source address inverse filtering"]
    #[inline(always)]
    #[must_use]
    pub fn saif(&mut self) -> SaifW<MacfrmfSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - Source address filter"]
    #[inline(always)]
    #[must_use]
    pub fn saf(&mut self) -> SafW<MacfrmfSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - Hash or perfect filter"]
    #[inline(always)]
    #[must_use]
    pub fn hpf(&mut self) -> HpfW<MacfrmfSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 31 - Receive all"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<MacfrmfSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "Ethernet MAC frame filter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macfrmf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macfrmf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacfrmfSpec;
impl crate::RegisterSpec for MacfrmfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macfrmf::R`](R) reader structure"]
impl crate::Readable for MacfrmfSpec {}
#[doc = "`write(|w| ..)` method takes [`macfrmf::W`](W) writer structure"]
impl crate::Writable for MacfrmfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACFRMF to value 0"]
impl crate::Resettable for MacfrmfSpec {
    const RESET_VALUE: u32 = 0;
}
