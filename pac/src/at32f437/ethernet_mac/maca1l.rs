#[doc = "Register `MACA1L` reader"]
pub type R = crate::R<Maca1lSpec>;
#[doc = "Register `MACA1L` writer"]
pub type W = crate::W<Maca1lSpec>;
#[doc = "Field `MA1L` reader - MAC address1 low"]
pub type Ma1lR = crate::FieldReader<u32>;
#[doc = "Field `MA1L` writer - MAC address1 low"]
pub type Ma1lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    pub fn ma1l(&self) -> Ma1lR {
        Ma1lR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA1L")
            .field("ma1l", &self.ma1l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address1 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma1l(&mut self) -> Ma1lW<Maca1lSpec> {
        Ma1lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address1 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca1l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca1l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca1lSpec;
impl crate::RegisterSpec for Maca1lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca1l::R`](R) reader structure"]
impl crate::Readable for Maca1lSpec {}
#[doc = "`write(|w| ..)` method takes [`maca1l::W`](W) writer structure"]
impl crate::Writable for Maca1lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA1L to value 0xffff_ffff"]
impl crate::Resettable for Maca1lSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
