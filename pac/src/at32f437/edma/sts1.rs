#[doc = "Register `STS1` reader"]
pub type R = crate::R<Sts1Spec>;
#[doc = "Field `FERRF1` reader - Stream 1 FIFO error interrupt flag"]
pub type Ferrf1R = crate::BitReader;
#[doc = "Field `DMERRF1` reader - Stream 1 direct mode error interrupt flag"]
pub type Dmerrf1R = crate::BitReader;
#[doc = "Field `DTERRF1` reader - Stream 1 transfer error interrupt flag"]
pub type Dterrf1R = crate::BitReader;
#[doc = "Field `HDTF1` reader - Stream 1 half data transfer interrupt flag"]
pub type Hdtf1R = crate::BitReader;
#[doc = "Field `FDTF1` reader - Stream 1 Full data transfer interrupt flag"]
pub type Fdtf1R = crate::BitReader;
#[doc = "Field `FERRF2` reader - Stream 2 FIFO error interrupt flag"]
pub type Ferrf2R = crate::BitReader;
#[doc = "Field `DMERRF2` reader - Stream 2 direct mode error interrupt flag"]
pub type Dmerrf2R = crate::BitReader;
#[doc = "Field `DTERRF2` reader - Stream 2 transfer error interrupt flag"]
pub type Dterrf2R = crate::BitReader;
#[doc = "Field `HDTF2` reader - Stream 2 half data transfer interrupt flag"]
pub type Hdtf2R = crate::BitReader;
#[doc = "Field `FDTF2` reader - Stream 2 Full data transfer interrupt flag"]
pub type Fdtf2R = crate::BitReader;
#[doc = "Field `FERRF3` reader - Stream 3 FIFO error interrupt flag"]
pub type Ferrf3R = crate::BitReader;
#[doc = "Field `DMERRF3` reader - Stream 3 direct mode error interrupt flag"]
pub type Dmerrf3R = crate::BitReader;
#[doc = "Field `DTERRF3` reader - Stream 3 transfer error interrupt flag"]
pub type Dterrf3R = crate::BitReader;
#[doc = "Field `HDTF3` reader - Stream 3 half data transfer interrupt flag"]
pub type Hdtf3R = crate::BitReader;
#[doc = "Field `FDTF3` reader - Stream 3 Full data transfer interrupt flag"]
pub type Fdtf3R = crate::BitReader;
#[doc = "Field `FERRF4` reader - Stream 4 FIFO error interrupt flag"]
pub type Ferrf4R = crate::BitReader;
#[doc = "Field `DMERRF4` reader - Stream 4 direct mode error interrupt flag"]
pub type Dmerrf4R = crate::BitReader;
#[doc = "Field `DTERRF4` reader - Stream 4 transfer error interrupt flag"]
pub type Dterrf4R = crate::BitReader;
#[doc = "Field `HDTF4` reader - Stream 4 half data transfer interrupt flag"]
pub type Hdtf4R = crate::BitReader;
#[doc = "Field `FDTF4` reader - Stream 4 Full data transfer interrupt flag"]
pub type Fdtf4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Stream 1 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf1(&self) -> Ferrf1R {
        Ferrf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Stream 1 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf1(&self) -> Dmerrf1R {
        Dmerrf1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Stream 1 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf1(&self) -> Dterrf1R {
        Dterrf1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream 1 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf1(&self) -> Hdtf1R {
        Hdtf1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stream 1 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf1(&self) -> Fdtf1R {
        Fdtf1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stream 2 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf2(&self) -> Ferrf2R {
        Ferrf2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Stream 2 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf2(&self) -> Dmerrf2R {
        Dmerrf2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stream 2 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf2(&self) -> Dterrf2R {
        Dterrf2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stream 2 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf2(&self) -> Hdtf2R {
        Hdtf2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stream 2 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf2(&self) -> Fdtf2R {
        Fdtf2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Stream 3 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf3(&self) -> Ferrf3R {
        Ferrf3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Stream 3 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf3(&self) -> Dmerrf3R {
        Dmerrf3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Stream 3 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf3(&self) -> Dterrf3R {
        Dterrf3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stream 3 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf3(&self) -> Hdtf3R {
        Hdtf3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Stream 3 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf3(&self) -> Fdtf3R {
        Fdtf3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Stream 4 FIFO error interrupt flag"]
    #[inline(always)]
    pub fn ferrf4(&self) -> Ferrf4R {
        Ferrf4R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Stream 4 direct mode error interrupt flag"]
    #[inline(always)]
    pub fn dmerrf4(&self) -> Dmerrf4R {
        Dmerrf4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stream 4 transfer error interrupt flag"]
    #[inline(always)]
    pub fn dterrf4(&self) -> Dterrf4R {
        Dterrf4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Stream 4 half data transfer interrupt flag"]
    #[inline(always)]
    pub fn hdtf4(&self) -> Hdtf4R {
        Hdtf4R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Stream 4 Full data transfer interrupt flag"]
    #[inline(always)]
    pub fn fdtf4(&self) -> Fdtf4R {
        Fdtf4R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS1")
            .field("fdtf4", &self.fdtf4())
            .field("hdtf4", &self.hdtf4())
            .field("dterrf4", &self.dterrf4())
            .field("dmerrf4", &self.dmerrf4())
            .field("ferrf4", &self.ferrf4())
            .field("fdtf3", &self.fdtf3())
            .field("hdtf3", &self.hdtf3())
            .field("dterrf3", &self.dterrf3())
            .field("dmerrf3", &self.dmerrf3())
            .field("ferrf3", &self.ferrf3())
            .field("fdtf2", &self.fdtf2())
            .field("hdtf2", &self.hdtf2())
            .field("dterrf2", &self.dterrf2())
            .field("dmerrf2", &self.dmerrf2())
            .field("ferrf2", &self.ferrf2())
            .field("fdtf1", &self.fdtf1())
            .field("hdtf1", &self.hdtf1())
            .field("dterrf1", &self.dterrf1())
            .field("dmerrf1", &self.dmerrf1())
            .field("ferrf1", &self.ferrf1())
            .finish()
    }
}
#[doc = "Interrupt status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts1Spec;
impl crate::RegisterSpec for Sts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts1::R`](R) reader structure"]
impl crate::Readable for Sts1Spec {}
#[doc = "`reset()` method sets STS1 to value 0"]
impl crate::Resettable for Sts1Spec {
    const RESET_VALUE: u32 = 0;
}
