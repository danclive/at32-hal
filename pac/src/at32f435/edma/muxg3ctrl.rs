#[doc = "Register `MUXG3CTRL` reader"]
pub type R = crate::R<Muxg3ctrlSpec>;
#[doc = "Register `MUXG3CTRL` writer"]
pub type W = crate::W<Muxg3ctrlSpec>;
#[doc = "Field `SIGSEL` reader - Signal select"]
pub type SigselR = crate::FieldReader;
#[doc = "Field `SIGSEL` writer - Signal select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRGOVIEN` reader - Trigger overrun interrupt enable"]
pub type TrgovienR = crate::BitReader;
#[doc = "Field `TRGOVIEN` writer - Trigger overrun interrupt enable"]
pub type TrgovienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GEN` reader - DMA request generator enable"]
pub type GenR = crate::BitReader;
#[doc = "Field `GEN` writer - DMA request generator enable"]
pub type GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPOL` reader - DMA request generator trigger polarity"]
pub type GpolR = crate::FieldReader;
#[doc = "Field `GPOL` writer - DMA request generator trigger polarity"]
pub type GpolW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GREQCNT` reader - Number of DMA requests to be generated"]
pub type GreqcntR = crate::FieldReader;
#[doc = "Field `GREQCNT` writer - Number of DMA requests to be generated"]
pub type GreqcntW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    pub fn trgovien(&self) -> TrgovienR {
        TrgovienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    pub fn gen(&self) -> GenR {
        GenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    pub fn gpol(&self) -> GpolR {
        GpolR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    pub fn greqcnt(&self) -> GreqcntR {
        GreqcntR::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MUXG3CTRL")
            .field("sigsel", &self.sigsel())
            .field("trgovien", &self.trgovien())
            .field("gen", &self.gen())
            .field("gpol", &self.gpol())
            .field("greqcnt", &self.greqcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Signal select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SigselW<Muxg3ctrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bit 8 - Trigger overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgovien(&mut self) -> TrgovienW<Muxg3ctrlSpec> {
        TrgovienW::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator enable"]
    #[inline(always)]
    #[must_use]
    pub fn gen(&mut self) -> GenW<Muxg3ctrlSpec> {
        GenW::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GpolW<Muxg3ctrlSpec> {
        GpolW::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to be generated"]
    #[inline(always)]
    #[must_use]
    pub fn greqcnt(&mut self) -> GreqcntW<Muxg3ctrlSpec> {
        GreqcntW::new(self, 19)
    }
}
#[doc = "Generator 3 Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`muxg3ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`muxg3ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Muxg3ctrlSpec;
impl crate::RegisterSpec for Muxg3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`muxg3ctrl::R`](R) reader structure"]
impl crate::Readable for Muxg3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`muxg3ctrl::W`](W) writer structure"]
impl crate::Writable for Muxg3ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MUXG3CTRL to value 0"]
impl crate::Resettable for Muxg3ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
