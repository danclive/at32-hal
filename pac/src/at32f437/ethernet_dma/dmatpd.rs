#[doc = "Register `DMATPD` reader"]
pub type R = crate::R<DmatpdSpec>;
#[doc = "Register `DMATPD` writer"]
pub type W = crate::W<DmatpdSpec>;
#[doc = "Field `TPD` reader - Transmit poll demand"]
pub type TpdR = crate::FieldReader<u32>;
#[doc = "Field `TPD` writer - Transmit poll demand"]
pub type TpdW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATPD").field("tpd", &self.tpd()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit poll demand"]
    #[inline(always)]
    #[must_use]
    pub fn tpd(&mut self) -> TpdW<DmatpdSpec> {
        TpdW::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit poll demand register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatpd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatpd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatpdSpec;
impl crate::RegisterSpec for DmatpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatpd::R`](R) reader structure"]
impl crate::Readable for DmatpdSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatpd::W`](W) writer structure"]
impl crate::Writable for DmatpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATPD to value 0"]
impl crate::Resettable for DmatpdSpec {
    const RESET_VALUE: u32 = 0;
}
