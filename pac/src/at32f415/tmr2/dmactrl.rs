#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Field `ADDR` reader - DMA transfer address offset"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - DMA transfer address offset"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DTB` reader - DMA transfer bytes"]
pub type DtbR = crate::FieldReader;
#[doc = "Field `DTB` writer - DMA transfer bytes"]
pub type DtbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    pub fn dtb(&self) -> DtbR {
        DtbR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACTRL")
            .field("dtb", &self.dtb())
            .field("addr", &self.addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA transfer address offset"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<DmactrlSpec> {
        AddrW::new(self, 0)
    }
    #[doc = "Bits 8:12 - DMA transfer bytes"]
    #[inline(always)]
    #[must_use]
    pub fn dtb(&mut self) -> DtbW<DmactrlSpec> {
        DtbW::new(self, 8)
    }
}
#[doc = "DMA control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {
    const RESET_VALUE: u32 = 0;
}
