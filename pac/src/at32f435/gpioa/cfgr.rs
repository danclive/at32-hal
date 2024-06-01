#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `IOMC0` reader - GPIOx pin 0 mode configurate"]
pub type Iomc0R = crate::FieldReader;
#[doc = "Field `IOMC0` writer - GPIOx pin 0 mode configurate"]
pub type Iomc0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC1` reader - GPIOx pin 1 mode configurate"]
pub type Iomc1R = crate::FieldReader;
#[doc = "Field `IOMC1` writer - GPIOx pin 1 mode configurate"]
pub type Iomc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC2` reader - GPIOx pin 2 mode configurate"]
pub type Iomc2R = crate::FieldReader;
#[doc = "Field `IOMC2` writer - GPIOx pin 2 mode configurate"]
pub type Iomc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC3` reader - GPIOx pin 3 mode configurate"]
pub type Iomc3R = crate::FieldReader;
#[doc = "Field `IOMC3` writer - GPIOx pin 3 mode configurate"]
pub type Iomc3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC4` reader - GPIOx pin 4 mode configurate"]
pub type Iomc4R = crate::FieldReader;
#[doc = "Field `IOMC4` writer - GPIOx pin 4 mode configurate"]
pub type Iomc4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC5` reader - GPIOx pin 5 mode configurate"]
pub type Iomc5R = crate::FieldReader;
#[doc = "Field `IOMC5` writer - GPIOx pin 5 mode configurate"]
pub type Iomc5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC6` reader - GPIOx pin 6 mode configurate"]
pub type Iomc6R = crate::FieldReader;
#[doc = "Field `IOMC6` writer - GPIOx pin 6 mode configurate"]
pub type Iomc6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC7` reader - GPIOx pin 7 mode configurate"]
pub type Iomc7R = crate::FieldReader;
#[doc = "Field `IOMC7` writer - GPIOx pin 7 mode configurate"]
pub type Iomc7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC8` reader - GPIOx pin 8 mode configurate"]
pub type Iomc8R = crate::FieldReader;
#[doc = "Field `IOMC8` writer - GPIOx pin 8 mode configurate"]
pub type Iomc8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC9` reader - GPIOx pin 9 mode configurate"]
pub type Iomc9R = crate::FieldReader;
#[doc = "Field `IOMC9` writer - GPIOx pin 9 mode configurate"]
pub type Iomc9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC10` reader - GPIOx pin 10 mode configurate"]
pub type Iomc10R = crate::FieldReader;
#[doc = "Field `IOMC10` writer - GPIOx pin 10 mode configurate"]
pub type Iomc10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC11` reader - GPIOx pin 11 mode configurate"]
pub type Iomc11R = crate::FieldReader;
#[doc = "Field `IOMC11` writer - GPIOx pin 11 mode configurate"]
pub type Iomc11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC12` reader - GPIOx pin 12 mode configurate"]
pub type Iomc12R = crate::FieldReader;
#[doc = "Field `IOMC12` writer - GPIOx pin 12 mode configurate"]
pub type Iomc12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC13` reader - GPIOx pin 13 mode configurate"]
pub type Iomc13R = crate::FieldReader;
#[doc = "Field `IOMC13` writer - GPIOx pin 13 mode configurate"]
pub type Iomc13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC14` reader - GPIOx pin 14 mode configurate"]
pub type Iomc14R = crate::FieldReader;
#[doc = "Field `IOMC14` writer - GPIOx pin 14 mode configurate"]
pub type Iomc14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC15` reader - GPIOx pin 15 mode configurate"]
pub type Iomc15R = crate::FieldReader;
#[doc = "Field `IOMC15` writer - GPIOx pin 15 mode configurate"]
pub type Iomc15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    pub fn iomc0(&self) -> Iomc0R {
        Iomc0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    pub fn iomc1(&self) -> Iomc1R {
        Iomc1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    pub fn iomc2(&self) -> Iomc2R {
        Iomc2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    pub fn iomc3(&self) -> Iomc3R {
        Iomc3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    pub fn iomc4(&self) -> Iomc4R {
        Iomc4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    pub fn iomc5(&self) -> Iomc5R {
        Iomc5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    pub fn iomc6(&self) -> Iomc6R {
        Iomc6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    pub fn iomc7(&self) -> Iomc7R {
        Iomc7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    pub fn iomc8(&self) -> Iomc8R {
        Iomc8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    pub fn iomc9(&self) -> Iomc9R {
        Iomc9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    pub fn iomc10(&self) -> Iomc10R {
        Iomc10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    pub fn iomc11(&self) -> Iomc11R {
        Iomc11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    pub fn iomc12(&self) -> Iomc12R {
        Iomc12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    pub fn iomc13(&self) -> Iomc13R {
        Iomc13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    pub fn iomc14(&self) -> Iomc14R {
        Iomc14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    pub fn iomc15(&self) -> Iomc15R {
        Iomc15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("iomc15", &self.iomc15())
            .field("iomc14", &self.iomc14())
            .field("iomc13", &self.iomc13())
            .field("iomc12", &self.iomc12())
            .field("iomc11", &self.iomc11())
            .field("iomc10", &self.iomc10())
            .field("iomc9", &self.iomc9())
            .field("iomc8", &self.iomc8())
            .field("iomc7", &self.iomc7())
            .field("iomc6", &self.iomc6())
            .field("iomc5", &self.iomc5())
            .field("iomc4", &self.iomc4())
            .field("iomc3", &self.iomc3())
            .field("iomc2", &self.iomc2())
            .field("iomc1", &self.iomc1())
            .field("iomc0", &self.iomc0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIOx pin 0 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> Iomc0W<CfgrSpec> {
        Iomc0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIOx pin 1 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> Iomc1W<CfgrSpec> {
        Iomc1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIOx pin 2 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> Iomc2W<CfgrSpec> {
        Iomc2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIOx pin 3 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> Iomc3W<CfgrSpec> {
        Iomc3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIOx pin 4 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> Iomc4W<CfgrSpec> {
        Iomc4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIOx pin 5 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> Iomc5W<CfgrSpec> {
        Iomc5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIOx pin 6 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> Iomc6W<CfgrSpec> {
        Iomc6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIOx pin 7 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> Iomc7W<CfgrSpec> {
        Iomc7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - GPIOx pin 8 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> Iomc8W<CfgrSpec> {
        Iomc8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - GPIOx pin 9 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> Iomc9W<CfgrSpec> {
        Iomc9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - GPIOx pin 10 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> Iomc10W<CfgrSpec> {
        Iomc10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - GPIOx pin 11 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> Iomc11W<CfgrSpec> {
        Iomc11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - GPIOx pin 12 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> Iomc12W<CfgrSpec> {
        Iomc12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - GPIOx pin 13 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> Iomc13W<CfgrSpec> {
        Iomc13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - GPIOx pin 14 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> Iomc14W<CfgrSpec> {
        Iomc14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - GPIOx pin 15 mode configurate"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> Iomc15W<CfgrSpec> {
        Iomc15W::new(self, 30)
    }
}
#[doc = "GPIO configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {
    const RESET_VALUE: u32 = 0;
}
