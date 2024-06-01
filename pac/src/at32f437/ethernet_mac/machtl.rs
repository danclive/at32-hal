#[doc = "Register `MACHTL` reader"]
pub type R = crate::R<MachtlSpec>;
#[doc = "Register `MACHTL` writer"]
pub type W = crate::W<MachtlSpec>;
#[doc = "Field `HTL` reader - Hash table low"]
pub type HtlR = crate::FieldReader<u32>;
#[doc = "Field `HTL` writer - Hash table low"]
pub type HtlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    pub fn htl(&self) -> HtlR {
        HtlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHTL").field("htl", &self.htl()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table low"]
    #[inline(always)]
    #[must_use]
    pub fn htl(&mut self) -> HtlW<MachtlSpec> {
        HtlW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machtl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machtl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MachtlSpec;
impl crate::RegisterSpec for MachtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machtl::R`](R) reader structure"]
impl crate::Readable for MachtlSpec {}
#[doc = "`write(|w| ..)` method takes [`machtl::W`](W) writer structure"]
impl crate::Writable for MachtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTL to value 0"]
impl crate::Resettable for MachtlSpec {
    const RESET_VALUE: u32 = 0;
}
