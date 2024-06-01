#[doc = "Register `MACHTH` reader"]
pub type R = crate::R<MachthSpec>;
#[doc = "Register `MACHTH` writer"]
pub type W = crate::W<MachthSpec>;
#[doc = "Field `HTH` reader - Hash table high"]
pub type HthR = crate::FieldReader<u32>;
#[doc = "Field `HTH` writer - Hash table high"]
pub type HthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    pub fn hth(&self) -> HthR {
        HthR::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACHTH").field("hth", &self.hth()).finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Hash table high"]
    #[inline(always)]
    #[must_use]
    pub fn hth(&mut self) -> HthW<MachthSpec> {
        HthW::new(self, 0)
    }
}
#[doc = "Ethernet MAC hash table high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`machth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`machth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MachthSpec;
impl crate::RegisterSpec for MachthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machth::R`](R) reader structure"]
impl crate::Readable for MachthSpec {}
#[doc = "`write(|w| ..)` method takes [`machth::W`](W) writer structure"]
impl crate::Writable for MachthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACHTH to value 0"]
impl crate::Resettable for MachthSpec {
    const RESET_VALUE: u32 = 0;
}
