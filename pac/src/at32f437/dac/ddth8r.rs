#[doc = "Register `DDTH8R` reader"]
pub type R = crate::R<Ddth8rSpec>;
#[doc = "Register `DDTH8R` writer"]
pub type W = crate::W<Ddth8rSpec>;
#[doc = "Field `DD1DT8R` reader - DAC1 8-bit right-aligned data"]
pub type Dd1dt8rR = crate::FieldReader;
#[doc = "Field `DD1DT8R` writer - DAC1 8-bit right-aligned data"]
pub type Dd1dt8rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DD2DT8R` reader - DAC2 8-bit right-aligned data"]
pub type Dd2dt8rR = crate::FieldReader;
#[doc = "Field `DD2DT8R` writer - DAC2 8-bit right-aligned data"]
pub type Dd2dt8rW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dd1dt8r(&self) -> Dd1dt8rR {
        Dd1dt8rR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    pub fn dd2dt8r(&self) -> Dd2dt8rR {
        Dd2dt8rR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DDTH8R")
            .field("dd1dt8r", &self.dd1dt8r())
            .field("dd2dt8r", &self.dd2dt8r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC1 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd1dt8r(&mut self) -> Dd1dt8rW<Ddth8rSpec> {
        Dd1dt8rW::new(self, 0)
    }
    #[doc = "Bits 8:15 - DAC2 8-bit right-aligned data"]
    #[inline(always)]
    #[must_use]
    pub fn dd2dt8r(&mut self) -> Dd2dt8rW<Ddth8rSpec> {
        Dd2dt8rW::new(self, 8)
    }
}
#[doc = "DUAL DAC 8-bit right aligned data holding register (DAC_DDTH8R), Bits 31:16 Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddth8r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddth8r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ddth8rSpec;
impl crate::RegisterSpec for Ddth8rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddth8r::R`](R) reader structure"]
impl crate::Readable for Ddth8rSpec {}
#[doc = "`write(|w| ..)` method takes [`ddth8r::W`](W) writer structure"]
impl crate::Writable for Ddth8rSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDTH8R to value 0"]
impl crate::Resettable for Ddth8rSpec {
    const RESET_VALUE: u32 = 0;
}
