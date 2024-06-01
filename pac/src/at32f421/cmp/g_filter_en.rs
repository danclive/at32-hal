#[doc = "Register `G_FILTER_EN` reader"]
pub type R = crate::R<GFilterEnSpec>;
#[doc = "Register `G_FILTER_EN` writer"]
pub type W = crate::W<GFilterEnSpec>;
#[doc = "Field `GFE` reader - Glitch filter enable"]
pub type GfeR = crate::BitReader;
#[doc = "Field `GFE` writer - Glitch filter enable"]
pub type GfeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gfe(&self) -> GfeR {
        GfeR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("G_FILTER_EN")
            .field("gfe", &self.gfe())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    #[must_use]
    pub fn gfe(&mut self) -> GfeW<GFilterEnSpec> {
        GfeW::new(self, 0)
    }
}
#[doc = "G_FILTER_EN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`g_filter_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`g_filter_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GFilterEnSpec;
impl crate::RegisterSpec for GFilterEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`g_filter_en::R`](R) reader structure"]
impl crate::Readable for GFilterEnSpec {}
#[doc = "`write(|w| ..)` method takes [`g_filter_en::W`](W) writer structure"]
impl crate::Writable for GFilterEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets G_FILTER_EN to value 0"]
impl crate::Resettable for GFilterEnSpec {
    const RESET_VALUE: u32 = 0;
}
