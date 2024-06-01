#[doc = "Register `MACA0H` reader"]
pub type R = crate::R<Maca0hSpec>;
#[doc = "Register `MACA0H` writer"]
pub type W = crate::W<Maca0hSpec>;
#[doc = "Field `MA0H` reader - MAC address0 high"]
pub type Ma0hR = crate::FieldReader<u16>;
#[doc = "Field `MA0H` writer - MAC address0 high"]
pub type Ma0hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AE` reader - Address enable"]
pub type AeR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    pub fn ma0h(&self) -> Ma0hR {
        Ma0hR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AeR {
        AeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACA0H")
            .field("ma0h", &self.ma0h())
            .field("ae", &self.ae())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address0 high"]
    #[inline(always)]
    #[must_use]
    pub fn ma0h(&mut self) -> Ma0hW<Maca0hSpec> {
        Ma0hW::new(self, 0)
    }
}
#[doc = "Ethernet MAC address 0 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca0h::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca0h::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Maca0hSpec;
impl crate::RegisterSpec for Maca0hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca0h::R`](R) reader structure"]
impl crate::Readable for Maca0hSpec {}
#[doc = "`write(|w| ..)` method takes [`maca0h::W`](W) writer structure"]
impl crate::Writable for Maca0hSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA0H to value 0x0010_ffff"]
impl crate::Resettable for Maca0hSpec {
    const RESET_VALUE: u32 = 0x0010_ffff;
}
