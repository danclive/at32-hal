#[doc = "Register `DMATDLADDR` reader"]
pub type R = crate::R<DmatdladdrSpec>;
#[doc = "Register `DMATDLADDR` writer"]
pub type W = crate::W<DmatdladdrSpec>;
#[doc = "Field `STL` reader - Start of transmit list"]
pub type StlR = crate::FieldReader<u32>;
#[doc = "Field `STL` writer - Start of transmit list"]
pub type StlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    pub fn stl(&self) -> StlR {
        StlR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMATDLADDR")
            .field("stl", &self.stl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of transmit list"]
    #[inline(always)]
    #[must_use]
    pub fn stl(&mut self) -> StlW<DmatdladdrSpec> {
        StlW::new(self, 0)
    }
}
#[doc = "Ethernet DMA transmit descriptor list address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatdladdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmatdladdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmatdladdrSpec;
impl crate::RegisterSpec for DmatdladdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatdladdr::R`](R) reader structure"]
impl crate::Readable for DmatdladdrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmatdladdr::W`](W) writer structure"]
impl crate::Writable for DmatdladdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMATDLADDR to value 0"]
impl crate::Resettable for DmatdladdrSpec {
    const RESET_VALUE: u32 = 0;
}
