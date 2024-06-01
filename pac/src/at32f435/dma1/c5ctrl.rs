#[doc = "Register `C5CTRL` reader"]
pub type R = crate::R<C5ctrlSpec>;
#[doc = "Register `C5CTRL` writer"]
pub type W = crate::W<C5ctrlSpec>;
#[doc = "Field `CHEN` reader - Channel enable"]
pub type ChenR = crate::BitReader;
#[doc = "Field `CHEN` writer - Channel enable"]
pub type ChenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTIEN` reader - Transfer complete interrupt enable"]
pub type FdtienR = crate::BitReader;
#[doc = "Field `FDTIEN` writer - Transfer complete interrupt enable"]
pub type FdtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTIEN` reader - Half transfer interrupt enable"]
pub type HdtienR = crate::BitReader;
#[doc = "Field `HDTIEN` writer - Half transfer interrupt enable"]
pub type HdtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DterrienR = crate::BitReader;
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DterrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DtdR = crate::BitReader;
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - Loop mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loop mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PincmR = crate::BitReader;
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PincmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MincmR = crate::BitReader;
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MincmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Peripheral data bit width"]
pub type PwidthR = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Peripheral data bit width"]
pub type PwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Memory data bit width"]
pub type MwidthR = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Memory data bit width"]
pub type MwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHPL` reader - Channel Priority level"]
pub type ChplR = crate::FieldReader;
#[doc = "Field `CHPL` writer - Channel Priority level"]
pub type ChplW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `M2M` reader - Memory to memory mode"]
pub type M2mR = crate::BitReader;
#[doc = "Field `M2M` writer - Memory to memory mode"]
pub type M2mW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FdtienR {
        FdtienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HdtienR {
        HdtienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DterrienR {
        DterrienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DtdR {
        DtdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PincmR {
        PincmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MincmR {
        MincmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PwidthR {
        PwidthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MwidthR {
        MwidthR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn chpl(&self) -> ChplR {
        ChplR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn m2m(&self) -> M2mR {
        M2mR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5CTRL")
            .field("chen", &self.chen())
            .field("fdtien", &self.fdtien())
            .field("hdtien", &self.hdtien())
            .field("dterrien", &self.dterrien())
            .field("dtd", &self.dtd())
            .field("lm", &self.lm())
            .field("pincm", &self.pincm())
            .field("mincm", &self.mincm())
            .field("pwidth", &self.pwidth())
            .field("mwidth", &self.mwidth())
            .field("chpl", &self.chpl())
            .field("m2m", &self.m2m())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn chen(&mut self) -> ChenW<C5ctrlSpec> {
        ChenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FdtienW<C5ctrlSpec> {
        FdtienW::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HdtienW<C5ctrlSpec> {
        HdtienW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DterrienW<C5ctrlSpec> {
        DterrienW::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DtdW<C5ctrlSpec> {
        DtdW::new(self, 4)
    }
    #[doc = "Bit 5 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<C5ctrlSpec> {
        LmW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PincmW<C5ctrlSpec> {
        PincmW::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MincmW<C5ctrlSpec> {
        MincmW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PwidthW<C5ctrlSpec> {
        PwidthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory data bit width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MwidthW<C5ctrlSpec> {
        MwidthW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn chpl(&mut self) -> ChplW<C5ctrlSpec> {
        ChplW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn m2m(&mut self) -> M2mW<C5ctrlSpec> {
        M2mW::new(self, 14)
    }
}
#[doc = "DMA channel configuration register (DMA_C5CTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5ctrlSpec;
impl crate::RegisterSpec for C5ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5ctrl::R`](R) reader structure"]
impl crate::Readable for C5ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`c5ctrl::W`](W) writer structure"]
impl crate::Writable for C5ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C5CTRL to value 0"]
impl crate::Resettable for C5ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
