#[doc = "Register `PTPSSINC` reader"]
pub type R = crate::R<PtpssincSpec>;
#[doc = "Register `PTPSSINC` writer"]
pub type W = crate::W<PtpssincSpec>;
#[doc = "Field `SSIV` reader - Sub-second increment value"]
pub type SsivR = crate::FieldReader;
#[doc = "Field `SSIV` writer - Sub-second increment value"]
pub type SsivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Sub-second increment value"]
    #[inline(always)]
    pub fn ssiv(&self) -> SsivR {
        SsivR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPSSINC")
            .field("ssiv", &self.ssiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Sub-second increment value"]
    #[inline(always)]
    #[must_use]
    pub fn ssiv(&mut self) -> SsivW<PtpssincSpec> {
        SsivW::new(self, 0)
    }
}
#[doc = "Ethernet PTP subsecond increment register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptpssinc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptpssinc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtpssincSpec;
impl crate::RegisterSpec for PtpssincSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptpssinc::R`](R) reader structure"]
impl crate::Readable for PtpssincSpec {}
#[doc = "`write(|w| ..)` method takes [`ptpssinc::W`](W) writer structure"]
impl crate::Writable for PtpssincSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPSSINC to value 0"]
impl crate::Resettable for PtpssincSpec {
    const RESET_VALUE: u32 = 0;
}
