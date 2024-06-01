#[doc = "Register `CFGLR` reader"]
pub type R = crate::R<CfglrSpec>;
#[doc = "Register `CFGLR` writer"]
pub type W = crate::W<CfglrSpec>;
#[doc = "Field `IOMC0` reader - Port n.0 mode configurate bits"]
pub type Iomc0R = crate::FieldReader;
#[doc = "Field `IOMC0` writer - Port n.0 mode configurate bits"]
pub type Iomc0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC0` reader - Port n.0 function configurate bits"]
pub type Iofc0R = crate::FieldReader;
#[doc = "Field `IOFC0` writer - Port n.0 function configurate bits"]
pub type Iofc0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC1` reader - Port n.1 mode configurate bits"]
pub type Iomc1R = crate::FieldReader;
#[doc = "Field `IOMC1` writer - Port n.1 mode configurate bits"]
pub type Iomc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC1` reader - Port n.1 function configurate bits"]
pub type Iofc1R = crate::FieldReader;
#[doc = "Field `IOFC1` writer - Port n.1 function configurate bits"]
pub type Iofc1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC2` reader - Port n.2 mode configurate bits"]
pub type Iomc2R = crate::FieldReader;
#[doc = "Field `IOMC2` writer - Port n.2 mode configurate bits"]
pub type Iomc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC2` reader - Port n.2 function configurate bits"]
pub type Iofc2R = crate::FieldReader;
#[doc = "Field `IOFC2` writer - Port n.2 function configurate bits"]
pub type Iofc2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC3` reader - Port n.3 mode configurate bits"]
pub type Iomc3R = crate::FieldReader;
#[doc = "Field `IOMC3` writer - Port n.3 mode configurate bits"]
pub type Iomc3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC3` reader - Port n.3 function configurate bits"]
pub type Iofc3R = crate::FieldReader;
#[doc = "Field `IOFC3` writer - Port n.3 function configurate bits"]
pub type Iofc3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC4` reader - Port n.4 mode configurate bits"]
pub type Iomc4R = crate::FieldReader;
#[doc = "Field `IOMC4` writer - Port n.4 mode configurate bits"]
pub type Iomc4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC4` reader - Port n.4 function configurate bits"]
pub type Iofc4R = crate::FieldReader;
#[doc = "Field `IOFC4` writer - Port n.4 function configurate bits"]
pub type Iofc4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC5` reader - Port n.5 mode configurate bits"]
pub type Iomc5R = crate::FieldReader;
#[doc = "Field `IOMC5` writer - Port n.5 mode configurate bits"]
pub type Iomc5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC5` reader - Port n.5 function configurate bits"]
pub type Iofc5R = crate::FieldReader;
#[doc = "Field `IOFC5` writer - Port n.5 function configurate bits"]
pub type Iofc5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC6` reader - Port n.6 mode configurate bits"]
pub type Iomc6R = crate::FieldReader;
#[doc = "Field `IOMC6` writer - Port n.6 mode configurate bits"]
pub type Iomc6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC6` reader - Port n.6 function configurate bits"]
pub type Iofc6R = crate::FieldReader;
#[doc = "Field `IOFC6` writer - Port n.6 function configurate bits"]
pub type Iofc6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC7` reader - Port n.7 mode configurate bits"]
pub type Iomc7R = crate::FieldReader;
#[doc = "Field `IOMC7` writer - Port n.7 mode configurate bits"]
pub type Iomc7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC7` reader - Port n.7 function configurate bits"]
pub type Iofc7R = crate::FieldReader;
#[doc = "Field `IOFC7` writer - Port n.7 function configurate bits"]
pub type Iofc7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    pub fn iomc0(&self) -> Iomc0R {
        Iomc0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    pub fn iofc0(&self) -> Iofc0R {
        Iofc0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    pub fn iomc1(&self) -> Iomc1R {
        Iomc1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    pub fn iofc1(&self) -> Iofc1R {
        Iofc1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    pub fn iomc2(&self) -> Iomc2R {
        Iomc2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    pub fn iofc2(&self) -> Iofc2R {
        Iofc2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    pub fn iomc3(&self) -> Iomc3R {
        Iomc3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    pub fn iofc3(&self) -> Iofc3R {
        Iofc3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    pub fn iomc4(&self) -> Iomc4R {
        Iomc4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    pub fn iofc4(&self) -> Iofc4R {
        Iofc4R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    pub fn iomc5(&self) -> Iomc5R {
        Iomc5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    pub fn iofc5(&self) -> Iofc5R {
        Iofc5R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    pub fn iomc6(&self) -> Iomc6R {
        Iomc6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    pub fn iofc6(&self) -> Iofc6R {
        Iofc6R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    pub fn iomc7(&self) -> Iomc7R {
        Iomc7R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    pub fn iofc7(&self) -> Iofc7R {
        Iofc7R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGLR")
            .field("iomc0", &self.iomc0())
            .field("iofc0", &self.iofc0())
            .field("iomc1", &self.iomc1())
            .field("iofc1", &self.iofc1())
            .field("iomc2", &self.iomc2())
            .field("iofc2", &self.iofc2())
            .field("iomc3", &self.iomc3())
            .field("iofc3", &self.iofc3())
            .field("iomc4", &self.iomc4())
            .field("iofc4", &self.iofc4())
            .field("iomc5", &self.iomc5())
            .field("iofc5", &self.iofc5())
            .field("iomc6", &self.iomc6())
            .field("iofc6", &self.iofc6())
            .field("iomc7", &self.iomc7())
            .field("iofc7", &self.iofc7())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.0 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc0(&mut self) -> Iomc0W<CfglrSpec> {
        Iomc0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port n.0 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc0(&mut self) -> Iofc0W<CfglrSpec> {
        Iofc0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port n.1 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc1(&mut self) -> Iomc1W<CfglrSpec> {
        Iomc1W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port n.1 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc1(&mut self) -> Iofc1W<CfglrSpec> {
        Iofc1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port n.2 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc2(&mut self) -> Iomc2W<CfglrSpec> {
        Iomc2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port n.2 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc2(&mut self) -> Iofc2W<CfglrSpec> {
        Iofc2W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port n.3 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc3(&mut self) -> Iomc3W<CfglrSpec> {
        Iomc3W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port n.3 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc3(&mut self) -> Iofc3W<CfglrSpec> {
        Iofc3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port n.4 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc4(&mut self) -> Iomc4W<CfglrSpec> {
        Iomc4W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port n.4 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc4(&mut self) -> Iofc4W<CfglrSpec> {
        Iofc4W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port n.5 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc5(&mut self) -> Iomc5W<CfglrSpec> {
        Iomc5W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port n.5 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc5(&mut self) -> Iofc5W<CfglrSpec> {
        Iofc5W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port n.6 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc6(&mut self) -> Iomc6W<CfglrSpec> {
        Iomc6W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port n.6 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc6(&mut self) -> Iofc6W<CfglrSpec> {
        Iofc6W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port n.7 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc7(&mut self) -> Iomc7W<CfglrSpec> {
        Iomc7W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port n.7 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc7(&mut self) -> Iofc7W<CfglrSpec> {
        Iofc7W::new(self, 30)
    }
}
#[doc = "GPIO function configurate low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfglr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfglr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfglrSpec;
impl crate::RegisterSpec for CfglrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfglr::R`](R) reader structure"]
impl crate::Readable for CfglrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfglr::W`](W) writer structure"]
impl crate::Writable for CfglrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGLR to value 0x4444_4444"]
impl crate::Resettable for CfglrSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
