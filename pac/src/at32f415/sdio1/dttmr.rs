#[doc = "Register `DTTMR` reader"]
pub type R = crate::R<DttmrSpec>;
#[doc = "Register `DTTMR` writer"]
pub type W = crate::W<DttmrSpec>;
#[doc = "Field `TIMEOUT` reader - Data timeout period"]
pub type TimeoutR = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT` writer - Data timeout period"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTTMR")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Data timeout period"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<DttmrSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "Bits 31:0 = DATATIME: Data timeout period\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dttmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dttmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DttmrSpec;
impl crate::RegisterSpec for DttmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttmr::R`](R) reader structure"]
impl crate::Readable for DttmrSpec {}
#[doc = "`write(|w| ..)` method takes [`dttmr::W`](W) writer structure"]
impl crate::Writable for DttmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTTMR to value 0"]
impl crate::Resettable for DttmrSpec {
    const RESET_VALUE: u32 = 0;
}
