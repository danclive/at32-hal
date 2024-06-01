#[doc = "Register `FRF` reader"]
pub type R = crate::R<FrfSpec>;
#[doc = "Register `FRF` writer"]
pub type W = crate::W<FrfSpec>;
#[doc = "Field `FRFSEL0` reader - Filter relation FIFO select"]
pub type Frfsel0R = crate::BitReader;
#[doc = "Field `FRFSEL0` writer - Filter relation FIFO select"]
pub type Frfsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL1` reader - Filter relation FIFO select"]
pub type Frfsel1R = crate::BitReader;
#[doc = "Field `FRFSEL1` writer - Filter relation FIFO select"]
pub type Frfsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL2` reader - Filter relation FIFO select"]
pub type Frfsel2R = crate::BitReader;
#[doc = "Field `FRFSEL2` writer - Filter relation FIFO select"]
pub type Frfsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL3` reader - Filter relation FIFO select"]
pub type Frfsel3R = crate::BitReader;
#[doc = "Field `FRFSEL3` writer - Filter relation FIFO select"]
pub type Frfsel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL4` reader - Filter relation FIFO select"]
pub type Frfsel4R = crate::BitReader;
#[doc = "Field `FRFSEL4` writer - Filter relation FIFO select"]
pub type Frfsel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL5` reader - Filter relation FIFO select"]
pub type Frfsel5R = crate::BitReader;
#[doc = "Field `FRFSEL5` writer - Filter relation FIFO select"]
pub type Frfsel5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL6` reader - Filter relation FIFO select"]
pub type Frfsel6R = crate::BitReader;
#[doc = "Field `FRFSEL6` writer - Filter relation FIFO select"]
pub type Frfsel6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL7` reader - Filter relation FIFO select"]
pub type Frfsel7R = crate::BitReader;
#[doc = "Field `FRFSEL7` writer - Filter relation FIFO select"]
pub type Frfsel7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL8` reader - Filter relation FIFO select"]
pub type Frfsel8R = crate::BitReader;
#[doc = "Field `FRFSEL8` writer - Filter relation FIFO select"]
pub type Frfsel8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL9` reader - Filter relation FIFO select"]
pub type Frfsel9R = crate::BitReader;
#[doc = "Field `FRFSEL9` writer - Filter relation FIFO select"]
pub type Frfsel9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL10` reader - Filter relation FIFO select"]
pub type Frfsel10R = crate::BitReader;
#[doc = "Field `FRFSEL10` writer - Filter relation FIFO select"]
pub type Frfsel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL11` reader - Filter relation FIFO select"]
pub type Frfsel11R = crate::BitReader;
#[doc = "Field `FRFSEL11` writer - Filter relation FIFO select"]
pub type Frfsel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL12` reader - Filter relation FIFO select"]
pub type Frfsel12R = crate::BitReader;
#[doc = "Field `FRFSEL12` writer - Filter relation FIFO select"]
pub type Frfsel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRFSEL13` reader - Filter relation FIFO select"]
pub type Frfsel13R = crate::BitReader;
#[doc = "Field `FRFSEL13` writer - Filter relation FIFO select"]
pub type Frfsel13W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel0(&self) -> Frfsel0R {
        Frfsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel1(&self) -> Frfsel1R {
        Frfsel1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel2(&self) -> Frfsel2R {
        Frfsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel3(&self) -> Frfsel3R {
        Frfsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel4(&self) -> Frfsel4R {
        Frfsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel5(&self) -> Frfsel5R {
        Frfsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel6(&self) -> Frfsel6R {
        Frfsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel7(&self) -> Frfsel7R {
        Frfsel7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel8(&self) -> Frfsel8R {
        Frfsel8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel9(&self) -> Frfsel9R {
        Frfsel9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel10(&self) -> Frfsel10R {
        Frfsel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel11(&self) -> Frfsel11R {
        Frfsel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel12(&self) -> Frfsel12R {
        Frfsel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    pub fn frfsel13(&self) -> Frfsel13R {
        Frfsel13R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRF")
            .field("frfsel0", &self.frfsel0())
            .field("frfsel1", &self.frfsel1())
            .field("frfsel2", &self.frfsel2())
            .field("frfsel3", &self.frfsel3())
            .field("frfsel4", &self.frfsel4())
            .field("frfsel5", &self.frfsel5())
            .field("frfsel6", &self.frfsel6())
            .field("frfsel7", &self.frfsel7())
            .field("frfsel8", &self.frfsel8())
            .field("frfsel9", &self.frfsel9())
            .field("frfsel10", &self.frfsel10())
            .field("frfsel11", &self.frfsel11())
            .field("frfsel12", &self.frfsel12())
            .field("frfsel13", &self.frfsel13())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel0(&mut self) -> Frfsel0W<FrfSpec> {
        Frfsel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel1(&mut self) -> Frfsel1W<FrfSpec> {
        Frfsel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel2(&mut self) -> Frfsel2W<FrfSpec> {
        Frfsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel3(&mut self) -> Frfsel3W<FrfSpec> {
        Frfsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel4(&mut self) -> Frfsel4W<FrfSpec> {
        Frfsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel5(&mut self) -> Frfsel5W<FrfSpec> {
        Frfsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel6(&mut self) -> Frfsel6W<FrfSpec> {
        Frfsel6W::new(self, 6)
    }
    #[doc = "Bit 7 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel7(&mut self) -> Frfsel7W<FrfSpec> {
        Frfsel7W::new(self, 7)
    }
    #[doc = "Bit 8 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel8(&mut self) -> Frfsel8W<FrfSpec> {
        Frfsel8W::new(self, 8)
    }
    #[doc = "Bit 9 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel9(&mut self) -> Frfsel9W<FrfSpec> {
        Frfsel9W::new(self, 9)
    }
    #[doc = "Bit 10 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel10(&mut self) -> Frfsel10W<FrfSpec> {
        Frfsel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel11(&mut self) -> Frfsel11W<FrfSpec> {
        Frfsel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel12(&mut self) -> Frfsel12W<FrfSpec> {
        Frfsel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Filter relation FIFO select"]
    #[inline(always)]
    #[must_use]
    pub fn frfsel13(&mut self) -> Frfsel13W<FrfSpec> {
        Frfsel13W::new(self, 13)
    }
}
#[doc = "Filter related FIFO register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrfSpec;
impl crate::RegisterSpec for FrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frf::R`](R) reader structure"]
impl crate::Readable for FrfSpec {}
#[doc = "`write(|w| ..)` method takes [`frf::W`](W) writer structure"]
impl crate::Writable for FrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRF to value 0"]
impl crate::Resettable for FrfSpec {
    const RESET_VALUE: u32 = 0;
}
