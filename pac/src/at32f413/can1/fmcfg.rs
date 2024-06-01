#[doc = "Register `FMCFG` reader"]
pub type R = crate::R<FmcfgSpec>;
#[doc = "Register `FMCFG` writer"]
pub type W = crate::W<FmcfgSpec>;
#[doc = "Field `FMSEL0` reader - Filter mode select"]
pub type Fmsel0R = crate::BitReader;
#[doc = "Field `FMSEL0` writer - Filter mode select"]
pub type Fmsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL1` reader - Filter mode select"]
pub type Fmsel1R = crate::BitReader;
#[doc = "Field `FMSEL1` writer - Filter mode select"]
pub type Fmsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL2` reader - Filter mode select"]
pub type Fmsel2R = crate::BitReader;
#[doc = "Field `FMSEL2` writer - Filter mode select"]
pub type Fmsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL3` reader - Filter mode select"]
pub type Fmsel3R = crate::BitReader;
#[doc = "Field `FMSEL3` writer - Filter mode select"]
pub type Fmsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL4` reader - Filter mode select"]
pub type Fmsel4R = crate::BitReader;
#[doc = "Field `FMSEL4` writer - Filter mode select"]
pub type Fmsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL5` reader - Filter mode select"]
pub type Fmsel5R = crate::BitReader;
#[doc = "Field `FMSEL5` writer - Filter mode select"]
pub type Fmsel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL6` reader - Filter mode select"]
pub type Fmsel6R = crate::BitReader;
#[doc = "Field `FMSEL6` writer - Filter mode select"]
pub type Fmsel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL7` reader - Filter mode select"]
pub type Fmsel7R = crate::BitReader;
#[doc = "Field `FMSEL7` writer - Filter mode select"]
pub type Fmsel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL8` reader - Filter mode select"]
pub type Fmsel8R = crate::BitReader;
#[doc = "Field `FMSEL8` writer - Filter mode select"]
pub type Fmsel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL9` reader - Filter mode select"]
pub type Fmsel9R = crate::BitReader;
#[doc = "Field `FMSEL9` writer - Filter mode select"]
pub type Fmsel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL10` reader - Filter mode select"]
pub type Fmsel10R = crate::BitReader;
#[doc = "Field `FMSEL10` writer - Filter mode select"]
pub type Fmsel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL11` reader - Filter mode select"]
pub type Fmsel11R = crate::BitReader;
#[doc = "Field `FMSEL11` writer - Filter mode select"]
pub type Fmsel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL12` reader - Filter mode select"]
pub type Fmsel12R = crate::BitReader;
#[doc = "Field `FMSEL12` writer - Filter mode select"]
pub type Fmsel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMSEL13` reader - Filter mode select"]
pub type Fmsel13R = crate::BitReader;
#[doc = "Field `FMSEL13` writer - Filter mode select"]
pub type Fmsel13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel0(&self) -> Fmsel0R {
        Fmsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel1(&self) -> Fmsel1R {
        Fmsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel2(&self) -> Fmsel2R {
        Fmsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel3(&self) -> Fmsel3R {
        Fmsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel4(&self) -> Fmsel4R {
        Fmsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel5(&self) -> Fmsel5R {
        Fmsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel6(&self) -> Fmsel6R {
        Fmsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel7(&self) -> Fmsel7R {
        Fmsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel8(&self) -> Fmsel8R {
        Fmsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel9(&self) -> Fmsel9R {
        Fmsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel10(&self) -> Fmsel10R {
        Fmsel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel11(&self) -> Fmsel11R {
        Fmsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel12(&self) -> Fmsel12R {
        Fmsel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    pub fn fmsel13(&self) -> Fmsel13R {
        Fmsel13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMCFG")
            .field("fmsel0", &self.fmsel0())
            .field("fmsel1", &self.fmsel1())
            .field("fmsel2", &self.fmsel2())
            .field("fmsel3", &self.fmsel3())
            .field("fmsel4", &self.fmsel4())
            .field("fmsel5", &self.fmsel5())
            .field("fmsel6", &self.fmsel6())
            .field("fmsel7", &self.fmsel7())
            .field("fmsel8", &self.fmsel8())
            .field("fmsel9", &self.fmsel9())
            .field("fmsel10", &self.fmsel10())
            .field("fmsel11", &self.fmsel11())
            .field("fmsel12", &self.fmsel12())
            .field("fmsel13", &self.fmsel13())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel0(&mut self) -> Fmsel0W<FmcfgSpec> {
        Fmsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel1(&mut self) -> Fmsel1W<FmcfgSpec> {
        Fmsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel2(&mut self) -> Fmsel2W<FmcfgSpec> {
        Fmsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel3(&mut self) -> Fmsel3W<FmcfgSpec> {
        Fmsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel4(&mut self) -> Fmsel4W<FmcfgSpec> {
        Fmsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel5(&mut self) -> Fmsel5W<FmcfgSpec> {
        Fmsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel6(&mut self) -> Fmsel6W<FmcfgSpec> {
        Fmsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel7(&mut self) -> Fmsel7W<FmcfgSpec> {
        Fmsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel8(&mut self) -> Fmsel8W<FmcfgSpec> {
        Fmsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel9(&mut self) -> Fmsel9W<FmcfgSpec> {
        Fmsel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel10(&mut self) -> Fmsel10W<FmcfgSpec> {
        Fmsel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel11(&mut self) -> Fmsel11W<FmcfgSpec> {
        Fmsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel12(&mut self) -> Fmsel12W<FmcfgSpec> {
        Fmsel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter mode select"]
    #[inline(always)]
    #[must_use]
    pub fn fmsel13(&mut self) -> Fmsel13W<FmcfgSpec> {
        Fmsel13W::new(self, 13)
    }
}
#[doc = "Filter mode config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcfgSpec;
impl crate::RegisterSpec for FmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfg::R`](R) reader structure"]
impl crate::Readable for FmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fmcfg::W`](W) writer structure"]
impl crate::Writable for FmcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMCFG to value 0"]
impl crate::Resettable for FmcfgSpec {
    const RESET_VALUE: u32 = 0;
}
