#[doc = "Register `AHBLPEN2` reader"]
pub type R = crate::R<Ahblpen2Spec>;
#[doc = "Register `AHBLPEN2` writer"]
pub type W = crate::W<Ahblpen2Spec>;
#[doc = "Field `DVPLPEN` reader - DVP clock enable during sleep mode"]
pub type DvplpenR = crate::BitReader;
#[doc = "Field `DVPLPEN` writer - DVP clock enable during sleep mode"]
pub type DvplpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTGFS1LPEN` reader - OTGFS1 clock enable during sleep mode"]
pub type Otgfs1lpenR = crate::BitReader;
#[doc = "Field `OTGFS1LPEN` writer - OTGFS1 clock enable during sleep mode"]
pub type Otgfs1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO1LPEN` reader - SDIO1 clock enable during sleep mode"]
pub type Sdio1lpenR = crate::BitReader;
#[doc = "Field `SDIO1LPEN` writer - SDIO1 clock enable during sleep mode"]
pub type Sdio1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    pub fn dvplpen(&self) -> DvplpenR {
        DvplpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn otgfs1lpen(&self) -> Otgfs1lpenR {
        Otgfs1lpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    pub fn sdio1lpen(&self) -> Sdio1lpenR {
        Sdio1lpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBLPEN2")
            .field("dvplpen", &self.dvplpen())
            .field("otgfs1lpen", &self.otgfs1lpen())
            .field("sdio1lpen", &self.sdio1lpen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DVP clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn dvplpen(&mut self) -> DvplpenW<Ahblpen2Spec> {
        DvplpenW::new(self, 0)
    }
    #[doc = "Bit 7 - OTGFS1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn otgfs1lpen(&mut self) -> Otgfs1lpenW<Ahblpen2Spec> {
        Otgfs1lpenW::new(self, 7)
    }
    #[doc = "Bit 15 - SDIO1 clock enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1lpen(&mut self) -> Sdio1lpenW<Ahblpen2Spec> {
        Sdio1lpenW::new(self, 15)
    }
}
#[doc = "AHB peripheral Low-power clock enable register 2 (CRM_AHBLPEN2)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahblpen2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahblpen2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahblpen2Spec;
impl crate::RegisterSpec for Ahblpen2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahblpen2::R`](R) reader structure"]
impl crate::Readable for Ahblpen2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahblpen2::W`](W) writer structure"]
impl crate::Writable for Ahblpen2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBLPEN2 to value 0x8081"]
impl crate::Resettable for Ahblpen2Spec {
    const RESET_VALUE: u32 = 0x8081;
}
