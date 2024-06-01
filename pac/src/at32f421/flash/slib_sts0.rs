#[doc = "Register `SLIB_STS0` reader"]
pub type R = crate::R<SlibSts0Spec>;
#[doc = "Register `SLIB_STS0` writer"]
pub type W = crate::W<SlibSts0Spec>;
#[doc = "Field `BTM_AP_ENF` reader - Boot memory store application code enabled flag"]
pub type BtmApEnfR = crate::BitReader;
#[doc = "Field `BTM_AP_ENF` writer - Boot memory store application code enabled flag"]
pub type BtmApEnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM_SLIB_ENF` reader - Extension memory sLib enabled flag"]
pub type EmSlibEnfR = crate::BitReader;
#[doc = "Field `EM_SLIB_ENF` writer - Extension memory sLib enabled flag"]
pub type EmSlibEnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLIB_ENF` reader - sLib enabled flag"]
pub type SlibEnfR = crate::BitReader;
#[doc = "Field `SLIB_ENF` writer - sLib enabled flag"]
pub type SlibEnfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM_SLIB_INST_SS` reader - Extension memory sLib instruction start sector"]
pub type EmSlibInstSsR = crate::FieldReader;
#[doc = "Field `EM_SLIB_INST_SS` writer - Extension memory sLib instruction start sector"]
pub type EmSlibInstSsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Boot memory store application code enabled flag"]
    #[inline(always)]
    pub fn btm_ap_enf(&self) -> BtmApEnfR {
        BtmApEnfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Extension memory sLib enabled flag"]
    #[inline(always)]
    pub fn em_slib_enf(&self) -> EmSlibEnfR {
        EmSlibEnfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    pub fn slib_enf(&self) -> SlibEnfR {
        SlibEnfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Extension memory sLib instruction start sector"]
    #[inline(always)]
    pub fn em_slib_inst_ss(&self) -> EmSlibInstSsR {
        EmSlibInstSsR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS0")
            .field("btm_ap_enf", &self.btm_ap_enf())
            .field("em_slib_enf", &self.em_slib_enf())
            .field("slib_enf", &self.slib_enf())
            .field("em_slib_inst_ss", &self.em_slib_inst_ss())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Boot memory store application code enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn btm_ap_enf(&mut self) -> BtmApEnfW<SlibSts0Spec> {
        BtmApEnfW::new(self, 0)
    }
    #[doc = "Bit 2 - Extension memory sLib enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_enf(&mut self) -> EmSlibEnfW<SlibSts0Spec> {
        EmSlibEnfW::new(self, 2)
    }
    #[doc = "Bit 3 - sLib enabled flag"]
    #[inline(always)]
    #[must_use]
    pub fn slib_enf(&mut self) -> SlibEnfW<SlibSts0Spec> {
        SlibEnfW::new(self, 3)
    }
    #[doc = "Bits 16:23 - Extension memory sLib instruction start sector"]
    #[inline(always)]
    #[must_use]
    pub fn em_slib_inst_ss(&mut self) -> EmSlibInstSsW<SlibSts0Spec> {
        EmSlibInstSsW::new(self, 16)
    }
}
#[doc = "sLib status 0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts0Spec;
impl crate::RegisterSpec for SlibSts0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts0::R`](R) reader structure"]
impl crate::Readable for SlibSts0Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts0::W`](W) writer structure"]
impl crate::Writable for SlibSts0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS0 to value 0"]
impl crate::Resettable for SlibSts0Spec {
    const RESET_VALUE: u32 = 0;
}
