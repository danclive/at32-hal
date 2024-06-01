#[doc = "Register `MUXH` reader"]
pub type R = crate::R<MuxhSpec>;
#[doc = "Register `MUXH` writer"]
pub type W = crate::W<MuxhSpec>;
#[doc = "Field `MUXH8` reader - GPIOx pin 8 muxing"]
pub type Muxh8R = crate::FieldReader;
#[doc = "Field `MUXH8` writer - GPIOx pin 8 muxing"]
pub type Muxh8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH9` reader - GPIOx pin 9 muxing"]
pub type Muxh9R = crate::FieldReader;
#[doc = "Field `MUXH9` writer - GPIOx pin 9 muxing"]
pub type Muxh9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH10` reader - GPIOx pin 10 muxing"]
pub type Muxh10R = crate::FieldReader;
#[doc = "Field `MUXH10` writer - GPIOx pin 10 muxing"]
pub type Muxh10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH11` reader - GPIOx pin 11 muxing"]
pub type Muxh11R = crate::FieldReader;
#[doc = "Field `MUXH11` writer - GPIOx pin 11 muxing"]
pub type Muxh11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH12` reader - GPIOx pin 12 muxing"]
pub type Muxh12R = crate::FieldReader;
#[doc = "Field `MUXH12` writer - GPIOx pin 12 muxing"]
pub type Muxh12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH13` reader - GPIOx pin 13 muxing"]
pub type Muxh13R = crate::FieldReader;
#[doc = "Field `MUXH13` writer - GPIOx pin 13 muxing"]
pub type Muxh13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH14` reader - GPIOx pin 14 muxing"]
pub type Muxh14R = crate::FieldReader;
#[doc = "Field `MUXH14` writer - GPIOx pin 14 muxing"]
pub type Muxh14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXH15` reader - GPIOx pin 15 muxing"]
pub type Muxh15R = crate::FieldReader;
#[doc = "Field `MUXH15` writer - GPIOx pin 15 muxing"]
pub type Muxh15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    pub fn muxh8(&self) -> Muxh8R {
        Muxh8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    pub fn muxh9(&self) -> Muxh9R {
        Muxh9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    pub fn muxh10(&self) -> Muxh10R {
        Muxh10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    pub fn muxh11(&self) -> Muxh11R {
        Muxh11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    pub fn muxh12(&self) -> Muxh12R {
        Muxh12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    pub fn muxh13(&self) -> Muxh13R {
        Muxh13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    pub fn muxh14(&self) -> Muxh14R {
        Muxh14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    pub fn muxh15(&self) -> Muxh15R {
        Muxh15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXH")
            .field("muxh15", &self.muxh15())
            .field("muxh14", &self.muxh14())
            .field("muxh13", &self.muxh13())
            .field("muxh12", &self.muxh12())
            .field("muxh11", &self.muxh11())
            .field("muxh10", &self.muxh10())
            .field("muxh9", &self.muxh9())
            .field("muxh8", &self.muxh8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOx pin 8 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh8(&mut self) -> Muxh8W<MuxhSpec> {
        Muxh8W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOx pin 9 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh9(&mut self) -> Muxh9W<MuxhSpec> {
        Muxh9W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOx pin 10 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh10(&mut self) -> Muxh10W<MuxhSpec> {
        Muxh10W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 11 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh11(&mut self) -> Muxh11W<MuxhSpec> {
        Muxh11W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOx pin 12 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh12(&mut self) -> Muxh12W<MuxhSpec> {
        Muxh12W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOx pin 13 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh13(&mut self) -> Muxh13W<MuxhSpec> {
        Muxh13W::new(self, 20)
    }
    #[doc = "Bits 24:27 - GPIOx pin 14 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh14(&mut self) -> Muxh14W<MuxhSpec> {
        Muxh14W::new(self, 24)
    }
    #[doc = "Bits 28:31 - GPIOx pin 15 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxh15(&mut self) -> Muxh15W<MuxhSpec> {
        Muxh15W::new(self, 28)
    }
}
#[doc = "GPIO muxing function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxhSpec;
impl crate::RegisterSpec for MuxhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxh::R`](R) reader structure"]
impl crate::Readable for MuxhSpec {}
#[doc = "`write(|w| ..)` method takes [`muxh::W`](W) writer structure"]
impl crate::Writable for MuxhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXH to value 0"]
impl crate::Resettable for MuxhSpec {
    const RESET_VALUE: u32 = 0;
}
