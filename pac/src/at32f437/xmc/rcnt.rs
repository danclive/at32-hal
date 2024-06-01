#[doc = "Register `RCNT` reader"]
pub type R = crate::R<RcntSpec>;
#[doc = "Register `RCNT` writer"]
pub type W = crate::W<RcntSpec>;
#[doc = "Field `ERRC` writer - error flag clear"]
pub type ErrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - Refresh Count"]
pub type RcR = crate::FieldReader<u16>;
#[doc = "Field `RC` writer - Refresh Count"]
pub type RcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `ERIEN` reader - error Interrupt Enable"]
pub type ErienR = crate::BitReader;
#[doc = "Field `ERIEN` writer - error Interrupt Enable"]
pub type ErienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 1:13 - Refresh Count"]
    #[inline(always)]
    pub fn rc(&self) -> RcR {
        RcR::new(((self.bits >> 1) & 0x1fff) as u16)
    }
    #[doc = "Bit 14 - error Interrupt Enable"]
    #[inline(always)]
    pub fn erien(&self) -> ErienR {
        ErienR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCNT")
            .field("rc", &self.rc())
            .field("erien", &self.erien())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ErrcW<RcntSpec> {
        ErrcW::new(self, 0)
    }
    #[doc = "Bits 1:13 - Refresh Count"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RcW<RcntSpec> {
        RcW::new(self, 1)
    }
    #[doc = "Bit 14 - error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erien(&mut self) -> ErienW<RcntSpec> {
        ErienW::new(self, 14)
    }
}
#[doc = "SDRAM Refresh Timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcntSpec;
impl crate::RegisterSpec for RcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcnt::R`](R) reader structure"]
impl crate::Readable for RcntSpec {}
#[doc = "`write(|w| ..)` method takes [`rcnt::W`](W) writer structure"]
impl crate::Writable for RcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCNT to value 0"]
impl crate::Resettable for RcntSpec {
    const RESET_VALUE: u32 = 0;
}
