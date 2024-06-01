#[doc = "Register `DMADT` reader"]
pub type R = crate::R<DmadtSpec>;
#[doc = "Register `DMADT` writer"]
pub type W = crate::W<DmadtSpec>;
#[doc = "Field `DMADT` reader - DMA data register"]
pub type DmadtR = crate::FieldReader<u16>;
#[doc = "Field `DMADT` writer - DMA data register"]
pub type DmadtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    pub fn dmadt(&self) -> DmadtR {
        DmadtR::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMADT")
            .field("dmadt", &self.dmadt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - DMA data register"]
    #[inline(always)]
    #[must_use]
    pub fn dmadt(&mut self) -> DmadtW<DmadtSpec> {
        DmadtW::new(self, 0)
    }
}
#[doc = "DMA data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmadt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmadt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadtSpec;
impl crate::RegisterSpec for DmadtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadt::R`](R) reader structure"]
impl crate::Readable for DmadtSpec {}
#[doc = "`write(|w| ..)` method takes [`dmadt::W`](W) writer structure"]
impl crate::Writable for DmadtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMADT to value 0"]
impl crate::Resettable for DmadtSpec {
    const RESET_VALUE: u32 = 0;
}
