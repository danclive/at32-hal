#[doc = "Register `BPR19DT` reader"]
pub type R = crate::R<Bpr19dtSpec>;
#[doc = "Register `BPR19DT` writer"]
pub type W = crate::W<Bpr19dtSpec>;
#[doc = "Field `DT` reader - Battery powered domain data"]
pub type DtR = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - Battery powered domain data"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BPR19DT").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Battery powered domain data"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<Bpr19dtSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "Battery powered domain register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bpr19dt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bpr19dt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bpr19dtSpec;
impl crate::RegisterSpec for Bpr19dtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpr19dt::R`](R) reader structure"]
impl crate::Readable for Bpr19dtSpec {}
#[doc = "`write(|w| ..)` method takes [`bpr19dt::W`](W) writer structure"]
impl crate::Writable for Bpr19dtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPR19DT to value 0"]
impl crate::Resettable for Bpr19dtSpec {
    const RESET_VALUE: u32 = 0;
}
