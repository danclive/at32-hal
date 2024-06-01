#[doc = "Register `DDTH12L` reader"]
pub type R = crate::R<Ddth12lSpec>;
#[doc = "Register `DDTH12L` writer"]
pub type W = crate::W<Ddth12lSpec>;
#[doc = "Field `DD1DT12L` reader - DAC1 12-bit left-aligned data"]
pub type Dd1dt12lR = crate::FieldReader<u16>;
#[doc = "Field `DD1DT12L` writer - DAC1 12-bit left-aligned data"]
pub type Dd1dt12lW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DD2DT12L` reader - DAC2 12-bit right-aligned data"]
pub type Dd2dt12lR = crate::FieldReader<u16>;
#[doc = "Field `DD2DT12L` writer - DAC2 12-bit right-aligned data"]
pub type Dd2dt12lW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    pub fn dd1dt12l(&self) -> Dd1dt12lR {
        Dd1dt12lR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    pub fn dd2dt12l(&self) -> Dd2dt12lR {
        Dd2dt12lR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDTH12L")
            .field("dd1dt12l", &self.dd1dt12l())
            .field("dd2dt12l", &self.dd2dt12l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:15 - DAC1 12-bit left-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt12l(&mut self) -> Dd1dt12lW<Ddth12lSpec> {
        Dd1dt12lW::new(self, 4)
    }
    #[doc = "Bits 20:31 - DAC2 12-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt12l(&mut self) -> Dd2dt12lW<Ddth12lSpec> {
        Dd2dt12lW::new(self, 20)
    }
}
#[doc = "DUAL DAC 12-bit left aligned data holding register (DAC_DDTH12L), Bits 19:16 Reserved, Bits 3:0 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth12l::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth12l::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddth12lSpec;
impl crate::RegisterSpec for Ddth12lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth12l::R`](R) reader structure"]
impl crate::Readable for Ddth12lSpec {}
#[doc = "`write(|w| ..)` method takes [`ddth12l::W`](W) writer structure"]
impl crate::Writable for Ddth12lSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDTH12L to value 0"]
impl crate::Resettable for Ddth12lSpec {
    const RESET_VALUE: u32 = 0;
}
