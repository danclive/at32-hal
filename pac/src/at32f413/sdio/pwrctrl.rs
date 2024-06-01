#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PwrctrlSpec>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PwrctrlSpec>;
#[doc = "Field `PS` reader - Power switch"]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - Power switch"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCTRL").field("ps", &self.ps()).finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Power switch"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<PwrctrlSpec> {
        PsW::new(self, 0)
    }
}
#[doc = "Bits 1:0 = PWRCTRL: Power supply control bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctrlSpec;
impl crate::RegisterSpec for PwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PwrctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PwrctrlSpec {
    const RESET_VALUE: u32 = 0;
}
