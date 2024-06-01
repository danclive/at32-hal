#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Field `WTCYC` reader - Wait cycle"]
pub type WtcycR = crate::FieldReader;
#[doc = "Field `WTCYC` writer - Wait cycle"]
pub type WtcycW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HFCYC_EN` reader - Half cycle acceleration access enable"]
pub type HfcycEnR = crate::BitReader;
#[doc = "Field `HFCYC_EN` writer - Half cycle acceleration access enable"]
pub type HfcycEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_EN` reader - Prefetch enable"]
pub type PftEnR = crate::BitReader;
#[doc = "Field `PFT_EN` writer - Prefetch enable"]
pub type PftEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFT_ENF` reader - Prefetch enabled flag"]
pub type PftEnfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    pub fn wtcyc(&self) -> WtcycR {
        WtcycR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    pub fn hfcyc_en(&self) -> HfcycEnR {
        HfcycEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    pub fn pft_en(&self) -> PftEnR {
        PftEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Prefetch enabled flag"]
    #[inline(always)]
    pub fn pft_enf(&self) -> PftEnfR {
        PftEnfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSR")
            .field("wtcyc", &self.wtcyc())
            .field("hfcyc_en", &self.hfcyc_en())
            .field("pft_en", &self.pft_en())
            .field("pft_enf", &self.pft_enf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Wait cycle"]
    #[inline(always)]
    #[must_use]
    pub fn wtcyc(&mut self) -> WtcycW<PsrSpec> {
        WtcycW::new(self, 0)
    }
    #[doc = "Bit 3 - Half cycle acceleration access enable"]
    #[inline(always)]
    #[must_use]
    pub fn hfcyc_en(&mut self) -> HfcycEnW<PsrSpec> {
        HfcycEnW::new(self, 3)
    }
    #[doc = "Bit 4 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn pft_en(&mut self) -> PftEnW<PsrSpec> {
        PftEnW::new(self, 4)
    }
}
#[doc = "Performance selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0x30"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0x30;
}
