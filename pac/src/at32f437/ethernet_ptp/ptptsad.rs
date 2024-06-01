#[doc = "Register `PTPTSAD` reader"]
pub type R = crate::R<PtptsadSpec>;
#[doc = "Register `PTPTSAD` writer"]
pub type W = crate::W<PtptsadSpec>;
#[doc = "Field `TAR` reader - Timestamp addend register"]
pub type TarR = crate::FieldReader<u32>;
#[doc = "Field `TAR` writer - Timestamp addend register"]
pub type TarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    pub fn tar(&self) -> TarR {
        TarR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSAD").field("tar", &self.tar()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp addend register"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TarW<PtptsadSpec> {
        TarW::new(self, 0)
    }
}
#[doc = "Ethernet PTP time stamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsad::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsad::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptsadSpec;
impl crate::RegisterSpec for PtptsadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsad::R`](R) reader structure"]
impl crate::Readable for PtptsadSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptsad::W`](W) writer structure"]
impl crate::Writable for PtptsadSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSAD to value 0"]
impl crate::Resettable for PtptsadSpec {
    const RESET_VALUE: u32 = 0;
}
