#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `DIVF` reader - Division value update complete flag"]
pub type DivfR = crate::BitReader;
#[doc = "Field `DIVF` writer - Division value update complete flag"]
pub type DivfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RLDF` reader - Reload value update complete flag"]
pub type RldfR = crate::BitReader;
#[doc = "Field `RLDF` writer - Reload value update complete flag"]
pub type RldfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Division value update complete flag"]
    #[inline(always)]
    pub fn divf(&self) -> DivfR {
        DivfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reload value update complete flag"]
    #[inline(always)]
    pub fn rldf(&self) -> RldfR {
        RldfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("divf", &self.divf())
            .field("rldf", &self.rldf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Division value update complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn divf(&mut self) -> DivfW<StsSpec> {
        DivfW::new(self, 0)
    }
    #[doc = "Bit 1 - Reload value update complete flag"]
    #[inline(always)]
    #[must_use]
    pub fn rldf(&mut self) -> RldfW<StsSpec> {
        RldfW::new(self, 1)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
