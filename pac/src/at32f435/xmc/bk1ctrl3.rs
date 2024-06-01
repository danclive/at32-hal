#[doc = "Register `BK1CTRL3` reader"]
pub type R = crate::R<Bk1ctrl3Spec>;
#[doc = "Register `BK1CTRL3` writer"]
pub type W = crate::W<Bk1ctrl3Spec>;
#[doc = "Field `EN` reader - Memory bank enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Memory bank enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMUXEN` reader - Address and data multiplexing enable"]
pub type AdmuxenR = crate::BitReader;
#[doc = "Field `ADMUXEN` writer - Address and data multiplexing enable"]
pub type AdmuxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV` reader - Memory device type"]
pub type DevR = crate::FieldReader;
#[doc = "Field `DEV` writer - Memory device type"]
pub type DevW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EXTMDBW` reader - External memory data bus width"]
pub type ExtmdbwR = crate::FieldReader;
#[doc = "Field `EXTMDBW` writer - External memory data bus width"]
pub type ExtmdbwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NOREN` reader - Nor flash access enable"]
pub type NorenR = crate::BitReader;
#[doc = "Field `NOREN` writer - Nor flash access enable"]
pub type NorenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCBEN` reader - Synchronous burst enable"]
pub type SyncbenR = crate::BitReader;
#[doc = "Field `SYNCBEN` writer - Synchronous burst enable"]
pub type SyncbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWPOL` reader - NWAIT polarity"]
pub type NwpolR = crate::BitReader;
#[doc = "Field `NWPOL` writer - NWAIT polarity"]
pub type NwpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRAPEN` reader - Wrapped enable"]
pub type WrapenR = crate::BitReader;
#[doc = "Field `WRAPEN` writer - Wrapped enable"]
pub type WrapenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWTCFG` reader - Wait timing configuration"]
pub type NwtcfgR = crate::BitReader;
#[doc = "Field `NWTCFG` writer - Wait timing configuration"]
pub type NwtcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEN` reader - Write enable"]
pub type WenR = crate::BitReader;
#[doc = "Field `WEN` writer - Write enable"]
pub type WenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWSEN` reader - NWAIT in synchronous transfer enable"]
pub type NwsenR = crate::BitReader;
#[doc = "Field `NWSEN` writer - NWAIT in synchronous transfer enable"]
pub type NwsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTD` reader - Read-write timing different"]
pub type RwtdR = crate::BitReader;
#[doc = "Field `RWTD` writer - Read-write timing different"]
pub type RwtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NWASEN` reader - NWAIT in asynchronous transfer enable"]
pub type NwasenR = crate::BitReader;
#[doc = "Field `NWASEN` writer - NWAIT in asynchronous transfer enable"]
pub type NwasenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRPGS` reader - CRAM page size"]
pub type CrpgsR = crate::FieldReader;
#[doc = "Field `CRPGS` writer - CRAM page size"]
pub type CrpgsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MWMC` reader - Memory write mode control"]
pub type MwmcR = crate::BitReader;
#[doc = "Field `MWMC` writer - Memory write mode control"]
pub type MwmcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Memory bank enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address and data multiplexing enable"]
    #[inline(always)]
    pub fn admuxen(&self) -> AdmuxenR {
        AdmuxenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Memory device type"]
    #[inline(always)]
    pub fn dev(&self) -> DevR {
        DevR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    pub fn extmdbw(&self) -> ExtmdbwR {
        ExtmdbwR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Nor flash access enable"]
    #[inline(always)]
    pub fn noren(&self) -> NorenR {
        NorenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    pub fn syncben(&self) -> SyncbenR {
        SyncbenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NWAIT polarity"]
    #[inline(always)]
    pub fn nwpol(&self) -> NwpolR {
        NwpolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wrapped enable"]
    #[inline(always)]
    pub fn wrapen(&self) -> WrapenR {
        WrapenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    pub fn nwtcfg(&self) -> NwtcfgR {
        NwtcfgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    pub fn wen(&self) -> WenR {
        WenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NWAIT in synchronous transfer enable"]
    #[inline(always)]
    pub fn nwsen(&self) -> NwsenR {
        NwsenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Read-write timing different"]
    #[inline(always)]
    pub fn rwtd(&self) -> RwtdR {
        RwtdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NWAIT in asynchronous transfer enable"]
    #[inline(always)]
    pub fn nwasen(&self) -> NwasenR {
        NwasenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    pub fn crpgs(&self) -> CrpgsR {
        CrpgsR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Memory write mode control"]
    #[inline(always)]
    pub fn mwmc(&self) -> MwmcR {
        MwmcR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK1CTRL3")
            .field("mwmc", &self.mwmc())
            .field("crpgs", &self.crpgs())
            .field("nwasen", &self.nwasen())
            .field("rwtd", &self.rwtd())
            .field("nwsen", &self.nwsen())
            .field("wen", &self.wen())
            .field("nwtcfg", &self.nwtcfg())
            .field("wrapen", &self.wrapen())
            .field("nwpol", &self.nwpol())
            .field("syncben", &self.syncben())
            .field("noren", &self.noren())
            .field("extmdbw", &self.extmdbw())
            .field("dev", &self.dev())
            .field("admuxen", &self.admuxen())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Memory bank enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Bk1ctrl3Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Address and data multiplexing enable"]
    #[inline(always)]
    #[must_use]
    pub fn admuxen(&mut self) -> AdmuxenW<Bk1ctrl3Spec> {
        AdmuxenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Memory device type"]
    #[inline(always)]
    #[must_use]
    pub fn dev(&mut self) -> DevW<Bk1ctrl3Spec> {
        DevW::new(self, 2)
    }
    #[doc = "Bits 4:5 - External memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn extmdbw(&mut self) -> ExtmdbwW<Bk1ctrl3Spec> {
        ExtmdbwW::new(self, 4)
    }
    #[doc = "Bit 6 - Nor flash access enable"]
    #[inline(always)]
    #[must_use]
    pub fn noren(&mut self) -> NorenW<Bk1ctrl3Spec> {
        NorenW::new(self, 6)
    }
    #[doc = "Bit 8 - Synchronous burst enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncben(&mut self) -> SyncbenW<Bk1ctrl3Spec> {
        SyncbenW::new(self, 8)
    }
    #[doc = "Bit 9 - NWAIT polarity"]
    #[inline(always)]
    #[must_use]
    pub fn nwpol(&mut self) -> NwpolW<Bk1ctrl3Spec> {
        NwpolW::new(self, 9)
    }
    #[doc = "Bit 10 - Wrapped enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrapen(&mut self) -> WrapenW<Bk1ctrl3Spec> {
        WrapenW::new(self, 10)
    }
    #[doc = "Bit 11 - Wait timing configuration"]
    #[inline(always)]
    #[must_use]
    pub fn nwtcfg(&mut self) -> NwtcfgW<Bk1ctrl3Spec> {
        NwtcfgW::new(self, 11)
    }
    #[doc = "Bit 12 - Write enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WenW<Bk1ctrl3Spec> {
        WenW::new(self, 12)
    }
    #[doc = "Bit 13 - NWAIT in synchronous transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwsen(&mut self) -> NwsenW<Bk1ctrl3Spec> {
        NwsenW::new(self, 13)
    }
    #[doc = "Bit 14 - Read-write timing different"]
    #[inline(always)]
    #[must_use]
    pub fn rwtd(&mut self) -> RwtdW<Bk1ctrl3Spec> {
        RwtdW::new(self, 14)
    }
    #[doc = "Bit 15 - NWAIT in asynchronous transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn nwasen(&mut self) -> NwasenW<Bk1ctrl3Spec> {
        NwasenW::new(self, 15)
    }
    #[doc = "Bits 16:18 - CRAM page size"]
    #[inline(always)]
    #[must_use]
    pub fn crpgs(&mut self) -> CrpgsW<Bk1ctrl3Spec> {
        CrpgsW::new(self, 16)
    }
    #[doc = "Bit 19 - Memory write mode control"]
    #[inline(always)]
    #[must_use]
    pub fn mwmc(&mut self) -> MwmcW<Bk1ctrl3Spec> {
        MwmcW::new(self, 19)
    }
}
#[doc = "SRAM/NOR-Flash chip-select control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1ctrl3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1ctrl3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk1ctrl3Spec;
impl crate::RegisterSpec for Bk1ctrl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk1ctrl3::R`](R) reader structure"]
impl crate::Readable for Bk1ctrl3Spec {}
#[doc = "`write(|w| ..)` method takes [`bk1ctrl3::W`](W) writer structure"]
impl crate::Writable for Bk1ctrl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK1CTRL3 to value 0x30d2"]
impl crate::Resettable for Bk1ctrl3Spec {
    const RESET_VALUE: u32 = 0x30d2;
}
