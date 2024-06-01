#[doc = "Register `FCTRL` reader"]
pub type R = crate::R<FctrlSpec>;
#[doc = "Register `FCTRL` writer"]
pub type W = crate::W<FctrlSpec>;
#[doc = "Field `FCS` reader - Filters configure switch"]
pub type FcsR = crate::BitReader;
#[doc = "Field `FCS` writer - Filters configure switch"]
pub type FcsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    pub fn fcs(&self) -> FcsR {
        FcsR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FCTRL").field("fcs", &self.fcs()).finish()
    }
}
impl W {
    #[doc = "Bit 0 - Filters configure switch"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FcsW<FctrlSpec> {
        FcsW::new(self, 0)
    }
}
#[doc = "Filter control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FctrlSpec;
impl crate::RegisterSpec for FctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fctrl::R`](R) reader structure"]
impl crate::Readable for FctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fctrl::W`](W) writer structure"]
impl crate::Writable for FctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCTRL to value 0"]
impl crate::Resettable for FctrlSpec {
    const RESET_VALUE: u32 = 0;
}
