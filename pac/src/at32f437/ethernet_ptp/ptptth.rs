#[doc = "Register `PTPTTH` reader"]
pub type R = crate::R<PtptthSpec>;
#[doc = "Register `PTPTTH` writer"]
pub type W = crate::W<PtptthSpec>;
#[doc = "Field `TTSR` reader - Target time seconds register"]
pub type TtsrR = crate::FieldReader<u32>;
#[doc = "Field `TTSR` writer - Target time seconds register"]
pub type TtsrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    pub fn ttsr(&self) -> TtsrR {
        TtsrR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTTH")
            .field("ttsr", &self.ttsr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Target time seconds register"]
    #[inline(always)]
    #[must_use]
    pub fn ttsr(&mut self) -> TtsrW<PtptthSpec> {
        TtsrW::new(self, 0)
    }
}
#[doc = "Ethernet PTP target time high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptthSpec;
impl crate::RegisterSpec for PtptthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptth::R`](R) reader structure"]
impl crate::Readable for PtptthSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptth::W`](W) writer structure"]
impl crate::Writable for PtptthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTTH to value 0"]
impl crate::Resettable for PtptthSpec {
    const RESET_VALUE: u32 = 0;
}
