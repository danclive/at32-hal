#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `INTEN0` reader - Interrupt enable or disable on line 0"]
pub type Inten0R = crate::BitReader;
#[doc = "Field `INTEN0` writer - Interrupt enable or disable on line 0"]
pub type Inten0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN1` reader - Interrupt enable or disable on line 1"]
pub type Inten1R = crate::BitReader;
#[doc = "Field `INTEN1` writer - Interrupt enable or disable on line 1"]
pub type Inten1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN2` reader - Interrupt enable or disable on line 2"]
pub type Inten2R = crate::BitReader;
#[doc = "Field `INTEN2` writer - Interrupt enable or disable on line 2"]
pub type Inten2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN3` reader - Interrupt enable or disable on line 3"]
pub type Inten3R = crate::BitReader;
#[doc = "Field `INTEN3` writer - Interrupt enable or disable on line 3"]
pub type Inten3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN4` reader - Interrupt enable or disable on line 4"]
pub type Inten4R = crate::BitReader;
#[doc = "Field `INTEN4` writer - Interrupt enable or disable on line 4"]
pub type Inten4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN5` reader - Interrupt enable or disable on line 5"]
pub type Inten5R = crate::BitReader;
#[doc = "Field `INTEN5` writer - Interrupt enable or disable on line 5"]
pub type Inten5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN6` reader - Interrupt enable or disable on line 6"]
pub type Inten6R = crate::BitReader;
#[doc = "Field `INTEN6` writer - Interrupt enable or disable on line 6"]
pub type Inten6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN7` reader - Interrupt enable or disable on line 7"]
pub type Inten7R = crate::BitReader;
#[doc = "Field `INTEN7` writer - Interrupt enable or disable on line 7"]
pub type Inten7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN8` reader - Interrupt enable or disable on line 8"]
pub type Inten8R = crate::BitReader;
#[doc = "Field `INTEN8` writer - Interrupt enable or disable on line 8"]
pub type Inten8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN9` reader - Interrupt enable or disable on line 9"]
pub type Inten9R = crate::BitReader;
#[doc = "Field `INTEN9` writer - Interrupt enable or disable on line 9"]
pub type Inten9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN10` reader - Interrupt enable or disable on line 10"]
pub type Inten10R = crate::BitReader;
#[doc = "Field `INTEN10` writer - Interrupt enable or disable on line 10"]
pub type Inten10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN11` reader - Interrupt enable or disable on line 11"]
pub type Inten11R = crate::BitReader;
#[doc = "Field `INTEN11` writer - Interrupt enable or disable on line 11"]
pub type Inten11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN12` reader - Interrupt enable or disable on line 12"]
pub type Inten12R = crate::BitReader;
#[doc = "Field `INTEN12` writer - Interrupt enable or disable on line 12"]
pub type Inten12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN13` reader - Interrupt enable or disable on line 13"]
pub type Inten13R = crate::BitReader;
#[doc = "Field `INTEN13` writer - Interrupt enable or disable on line 13"]
pub type Inten13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN14` reader - Interrupt enable or disable on line 14"]
pub type Inten14R = crate::BitReader;
#[doc = "Field `INTEN14` writer - Interrupt enable or disable on line 14"]
pub type Inten14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN15` reader - Interrupt enable or disable on line 15"]
pub type Inten15R = crate::BitReader;
#[doc = "Field `INTEN15` writer - Interrupt enable or disable on line 15"]
pub type Inten15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN16` reader - Interrupt enable or disable on line 16"]
pub type Inten16R = crate::BitReader;
#[doc = "Field `INTEN16` writer - Interrupt enable or disable on line 16"]
pub type Inten16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN17` reader - Interrupt enable or disable on line 17"]
pub type Inten17R = crate::BitReader;
#[doc = "Field `INTEN17` writer - Interrupt enable or disable on line 17"]
pub type Inten17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTEN18` reader - Interrupt enable or disable on line 18"]
pub type Inten18R = crate::BitReader;
#[doc = "Field `INTEN18` writer - Interrupt enable or disable on line 18"]
pub type Inten18W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    pub fn inten0(&self) -> Inten0R {
        Inten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    pub fn inten1(&self) -> Inten1R {
        Inten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    pub fn inten2(&self) -> Inten2R {
        Inten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    pub fn inten3(&self) -> Inten3R {
        Inten3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    pub fn inten4(&self) -> Inten4R {
        Inten4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    pub fn inten5(&self) -> Inten5R {
        Inten5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    pub fn inten6(&self) -> Inten6R {
        Inten6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    pub fn inten7(&self) -> Inten7R {
        Inten7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    pub fn inten8(&self) -> Inten8R {
        Inten8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    pub fn inten9(&self) -> Inten9R {
        Inten9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    pub fn inten10(&self) -> Inten10R {
        Inten10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    pub fn inten11(&self) -> Inten11R {
        Inten11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    pub fn inten12(&self) -> Inten12R {
        Inten12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    pub fn inten13(&self) -> Inten13R {
        Inten13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    pub fn inten14(&self) -> Inten14R {
        Inten14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    pub fn inten15(&self) -> Inten15R {
        Inten15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    pub fn inten16(&self) -> Inten16R {
        Inten16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    pub fn inten17(&self) -> Inten17R {
        Inten17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    pub fn inten18(&self) -> Inten18R {
        Inten18R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTEN")
            .field("inten0", &self.inten0())
            .field("inten1", &self.inten1())
            .field("inten2", &self.inten2())
            .field("inten3", &self.inten3())
            .field("inten4", &self.inten4())
            .field("inten5", &self.inten5())
            .field("inten6", &self.inten6())
            .field("inten7", &self.inten7())
            .field("inten8", &self.inten8())
            .field("inten9", &self.inten9())
            .field("inten10", &self.inten10())
            .field("inten11", &self.inten11())
            .field("inten12", &self.inten12())
            .field("inten13", &self.inten13())
            .field("inten14", &self.inten14())
            .field("inten15", &self.inten15())
            .field("inten16", &self.inten16())
            .field("inten17", &self.inten17())
            .field("inten18", &self.inten18())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable or disable on line 0"]
    #[inline(always)]
    #[must_use]
    pub fn inten0(&mut self) -> Inten0W<IntenSpec> {
        Inten0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable or disable on line 1"]
    #[inline(always)]
    #[must_use]
    pub fn inten1(&mut self) -> Inten1W<IntenSpec> {
        Inten1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable or disable on line 2"]
    #[inline(always)]
    #[must_use]
    pub fn inten2(&mut self) -> Inten2W<IntenSpec> {
        Inten2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable or disable on line 3"]
    #[inline(always)]
    #[must_use]
    pub fn inten3(&mut self) -> Inten3W<IntenSpec> {
        Inten3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable or disable on line 4"]
    #[inline(always)]
    #[must_use]
    pub fn inten4(&mut self) -> Inten4W<IntenSpec> {
        Inten4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable or disable on line 5"]
    #[inline(always)]
    #[must_use]
    pub fn inten5(&mut self) -> Inten5W<IntenSpec> {
        Inten5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable or disable on line 6"]
    #[inline(always)]
    #[must_use]
    pub fn inten6(&mut self) -> Inten6W<IntenSpec> {
        Inten6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable or disable on line 7"]
    #[inline(always)]
    #[must_use]
    pub fn inten7(&mut self) -> Inten7W<IntenSpec> {
        Inten7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt enable or disable on line 8"]
    #[inline(always)]
    #[must_use]
    pub fn inten8(&mut self) -> Inten8W<IntenSpec> {
        Inten8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt enable or disable on line 9"]
    #[inline(always)]
    #[must_use]
    pub fn inten9(&mut self) -> Inten9W<IntenSpec> {
        Inten9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt enable or disable on line 10"]
    #[inline(always)]
    #[must_use]
    pub fn inten10(&mut self) -> Inten10W<IntenSpec> {
        Inten10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt enable or disable on line 11"]
    #[inline(always)]
    #[must_use]
    pub fn inten11(&mut self) -> Inten11W<IntenSpec> {
        Inten11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt enable or disable on line 12"]
    #[inline(always)]
    #[must_use]
    pub fn inten12(&mut self) -> Inten12W<IntenSpec> {
        Inten12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt enable or disable on line 13"]
    #[inline(always)]
    #[must_use]
    pub fn inten13(&mut self) -> Inten13W<IntenSpec> {
        Inten13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt enable or disable on line 14"]
    #[inline(always)]
    #[must_use]
    pub fn inten14(&mut self) -> Inten14W<IntenSpec> {
        Inten14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt enable or disable on line 15"]
    #[inline(always)]
    #[must_use]
    pub fn inten15(&mut self) -> Inten15W<IntenSpec> {
        Inten15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt enable or disable on line 16"]
    #[inline(always)]
    #[must_use]
    pub fn inten16(&mut self) -> Inten16W<IntenSpec> {
        Inten16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt enable or disable on line 17"]
    #[inline(always)]
    #[must_use]
    pub fn inten17(&mut self) -> Inten17W<IntenSpec> {
        Inten17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt enable or disable on line 18"]
    #[inline(always)]
    #[must_use]
    pub fn inten18(&mut self) -> Inten18W<IntenSpec> {
        Inten18W::new(self, 18)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
