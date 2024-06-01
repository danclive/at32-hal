#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Field `GF1` reader - Channel 1 Global event flag"]
pub type Gf1R = crate::BitReader;
#[doc = "Field `FDTF1` reader - Channel 1 full data transfer event flag"]
pub type Fdtf1R = crate::BitReader;
#[doc = "Field `HDTF1` reader - Channel 1 half data transfer event flag"]
pub type Hdtf1R = crate::BitReader;
#[doc = "Field `DTERRF1` reader - Channel 1 data transfer error event flag"]
pub type Dterrf1R = crate::BitReader;
#[doc = "Field `GF2` reader - Channel 2 Global event flag"]
pub type Gf2R = crate::BitReader;
#[doc = "Field `FDTF2` reader - Channel 2 full data transfer event flag"]
pub type Fdtf2R = crate::BitReader;
#[doc = "Field `HDTF2` reader - Channel 2 half data transfer event flag"]
pub type Hdtf2R = crate::BitReader;
#[doc = "Field `DTERRF2` reader - Channel 2 data transfer error event flag"]
pub type Dterrf2R = crate::BitReader;
#[doc = "Field `GF3` reader - Channel 3 Global event flag"]
pub type Gf3R = crate::BitReader;
#[doc = "Field `FDTF3` reader - Channel 3 full data transfer event flag"]
pub type Fdtf3R = crate::BitReader;
#[doc = "Field `HDTF3` reader - Channel 3 half data transfer event flag"]
pub type Hdtf3R = crate::BitReader;
#[doc = "Field `DTERRF3` reader - Channel 3 data transfer error event flag"]
pub type Dterrf3R = crate::BitReader;
#[doc = "Field `GF4` reader - Channel 4 Global event flag"]
pub type Gf4R = crate::BitReader;
#[doc = "Field `FDTF4` reader - Channel 4 full data transfer event flag"]
pub type Fdtf4R = crate::BitReader;
#[doc = "Field `HDTF4` reader - Channel 4 half data transfer event flag"]
pub type Hdtf4R = crate::BitReader;
#[doc = "Field `DTERRF4` reader - Channel 4 data transfer error event flag"]
pub type Dterrf4R = crate::BitReader;
#[doc = "Field `GF5` reader - Channel 5 Global event flag"]
pub type Gf5R = crate::BitReader;
#[doc = "Field `FDTF5` reader - Channel 5 full data transfer event flag"]
pub type Fdtf5R = crate::BitReader;
#[doc = "Field `HDTF5` reader - Channel 5 half data transfer event flag"]
pub type Hdtf5R = crate::BitReader;
#[doc = "Field `DTERRF5` reader - Channel 5 data transfer error event flag"]
pub type Dterrf5R = crate::BitReader;
#[doc = "Field `GF6` reader - Channel 6 Global event flag"]
pub type Gf6R = crate::BitReader;
#[doc = "Field `FDTF6` reader - Channel 6 full data transfer event flag"]
pub type Fdtf6R = crate::BitReader;
#[doc = "Field `HDTF6` reader - Channel 6 half data transfer event flag"]
pub type Hdtf6R = crate::BitReader;
#[doc = "Field `DTERRF6` reader - Channel 6 data transfer error event flag"]
pub type Dterrf6R = crate::BitReader;
#[doc = "Field `GF7` reader - Channel 7 Global event flag"]
pub type Gf7R = crate::BitReader;
#[doc = "Field `FDTF7` reader - Channel 7 full data transfer event flag"]
pub type Fdtf7R = crate::BitReader;
#[doc = "Field `HDTF7` reader - Channel 7 half data transfer event flag"]
pub type Hdtf7R = crate::BitReader;
#[doc = "Field `DTERRF7` reader - Channel 7 data transfer error event flag"]
pub type Dterrf7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel 1 Global event flag"]
    #[inline(always)]
    pub fn gf1(&self) -> Gf1R {
        Gf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf1(&self) -> Fdtf1R {
        Fdtf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf1(&self) -> Hdtf1R {
        Hdtf1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf1(&self) -> Dterrf1R {
        Dterrf1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 Global event flag"]
    #[inline(always)]
    pub fn gf2(&self) -> Gf2R {
        Gf2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf2(&self) -> Fdtf2R {
        Fdtf2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 2 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf2(&self) -> Hdtf2R {
        Hdtf2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 2 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf2(&self) -> Dterrf2R {
        Dterrf2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 Global event flag"]
    #[inline(always)]
    pub fn gf3(&self) -> Gf3R {
        Gf3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf3(&self) -> Fdtf3R {
        Fdtf3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel 3 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf3(&self) -> Hdtf3R {
        Hdtf3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel 3 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf3(&self) -> Dterrf3R {
        Dterrf3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 Global event flag"]
    #[inline(always)]
    pub fn gf4(&self) -> Gf4R {
        Gf4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf4(&self) -> Fdtf4R {
        Fdtf4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel 4 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf4(&self) -> Hdtf4R {
        Hdtf4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel 4 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf4(&self) -> Dterrf4R {
        Dterrf4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 5 Global event flag"]
    #[inline(always)]
    pub fn gf5(&self) -> Gf5R {
        Gf5R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Channel 5 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf5(&self) -> Fdtf5R {
        Fdtf5R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Channel 5 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf5(&self) -> Hdtf5R {
        Hdtf5R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Channel 5 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf5(&self) -> Dterrf5R {
        Dterrf5R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Channel 6 Global event flag"]
    #[inline(always)]
    pub fn gf6(&self) -> Gf6R {
        Gf6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Channel 6 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf6(&self) -> Fdtf6R {
        Fdtf6R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Channel 6 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf6(&self) -> Hdtf6R {
        Hdtf6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Channel 6 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf6(&self) -> Dterrf6R {
        Dterrf6R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Channel 7 Global event flag"]
    #[inline(always)]
    pub fn gf7(&self) -> Gf7R {
        Gf7R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Channel 7 full data transfer event flag"]
    #[inline(always)]
    pub fn fdtf7(&self) -> Fdtf7R {
        Fdtf7R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Channel 7 half data transfer event flag"]
    #[inline(always)]
    pub fn hdtf7(&self) -> Hdtf7R {
        Hdtf7R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Channel 7 data transfer error event flag"]
    #[inline(always)]
    pub fn dterrf7(&self) -> Dterrf7R {
        Dterrf7R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("gf1", &self.gf1())
            .field("gf2", &self.gf2())
            .field("gf3", &self.gf3())
            .field("gf4", &self.gf4())
            .field("gf5", &self.gf5())
            .field("gf6", &self.gf6())
            .field("gf7", &self.gf7())
            .field("fdtf1", &self.fdtf1())
            .field("fdtf2", &self.fdtf2())
            .field("fdtf3", &self.fdtf3())
            .field("fdtf4", &self.fdtf4())
            .field("fdtf5", &self.fdtf5())
            .field("fdtf6", &self.fdtf6())
            .field("fdtf7", &self.fdtf7())
            .field("hdtf1", &self.hdtf1())
            .field("hdtf2", &self.hdtf2())
            .field("hdtf3", &self.hdtf3())
            .field("hdtf4", &self.hdtf4())
            .field("hdtf5", &self.hdtf5())
            .field("hdtf6", &self.hdtf6())
            .field("hdtf7", &self.hdtf7())
            .field("dterrf1", &self.dterrf1())
            .field("dterrf2", &self.dterrf2())
            .field("dterrf3", &self.dterrf3())
            .field("dterrf4", &self.dterrf4())
            .field("dterrf5", &self.dterrf5())
            .field("dterrf6", &self.dterrf6())
            .field("dterrf7", &self.dterrf7())
            .finish()
    }
}
#[doc = "DMA status register (DMA_STS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
