#[doc = "Register `PTPTSHUD` reader"]
pub type R = crate::R<PtptshudSpec>;
#[doc = "Register `PTPTSHUD` writer"]
pub type W = crate::W<PtptshudSpec>;
#[doc = "Field `TS` reader - Timestamp second"]
pub type TsR = crate::FieldReader<u32>;
#[doc = "Field `TS` writer - Timestamp second"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSHUD").field("ts", &self.ts()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp second"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TsW<PtptshudSpec> {
        TsW::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp high update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptshud::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptshud::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptshudSpec;
impl crate::RegisterSpec for PtptshudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptshud::R`](R) reader structure"]
impl crate::Readable for PtptshudSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptshud::W`](W) writer structure"]
impl crate::Writable for PtptshudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSHUD to value 0"]
impl crate::Resettable for PtptshudSpec {
    const RESET_VALUE: u32 = 0;
}
