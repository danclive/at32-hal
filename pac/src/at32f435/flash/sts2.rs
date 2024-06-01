#[doc = "Register `STS2` reader"]
pub type R = crate::R<Sts2Spec>;
#[doc = "Register `STS2` writer"]
pub type W = crate::W<Sts2Spec>;
#[doc = "Field `OBF` reader - Operate busy flag"]
pub type ObfR = crate::BitReader;
#[doc = "Field `PRGMERR` reader - program error"]
pub type PrgmerrR = crate::BitReader;
#[doc = "Field `PRGMERR` writer - program error"]
pub type PrgmerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPERR` reader - Erase/program protection error"]
pub type EpperrR = crate::BitReader;
#[doc = "Field `EPPERR` writer - Erase/program protection error"]
pub type EpperrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODF` reader - Operate done flag"]
pub type OdfR = crate::BitReader;
#[doc = "Field `ODF` writer - Operate done flag"]
pub type OdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operate busy flag"]
    #[inline(always)]
    pub fn obf(&self) -> ObfR {
        ObfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    pub fn prgmerr(&self) -> PrgmerrR {
        PrgmerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    pub fn epperr(&self) -> EpperrR {
        EpperrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    pub fn odf(&self) -> OdfR {
        OdfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS2")
            .field("obf", &self.obf())
            .field("prgmerr", &self.prgmerr())
            .field("epperr", &self.epperr())
            .field("odf", &self.odf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - program error"]
    #[inline(always)]
    #[must_use]
    pub fn prgmerr(&mut self) -> PrgmerrW<Sts2Spec> {
        PrgmerrW::new(self, 2)
    }
    #[doc = "Bit 4 - Erase/program protection error"]
    #[inline(always)]
    #[must_use]
    pub fn epperr(&mut self) -> EpperrW<Sts2Spec> {
        EpperrW::new(self, 4)
    }
    #[doc = "Bit 5 - Operate done flag"]
    #[inline(always)]
    #[must_use]
    pub fn odf(&mut self) -> OdfW<Sts2Spec> {
        OdfW::new(self, 5)
    }
}
#[doc = "Status 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sts2Spec;
impl crate::RegisterSpec for Sts2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts2::R`](R) reader structure"]
impl crate::Readable for Sts2Spec {}
#[doc = "`write(|w| ..)` method takes [`sts2::W`](W) writer structure"]
impl crate::Writable for Sts2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS2 to value 0"]
impl crate::Resettable for Sts2Spec {
    const RESET_VALUE: u32 = 0;
}
