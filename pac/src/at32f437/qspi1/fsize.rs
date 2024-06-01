#[doc = "Register `FSIZE` reader"]
pub type R = crate::R<FsizeSpec>;
#[doc = "Register `FSIZE` writer"]
pub type W = crate::W<FsizeSpec>;
#[doc = "Field `SPIFSIZE` reader - SPI flash size"]
pub type SpifsizeR = crate::FieldReader<u32>;
#[doc = "Field `SPIFSIZE` writer - SPI flash size"]
pub type SpifsizeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SPI flash size"]
    #[inline(always)]
    pub fn spifsize(&self) -> SpifsizeR {
        SpifsizeR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSIZE")
            .field("spifsize", &self.spifsize())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI flash size"]
    #[inline(always)]
    #[must_use]
    pub fn spifsize(&mut self) -> SpifsizeW<FsizeSpec> {
        SpifsizeW::new(self, 0)
    }
}
#[doc = "SPI flash size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsizeSpec;
impl crate::RegisterSpec for FsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsize::R`](R) reader structure"]
impl crate::Readable for FsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`fsize::W`](W) writer structure"]
impl crate::Writable for FsizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSIZE to value 0"]
impl crate::Resettable for FsizeSpec {
    const RESET_VALUE: u32 = 0;
}
