#[doc = "Register `CLR` reader"]
pub type R = crate::R<ClrSpec>;
#[doc = "Register `CLR` writer"]
pub type W = crate::W<ClrSpec>;
#[doc = "Field `IOCB0` reader - Clear bit 0"]
pub type Iocb0R = crate::BitReader;
#[doc = "Field `IOCB0` writer - Clear bit 0"]
pub type Iocb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB1` reader - Clear bit 1"]
pub type Iocb1R = crate::BitReader;
#[doc = "Field `IOCB1` writer - Clear bit 1"]
pub type Iocb1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB2` reader - Clear bit 1"]
pub type Iocb2R = crate::BitReader;
#[doc = "Field `IOCB2` writer - Clear bit 1"]
pub type Iocb2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB3` reader - Clear bit 3"]
pub type Iocb3R = crate::BitReader;
#[doc = "Field `IOCB3` writer - Clear bit 3"]
pub type Iocb3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB4` reader - Clear bit 4"]
pub type Iocb4R = crate::BitReader;
#[doc = "Field `IOCB4` writer - Clear bit 4"]
pub type Iocb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB5` reader - Clear bit 5"]
pub type Iocb5R = crate::BitReader;
#[doc = "Field `IOCB5` writer - Clear bit 5"]
pub type Iocb5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB6` reader - Clear bit 6"]
pub type Iocb6R = crate::BitReader;
#[doc = "Field `IOCB6` writer - Clear bit 6"]
pub type Iocb6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB7` reader - Clear bit 7"]
pub type Iocb7R = crate::BitReader;
#[doc = "Field `IOCB7` writer - Clear bit 7"]
pub type Iocb7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB8` reader - Clear bit 8"]
pub type Iocb8R = crate::BitReader;
#[doc = "Field `IOCB8` writer - Clear bit 8"]
pub type Iocb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB9` reader - Clear bit 9"]
pub type Iocb9R = crate::BitReader;
#[doc = "Field `IOCB9` writer - Clear bit 9"]
pub type Iocb9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB10` reader - Clear bit 10"]
pub type Iocb10R = crate::BitReader;
#[doc = "Field `IOCB10` writer - Clear bit 10"]
pub type Iocb10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB11` reader - Clear bit 11"]
pub type Iocb11R = crate::BitReader;
#[doc = "Field `IOCB11` writer - Clear bit 11"]
pub type Iocb11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB12` reader - Clear bit 12"]
pub type Iocb12R = crate::BitReader;
#[doc = "Field `IOCB12` writer - Clear bit 12"]
pub type Iocb12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB13` reader - Clear bit 13"]
pub type Iocb13R = crate::BitReader;
#[doc = "Field `IOCB13` writer - Clear bit 13"]
pub type Iocb13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB14` reader - Clear bit 14"]
pub type Iocb14R = crate::BitReader;
#[doc = "Field `IOCB14` writer - Clear bit 14"]
pub type Iocb14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOCB15` reader - Clear bit 15"]
pub type Iocb15R = crate::BitReader;
#[doc = "Field `IOCB15` writer - Clear bit 15"]
pub type Iocb15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    pub fn iocb0(&self) -> Iocb0R {
        Iocb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    pub fn iocb1(&self) -> Iocb1R {
        Iocb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear bit 1"]
    #[inline(always)]
    pub fn iocb2(&self) -> Iocb2R {
        Iocb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    pub fn iocb3(&self) -> Iocb3R {
        Iocb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    pub fn iocb4(&self) -> Iocb4R {
        Iocb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    pub fn iocb5(&self) -> Iocb5R {
        Iocb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    pub fn iocb6(&self) -> Iocb6R {
        Iocb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    pub fn iocb7(&self) -> Iocb7R {
        Iocb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    pub fn iocb8(&self) -> Iocb8R {
        Iocb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    pub fn iocb9(&self) -> Iocb9R {
        Iocb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    pub fn iocb10(&self) -> Iocb10R {
        Iocb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    pub fn iocb11(&self) -> Iocb11R {
        Iocb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    pub fn iocb12(&self) -> Iocb12R {
        Iocb12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    pub fn iocb13(&self) -> Iocb13R {
        Iocb13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    pub fn iocb14(&self) -> Iocb14R {
        Iocb14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    pub fn iocb15(&self) -> Iocb15R {
        Iocb15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLR")
            .field("iocb0", &self.iocb0())
            .field("iocb1", &self.iocb1())
            .field("iocb2", &self.iocb2())
            .field("iocb3", &self.iocb3())
            .field("iocb4", &self.iocb4())
            .field("iocb5", &self.iocb5())
            .field("iocb6", &self.iocb6())
            .field("iocb7", &self.iocb7())
            .field("iocb8", &self.iocb8())
            .field("iocb9", &self.iocb9())
            .field("iocb10", &self.iocb10())
            .field("iocb11", &self.iocb11())
            .field("iocb12", &self.iocb12())
            .field("iocb13", &self.iocb13())
            .field("iocb14", &self.iocb14())
            .field("iocb15", &self.iocb15())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn iocb0(&mut self) -> Iocb0W<ClrSpec> {
        Iocb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb1(&mut self) -> Iocb1W<ClrSpec> {
        Iocb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn iocb2(&mut self) -> Iocb2W<ClrSpec> {
        Iocb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn iocb3(&mut self) -> Iocb3W<ClrSpec> {
        Iocb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn iocb4(&mut self) -> Iocb4W<ClrSpec> {
        Iocb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn iocb5(&mut self) -> Iocb5W<ClrSpec> {
        Iocb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn iocb6(&mut self) -> Iocb6W<ClrSpec> {
        Iocb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn iocb7(&mut self) -> Iocb7W<ClrSpec> {
        Iocb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn iocb8(&mut self) -> Iocb8W<ClrSpec> {
        Iocb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn iocb9(&mut self) -> Iocb9W<ClrSpec> {
        Iocb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn iocb10(&mut self) -> Iocb10W<ClrSpec> {
        Iocb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn iocb11(&mut self) -> Iocb11W<ClrSpec> {
        Iocb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn iocb12(&mut self) -> Iocb12W<ClrSpec> {
        Iocb12W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn iocb13(&mut self) -> Iocb13W<ClrSpec> {
        Iocb13W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn iocb14(&mut self) -> Iocb14W<ClrSpec> {
        Iocb14W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn iocb15(&mut self) -> Iocb15W<ClrSpec> {
        Iocb15W::new(self, 15)
    }
}
#[doc = "Port bit reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrSpec;
impl crate::RegisterSpec for ClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr::R`](R) reader structure"]
impl crate::Readable for ClrSpec {}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for ClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for ClrSpec {
    const RESET_VALUE: u32 = 0;
}
