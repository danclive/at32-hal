#[doc = "Register `CNTH` reader"]
pub type R = crate::R<CnthSpec>;
#[doc = "Register `CNTH` writer"]
pub type W = crate::W<CnthSpec>;
#[doc = "Field `CNT` reader - RTC counter register high"]
pub type CntR = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - RTC counter register high"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTH").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<CnthSpec> {
        CntW::new(self, 0)
    }
}
#[doc = "RTC Counter Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CnthSpec;
impl crate::RegisterSpec for CnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnth::R`](R) reader structure"]
impl crate::Readable for CnthSpec {}
#[doc = "`write(|w| ..)` method takes [`cnth::W`](W) writer structure"]
impl crate::Writable for CnthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CnthSpec {
    const RESET_VALUE: u32 = 0;
}
