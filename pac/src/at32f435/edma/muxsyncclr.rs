#[doc = "Register `MUXSYNCCLR` reader"]
pub type R = crate::R<MuxsyncclrSpec>;
#[doc = "Register `MUXSYNCCLR` writer"]
pub type W = crate::W<MuxsyncclrSpec>;
#[doc = "Field `SYNCOVFC1` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc1R = crate::BitReader;
#[doc = "Field `SYNCOVFC1` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC2` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc2R = crate::BitReader;
#[doc = "Field `SYNCOVFC2` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC3` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc3R = crate::BitReader;
#[doc = "Field `SYNCOVFC3` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC4` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc4R = crate::BitReader;
#[doc = "Field `SYNCOVFC4` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC5` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc5R = crate::BitReader;
#[doc = "Field `SYNCOVFC5` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC6` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc6R = crate::BitReader;
#[doc = "Field `SYNCOVFC6` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC7` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc7R = crate::BitReader;
#[doc = "Field `SYNCOVFC7` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCOVFC8` reader - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc8R = crate::BitReader;
#[doc = "Field `SYNCOVFC8` writer - Clear synchronizaton overrun interrupt flag"]
pub type Syncovfc8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc1(&self) -> Syncovfc1R {
        Syncovfc1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc2(&self) -> Syncovfc2R {
        Syncovfc2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc3(&self) -> Syncovfc3R {
        Syncovfc3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc4(&self) -> Syncovfc4R {
        Syncovfc4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc5(&self) -> Syncovfc5R {
        Syncovfc5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc6(&self) -> Syncovfc6R {
        Syncovfc6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc7(&self) -> Syncovfc7R {
        Syncovfc7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovfc8(&self) -> Syncovfc8R {
        Syncovfc8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCCLR")
            .field("syncovfc1", &self.syncovfc1())
            .field("syncovfc2", &self.syncovfc2())
            .field("syncovfc3", &self.syncovfc3())
            .field("syncovfc4", &self.syncovfc4())
            .field("syncovfc5", &self.syncovfc5())
            .field("syncovfc6", &self.syncovfc6())
            .field("syncovfc7", &self.syncovfc7())
            .field("syncovfc8", &self.syncovfc8())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc1(&mut self) -> Syncovfc1W<MuxsyncclrSpec> {
        Syncovfc1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc2(&mut self) -> Syncovfc2W<MuxsyncclrSpec> {
        Syncovfc2W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc3(&mut self) -> Syncovfc3W<MuxsyncclrSpec> {
        Syncovfc3W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc4(&mut self) -> Syncovfc4W<MuxsyncclrSpec> {
        Syncovfc4W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc5(&mut self) -> Syncovfc5W<MuxsyncclrSpec> {
        Syncovfc5W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc6(&mut self) -> Syncovfc6W<MuxsyncclrSpec> {
        Syncovfc6W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc7(&mut self) -> Syncovfc7W<MuxsyncclrSpec> {
        Syncovfc7W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear synchronizaton overrun interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncovfc8(&mut self) -> Syncovfc8W<MuxsyncclrSpec> {
        Syncovfc8W::new(self, 7)
    }
}
#[doc = "Channel Interrupt Clear Flag Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxsyncclrSpec;
impl crate::RegisterSpec for MuxsyncclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncclr::R`](R) reader structure"]
impl crate::Readable for MuxsyncclrSpec {}
#[doc = "`write(|w| ..)` method takes [`muxsyncclr::W`](W) writer structure"]
impl crate::Writable for MuxsyncclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXSYNCCLR to value 0"]
impl crate::Resettable for MuxsyncclrSpec {
    const RESET_VALUE: u32 = 0;
}
