#[doc = "Register `C7MADDR` reader"]
pub type R = crate::R<C7maddrSpec>;
#[doc = "Register `C7MADDR` writer"]
pub type W = crate::W<C7maddrSpec>;
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
        f.debug_struct("C7MADDR")
            .field("maddr", &self.maddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MaddrW<C7maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 7 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C7maddrSpec;
impl crate::RegisterSpec for C7maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c7maddr::R`](R) reader structure"]
impl crate::Readable for C7maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c7maddr::W`](W) writer structure"]
impl crate::Writable for C7maddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C7MADDR to value 0"]
impl crate::Resettable for C7maddrSpec {
    const RESET_VALUE: u32 = 0;
}
