#[doc = "Register `MACA3L` reader"]
pub type R = crate::R<Maca3lSpec>;
#[doc = "Register `MACA3L` writer"]
pub type W = crate::W<Maca3lSpec>;
#[doc = "Field `MA3L` reader - MAC address3 low"]
pub type Ma3lR = crate::FieldReader<u32>;
#[doc = "Field `MA3L` writer - MAC address3 low"]
pub type Ma3lW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    pub fn ma3l(&self) -> Ma3lR {
        Ma3lR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA3L")
            .field("ma3l", &self.ma3l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC address3 low"]
    #[inline(always)]
    #[must_use]
    pub fn ma3l(&mut self) -> Ma3lW<Maca3lSpec> {
        Ma3lW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 3 low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca3lSpec;
impl crate::RegisterSpec for Maca3lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3l::R`](R) reader structure"]
impl crate::Readable for Maca3lSpec {}
#[doc = "`write(|w| ..)` method takes [`maca3l::W`](W) writer structure"]
impl crate::Writable for Maca3lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3L to value 0xffff_ffff"]
impl crate::Resettable for Maca3lSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
