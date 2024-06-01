#[doc = "Register `C1MADDR` reader"]
pub type R = crate::R<C1maddrSpec>;
#[doc = "Register `C1MADDR` writer"]
pub type W = crate::W<C1maddrSpec>;
#[doc = "Field `MADDR` reader - Memory address"]
pub type MaddrR = crate::FieldReader<u32>;
#[doc = "Field `MADDR` writer - Memory address"]
pub type MaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn maddr(&self) -> MaddrR {
        MaddrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1MADDR")
            .field("maddr", &self.maddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MaddrW<C1maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 1 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1maddrSpec;
impl crate::RegisterSpec for C1maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1maddr::R`](R) reader structure"]
impl crate::Readable for C1maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1maddr::W`](W) writer structure"]
impl crate::Writable for C1maddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C1MADDR to value 0"]
impl crate::Resettable for C1maddrSpec {
    const RESET_VALUE: u32 = 0;
}
