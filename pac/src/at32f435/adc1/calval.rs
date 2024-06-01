#[doc = "Register `CALVAL` reader"]
pub type R = crate::R<CalvalSpec>;
#[doc = "Register `CALVAL` writer"]
pub type W = crate::W<CalvalSpec>;
#[doc = "Field `CALVAL` reader - A/D Calibration value"]
pub type CalvalR = crate::FieldReader;
#[doc = "Field `CALVAL` writer - A/D Calibration value"]
pub type CalvalW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - A/D Calibration value"]
    #[inline(always)]
    pub fn calval(&self) -> CalvalR {
        CalvalR::new((self.bits & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CALVAL")
            .field("calval", &self.calval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - A/D Calibration value"]
    #[inline(always)]
    #[must_use]
    pub fn calval(&mut self) -> CalvalW<CalvalSpec> {
        CalvalW::new(self, 0)
    }
}
#[doc = "Calibration value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalvalSpec;
impl crate::RegisterSpec for CalvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calval::R`](R) reader structure"]
impl crate::Readable for CalvalSpec {}
#[doc = "`write(|w| ..)` method takes [`calval::W`](W) writer structure"]
impl crate::Writable for CalvalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALVAL to value 0"]
impl crate::Resettable for CalvalSpec {
    const RESET_VALUE: u32 = 0;
}
