#[doc = "Register `ARGU` reader"]
pub type R = crate::R<ArguSpec>;
#[doc = "Register `ARGU` writer"]
pub type W = crate::W<ArguSpec>;
#[doc = "Field `ARGU` reader - Command argument"]
pub type ArguR = crate::FieldReader<u32>;
#[doc = "Field `ARGU` writer - Command argument"]
pub type ArguW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    pub fn argu(&self) -> ArguR {
        ArguR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARGU").field("argu", &self.argu()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Command argument"]
    #[inline(always)]
    #[must_use]
    pub fn argu(&mut self) -> ArguW<ArguSpec> {
        ArguW::new(self, 0)
    }
}
#[doc = "Bits 31:0 = : Command argument\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`argu::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`argu::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArguSpec;
impl crate::RegisterSpec for ArguSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`argu::R`](R) reader structure"]
impl crate::Readable for ArguSpec {}
#[doc = "`write(|w| ..)` method takes [`argu::W`](W) writer structure"]
impl crate::Writable for ArguSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARGU to value 0"]
impl crate::Resettable for ArguSpec {
    const RESET_VALUE: u32 = 0;
}
