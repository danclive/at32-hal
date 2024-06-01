#[doc = "Register `S8M0ADDR` reader"]
pub type R = crate::R<S8m0addrSpec>;
#[doc = "Register `S8M0ADDR` writer"]
pub type W = crate::W<S8m0addrSpec>;
#[doc = "Field `M0ADDR` reader - Memory 0 address"]
pub type M0addrR = crate::FieldReader<u32>;
#[doc = "Field `M0ADDR` writer - Memory 0 address"]
pub type M0addrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0addr(&self) -> M0addrR {
        M0addrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S8M0ADDR")
            .field("m0addr", &self.m0addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    #[must_use]
    pub fn m0addr(&mut self) -> M0addrW<S8m0addrSpec> {
        M0addrW::new(self, 0)
    }
}
#[doc = "stream 8 memory 0 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s8m0addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s8m0addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S8m0addrSpec;
impl crate::RegisterSpec for S8m0addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s8m0addr::R`](R) reader structure"]
impl crate::Readable for S8m0addrSpec {}
#[doc = "`write(|w| ..)` method takes [`s8m0addr::W`](W) writer structure"]
impl crate::Writable for S8m0addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S8M0ADDR to value 0"]
impl crate::Resettable for S8m0addrSpec {
    const RESET_VALUE: u32 = 0;
}
