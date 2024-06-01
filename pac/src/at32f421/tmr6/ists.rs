#[doc = "Register `ISTS` reader"]
pub type R = crate::R<IstsSpec>;
#[doc = "Register `ISTS` writer"]
pub type W = crate::W<IstsSpec>;
#[doc = "Field `OVFIF` reader - Overflow interrupt flag"]
pub type OvfifR = crate::BitReader;
#[doc = "Field `OVFIF` writer - Overflow interrupt flag"]
pub type OvfifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    pub fn ovfif(&self) -> OvfifR {
        OvfifR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISTS")
            .field("ovfif", &self.ovfif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Overflow interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovfif(&mut self) -> OvfifW<IstsSpec> {
        OvfifW::new(self, 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ists::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ists::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstsSpec;
impl crate::RegisterSpec for IstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ists::R`](R) reader structure"]
impl crate::Readable for IstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ists::W`](W) writer structure"]
impl crate::Writable for IstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISTS to value 0"]
impl crate::Resettable for IstsSpec {
    const RESET_VALUE: u32 = 0;
}
