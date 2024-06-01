#[doc = "Register `MACA0L` reader"]
pub type R = crate::R<Maca0lSpec>;
#[doc = "Register `MACA0L` writer"]
pub type W = crate::W<Maca0lSpec>;
#[doc = "Field `MA0L` reader - MAC address0 low"]
pub type Ma0lR = crate::FieldReader<u32>;
#[doc = "Field `MA0L` writer - MAC address0 low"]
pub type Ma0lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    pub fn ma0l(&self) -> Ma0lR {
        Ma0lR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0L")
            .field("ma0l", &self.ma0l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address0 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma0l(&mut self) -> Ma0lW<Maca0lSpec> {
        Ma0lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 0 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca0lSpec;
impl crate::RegisterSpec for Maca0lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0l::R`](R) reader structure"]
impl crate::Readable for Maca0lSpec {}
#[doc = "`write(|w| ..)` method takes [`maca0l::W`](W) writer structure"]
impl crate::Writable for Maca0lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0L to value 0xffff_ffff"]
impl crate::Resettable for Maca0lSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
