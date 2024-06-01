#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TimeoutSpec>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TimeoutSpec>;
#[doc = "Field `TOTIME` reader - Clock timeout detection time"]
pub type TotimeR = crate::FieldReader<u16>;
#[doc = "Field `TOTIME` writer - Clock timeout detection time"]
pub type TotimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TOMOED` reader - Clock timeout detection mode"]
pub type TomoedR = crate::BitReader;
#[doc = "Field `TOMOED` writer - Clock timeout detection mode"]
pub type TomoedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOEN` reader - Detect clock low/high timeout enable"]
pub type ToenR = crate::BitReader;
#[doc = "Field `TOEN` writer - Detect clock low/high timeout enable"]
pub type ToenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTTIME` reader - Cumulative clock low extend timeout value"]
pub type ExttimeR = crate::FieldReader<u16>;
#[doc = "Field `EXTTIME` writer - Cumulative clock low extend timeout value"]
pub type ExttimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `EXTEN` reader - Cumulative clock low extend timeout enable"]
pub type ExtenR = crate::BitReader;
#[doc = "Field `EXTEN` writer - Cumulative clock low extend timeout enable"]
pub type ExtenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Clock timeout detection time"]
    #[inline(always)]
    pub fn totime(&self) -> TotimeR {
        TotimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    pub fn tomoed(&self) -> TomoedR {
        TomoedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    pub fn toen(&self) -> ToenR {
        ToenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value"]
    #[inline(always)]
    pub fn exttime(&self) -> ExttimeR {
        ExttimeR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("totime", &self.totime())
            .field("tomoed", &self.tomoed())
            .field("toen", &self.toen())
            .field("exttime", &self.exttime())
            .field("exten", &self.exten())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Clock timeout detection time"]
    #[inline(always)]
    #[must_use]
    pub fn totime(&mut self) -> TotimeW<TimeoutSpec> {
        TotimeW::new(self, 0)
    }
    #[doc = "Bit 12 - Clock timeout detection mode"]
    #[inline(always)]
    #[must_use]
    pub fn tomoed(&mut self) -> TomoedW<TimeoutSpec> {
        TomoedW::new(self, 12)
    }
    #[doc = "Bit 15 - Detect clock low/high timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn toen(&mut self) -> ToenW<TimeoutSpec> {
        ToenW::new(self, 15)
    }
    #[doc = "Bits 16:27 - Cumulative clock low extend timeout value"]
    #[inline(always)]
    #[must_use]
    pub fn exttime(&mut self) -> ExttimeW<TimeoutSpec> {
        ExttimeW::new(self, 16)
    }
    #[doc = "Bit 31 - Cumulative clock low extend timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> ExtenW<TimeoutSpec> {
        ExtenW::new(self, 31)
    }
}
#[doc = "Timeout register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutSpec;
impl crate::RegisterSpec for TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TimeoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TimeoutSpec {
    const RESET_VALUE: u32 = 0;
}
