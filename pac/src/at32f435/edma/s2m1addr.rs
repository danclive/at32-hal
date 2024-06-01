#[doc = "Register `S2M1ADDR` reader"]
pub type R = crate::R<S2m1addrSpec>;
#[doc = "Register `S2M1ADDR` writer"]
pub type W = crate::W<S2m1addrSpec>;
#[doc = "Field `M1ADDR` reader - Memory 1 address (used in case of Double buffer mode)"]
pub type M1addrR = crate::FieldReader<u32>;
#[doc = "Field `M1ADDR` writer - Memory 1 address (used in case of Double buffer mode)"]
pub type M1addrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    pub fn m1addr(&self) -> M1addrR {
        M1addrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S2M1ADDR")
            .field("m1addr", &self.m1addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 1 address (used in case of Double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn m1addr(&mut self) -> M1addrW<S2m1addrSpec> {
        M1addrW::new(self, 0)
    }
}
#[doc = "stream 2 memory 1 address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s2m1addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s2m1addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2m1addrSpec;
impl crate::RegisterSpec for S2m1addrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2m1addr::R`](R) reader structure"]
impl crate::Readable for S2m1addrSpec {}
#[doc = "`write(|w| ..)` method takes [`s2m1addr::W`](W) writer structure"]
impl crate::Writable for S2m1addrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S2M1ADDR to value 0"]
impl crate::Resettable for S2m1addrSpec {
    const RESET_VALUE: u32 = 0;
}
