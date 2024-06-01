#[doc = "Register `MUXGCLR` reader"]
pub type R = crate::R<MuxgclrSpec>;
#[doc = "Register `MUXGCLR` writer"]
pub type W = crate::W<MuxgclrSpec>;
#[doc = "Field `TRGOVFC1` reader - Clear trigger overrun interrupt flag"]
pub type Trgovfc1R = crate::BitReader;
#[doc = "Field `TRGOVFC1` writer - Clear trigger overrun interrupt flag"]
pub type Trgovfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC2` reader - Clear trigger overrun interrupt flag"]
pub type Trgovfc2R = crate::BitReader;
#[doc = "Field `TRGOVFC2` writer - Clear trigger overrun interrupt flag"]
pub type Trgovfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC3` reader - Clear trigger overrun interrupt flag"]
pub type Trgovfc3R = crate::BitReader;
#[doc = "Field `TRGOVFC3` writer - Clear trigger overrun interrupt flag"]
pub type Trgovfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOVFC4` reader - Clear trigger overrun interrupt flag"]
pub type Trgovfc4R = crate::BitReader;
#[doc = "Field `TRGOVFC4` writer - Clear trigger overrun interrupt flag"]
pub type Trgovfc4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc1(&self) -> Trgovfc1R {
        Trgovfc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc2(&self) -> Trgovfc2R {
        Trgovfc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc3(&self) -> Trgovfc3R {
        Trgovfc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    pub fn trgovfc4(&self) -> Trgovfc4R {
        Trgovfc4R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXGCLR")
            .field("trgovfc1", &self.trgovfc1())
            .field("trgovfc2", &self.trgovfc2())
            .field("trgovfc3", &self.trgovfc3())
            .field("trgovfc4", &self.trgovfc4())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc1(&mut self) -> Trgovfc1W<MuxgclrSpec> {
        Trgovfc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc2(&mut self) -> Trgovfc2W<MuxgclrSpec> {
        Trgovfc2W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc3(&mut self) -> Trgovfc3W<MuxgclrSpec> {
        Trgovfc3W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear trigger overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn trgovfc4(&mut self) -> Trgovfc4W<MuxgclrSpec> {
        Trgovfc4W::new(self, 3)
    }
}
#[doc = "Generator Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxgclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxgclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxgclrSpec;
impl crate::RegisterSpec for MuxgclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxgclr::R`](R) reader structure"]
impl crate::Readable for MuxgclrSpec {}
#[doc = "`write(|w| ..)` method takes [`muxgclr::W`](W) writer structure"]
impl crate::Writable for MuxgclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXGCLR to value 0"]
impl crate::Resettable for MuxgclrSpec {
    const RESET_VALUE: u32 = 0;
}
