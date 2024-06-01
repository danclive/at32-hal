#[doc = "Register `DMASTS` reader"]
pub type R = crate::R<DmastsSpec>;
#[doc = "Register `DMASTS` writer"]
pub type W = crate::W<DmastsSpec>;
#[doc = "Field `TI` reader - Transmit interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit interrupt"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit process stopped"]
pub type TpsR = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit process stopped"]
pub type TpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - Transmit buffer unavailable"]
pub type TbuR = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit buffer unavailable"]
pub type TbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJT` reader - Transmit jabber timeout"]
pub type TjtR = crate::BitReader;
#[doc = "Field `TJT` writer - Transmit jabber timeout"]
pub type TjtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - Receive overflow"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Receive overflow"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNF` reader - Transmit underflow"]
pub type UnfR = crate::BitReader;
#[doc = "Field `UNF` writer - Transmit underflow"]
pub type UnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive interrupt"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBU` reader - Receive buffer unavailable"]
pub type RbuR = crate::BitReader;
#[doc = "Field `RBU` writer - Receive buffer unavailable"]
pub type RbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive process stopped"]
pub type RpsR = crate::BitReader;
#[doc = "Field `RPS` writer - Receive process stopped"]
pub type RpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive watchdog timeout"]
pub type RwtR = crate::BitReader;
#[doc = "Field `RWT` writer - Receive watchdog timeout"]
pub type RwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETI` reader - Early transmit interrupt"]
pub type EtiR = crate::BitReader;
#[doc = "Field `ETI` writer - Early transmit interrupt"]
pub type EtiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEI` reader - Fatal bus error interrupt"]
pub type FbeiR = crate::BitReader;
#[doc = "Field `FBEI` writer - Fatal bus error interrupt"]
pub type FbeiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERI` reader - Early receive interrupt"]
pub type EriR = crate::BitReader;
#[doc = "Field `ERI` writer - Early receive interrupt"]
pub type EriW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal interrupt summary"]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal interrupt summary"]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal interrupt summary"]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal interrupt summary"]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Receive process state"]
pub type RsR = crate::FieldReader;
#[doc = "Field `TS` reader - Transmit process state"]
pub type TsR = crate::FieldReader;
#[doc = "Field `EB` reader - Error bits"]
pub type EbR = crate::FieldReader;
#[doc = "Field `MMI` reader - MAC MMC interrupt"]
pub type MmiR = crate::BitReader;
#[doc = "Field `MPI` reader - MAC PMT interrupt"]
pub type MpiR = crate::BitReader;
#[doc = "Field `TTI` reader - Timestamp trigger interrupt"]
pub type TtiR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TbuR {
        TbuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    pub fn unf(&self) -> UnfR {
        UnfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RbuR {
        RbuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    pub fn eti(&self) -> EtiR {
        EtiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    pub fn fbei(&self) -> FbeiR {
        FbeiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    pub fn eri(&self) -> EriR {
        EriR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Receive process state"]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Transmit process state"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - Error bits"]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 27 - MAC MMC interrupt"]
    #[inline(always)]
    pub fn mmi(&self) -> MmiR {
        MmiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MAC PMT interrupt"]
    #[inline(always)]
    pub fn mpi(&self) -> MpiR {
        MpiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Timestamp trigger interrupt"]
    #[inline(always)]
    pub fn tti(&self) -> TtiR {
        TtiR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMASTS")
            .field("ti", &self.ti())
            .field("tps", &self.tps())
            .field("tbu", &self.tbu())
            .field("tjt", &self.tjt())
            .field("ovf", &self.ovf())
            .field("unf", &self.unf())
            .field("ri", &self.ri())
            .field("rbu", &self.rbu())
            .field("rps", &self.rps())
            .field("rwt", &self.rwt())
            .field("eti", &self.eti())
            .field("fbei", &self.fbei())
            .field("eri", &self.eri())
            .field("ais", &self.ais())
            .field("nis", &self.nis())
            .field("rs", &self.rs())
            .field("ts", &self.ts())
            .field("eb", &self.eb())
            .field("mmi", &self.mmi())
            .field("mpi", &self.mpi())
            .field("tti", &self.tti())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<DmastsSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit process stopped"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TpsW<DmastsSpec> {
        TpsW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TbuW<DmastsSpec> {
        TbuW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout"]
    #[inline(always)]
    #[must_use]
    pub fn tjt(&mut self) -> TjtW<DmastsSpec> {
        TjtW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OvfW<DmastsSpec> {
        OvfW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit underflow"]
    #[inline(always)]
    #[must_use]
    pub fn unf(&mut self) -> UnfW<DmastsSpec> {
        UnfW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ri(&mut self) -> RiW<DmastsSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable"]
    #[inline(always)]
    #[must_use]
    pub fn rbu(&mut self) -> RbuW<DmastsSpec> {
        RbuW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive process stopped"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RpsW<DmastsSpec> {
        RpsW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive watchdog timeout"]
    #[inline(always)]
    #[must_use]
    pub fn rwt(&mut self) -> RwtW<DmastsSpec> {
        RwtW::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eti(&mut self) -> EtiW<DmastsSpec> {
        EtiW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn fbei(&mut self) -> FbeiW<DmastsSpec> {
        FbeiW::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn eri(&mut self) -> EriW<DmastsSpec> {
        EriW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn ais(&mut self) -> AisW<DmastsSpec> {
        AisW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt summary"]
    #[inline(always)]
    #[must_use]
    pub fn nis(&mut self) -> NisW<DmastsSpec> {
        NisW::new(self, 16)
    }
}
#[doc = "Ethernet DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmasts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmastsSpec;
impl crate::RegisterSpec for DmastsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmasts::R`](R) reader structure"]
impl crate::Readable for DmastsSpec {}
#[doc = "`write(|w| ..)` method takes [`dmasts::W`](W) writer structure"]
impl crate::Writable for DmastsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMASTS to value 0"]
impl crate::Resettable for DmastsSpec {
    const RESET_VALUE: u32 = 0;
}
