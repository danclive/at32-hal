#[doc = "Register `MUXSYNCSTS` reader"]
pub type R = crate::R<MuxsyncstsSpec>;
#[doc = "Register `MUXSYNCSTS` writer"]
pub type W = crate::W<MuxsyncstsSpec>;
#[doc = "Field `SYNCOVF1` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf1R = crate::BitReader;
#[doc = "Field `SYNCOVF2` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf2R = crate::BitReader;
#[doc = "Field `SYNCOVF3` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf3R = crate::BitReader;
#[doc = "Field `SYNCOVF4` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf4R = crate::BitReader;
#[doc = "Field `SYNCOVF5` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf5R = crate::BitReader;
#[doc = "Field `SYNCOVF6` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf6R = crate::BitReader;
#[doc = "Field `SYNCOVF7` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf7R = crate::BitReader;
#[doc = "Field `SYNCOVF8` reader - Synchronizaton overrun interrupt flag"]
pub type Syncovf8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf1(&self) -> Syncovf1R {
        Syncovf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf2(&self) -> Syncovf2R {
        Syncovf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf3(&self) -> Syncovf3R {
        Syncovf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf4(&self) -> Syncovf4R {
        Syncovf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf5(&self) -> Syncovf5R {
        Syncovf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf6(&self) -> Syncovf6R {
        Syncovf6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf7(&self) -> Syncovf7R {
        Syncovf7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronizaton overrun interrupt flag"]
    #[inline(always)]
    pub fn syncovf8(&self) -> Syncovf8R {
        Syncovf8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXSYNCSTS")
            .field("syncovf1", &self.syncovf1())
            .field("syncovf2", &self.syncovf2())
            .field("syncovf3", &self.syncovf3())
            .field("syncovf4", &self.syncovf4())
            .field("syncovf5", &self.syncovf5())
            .field("syncovf6", &self.syncovf6())
            .field("syncovf7", &self.syncovf7())
            .field("syncovf8", &self.syncovf8())
            .finish()
    }
}
impl W {}
#[doc = "Channel Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxsyncsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxsyncsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxsyncstsSpec;
impl crate::RegisterSpec for MuxsyncstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxsyncsts::R`](R) reader structure"]
impl crate::Readable for MuxsyncstsSpec {}
#[doc = "`write(|w| ..)` method takes [`muxsyncsts::W`](W) writer structure"]
impl crate::Writable for MuxsyncstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXSYNCSTS to value 0"]
impl crate::Resettable for MuxsyncstsSpec {
    const RESET_VALUE: u32 = 0;
}
