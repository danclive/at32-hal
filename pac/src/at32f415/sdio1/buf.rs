#[doc = "Register `BUF` reader"]
pub type R = crate::R<BufSpec>;
#[doc = "Register `BUF` writer"]
pub type W = crate::W<BufSpec>;
#[doc = "Field `DT` reader - FIFOData"]
pub type DtR = crate::FieldReader<u32>;
#[doc = "Field `DT` writer - FIFOData"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIFOData"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUF").field("dt", &self.dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - FIFOData"]
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DtW<BufSpec> {
        DtW::new(self, 0)
    }
}
#[doc = "bits 31:0 = FIFOData: Receive and transmit FIFO data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`buf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufSpec;
impl crate::RegisterSpec for BufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf::R`](R) reader structure"]
impl crate::Readable for BufSpec {}
#[doc = "`write(|w| ..)` method takes [`buf::W`](W) writer structure"]
impl crate::Writable for BufSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUF to value 0"]
impl crate::Resettable for BufSpec {
    const RESET_VALUE: u32 = 0;
}
