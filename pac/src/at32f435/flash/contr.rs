#[doc = "Register `CONTR` reader"]
pub type R = crate::R<ContrSpec>;
#[doc = "Register `CONTR` writer"]
pub type W = crate::W<ContrSpec>;
#[doc = "Field `FCONTR_EN` reader - Flash continue read enable"]
pub type FcontrEnR = crate::BitReader;
#[doc = "Field `FCONTR_EN` writer - Flash continue read enable"]
pub type FcontrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    pub fn fcontr_en(&self) -> FcontrEnR {
        FcontrEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONTR")
            .field("fcontr_en", &self.fcontr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Flash continue read enable"]
    #[inline(always)]
    #[must_use]
    pub fn fcontr_en(&mut self) -> FcontrEnW<ContrSpec> {
        FcontrEnW::new(self, 31)
    }
}
#[doc = "Flash continue read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`contr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`contr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ContrSpec;
impl crate::RegisterSpec for ContrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`contr::R`](R) reader structure"]
impl crate::Readable for ContrSpec {}
#[doc = "`write(|w| ..)` method takes [`contr::W`](W) writer structure"]
impl crate::Writable for ContrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTR to value 0x80"]
impl crate::Resettable for ContrSpec {
    const RESET_VALUE: u32 = 0x80;
}
