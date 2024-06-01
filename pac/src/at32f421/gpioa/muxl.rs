#[doc = "Register `MUXL` reader"]
pub type R = crate::R<MuxlSpec>;
#[doc = "Register `MUXL` writer"]
pub type W = crate::W<MuxlSpec>;
#[doc = "Field `MUXL0` reader - GPIOx pin 0 muxing"]
pub type Muxl0R = crate::FieldReader;
#[doc = "Field `MUXL0` writer - GPIOx pin 0 muxing"]
pub type Muxl0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL1` reader - GPIOx pin 1 muxing"]
pub type Muxl1R = crate::FieldReader;
#[doc = "Field `MUXL1` writer - GPIOx pin 1 muxing"]
pub type Muxl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL2` reader - GPIOx pin 2 muxing"]
pub type Muxl2R = crate::FieldReader;
#[doc = "Field `MUXL2` writer - GPIOx pin 2 muxing"]
pub type Muxl2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL3` reader - GPIOx pin 3 muxing"]
pub type Muxl3R = crate::FieldReader;
#[doc = "Field `MUXL3` writer - GPIOx pin 3 muxing"]
pub type Muxl3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL4` reader - GPIOx pin 4 muxing"]
pub type Muxl4R = crate::FieldReader;
#[doc = "Field `MUXL4` writer - GPIOx pin 4 muxing"]
pub type Muxl4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL5` reader - GPIOx pin 5 muxing"]
pub type Muxl5R = crate::FieldReader;
#[doc = "Field `MUXL5` writer - GPIOx pin 5 muxing"]
pub type Muxl5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL6` reader - GPIOx pin 6 muxing"]
pub type Muxl6R = crate::FieldReader;
#[doc = "Field `MUXL6` writer - GPIOx pin 6 muxing"]
pub type Muxl6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MUXL7` reader - GPIOx pin 7 muxing"]
pub type Muxl7R = crate::FieldReader;
#[doc = "Field `MUXL7` writer - GPIOx pin 7 muxing"]
pub type Muxl7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    pub fn muxl0(&self) -> Muxl0R {
        Muxl0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    pub fn muxl1(&self) -> Muxl1R {
        Muxl1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    pub fn muxl2(&self) -> Muxl2R {
        Muxl2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    pub fn muxl3(&self) -> Muxl3R {
        Muxl3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    pub fn muxl4(&self) -> Muxl4R {
        Muxl4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    pub fn muxl5(&self) -> Muxl5R {
        Muxl5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    pub fn muxl6(&self) -> Muxl6R {
        Muxl6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    pub fn muxl7(&self) -> Muxl7R {
        Muxl7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXL")
            .field("muxl7", &self.muxl7())
            .field("muxl6", &self.muxl6())
            .field("muxl5", &self.muxl5())
            .field("muxl4", &self.muxl4())
            .field("muxl3", &self.muxl3())
            .field("muxl2", &self.muxl2())
            .field("muxl1", &self.muxl1())
            .field("muxl0", &self.muxl0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - GPIOx pin 0 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl0(&mut self) -> Muxl0W<MuxlSpec> {
        Muxl0W::new(self, 0)
    }
    #[doc = "Bits 4:7 - GPIOx pin 1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl1(&mut self) -> Muxl1W<MuxlSpec> {
        Muxl1W::new(self, 4)
    }
    #[doc = "Bits 8:11 - GPIOx pin 2 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl2(&mut self) -> Muxl2W<MuxlSpec> {
        Muxl2W::new(self, 8)
    }
    #[doc = "Bits 12:15 - GPIOx pin 3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl3(&mut self) -> Muxl3W<MuxlSpec> {
        Muxl3W::new(self, 12)
    }
    #[doc = "Bits 16:19 - GPIOx pin 4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl4(&mut self) -> Muxl4W<MuxlSpec> {
        Muxl4W::new(self, 16)
    }
    #[doc = "Bits 20:23 - GPIOx pin 5 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl5(&mut self) -> Muxl5W<MuxlSpec> {
        Muxl5W::new(self, 20)
    }
    #[doc = "Bits 24:27 - GPIOx pin 6 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl6(&mut self) -> Muxl6W<MuxlSpec> {
        Muxl6W::new(self, 24)
    }
    #[doc = "Bits 28:31 - GPIOx pin 7 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn muxl7(&mut self) -> Muxl7W<MuxlSpec> {
        Muxl7W::new(self, 28)
    }
}
#[doc = "GPIO muxing function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxlSpec;
impl crate::RegisterSpec for MuxlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxl::R`](R) reader structure"]
impl crate::Readable for MuxlSpec {}
#[doc = "`write(|w| ..)` method takes [`muxl::W`](W) writer structure"]
impl crate::Writable for MuxlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXL to value 0"]
impl crate::Resettable for MuxlSpec {
    const RESET_VALUE: u32 = 0;
}
