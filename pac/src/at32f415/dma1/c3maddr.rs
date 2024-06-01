#[doc = "Register `C3MADDR` reader"]
pub type R = crate::R<C3maddrSpec>;
#[doc = "Register `C3MADDR` writer"]
pub type W = crate::W<C3maddrSpec>;
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
        f.debug_struct("C3MADDR")
            .field("maddr", &self.maddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    #[must_use]
    pub fn maddr(&mut self) -> MaddrW<C3maddrSpec> {
        MaddrW::new(self, 0)
    }
}
#[doc = "DMA channel 3 memory base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3maddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3maddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3maddrSpec;
impl crate::RegisterSpec for C3maddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c3maddr::R`](R) reader structure"]
impl crate::Readable for C3maddrSpec {}
#[doc = "`write(|w| ..)` method takes [`c3maddr::W`](W) writer structure"]
impl crate::Writable for C3maddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C3MADDR to value 0"]
impl crate::Resettable for C3maddrSpec {
    const RESET_VALUE: u32 = 0;
}
