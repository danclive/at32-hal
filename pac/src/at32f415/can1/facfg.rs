#[doc = "Register `FACFG` reader"]
pub type R = crate::R<FacfgSpec>;
#[doc = "Register `FACFG` writer"]
pub type W = crate::W<FacfgSpec>;
#[doc = "Field `FAEN0` reader - Filter activate enable"]
pub type Faen0R = crate::BitReader;
#[doc = "Field `FAEN0` writer - Filter activate enable"]
pub type Faen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN1` reader - Filter activate enable"]
pub type Faen1R = crate::BitReader;
#[doc = "Field `FAEN1` writer - Filter activate enable"]
pub type Faen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN2` reader - Filter activate enable"]
pub type Faen2R = crate::BitReader;
#[doc = "Field `FAEN2` writer - Filter activate enable"]
pub type Faen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN3` reader - Filter activate enable"]
pub type Faen3R = crate::BitReader;
#[doc = "Field `FAEN3` writer - Filter activate enable"]
pub type Faen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN4` reader - Filter activate enable"]
pub type Faen4R = crate::BitReader;
#[doc = "Field `FAEN4` writer - Filter activate enable"]
pub type Faen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN5` reader - Filter activate enable"]
pub type Faen5R = crate::BitReader;
#[doc = "Field `FAEN5` writer - Filter activate enable"]
pub type Faen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN6` reader - Filter activate enable"]
pub type Faen6R = crate::BitReader;
#[doc = "Field `FAEN6` writer - Filter activate enable"]
pub type Faen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN7` reader - Filter activate enable"]
pub type Faen7R = crate::BitReader;
#[doc = "Field `FAEN7` writer - Filter activate enable"]
pub type Faen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN8` reader - Filter activate enable"]
pub type Faen8R = crate::BitReader;
#[doc = "Field `FAEN8` writer - Filter activate enable"]
pub type Faen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN9` reader - Filter activate enable"]
pub type Faen9R = crate::BitReader;
#[doc = "Field `FAEN9` writer - Filter activate enable"]
pub type Faen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN10` reader - Filter activate enable"]
pub type Faen10R = crate::BitReader;
#[doc = "Field `FAEN10` writer - Filter activate enable"]
pub type Faen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN11` reader - Filter activate enable"]
pub type Faen11R = crate::BitReader;
#[doc = "Field `FAEN11` writer - Filter activate enable"]
pub type Faen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN12` reader - Filter activate enable"]
pub type Faen12R = crate::BitReader;
#[doc = "Field `FAEN12` writer - Filter activate enable"]
pub type Faen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAEN13` reader - Filter activate enable"]
pub type Faen13R = crate::BitReader;
#[doc = "Field `FAEN13` writer - Filter activate enable"]
pub type Faen13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    pub fn faen0(&self) -> Faen0R {
        Faen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    pub fn faen1(&self) -> Faen1R {
        Faen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    pub fn faen2(&self) -> Faen2R {
        Faen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    pub fn faen3(&self) -> Faen3R {
        Faen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    pub fn faen4(&self) -> Faen4R {
        Faen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    pub fn faen5(&self) -> Faen5R {
        Faen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    pub fn faen6(&self) -> Faen6R {
        Faen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    pub fn faen7(&self) -> Faen7R {
        Faen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    pub fn faen8(&self) -> Faen8R {
        Faen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    pub fn faen9(&self) -> Faen9R {
        Faen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    pub fn faen10(&self) -> Faen10R {
        Faen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    pub fn faen11(&self) -> Faen11R {
        Faen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    pub fn faen12(&self) -> Faen12R {
        Faen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    pub fn faen13(&self) -> Faen13R {
        Faen13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FACFG")
            .field("faen0", &self.faen0())
            .field("faen1", &self.faen1())
            .field("faen2", &self.faen2())
            .field("faen3", &self.faen3())
            .field("faen4", &self.faen4())
            .field("faen5", &self.faen5())
            .field("faen6", &self.faen6())
            .field("faen7", &self.faen7())
            .field("faen8", &self.faen8())
            .field("faen9", &self.faen9())
            .field("faen10", &self.faen10())
            .field("faen11", &self.faen11())
            .field("faen12", &self.faen12())
            .field("faen13", &self.faen13())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen0(&mut self) -> Faen0W<FacfgSpec> {
        Faen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen1(&mut self) -> Faen1W<FacfgSpec> {
        Faen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen2(&mut self) -> Faen2W<FacfgSpec> {
        Faen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen3(&mut self) -> Faen3W<FacfgSpec> {
        Faen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen4(&mut self) -> Faen4W<FacfgSpec> {
        Faen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen5(&mut self) -> Faen5W<FacfgSpec> {
        Faen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen6(&mut self) -> Faen6W<FacfgSpec> {
        Faen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen7(&mut self) -> Faen7W<FacfgSpec> {
        Faen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen8(&mut self) -> Faen8W<FacfgSpec> {
        Faen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen9(&mut self) -> Faen9W<FacfgSpec> {
        Faen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen10(&mut self) -> Faen10W<FacfgSpec> {
        Faen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen11(&mut self) -> Faen11W<FacfgSpec> {
        Faen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen12(&mut self) -> Faen12W<FacfgSpec> {
        Faen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter activate enable"]
    #[inline(always)]
    #[must_use]
    pub fn faen13(&mut self) -> Faen13W<FacfgSpec> {
        Faen13W::new(self, 13)
    }
}
#[doc = "Filter activate configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`facfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`facfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FacfgSpec;
impl crate::RegisterSpec for FacfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`facfg::R`](R) reader structure"]
impl crate::Readable for FacfgSpec {}
#[doc = "`write(|w| ..)` method takes [`facfg::W`](W) writer structure"]
impl crate::Writable for FacfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FACFG to value 0"]
impl crate::Resettable for FacfgSpec {
    const RESET_VALUE: u32 = 0;
}
