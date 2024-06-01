#[doc = "Register `REMAP2` reader"]
pub type R = crate::R<Remap2Spec>;
#[doc = "Register `REMAP2` writer"]
pub type W = crate::W<Remap2Spec>;
#[doc = "Field `EXT_SPIM_EN_MUX` reader - SPIM enable"]
pub type ExtSpimEnMuxR = crate::BitReader;
#[doc = "Field `EXT_SPIM_EN_MUX` writer - SPIM enable"]
pub type ExtSpimEnMuxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 21 - SPIM enable"]
    #[inline(always)]
    pub fn ext_spim_en_mux(&self) -> ExtSpimEnMuxR {
        ExtSpimEnMuxR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP2")
            .field("ext_spim_en_mux", &self.ext_spim_en_mux())
            .finish()
    }
}
impl W {
    #[doc = "Bit 21 - SPIM enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_spim_en_mux(&mut self) -> ExtSpimEnMuxW<Remap2Spec> {
        ExtSpimEnMuxW::new(self, 21)
    }
}
#[doc = "IO MUX remap register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap2Spec;
impl crate::RegisterSpec for Remap2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap2::R`](R) reader structure"]
impl crate::Readable for Remap2Spec {}
#[doc = "`write(|w| ..)` method takes [`remap2::W`](W) writer structure"]
impl crate::Writable for Remap2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP2 to value 0"]
impl crate::Resettable for Remap2Spec {
    const RESET_VALUE: u32 = 0;
}
