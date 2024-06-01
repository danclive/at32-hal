#[doc = "Register `SLIB_STS1` reader"]
pub type R = crate::R<SlibSts1Spec>;
#[doc = "Register `SLIB_STS1` writer"]
pub type W = crate::W<SlibSts1Spec>;
#[doc = "Field `SLIB_SS` reader - sLib start sector"]
pub type SlibSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_SS` writer - sLib start sector"]
pub type SlibSsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_INST_SS` reader - sLib instruction start sector"]
pub type SlibInstSsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_INST_SS` writer - sLib instruction start sector"]
pub type SlibInstSsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `SLIB_ES` reader - sLib end sector"]
pub type SlibEsR = crate::FieldReader<u16>;
#[doc = "Field `SLIB_ES` writer - sLib end sector"]
pub type SlibEsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    pub fn slib_ss(&self) -> SlibSsR {
        SlibSsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:21 - sLib instruction start sector"]
    #[inline(always)]
    pub fn slib_inst_ss(&self) -> SlibInstSsR {
        SlibInstSsR::new(((self.bits >> 11) & 0x07ff) as u16)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    pub fn slib_es(&self) -> SlibEsR {
        SlibEsR::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLIB_STS1")
            .field("slib_ss", &self.slib_ss())
            .field("slib_inst_ss", &self.slib_inst_ss())
            .field("slib_es", &self.slib_es())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - sLib start sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_ss(&mut self) -> SlibSsW<SlibSts1Spec> {
        SlibSsW::new(self, 0)
    }
    #[doc = "Bits 11:21 - sLib instruction start sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_inst_ss(&mut self) -> SlibInstSsW<SlibSts1Spec> {
        SlibInstSsW::new(self, 11)
    }
    #[doc = "Bits 22:31 - sLib end sector"]
    #[inline(always)]
    #[must_use]
    pub fn slib_es(&mut self) -> SlibEsW<SlibSts1Spec> {
        SlibEsW::new(self, 22)
    }
}
#[doc = "sLib status 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slib_sts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slib_sts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlibSts1Spec;
impl crate::RegisterSpec for SlibSts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slib_sts1::R`](R) reader structure"]
impl crate::Readable for SlibSts1Spec {}
#[doc = "`write(|w| ..)` method takes [`slib_sts1::W`](W) writer structure"]
impl crate::Writable for SlibSts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLIB_STS1 to value 0"]
impl crate::Resettable for SlibSts1Spec {
    const RESET_VALUE: u32 = 0;
}
