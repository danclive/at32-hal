#[doc = "Register `FRF` reader"]
pub type R = crate::R<FrfSpec>;
#[doc = "Register `FRF` writer"]
pub type W = crate::W<FrfSpec>;
#[doc = "Field `EFRCSF` reader - Enhanced frame rate contorl source factor"]
pub type EfrcsfR = crate::FieldReader;
#[doc = "Field `EFRCSF` writer - Enhanced frame rate contorl source factor"]
pub type EfrcsfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFRCTF` reader - Enhanced frame rate control target factor"]
pub type EfrctfR = crate::FieldReader;
#[doc = "Field `EFRCTF` writer - Enhanced frame rate control target factor"]
pub type EfrctfW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Enhanced frame rate contorl source factor"]
    #[inline(always)]
    pub fn efrcsf(&self) -> EfrcsfR {
        EfrcsfR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Enhanced frame rate control target factor"]
    #[inline(always)]
    pub fn efrctf(&self) -> EfrctfR {
        EfrctfR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FRF")
            .field("efrctf", &self.efrctf())
            .field("efrcsf", &self.efrcsf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Enhanced frame rate contorl source factor"]
    #[inline(always)]
    #[must_use]
    pub fn efrcsf(&mut self) -> EfrcsfW<FrfSpec> {
        EfrcsfW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Enhanced frame rate control target factor"]
    #[inline(always)]
    #[must_use]
    pub fn efrctf(&mut self) -> EfrctfW<FrfSpec> {
        EfrctfW::new(self, 8)
    }
}
#[doc = "Frame rate flow\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrfSpec;
impl crate::RegisterSpec for FrfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frf::R`](R) reader structure"]
impl crate::Readable for FrfSpec {}
#[doc = "`write(|w| ..)` method takes [`frf::W`](W) writer structure"]
impl crate::Writable for FrfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRF to value 0"]
impl crate::Resettable for FrfSpec {
    const RESET_VALUE: u32 = 0;
}
