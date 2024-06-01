#[doc = "Register `WPR` reader"]
pub type R = crate::R<WprSpec>;
#[doc = "Register `WPR` writer"]
pub type W = crate::W<WprSpec>;
#[doc = "Field `WPEN0` reader - Write protect enable 0"]
pub type Wpen0R = crate::BitReader;
#[doc = "Field `WPEN0` writer - Write protect enable 0"]
pub type Wpen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN1` reader - Write protect enable 1"]
pub type Wpen1R = crate::BitReader;
#[doc = "Field `WPEN1` writer - Write protect enable 1"]
pub type Wpen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN2` reader - Write protect enable 2"]
pub type Wpen2R = crate::BitReader;
#[doc = "Field `WPEN2` writer - Write protect enable 2"]
pub type Wpen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN3` reader - Write protect enable 3"]
pub type Wpen3R = crate::BitReader;
#[doc = "Field `WPEN3` writer - Write protect enable 3"]
pub type Wpen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN4` reader - Write protect enable 4"]
pub type Wpen4R = crate::BitReader;
#[doc = "Field `WPEN4` writer - Write protect enable 4"]
pub type Wpen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN5` reader - Write protect enable 5"]
pub type Wpen5R = crate::BitReader;
#[doc = "Field `WPEN5` writer - Write protect enable 5"]
pub type Wpen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN6` reader - Write protect enable 6"]
pub type Wpen6R = crate::BitReader;
#[doc = "Field `WPEN6` writer - Write protect enable 6"]
pub type Wpen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN7` reader - Write protect enable 7"]
pub type Wpen7R = crate::BitReader;
#[doc = "Field `WPEN7` writer - Write protect enable 7"]
pub type Wpen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN8` reader - Write protect enable 8"]
pub type Wpen8R = crate::BitReader;
#[doc = "Field `WPEN8` writer - Write protect enable 8"]
pub type Wpen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN9` reader - Write protect enable 9"]
pub type Wpen9R = crate::BitReader;
#[doc = "Field `WPEN9` writer - Write protect enable 9"]
pub type Wpen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN10` reader - Write protect enable 10"]
pub type Wpen10R = crate::BitReader;
#[doc = "Field `WPEN10` writer - Write protect enable 10"]
pub type Wpen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN11` reader - Write protect enable 11"]
pub type Wpen11R = crate::BitReader;
#[doc = "Field `WPEN11` writer - Write protect enable 11"]
pub type Wpen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN12` reader - Write protect enable 12"]
pub type Wpen12R = crate::BitReader;
#[doc = "Field `WPEN12` writer - Write protect enable 12"]
pub type Wpen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN13` reader - Write protect enable 13"]
pub type Wpen13R = crate::BitReader;
#[doc = "Field `WPEN13` writer - Write protect enable 13"]
pub type Wpen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN14` reader - Write protect enable 14"]
pub type Wpen14R = crate::BitReader;
#[doc = "Field `WPEN14` writer - Write protect enable 14"]
pub type Wpen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPEN15` reader - Write protect enable 15"]
pub type Wpen15R = crate::BitReader;
#[doc = "Field `WPEN15` writer - Write protect enable 15"]
pub type Wpen15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WPSEQ` reader - Write protect sequence"]
pub type WpseqR = crate::BitReader;
#[doc = "Field `WPSEQ` writer - Write protect sequence"]
pub type WpseqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    pub fn wpen0(&self) -> Wpen0R {
        Wpen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    pub fn wpen1(&self) -> Wpen1R {
        Wpen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    pub fn wpen2(&self) -> Wpen2R {
        Wpen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    pub fn wpen3(&self) -> Wpen3R {
        Wpen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    pub fn wpen4(&self) -> Wpen4R {
        Wpen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    pub fn wpen5(&self) -> Wpen5R {
        Wpen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    pub fn wpen6(&self) -> Wpen6R {
        Wpen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    pub fn wpen7(&self) -> Wpen7R {
        Wpen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    pub fn wpen8(&self) -> Wpen8R {
        Wpen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    pub fn wpen9(&self) -> Wpen9R {
        Wpen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    pub fn wpen10(&self) -> Wpen10R {
        Wpen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    pub fn wpen11(&self) -> Wpen11R {
        Wpen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    pub fn wpen12(&self) -> Wpen12R {
        Wpen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    pub fn wpen13(&self) -> Wpen13R {
        Wpen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    pub fn wpen14(&self) -> Wpen14R {
        Wpen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    pub fn wpen15(&self) -> Wpen15R {
        Wpen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    pub fn wpseq(&self) -> WpseqR {
        WpseqR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPR")
            .field("wpen0", &self.wpen0())
            .field("wpen1", &self.wpen1())
            .field("wpen2", &self.wpen2())
            .field("wpen3", &self.wpen3())
            .field("wpen4", &self.wpen4())
            .field("wpen5", &self.wpen5())
            .field("wpen6", &self.wpen6())
            .field("wpen7", &self.wpen7())
            .field("wpen8", &self.wpen8())
            .field("wpen9", &self.wpen9())
            .field("wpen10", &self.wpen10())
            .field("wpen11", &self.wpen11())
            .field("wpen12", &self.wpen12())
            .field("wpen13", &self.wpen13())
            .field("wpen14", &self.wpen14())
            .field("wpen15", &self.wpen15())
            .field("wpseq", &self.wpseq())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write protect enable 0"]
    #[inline(always)]
    #[must_use]
    pub fn wpen0(&mut self) -> Wpen0W<WprSpec> {
        Wpen0W::new(self, 0)
    }
    #[doc = "Bit 1 - Write protect enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn wpen1(&mut self) -> Wpen1W<WprSpec> {
        Wpen1W::new(self, 1)
    }
    #[doc = "Bit 2 - Write protect enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn wpen2(&mut self) -> Wpen2W<WprSpec> {
        Wpen2W::new(self, 2)
    }
    #[doc = "Bit 3 - Write protect enable 3"]
    #[inline(always)]
    #[must_use]
    pub fn wpen3(&mut self) -> Wpen3W<WprSpec> {
        Wpen3W::new(self, 3)
    }
    #[doc = "Bit 4 - Write protect enable 4"]
    #[inline(always)]
    #[must_use]
    pub fn wpen4(&mut self) -> Wpen4W<WprSpec> {
        Wpen4W::new(self, 4)
    }
    #[doc = "Bit 5 - Write protect enable 5"]
    #[inline(always)]
    #[must_use]
    pub fn wpen5(&mut self) -> Wpen5W<WprSpec> {
        Wpen5W::new(self, 5)
    }
    #[doc = "Bit 6 - Write protect enable 6"]
    #[inline(always)]
    #[must_use]
    pub fn wpen6(&mut self) -> Wpen6W<WprSpec> {
        Wpen6W::new(self, 6)
    }
    #[doc = "Bit 7 - Write protect enable 7"]
    #[inline(always)]
    #[must_use]
    pub fn wpen7(&mut self) -> Wpen7W<WprSpec> {
        Wpen7W::new(self, 7)
    }
    #[doc = "Bit 8 - Write protect enable 8"]
    #[inline(always)]
    #[must_use]
    pub fn wpen8(&mut self) -> Wpen8W<WprSpec> {
        Wpen8W::new(self, 8)
    }
    #[doc = "Bit 9 - Write protect enable 9"]
    #[inline(always)]
    #[must_use]
    pub fn wpen9(&mut self) -> Wpen9W<WprSpec> {
        Wpen9W::new(self, 9)
    }
    #[doc = "Bit 10 - Write protect enable 10"]
    #[inline(always)]
    #[must_use]
    pub fn wpen10(&mut self) -> Wpen10W<WprSpec> {
        Wpen10W::new(self, 10)
    }
    #[doc = "Bit 11 - Write protect enable 11"]
    #[inline(always)]
    #[must_use]
    pub fn wpen11(&mut self) -> Wpen11W<WprSpec> {
        Wpen11W::new(self, 11)
    }
    #[doc = "Bit 12 - Write protect enable 12"]
    #[inline(always)]
    #[must_use]
    pub fn wpen12(&mut self) -> Wpen12W<WprSpec> {
        Wpen12W::new(self, 12)
    }
    #[doc = "Bit 13 - Write protect enable 13"]
    #[inline(always)]
    #[must_use]
    pub fn wpen13(&mut self) -> Wpen13W<WprSpec> {
        Wpen13W::new(self, 13)
    }
    #[doc = "Bit 14 - Write protect enable 14"]
    #[inline(always)]
    #[must_use]
    pub fn wpen14(&mut self) -> Wpen14W<WprSpec> {
        Wpen14W::new(self, 14)
    }
    #[doc = "Bit 15 - Write protect enable 15"]
    #[inline(always)]
    #[must_use]
    pub fn wpen15(&mut self) -> Wpen15W<WprSpec> {
        Wpen15W::new(self, 15)
    }
    #[doc = "Bit 16 - Write protect sequence"]
    #[inline(always)]
    #[must_use]
    pub fn wpseq(&mut self) -> WpseqW<WprSpec> {
        WpseqW::new(self, 16)
    }
}
#[doc = "Port write protect register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WprSpec;
impl crate::RegisterSpec for WprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpr::R`](R) reader structure"]
impl crate::Readable for WprSpec {}
#[doc = "`write(|w| ..)` method takes [`wpr::W`](W) writer structure"]
impl crate::Writable for WprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPR to value 0"]
impl crate::Resettable for WprSpec {
    const RESET_VALUE: u32 = 0;
}
