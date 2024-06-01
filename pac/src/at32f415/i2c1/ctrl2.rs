#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `CLKFREQ` reader - Input clock frequency"]
pub type ClkfreqR = crate::FieldReader;
#[doc = "Field `CLKFREQ` writer - Input clock frequency"]
pub type ClkfreqW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERRIEN` reader - Error interrupt enable"]
pub type ErrienR = crate::BitReader;
#[doc = "Field `ERRIEN` writer - Error interrupt enable"]
pub type ErrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTIEN` reader - Event interrupt enable"]
pub type EvtienR = crate::BitReader;
#[doc = "Field `EVTIEN` writer - Event interrupt enable"]
pub type EvtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAIEN` reader - Data transmission interrupt enable"]
pub type DataienR = crate::BitReader;
#[doc = "Field `DATAIEN` writer - Data transmission interrupt enable"]
pub type DataienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA transfer enable"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA transfer enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEND` reader - DMA transfer end indication"]
pub type DmaendR = crate::BitReader;
#[doc = "Field `DMAEND` writer - DMA transfer end indication"]
pub type DmaendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    pub fn clkfreq(&self) -> ClkfreqR {
        ClkfreqR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    pub fn errien(&self) -> ErrienR {
        ErrienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    pub fn evtien(&self) -> EvtienR {
        EvtienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    pub fn dataien(&self) -> DataienR {
        DataienR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    pub fn dmaend(&self) -> DmaendR {
        DmaendR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("dmaend", &self.dmaend())
            .field("dmaen", &self.dmaen())
            .field("dataien", &self.dataien())
            .field("evtien", &self.evtien())
            .field("errien", &self.errien())
            .field("clkfreq", &self.clkfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Input clock frequency"]
    #[inline(always)]
    #[must_use]
    pub fn clkfreq(&mut self) -> ClkfreqW<Ctrl2Spec> {
        ClkfreqW::new(self, 0)
    }
    #[doc = "Bit 8 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errien(&mut self) -> ErrienW<Ctrl2Spec> {
        ErrienW::new(self, 8)
    }
    #[doc = "Bit 9 - Event interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn evtien(&mut self) -> EvtienW<Ctrl2Spec> {
        EvtienW::new(self, 9)
    }
    #[doc = "Bit 10 - Data transmission interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dataien(&mut self) -> DataienW<Ctrl2Spec> {
        DataienW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA transfer enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Ctrl2Spec> {
        DmaenW::new(self, 11)
    }
    #[doc = "Bit 12 - DMA transfer end indication"]
    #[inline(always)]
    #[must_use]
    pub fn dmaend(&mut self) -> DmaendW<Ctrl2Spec> {
        DmaendW::new(self, 12)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
