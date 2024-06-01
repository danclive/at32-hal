#[doc = "Register `EVTOUT` reader"]
pub type R = crate::R<EvtoutSpec>;
#[doc = "Register `EVTOUT` writer"]
pub type W = crate::W<EvtoutSpec>;
#[doc = "Field `SELPIN` reader - Select pin"]
pub type SelpinR = crate::FieldReader;
#[doc = "Field `SELPIN` writer - Select pin"]
pub type SelpinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELPORT` reader - Select port"]
pub type SelportR = crate::FieldReader;
#[doc = "Field `SELPORT` writer - Select port"]
pub type SelportW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EVOEN` reader - Event output enable"]
pub type EvoenR = crate::BitReader;
#[doc = "Field `EVOEN` writer - Event output enable"]
pub type EvoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    pub fn selpin(&self) -> SelpinR {
        SelpinR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    pub fn selport(&self) -> SelportR {
        SelportR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    pub fn evoen(&self) -> EvoenR {
        EvoenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVTOUT")
            .field("selpin", &self.selpin())
            .field("selport", &self.selport())
            .field("evoen", &self.evoen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Select pin"]
    #[inline(always)]
    #[must_use]
    pub fn selpin(&mut self) -> SelpinW<EvtoutSpec> {
        SelpinW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select port"]
    #[inline(always)]
    #[must_use]
    pub fn selport(&mut self) -> SelportW<EvtoutSpec> {
        SelportW::new(self, 4)
    }
    #[doc = "Bit 7 - Event output enable"]
    #[inline(always)]
    #[must_use]
    pub fn evoen(&mut self) -> EvoenW<EvtoutSpec> {
        EvoenW::new(self, 7)
    }
}
#[doc = "Event output register (IOMUX_EVTOUT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtoutSpec;
impl crate::RegisterSpec for EvtoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evtout::R`](R) reader structure"]
impl crate::Readable for EvtoutSpec {}
#[doc = "`write(|w| ..)` method takes [`evtout::W`](W) writer structure"]
impl crate::Writable for EvtoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVTOUT to value 0"]
impl crate::Resettable for EvtoutSpec {
    const RESET_VALUE: u32 = 0;
}
