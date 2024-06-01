#[doc = "Register `ODRVR` reader"]
pub type R = crate::R<OdrvrSpec>;
#[doc = "Register `ODRVR` writer"]
pub type W = crate::W<OdrvrSpec>;
#[doc = "Field `ODRV0` reader - GPIOx pin 0 output drive capability"]
pub type Odrv0R = crate::FieldReader;
#[doc = "Field `ODRV0` writer - GPIOx pin 0 output drive capability"]
pub type Odrv0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV1` reader - GPIOx pin 1 output drive capability"]
pub type Odrv1R = crate::FieldReader;
#[doc = "Field `ODRV1` writer - GPIOx pin 1 output drive capability"]
pub type Odrv1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV2` reader - GPIOx pin 2 output drive capability"]
pub type Odrv2R = crate::FieldReader;
#[doc = "Field `ODRV2` writer - GPIOx pin 2 output drive capability"]
pub type Odrv2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV3` reader - GPIOx pin 3 output drive capability"]
pub type Odrv3R = crate::FieldReader;
#[doc = "Field `ODRV3` writer - GPIOx pin 3 output drive capability"]
pub type Odrv3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV4` reader - GPIOx pin 4 output drive capability"]
pub type Odrv4R = crate::FieldReader;
#[doc = "Field `ODRV4` writer - GPIOx pin 4 output drive capability"]
pub type Odrv4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV5` reader - GPIOx pin 5 output drive capability"]
pub type Odrv5R = crate::FieldReader;
#[doc = "Field `ODRV5` writer - GPIOx pin 5 output drive capability"]
pub type Odrv5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV6` reader - GPIOx pin 6 output drive capability"]
pub type Odrv6R = crate::FieldReader;
#[doc = "Field `ODRV6` writer - GPIOx pin 6 output drive capability"]
pub type Odrv6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV7` reader - GPIOx pin 7 output drive capability"]
pub type Odrv7R = crate::FieldReader;
#[doc = "Field `ODRV7` writer - GPIOx pin 7 output drive capability"]
pub type Odrv7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV8` reader - GPIOx pin 8 output drive capability"]
pub type Odrv8R = crate::FieldReader;
#[doc = "Field `ODRV8` writer - GPIOx pin 8 output drive capability"]
pub type Odrv8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV9` reader - GPIOx pin 9 output drive capability"]
pub type Odrv9R = crate::FieldReader;
#[doc = "Field `ODRV9` writer - GPIOx pin 9 output drive capability"]
pub type Odrv9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV10` reader - GPIOx pin 10 output drive capability"]
pub type Odrv10R = crate::FieldReader;
#[doc = "Field `ODRV10` writer - GPIOx pin 10 output drive capability"]
pub type Odrv10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV11` reader - GPIOx pin 11 output drive capability"]
pub type Odrv11R = crate::FieldReader;
#[doc = "Field `ODRV11` writer - GPIOx pin 11 output drive capability"]
pub type Odrv11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV12` reader - GPIOx pin 12 output drive capability"]
pub type Odrv12R = crate::FieldReader;
#[doc = "Field `ODRV12` writer - GPIOx pin 12 output drive capability"]
pub type Odrv12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV13` reader - GPIOx pin 13 output drive capability"]
pub type Odrv13R = crate::FieldReader;
#[doc = "Field `ODRV13` writer - GPIOx pin 13 output drive capability"]
pub type Odrv13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV14` reader - GPIOx pin 14 output drive capability"]
pub type Odrv14R = crate::FieldReader;
#[doc = "Field `ODRV14` writer - GPIOx pin 14 output drive capability"]
pub type Odrv14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ODRV15` reader - GPIOx pin 15 output drive capability"]
pub type Odrv15R = crate::FieldReader;
#[doc = "Field `ODRV15` writer - GPIOx pin 15 output drive capability"]
pub type Odrv15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    pub fn odrv0(&self) -> Odrv0R {
        Odrv0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    pub fn odrv1(&self) -> Odrv1R {
        Odrv1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    pub fn odrv2(&self) -> Odrv2R {
        Odrv2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    pub fn odrv3(&self) -> Odrv3R {
        Odrv3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    pub fn odrv4(&self) -> Odrv4R {
        Odrv4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    pub fn odrv5(&self) -> Odrv5R {
        Odrv5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    pub fn odrv6(&self) -> Odrv6R {
        Odrv6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    pub fn odrv7(&self) -> Odrv7R {
        Odrv7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    pub fn odrv8(&self) -> Odrv8R {
        Odrv8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    pub fn odrv9(&self) -> Odrv9R {
        Odrv9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    pub fn odrv10(&self) -> Odrv10R {
        Odrv10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    pub fn odrv11(&self) -> Odrv11R {
        Odrv11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    pub fn odrv12(&self) -> Odrv12R {
        Odrv12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    pub fn odrv13(&self) -> Odrv13R {
        Odrv13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    pub fn odrv14(&self) -> Odrv14R {
        Odrv14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    pub fn odrv15(&self) -> Odrv15R {
        Odrv15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ODRVR")
            .field("odrv15", &self.odrv15())
            .field("odrv14", &self.odrv14())
            .field("odrv13", &self.odrv13())
            .field("odrv12", &self.odrv12())
            .field("odrv11", &self.odrv11())
            .field("odrv10", &self.odrv10())
            .field("odrv9", &self.odrv9())
            .field("odrv8", &self.odrv8())
            .field("odrv7", &self.odrv7())
            .field("odrv6", &self.odrv6())
            .field("odrv5", &self.odrv5())
            .field("odrv4", &self.odrv4())
            .field("odrv3", &self.odrv3())
            .field("odrv2", &self.odrv2())
            .field("odrv1", &self.odrv1())
            .field("odrv0", &self.odrv0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv0(&mut self) -> Odrv0W<OdrvrSpec> {
        Odrv0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv1(&mut self) -> Odrv1W<OdrvrSpec> {
        Odrv1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv2(&mut self) -> Odrv2W<OdrvrSpec> {
        Odrv2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv3(&mut self) -> Odrv3W<OdrvrSpec> {
        Odrv3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv4(&mut self) -> Odrv4W<OdrvrSpec> {
        Odrv4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv5(&mut self) -> Odrv5W<OdrvrSpec> {
        Odrv5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv6(&mut self) -> Odrv6W<OdrvrSpec> {
        Odrv6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv7(&mut self) -> Odrv7W<OdrvrSpec> {
        Odrv7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv8(&mut self) -> Odrv8W<OdrvrSpec> {
        Odrv8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv9(&mut self) -> Odrv9W<OdrvrSpec> {
        Odrv9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv10(&mut self) -> Odrv10W<OdrvrSpec> {
        Odrv10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv11(&mut self) -> Odrv11W<OdrvrSpec> {
        Odrv11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv12(&mut self) -> Odrv12W<OdrvrSpec> {
        Odrv12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv13(&mut self) -> Odrv13W<OdrvrSpec> {
        Odrv13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv14(&mut self) -> Odrv14W<OdrvrSpec> {
        Odrv14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 output drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn odrv15(&mut self) -> Odrv15W<OdrvrSpec> {
        Odrv15W::new(self, 30)
    }
}
#[doc = "GPIO drive capability register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`odrvr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`odrvr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrvrSpec;
impl crate::RegisterSpec for OdrvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odrvr::R`](R) reader structure"]
impl crate::Readable for OdrvrSpec {}
#[doc = "`write(|w| ..)` method takes [`odrvr::W`](W) writer structure"]
impl crate::Writable for OdrvrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ODRVR to value 0"]
impl crate::Resettable for OdrvrSpec {
    const RESET_VALUE: u32 = 0;
}
