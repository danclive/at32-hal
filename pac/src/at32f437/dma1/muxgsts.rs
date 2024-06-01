#[doc = "Register `MUXGSTS` reader"]
pub type R = crate::R<MuxgstsSpec>;
#[doc = "Register `MUXGSTS` writer"]
pub type W = crate::W<MuxgstsSpec>;
#[doc = "Field `TRGOVF1` reader - Trigger overrun interrupt flag"]
pub type Trgovf1R = crate::BitReader;
#[doc = "Field `TRGOVF1` writer - Trigger overrun interrupt flag"]
pub type Trgovf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF2` reader - Trigger overrun interrupt flag"]
pub type Trgovf2R = crate::BitReader;
#[doc = "Field `TRGOVF2` writer - Trigger overrun interrupt flag"]
pub type Trgovf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF3` reader - Trigger overrun interrupt flag"]
pub type Trgovf3R = crate::BitReader;
#[doc = "Field `TRGOVF3` writer - Trigger overrun interrupt flag"]
pub type Trgovf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVF4` reader - Trigger overrun interrupt flag"]
pub type Trgovf4R = crate::BitReader;
#[doc = "Field `TRGOVF4` writer - Trigger overrun interrupt flag"]
pub type Trgovf4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf1(&self) -> Trgovf1R {
        Trgovf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf2(&self) -> Trgovf2R {
        Trgovf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf3(&self) -> Trgovf3R {
        Trgovf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovf4(&self) -> Trgovf4R {
        Trgovf4R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGSTS")
            .field("trgovf1", &self.trgovf1())
            .field("trgovf2", &self.trgovf2())
            .field("trgovf3", &self.trgovf3())
            .field("trgovf4", &self.trgovf4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf1(&mut self) -> Trgovf1W<MuxgstsSpec> {
        Trgovf1W::new(self, 0)
    }
    #[doc = "Bit 1 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf2(&mut self) -> Trgovf2W<MuxgstsSpec> {
        Trgovf2W::new(self, 1)
    }
    #[doc = "Bit 2 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf3(&mut self) -> Trgovf3W<MuxgstsSpec> {
        Trgovf3W::new(self, 2)
    }
    #[doc = "Bit 3 - Trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovf4(&mut self) -> Trgovf4W<MuxgstsSpec> {
        Trgovf4W::new(self, 3)
    }
}
#[doc = "Generator Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxgstsSpec;
impl crate::RegisterSpec for MuxgstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgsts::R`](R) reader structure"]
impl crate::Readable for MuxgstsSpec {}
#[doc = "`write(|w| ..)` method takes [`muxgsts::W`](W) writer structure"]
impl crate::Writable for MuxgstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXGSTS to value 0"]
impl crate::Resettable for MuxgstsSpec {
    const RESET_VALUE: u32 = 0;
}
