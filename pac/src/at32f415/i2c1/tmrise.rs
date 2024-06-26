#[doc = "Register `TMRISE` reader"]
pub type R = crate::R<TmriseSpec>;
#[doc = "Register `TMRISE` writer"]
pub type W = crate::W<TmriseSpec>;
#[doc = "Field `RISETIME` reader - I2C bus rise time"]
pub type RisetimeR = crate::FieldReader;
#[doc = "Field `RISETIME` writer - I2C bus rise time"]
pub type RisetimeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    pub fn risetime(&self) -> RisetimeR {
        RisetimeR::new((self.bits & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMRISE")
            .field("risetime", &self.risetime())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - I2C bus rise time"]
    #[inline(always)]
    #[must_use]
    pub fn risetime(&mut self) -> RisetimeW<TmriseSpec> {
        RisetimeW::new(self, 0)
    }
}
#[doc = "TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmrise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmrise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmriseSpec;
impl crate::RegisterSpec for TmriseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmrise::R`](R) reader structure"]
impl crate::Readable for TmriseSpec {}
#[doc = "`write(|w| ..)` method takes [`tmrise::W`](W) writer structure"]
impl crate::Writable for TmriseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMRISE to value 0x02"]
impl crate::Resettable for TmriseSpec {
    const RESET_VALUE: u32 = 0x02;
}
