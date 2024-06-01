#[doc = "Register `DMABM` reader"]
pub type R = crate::R<DmabmSpec>;
#[doc = "Register `DMABM` writer"]
pub type W = crate::W<DmabmSpec>;
#[doc = "Field `SWR` reader - Software reset"]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software reset"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DaR = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Priority ratio"]
pub type PrR = crate::FieldReader;
#[doc = "Field `PR` writer - Priority ratio"]
pub type PrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed burst"]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RdpR = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RdpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type UspR = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type UspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLx8` reader - PNLx8 mode"]
pub type Pblx8R = crate::BitReader;
#[doc = "Field `PBLx8` writer - PNLx8 mode"]
pub type Pblx8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AabR = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AabW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> UspR {
        UspR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> Pblx8R {
        Pblx8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AabR {
        AabR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABM")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("pbl", &self.pbl())
            .field("pr", &self.pr())
            .field("fb", &self.fb())
            .field("rdp", &self.rdp())
            .field("usp", &self.usp())
            .field("pblx8", &self.pblx8())
            .field("aab", &self.aab())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SwrW<DmabmSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DaW<DmabmSpec> {
        DaW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DslW<DmabmSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PblW<DmabmSpec> {
        PblW::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<DmabmSpec> {
        PrW::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FbW<DmabmSpec> {
        FbW::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RdpW<DmabmSpec> {
        RdpW::new(self, 17)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> UspW<DmabmSpec> {
        UspW::new(self, 23)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    #[must_use]
    pub fn pblx8(&mut self) -> Pblx8W<DmabmSpec> {
        Pblx8W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    #[must_use]
    pub fn aab(&mut self) -> AabW<DmabmSpec> {
        AabW::new(self, 25)
    }
}
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmabm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmabm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmabmSpec;
impl crate::RegisterSpec for DmabmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabm::R`](R) reader structure"]
impl crate::Readable for DmabmSpec {}
#[doc = "`write(|w| ..)` method takes [`dmabm::W`](W) writer structure"]
impl crate::Writable for DmabmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABM to value 0x0002_0101"]
impl crate::Resettable for DmabmSpec {
    const RESET_VALUE: u32 = 0x0002_0101;
}
