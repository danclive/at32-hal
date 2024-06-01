#[doc = "Register `DIVCNTH` reader"]
pub type R = crate::R<DivcnthSpec>;
#[doc = "Register `DIVCNTH` writer"]
pub type W = crate::W<DivcnthSpec>;
#[doc = "Field `DIVCNT` reader - RTC divider register high"]
pub type DivcntR = crate::FieldReader;
#[doc = "Field `DIVCNT` writer - RTC divider register high"]
pub type DivcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    pub fn divcnt(&self) -> DivcntR {
        DivcntR::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIVCNTH")
            .field("divcnt", &self.divcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC divider register high"]
    #[inline(always)]
    #[must_use]
    pub fn divcnt(&mut self) -> DivcntW<DivcnthSpec> {
        DivcntW::new(self, 0)
    }
}
#[doc = "RTC Divider Register High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcnth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcnth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivcnthSpec;
impl crate::RegisterSpec for DivcnthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcnth::R`](R) reader structure"]
impl crate::Readable for DivcnthSpec {}
#[doc = "`write(|w| ..)` method takes [`divcnth::W`](W) writer structure"]
impl crate::Writable for DivcnthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVCNTH to value 0"]
impl crate::Resettable for DivcnthSpec {
    const RESET_VALUE: u32 = 0;
}
