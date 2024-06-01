#[doc = "Register `CFGHR` reader"]
pub type R = crate::R<CfghrSpec>;
#[doc = "Register `CFGHR` writer"]
pub type W = crate::W<CfghrSpec>;
#[doc = "Field `IOMC8` reader - Port n.8 mode configurate bits"]
pub type Iomc8R = crate::FieldReader;
#[doc = "Field `IOMC8` writer - Port n.8 mode configurate bits"]
pub type Iomc8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC8` reader - Port n.8 function configurate bits"]
pub type Iofc8R = crate::FieldReader;
#[doc = "Field `IOFC8` writer - Port n.8 function configurate bits"]
pub type Iofc8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC9` reader - Port n.9 mode configurate bits"]
pub type Iomc9R = crate::FieldReader;
#[doc = "Field `IOMC9` writer - Port n.9 mode configurate bits"]
pub type Iomc9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC9` reader - Port n.9 function configurate bits"]
pub type Iofc9R = crate::FieldReader;
#[doc = "Field `IOFC9` writer - Port n.9 function configurate bits"]
pub type Iofc9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC10` reader - Port n.10 mode configurate bits"]
pub type Iomc10R = crate::FieldReader;
#[doc = "Field `IOMC10` writer - Port n.10 mode configurate bits"]
pub type Iomc10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC10` reader - Port n.10 function configurate bits"]
pub type Iofc10R = crate::FieldReader;
#[doc = "Field `IOFC10` writer - Port n.10 function configurate bits"]
pub type Iofc10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC11` reader - Port n.11 mode configurate bits"]
pub type Iomc11R = crate::FieldReader;
#[doc = "Field `IOMC11` writer - Port n.11 mode configurate bits"]
pub type Iomc11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC11` reader - Port n.11 function configurate bits"]
pub type Iofc11R = crate::FieldReader;
#[doc = "Field `IOFC11` writer - Port n.11 function configurate bits"]
pub type Iofc11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC12` reader - Port n.12 mode configurate bits"]
pub type Iomc12R = crate::FieldReader;
#[doc = "Field `IOMC12` writer - Port n.12 mode configurate bits"]
pub type Iomc12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC12` reader - Port n.12 function configurate bits"]
pub type Iofc12R = crate::FieldReader;
#[doc = "Field `IOFC12` writer - Port n.12 function configurate bits"]
pub type Iofc12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC13` reader - Port n.13 mode configurate bits"]
pub type Iomc13R = crate::FieldReader;
#[doc = "Field `IOMC13` writer - Port n.13 mode configurate bits"]
pub type Iomc13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC13` reader - Port n.13 function configurate bits"]
pub type Iofc13R = crate::FieldReader;
#[doc = "Field `IOFC13` writer - Port n.13 function configurate bits"]
pub type Iofc13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC14` reader - Port n.14 mode configurate bits"]
pub type Iomc14R = crate::FieldReader;
#[doc = "Field `IOMC14` writer - Port n.14 mode configurate bits"]
pub type Iomc14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC14` reader - Port n.14 function configurate bits"]
pub type Iofc14R = crate::FieldReader;
#[doc = "Field `IOFC14` writer - Port n.14 function configurate bits"]
pub type Iofc14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOMC15` reader - Port n.15 mode configurate bits"]
pub type Iomc15R = crate::FieldReader;
#[doc = "Field `IOMC15` writer - Port n.15 mode configurate bits"]
pub type Iomc15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IOFC15` reader - Port n.15 function configurate bits"]
pub type Iofc15R = crate::FieldReader;
#[doc = "Field `IOFC15` writer - Port n.15 function configurate bits"]
pub type Iofc15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    pub fn iomc8(&self) -> Iomc8R {
        Iomc8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    pub fn iofc8(&self) -> Iofc8R {
        Iofc8R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Port n.9 mode configurate bits"]
    #[inline(always)]
    pub fn iomc9(&self) -> Iomc9R {
        Iomc9R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Port n.9 function configurate bits"]
    #[inline(always)]
    pub fn iofc9(&self) -> Iofc9R {
        Iofc9R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Port n.10 mode configurate bits"]
    #[inline(always)]
    pub fn iomc10(&self) -> Iomc10R {
        Iomc10R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Port n.10 function configurate bits"]
    #[inline(always)]
    pub fn iofc10(&self) -> Iofc10R {
        Iofc10R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Port n.11 mode configurate bits"]
    #[inline(always)]
    pub fn iomc11(&self) -> Iomc11R {
        Iomc11R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Port n.11 function configurate bits"]
    #[inline(always)]
    pub fn iofc11(&self) -> Iofc11R {
        Iofc11R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Port n.12 mode configurate bits"]
    #[inline(always)]
    pub fn iomc12(&self) -> Iomc12R {
        Iomc12R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Port n.12 function configurate bits"]
    #[inline(always)]
    pub fn iofc12(&self) -> Iofc12R {
        Iofc12R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Port n.13 mode configurate bits"]
    #[inline(always)]
    pub fn iomc13(&self) -> Iomc13R {
        Iomc13R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Port n.13 function configurate bits"]
    #[inline(always)]
    pub fn iofc13(&self) -> Iofc13R {
        Iofc13R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Port n.14 mode configurate bits"]
    #[inline(always)]
    pub fn iomc14(&self) -> Iomc14R {
        Iomc14R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Port n.14 function configurate bits"]
    #[inline(always)]
    pub fn iofc14(&self) -> Iofc14R {
        Iofc14R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Port n.15 mode configurate bits"]
    #[inline(always)]
    pub fn iomc15(&self) -> Iomc15R {
        Iomc15R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Port n.15 function configurate bits"]
    #[inline(always)]
    pub fn iofc15(&self) -> Iofc15R {
        Iofc15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGHR")
            .field("iomc8", &self.iomc8())
            .field("iofc8", &self.iofc8())
            .field("iomc9", &self.iomc9())
            .field("iofc9", &self.iofc9())
            .field("iomc10", &self.iomc10())
            .field("iofc10", &self.iofc10())
            .field("iomc11", &self.iomc11())
            .field("iofc11", &self.iofc11())
            .field("iomc12", &self.iomc12())
            .field("iofc12", &self.iofc12())
            .field("iomc13", &self.iomc13())
            .field("iofc13", &self.iofc13())
            .field("iomc14", &self.iomc14())
            .field("iofc14", &self.iofc14())
            .field("iomc15", &self.iomc15())
            .field("iofc15", &self.iofc15())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Port n.8 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc8(&mut self) -> Iomc8W<CfghrSpec> {
        Iomc8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Port n.8 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc8(&mut self) -> Iofc8W<CfghrSpec> {
        Iofc8W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Port n.9 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc9(&mut self) -> Iomc9W<CfghrSpec> {
        Iomc9W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Port n.9 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc9(&mut self) -> Iofc9W<CfghrSpec> {
        Iofc9W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Port n.10 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc10(&mut self) -> Iomc10W<CfghrSpec> {
        Iomc10W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Port n.10 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc10(&mut self) -> Iofc10W<CfghrSpec> {
        Iofc10W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Port n.11 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc11(&mut self) -> Iomc11W<CfghrSpec> {
        Iomc11W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Port n.11 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc11(&mut self) -> Iofc11W<CfghrSpec> {
        Iofc11W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Port n.12 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc12(&mut self) -> Iomc12W<CfghrSpec> {
        Iomc12W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Port n.12 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc12(&mut self) -> Iofc12W<CfghrSpec> {
        Iofc12W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Port n.13 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc13(&mut self) -> Iomc13W<CfghrSpec> {
        Iomc13W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Port n.13 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc13(&mut self) -> Iofc13W<CfghrSpec> {
        Iofc13W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Port n.14 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc14(&mut self) -> Iomc14W<CfghrSpec> {
        Iomc14W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Port n.14 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc14(&mut self) -> Iofc14W<CfghrSpec> {
        Iofc14W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Port n.15 mode configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iomc15(&mut self) -> Iomc15W<CfghrSpec> {
        Iomc15W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Port n.15 function configurate bits"]
    #[inline(always)]
    #[must_use]
    pub fn iofc15(&mut self) -> Iofc15W<CfghrSpec> {
        Iofc15W::new(self, 30)
    }
}
#[doc = "GPIO function configurate high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfghr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfghr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfghrSpec;
impl crate::RegisterSpec for CfghrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfghr::R`](R) reader structure"]
impl crate::Readable for CfghrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfghr::W`](W) writer structure"]
impl crate::Writable for CfghrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGHR to value 0x4444_4444"]
impl crate::Resettable for CfghrSpec {
    const RESET_VALUE: u32 = 0x4444_4444;
}
