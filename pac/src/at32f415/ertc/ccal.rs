#[doc = "Register `CCAL` reader"]
pub type R = crate::R<CcalSpec>;
#[doc = "Register `CCAL` writer"]
pub type W = crate::W<CcalSpec>;
#[doc = "Field `CALVAL` reader - Calibration value"]
pub type CalvalR = crate::FieldReader;
#[doc = "Field `CALVAL` writer - Calibration value"]
pub type CalvalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CaldirR = crate::BitReader;
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CaldirW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CalvalR {
        CalvalR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CaldirR {
        CaldirR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCAL")
            .field("caldir", &self.caldir())
            .field("calval", &self.calval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CalvalW<CcalSpec> {
        CalvalW::new(self, 0)
    }
    #[doc = "Bit 7 - Calibration direction"]
    #[inline(always)]
    #[must_use]
    pub fn caldir(&mut self) -> CaldirW<CcalSpec> {
        CaldirW::new(self, 7)
    }
}
#[doc = "Calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcalSpec;
impl crate::RegisterSpec for CcalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccal::R`](R) reader structure"]
impl crate::Readable for CcalSpec {}
#[doc = "`write(|w| ..)` method takes [`ccal::W`](W) writer structure"]
impl crate::Writable for CcalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCAL to value 0"]
impl crate::Resettable for CcalSpec {
    const RESET_VALUE: u32 = 0;
}
