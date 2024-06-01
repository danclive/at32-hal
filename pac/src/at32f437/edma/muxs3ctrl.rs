#[doc = "Register `MUXS3CTRL` reader"]
pub type R = crate::R<Muxs3ctrlSpec>;
#[doc = "Register `MUXS3CTRL` writer"]
pub type W = crate::W<Muxs3ctrlSpec>;
#[doc = "Field `REQSEL` reader - DMA request select"]
pub type ReqselR = crate::FieldReader;
#[doc = "Field `REQSEL` writer - DMA request select"]
pub type ReqselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SYNCOVIEN` reader - Synchronization overrun interrupt enable"]
pub type SyncovienR = crate::BitReader;
#[doc = "Field `SYNCOVIEN` writer - Synchronization overrun interrupt enable"]
pub type SyncovienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTGEN` reader - Event generation enable"]
pub type EvtgenR = crate::BitReader;
#[doc = "Field `EVTGEN` writer - Event generation enable"]
pub type EvtgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchroniztion enable"]
pub type SyncenR = crate::BitReader;
#[doc = "Field `SYNCEN` writer - Synchroniztion enable"]
pub type SyncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCPOL` reader - Synchronization polarity"]
pub type SyncpolR = crate::FieldReader;
#[doc = "Field `SYNCPOL` writer - Synchronization polarity"]
pub type SyncpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REQCNT` reader - Number of DMA requests"]
pub type ReqcntR = crate::FieldReader;
#[doc = "Field `REQCNT` writer - Number of DMA requests"]
pub type ReqcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNCSEL` reader - Synchronization select"]
pub type SyncselR = crate::FieldReader;
#[doc = "Field `SYNCSEL` writer - Synchronization select"]
pub type SyncselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    pub fn reqsel(&self) -> ReqselR {
        ReqselR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    pub fn syncovien(&self) -> SyncovienR {
        SyncovienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    pub fn evtgen(&self) -> EvtgenR {
        EvtgenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    pub fn syncpol(&self) -> SyncpolR {
        SyncpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    pub fn reqcnt(&self) -> ReqcntR {
        ReqcntR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization select"]
    #[inline(always)]
    pub fn syncsel(&self) -> SyncselR {
        SyncselR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXS3CTRL")
            .field("reqsel", &self.reqsel())
            .field("syncovien", &self.syncovien())
            .field("evtgen", &self.evtgen())
            .field("syncen", &self.syncen())
            .field("syncpol", &self.syncpol())
            .field("reqcnt", &self.reqcnt())
            .field("syncsel", &self.syncsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA request select"]
    #[inline(always)]
    #[must_use]
    pub fn reqsel(&mut self) -> ReqselW<Muxs3ctrlSpec> {
        ReqselW::new(self, 0)
    }
    #[doc = "Bit 8 - Synchronization overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncovien(&mut self) -> SyncovienW<Muxs3ctrlSpec> {
        SyncovienW::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtgen(&mut self) -> EvtgenW<Muxs3ctrlSpec> {
        EvtgenW::new(self, 9)
    }
    #[doc = "Bit 16 - Synchroniztion enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SyncenW<Muxs3ctrlSpec> {
        SyncenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization polarity"]
    #[inline(always)]
    #[must_use]
    pub fn syncpol(&mut self) -> SyncpolW<Muxs3ctrlSpec> {
        SyncpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests"]
    #[inline(always)]
    #[must_use]
    pub fn reqcnt(&mut self) -> ReqcntW<Muxs3ctrlSpec> {
        ReqcntW::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization select"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SyncselW<Muxs3ctrlSpec> {
        SyncselW::new(self, 24)
    }
}
#[doc = "Stream 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxs3ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxs3ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Muxs3ctrlSpec;
impl crate::RegisterSpec for Muxs3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxs3ctrl::R`](R) reader structure"]
impl crate::Readable for Muxs3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`muxs3ctrl::W`](W) writer structure"]
impl crate::Writable for Muxs3ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXS3CTRL to value 0"]
impl crate::Resettable for Muxs3ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
