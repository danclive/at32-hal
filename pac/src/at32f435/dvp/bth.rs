#[doc = "Register `BTH` reader"]
pub type R = crate::R<BthSpec>;
#[doc = "Register `BTH` writer"]
pub type W = crate::W<BthSpec>;
#[doc = "Field `MIBTHD` reader - Monochrome image binarization threshold"]
pub type MibthdR = crate::FieldReader;
#[doc = "Field `MIBTHD` writer - Monochrome image binarization threshold"]
pub type MibthdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Monochrome image binarization threshold"]
    #[inline(always)]
    pub fn mibthd(&self) -> MibthdR {
        MibthdR::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTH")
            .field("mibthd", &self.mibthd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Monochrome image binarization threshold"]
    #[inline(always)]
    #[must_use]
    pub fn mibthd(&mut self) -> MibthdW<BthSpec> {
        MibthdW::new(self, 0)
    }
}
#[doc = "Binarization threshold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BthSpec;
impl crate::RegisterSpec for BthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bth::R`](R) reader structure"]
impl crate::Readable for BthSpec {}
#[doc = "`write(|w| ..)` method takes [`bth::W`](W) writer structure"]
impl crate::Writable for BthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTH to value 0"]
impl crate::Resettable for BthSpec {
    const RESET_VALUE: u32 = 0;
}
