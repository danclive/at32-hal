#[doc = "Register `AHBEN2` reader"]
pub type R = crate::R<Ahben2Spec>;
#[doc = "Register `AHBEN2` writer"]
pub type W = crate::W<Ahben2Spec>;
#[doc = "Field `DVPEN` reader - DVP clock enable"]
pub type DvpenR = crate::BitReader;
#[doc = "Field `DVPEN` writer - DVP clock enable"]
pub type DvpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1EN` reader - OTGFS1 clock enable"]
pub type Otgfs1enR = crate::BitReader;
#[doc = "Field `OTGFS1EN` writer - OTGFS1 clock enable"]
pub type Otgfs1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1EN` reader - SDIO1 clock enable"]
pub type Sdio1enR = crate::BitReader;
#[doc = "Field `SDIO1EN` writer - SDIO1 clock enable"]
pub type Sdio1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    pub fn dvpen(&self) -> DvpenR {
        DvpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    pub fn otgfs1en(&self) -> Otgfs1enR {
        Otgfs1enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    pub fn sdio1en(&self) -> Sdio1enR {
        Sdio1enR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN2")
            .field("dvpen", &self.dvpen())
            .field("otgfs1en", &self.otgfs1en())
            .field("sdio1en", &self.sdio1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn dvpen(&mut self) -> DvpenW<Ahben2Spec> {
        DvpenW::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1en(&mut self) -> Otgfs1enW<Ahben2Spec> {
        Otgfs1enW::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1en(&mut self) -> Sdio1enW<Ahben2Spec> {
        Sdio1enW::new(self, 15)
    }
}
#[doc = "AHB peripheral clock enable register 2 (CRM_AHBEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahben2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahben2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahben2Spec;
impl crate::RegisterSpec for Ahben2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben2::R`](R) reader structure"]
impl crate::Readable for Ahben2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahben2::W`](W) writer structure"]
impl crate::Writable for Ahben2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBEN2 to value 0"]
impl crate::Resettable for Ahben2Spec {
    const RESET_VALUE: u32 = 0;
}
