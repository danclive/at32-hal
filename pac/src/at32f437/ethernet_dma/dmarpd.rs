#[doc = "Register `DMARPD` reader"]
pub type R = crate::R<DmarpdSpec>;
#[doc = "Register `DMARPD` writer"]
pub type W = crate::W<DmarpdSpec>;
#[doc = "Field `RPD` reader - Receive poll demand"]
pub type RpdR = crate::FieldReader<u32>;
#[doc = "Field `RPD` writer - Receive poll demand"]
pub type RpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARPD").field("rpd", &self.rpd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Receive poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn rpd(&mut self) -> RpdW<DmarpdSpec> {
        RpdW::new(self, 0)
    }
}
#[doc = "EHERNET DMA receive poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmarpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarpdSpec;
impl crate::RegisterSpec for DmarpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarpd::R`](R) reader structure"]
impl crate::Readable for DmarpdSpec {}
#[doc = "`write(|w| ..)` method takes [`dmarpd::W`](W) writer structure"]
impl crate::Writable for DmarpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARPD to value 0"]
impl crate::Resettable for DmarpdSpec {
    const RESET_VALUE: u32 = 0;
}
