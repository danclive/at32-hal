#[doc = "Register `C1PADDR` reader"]
pub type R = crate::R<C1paddrSpec>;
#[doc = "Register `C1PADDR` writer"]
pub type W = crate::W<C1paddrSpec>;
#[doc = "Field `PADDR` reader - Peripheral address"]
pub type PaddrR = crate::FieldReader<u32>;
#[doc = "Field `PADDR` writer - Peripheral address"]
pub type PaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    pub fn paddr(&self) -> PaddrR {
        PaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1PADDR")
            .field("paddr", &self.paddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address"]
    #[inline(always)]
    #[must_use]
    pub fn paddr(&mut self) -> PaddrW<C1paddrSpec> {
        PaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 1 peripheral base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1paddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1paddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1paddrSpec;
impl crate::RegisterSpec for C1paddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1paddr::R`](R) reader structure"]
impl crate::Readable for C1paddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1paddr::W`](W) writer structure"]
impl crate::Writable for C1paddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1PADDR to value 0"]
impl crate::Resettable for C1paddrSpec {
    const RESET_VALUE: u32 = 0;
}
