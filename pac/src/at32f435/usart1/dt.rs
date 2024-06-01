#[doc = "Register `DT` reader"]
pub type R = crate::R<DtSpec>;
#[doc = "Register `DT` writer"]
pub type W = crate::W<DtSpec>;
#[doc = "Field `DT` reader - Data value"]
pub type DtR = crate::FieldReader<u16>;
#[doc = "Field `DT` writer - Data value"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Data value"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0x01ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Data value"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<DtSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtSpec;
impl crate::RegisterSpec for DtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DtSpec {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DT to value 0"]
impl crate::Resettable for DtSpec {
    const RESET_VALUE: u32 = 0;
}
