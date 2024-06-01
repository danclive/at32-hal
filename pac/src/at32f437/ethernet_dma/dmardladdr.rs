#[doc = "Register `DMARDLADDR` reader"]
pub type R = crate::R<DmardladdrSpec>;
#[doc = "Register `DMARDLADDR` writer"]
pub type W = crate::W<DmardladdrSpec>;
#[doc = "Field `SRL` reader - Start of receive list"]
pub type SrlR = crate::FieldReader<u32>;
#[doc = "Field `SRL` writer - Start of receive list"]
pub type SrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    pub fn srl(&self) -> SrlR {
        SrlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMARDLADDR")
            .field("srl", &self.srl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of receive list"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SrlW<DmardladdrSpec> {
        SrlW::new(self, 0)
    }
}
#[doc = "Ethernet DMA receive descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmardladdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmardladdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmardladdrSpec;
impl crate::RegisterSpec for DmardladdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmardladdr::R`](R) reader structure"]
impl crate::Readable for DmardladdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmardladdr::W`](W) writer structure"]
impl crate::Writable for DmardladdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMARDLADDR to value 0"]
impl crate::Resettable for DmardladdrSpec {
    const RESET_VALUE: u32 = 0;
}
