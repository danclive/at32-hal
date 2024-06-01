#[doc = "Register `ODT` reader"]
pub type R = crate::R<OdtSpec>;
#[doc = "Register `ODT` writer"]
pub type W = crate::W<OdtSpec>;
#[doc = "Field `ODT0` reader - Port output data"]
pub type Odt0R = crate::BitReader;
#[doc = "Field `ODT0` writer - Port output data"]
pub type Odt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT1` reader - Port output data"]
pub type Odt1R = crate::BitReader;
#[doc = "Field `ODT1` writer - Port output data"]
pub type Odt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT2` reader - Port output data"]
pub type Odt2R = crate::BitReader;
#[doc = "Field `ODT2` writer - Port output data"]
pub type Odt2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT3` reader - Port output data"]
pub type Odt3R = crate::BitReader;
#[doc = "Field `ODT3` writer - Port output data"]
pub type Odt3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT4` reader - Port output data"]
pub type Odt4R = crate::BitReader;
#[doc = "Field `ODT4` writer - Port output data"]
pub type Odt4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT5` reader - Port output data"]
pub type Odt5R = crate::BitReader;
#[doc = "Field `ODT5` writer - Port output data"]
pub type Odt5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT6` reader - Port output data"]
pub type Odt6R = crate::BitReader;
#[doc = "Field `ODT6` writer - Port output data"]
pub type Odt6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT7` reader - Port output data"]
pub type Odt7R = crate::BitReader;
#[doc = "Field `ODT7` writer - Port output data"]
pub type Odt7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT8` reader - Port output data"]
pub type Odt8R = crate::BitReader;
#[doc = "Field `ODT8` writer - Port output data"]
pub type Odt8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT9` reader - Port output data"]
pub type Odt9R = crate::BitReader;
#[doc = "Field `ODT9` writer - Port output data"]
pub type Odt9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT10` reader - Port output data"]
pub type Odt10R = crate::BitReader;
#[doc = "Field `ODT10` writer - Port output data"]
pub type Odt10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT11` reader - Port output data"]
pub type Odt11R = crate::BitReader;
#[doc = "Field `ODT11` writer - Port output data"]
pub type Odt11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT12` reader - Port output data"]
pub type Odt12R = crate::BitReader;
#[doc = "Field `ODT12` writer - Port output data"]
pub type Odt12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT13` reader - Port output data"]
pub type Odt13R = crate::BitReader;
#[doc = "Field `ODT13` writer - Port output data"]
pub type Odt13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT14` reader - Port output data"]
pub type Odt14R = crate::BitReader;
#[doc = "Field `ODT14` writer - Port output data"]
pub type Odt14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODT15` reader - Port output data"]
pub type Odt15R = crate::BitReader;
#[doc = "Field `ODT15` writer - Port output data"]
pub type Odt15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    pub fn odt0(&self) -> Odt0R {
        Odt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    pub fn odt1(&self) -> Odt1R {
        Odt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    pub fn odt2(&self) -> Odt2R {
        Odt2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    pub fn odt3(&self) -> Odt3R {
        Odt3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    pub fn odt4(&self) -> Odt4R {
        Odt4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    pub fn odt5(&self) -> Odt5R {
        Odt5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    pub fn odt6(&self) -> Odt6R {
        Odt6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    pub fn odt7(&self) -> Odt7R {
        Odt7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    pub fn odt8(&self) -> Odt8R {
        Odt8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    pub fn odt9(&self) -> Odt9R {
        Odt9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    pub fn odt10(&self) -> Odt10R {
        Odt10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    pub fn odt11(&self) -> Odt11R {
        Odt11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    pub fn odt12(&self) -> Odt12R {
        Odt12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    pub fn odt13(&self) -> Odt13R {
        Odt13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    pub fn odt14(&self) -> Odt14R {
        Odt14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    pub fn odt15(&self) -> Odt15R {
        Odt15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODT")
            .field("odt0", &self.odt0())
            .field("odt1", &self.odt1())
            .field("odt2", &self.odt2())
            .field("odt3", &self.odt3())
            .field("odt4", &self.odt4())
            .field("odt5", &self.odt5())
            .field("odt6", &self.odt6())
            .field("odt7", &self.odt7())
            .field("odt8", &self.odt8())
            .field("odt9", &self.odt9())
            .field("odt10", &self.odt10())
            .field("odt11", &self.odt11())
            .field("odt12", &self.odt12())
            .field("odt13", &self.odt13())
            .field("odt14", &self.odt14())
            .field("odt15", &self.odt15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt0(&mut self) -> Odt0W<OdtSpec> {
        Odt0W::new(self, 0)
    }
    #[doc = "Bit 1 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt1(&mut self) -> Odt1W<OdtSpec> {
        Odt1W::new(self, 1)
    }
    #[doc = "Bit 2 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt2(&mut self) -> Odt2W<OdtSpec> {
        Odt2W::new(self, 2)
    }
    #[doc = "Bit 3 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt3(&mut self) -> Odt3W<OdtSpec> {
        Odt3W::new(self, 3)
    }
    #[doc = "Bit 4 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt4(&mut self) -> Odt4W<OdtSpec> {
        Odt4W::new(self, 4)
    }
    #[doc = "Bit 5 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt5(&mut self) -> Odt5W<OdtSpec> {
        Odt5W::new(self, 5)
    }
    #[doc = "Bit 6 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt6(&mut self) -> Odt6W<OdtSpec> {
        Odt6W::new(self, 6)
    }
    #[doc = "Bit 7 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt7(&mut self) -> Odt7W<OdtSpec> {
        Odt7W::new(self, 7)
    }
    #[doc = "Bit 8 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt8(&mut self) -> Odt8W<OdtSpec> {
        Odt8W::new(self, 8)
    }
    #[doc = "Bit 9 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt9(&mut self) -> Odt9W<OdtSpec> {
        Odt9W::new(self, 9)
    }
    #[doc = "Bit 10 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt10(&mut self) -> Odt10W<OdtSpec> {
        Odt10W::new(self, 10)
    }
    #[doc = "Bit 11 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt11(&mut self) -> Odt11W<OdtSpec> {
        Odt11W::new(self, 11)
    }
    #[doc = "Bit 12 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt12(&mut self) -> Odt12W<OdtSpec> {
        Odt12W::new(self, 12)
    }
    #[doc = "Bit 13 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt13(&mut self) -> Odt13W<OdtSpec> {
        Odt13W::new(self, 13)
    }
    #[doc = "Bit 14 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt14(&mut self) -> Odt14W<OdtSpec> {
        Odt14W::new(self, 14)
    }
    #[doc = "Bit 15 - Port output data"]
    #[inline(always)]
    #[must_use]
    pub fn odt15(&mut self) -> Odt15W<OdtSpec> {
        Odt15W::new(self, 15)
    }
}
#[doc = "Port output data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdtSpec;
impl crate::RegisterSpec for OdtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odt::R`](R) reader structure"]
impl crate::Readable for OdtSpec {}
#[doc = "`write(|w| ..)` method takes [`odt::W`](W) writer structure"]
impl crate::Writable for OdtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODT to value 0"]
impl crate::Resettable for OdtSpec {
    const RESET_VALUE: u32 = 0;
}
