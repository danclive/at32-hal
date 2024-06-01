#[doc = "Register `BK3ECC` reader"]
pub type R = crate::R<Bk3eccSpec>;
#[doc = "Register `BK3ECC` writer"]
pub type W = crate::W<Bk3eccSpec>;
#[doc = "Field `ECC` reader - ECC result"]
pub type EccR = crate::FieldReader<u32>;
#[doc = "Field `ECC` writer - ECC result"]
pub type EccW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK3ECC").field("ecc", &self.ecc()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC result"]
    #[inline(always)]
    #[must_use]
    pub fn ecc(&mut self) -> EccW<Bk3eccSpec> {
        EccW::new(self, 0)
    }
}
#[doc = "ECC result register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk3ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk3ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk3eccSpec;
impl crate::RegisterSpec for Bk3eccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk3ecc::R`](R) reader structure"]
impl crate::Readable for Bk3eccSpec {}
#[doc = "`write(|w| ..)` method takes [`bk3ecc::W`](W) writer structure"]
impl crate::Writable for Bk3eccSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK3ECC to value 0"]
impl crate::Resettable for Bk3eccSpec {
    const RESET_VALUE: u32 = 0;
}
