#[doc = "Register `AHBRST2` reader"]
pub type R = crate::R<Ahbrst2Spec>;
#[doc = "Register `AHBRST2` writer"]
pub type W = crate::W<Ahbrst2Spec>;
#[doc = "Field `DVPRST` reader - DVP reset"]
pub type DvprstR = crate::BitReader;
#[doc = "Field `DVPRST` writer - DVP reset"]
pub type DvprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1RST` reader - OTGFS1 reset"]
pub type Otgfs1rstR = crate::BitReader;
#[doc = "Field `OTGFS1RST` writer - OTGFS1 reset"]
pub type Otgfs1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1RST` reader - SDIO1 reset"]
pub type Sdio1rstR = crate::BitReader;
#[doc = "Field `SDIO1RST` writer - SDIO1 reset"]
pub type Sdio1rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    pub fn dvprst(&self) -> DvprstR {
        DvprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    pub fn otgfs1rst(&self) -> Otgfs1rstR {
        Otgfs1rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    pub fn sdio1rst(&self) -> Sdio1rstR {
        Sdio1rstR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBRST2")
            .field("dvprst", &self.dvprst())
            .field("otgfs1rst", &self.otgfs1rst())
            .field("sdio1rst", &self.sdio1rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP reset"]
    #[inline(always)]
    #[must_use]
    pub fn dvprst(&mut self) -> DvprstW<Ahbrst2Spec> {
        DvprstW::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1rst(&mut self) -> Otgfs1rstW<Ahbrst2Spec> {
        Otgfs1rstW::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1rst(&mut self) -> Sdio1rstW<Ahbrst2Spec> {
        Sdio1rstW::new(self, 15)
    }
}
#[doc = "AHB peripheral reset register 2 (CRM_AHBRST2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahbrst2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahbrst2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahbrst2Spec;
impl crate::RegisterSpec for Ahbrst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrst2::R`](R) reader structure"]
impl crate::Readable for Ahbrst2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahbrst2::W`](W) writer structure"]
impl crate::Writable for Ahbrst2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBRST2 to value 0"]
impl crate::Resettable for Ahbrst2Spec {
    const RESET_VALUE: u32 = 0;
}
