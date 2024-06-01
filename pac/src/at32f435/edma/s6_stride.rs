#[doc = "Register `S6_STRIDE` reader"]
pub type R = crate::R<S6StrideSpec>;
#[doc = "Register `S6_STRIDE` writer"]
pub type W = crate::W<S6StrideSpec>;
#[doc = "Field `SRCSTD` reader - Source stride"]
pub type SrcstdR = crate::FieldReader<u16>;
#[doc = "Field `SRCSTD` writer - Source stride"]
pub type SrcstdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DSTSTD` reader - Destination stride"]
pub type DststdR = crate::FieldReader<u16>;
#[doc = "Field `DSTSTD` writer - Destination stride"]
pub type DststdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    pub fn srcstd(&self) -> SrcstdR {
        SrcstdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    pub fn dststd(&self) -> DststdR {
        DststdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S6_STRIDE")
            .field("srcstd", &self.srcstd())
            .field("dststd", &self.dststd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Source stride"]
    #[inline(always)]
    #[must_use]
    pub fn srcstd(&mut self) -> SrcstdW<S6StrideSpec> {
        SrcstdW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Destination stride"]
    #[inline(always)]
    #[must_use]
    pub fn dststd(&mut self) -> DststdW<S6StrideSpec> {
        DststdW::new(self, 16)
    }
}
#[doc = "Stream 6 2D Transfer Stride\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s6_stride::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s6_stride::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S6StrideSpec;
impl crate::RegisterSpec for S6StrideSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s6_stride::R`](R) reader structure"]
impl crate::Readable for S6StrideSpec {}
#[doc = "`write(|w| ..)` method takes [`s6_stride::W`](W) writer structure"]
impl crate::Writable for S6StrideSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S6_STRIDE to value 0"]
impl crate::Resettable for S6StrideSpec {
    const RESET_VALUE: u32 = 0;
}
