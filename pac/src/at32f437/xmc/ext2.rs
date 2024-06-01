#[doc = "Register `EXT2` reader"]
pub type R = crate::R<Ext2Spec>;
#[doc = "Register `EXT2` writer"]
pub type W = crate::W<Ext2Spec>;
#[doc = "Field `BUSLATW2W` reader - Bus turnaround phase for consecutive write duration"]
pub type Buslatw2wR = crate::FieldReader;
#[doc = "Field `BUSLATW2W` writer - Bus turnaround phase for consecutive write duration"]
pub type Buslatw2wW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLATR2R` reader - Bus turnaround phase for consecutive read duration"]
pub type Buslatr2rR = crate::FieldReader;
#[doc = "Field `BUSLATR2R` writer - Bus turnaround phase for consecutive read duration"]
pub type Buslatr2rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bus turnaround phase for consecutive write duration"]
    #[inline(always)]
    pub fn buslatw2w(&self) -> Buslatw2wR {
        Buslatw2wR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Bus turnaround phase for consecutive read duration"]
    #[inline(always)]
    pub fn buslatr2r(&self) -> Buslatr2rR {
        Buslatr2rR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT2")
            .field("buslatw2w", &self.buslatw2w())
            .field("buslatr2r", &self.buslatr2r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus turnaround phase for consecutive write duration"]
    #[inline(always)]
    #[must_use]
    pub fn buslatw2w(&mut self) -> Buslatw2wW<Ext2Spec> {
        Buslatw2wW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Bus turnaround phase for consecutive read duration"]
    #[inline(always)]
    #[must_use]
    pub fn buslatr2r(&mut self) -> Buslatr2rW<Ext2Spec> {
        Buslatr2rW::new(self, 8)
    }
}
#[doc = "externl timeing register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ext2Spec;
impl crate::RegisterSpec for Ext2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext2::R`](R) reader structure"]
impl crate::Readable for Ext2Spec {}
#[doc = "`write(|w| ..)` method takes [`ext2::W`](W) writer structure"]
impl crate::Writable for Ext2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT2 to value 0x0808"]
impl crate::Resettable for Ext2Spec {
    const RESET_VALUE: u32 = 0x0808;
}
