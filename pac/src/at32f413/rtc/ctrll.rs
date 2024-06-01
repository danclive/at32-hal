#[doc = "Register `CTRLL` reader"]
pub type R = crate::R<CtrllSpec>;
#[doc = "Register `CTRLL` writer"]
pub type W = crate::W<CtrllSpec>;
#[doc = "Field `TSF` reader - Time second flag"]
pub type TsfR = crate::BitReader;
#[doc = "Field `TSF` writer - Time second flag"]
pub type TsfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAF` reader - Time alarm flag"]
pub type TafR = crate::BitReader;
#[doc = "Field `TAF` writer - Time alarm flag"]
pub type TafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFF` reader - Overflow Flag"]
pub type OvffR = crate::BitReader;
#[doc = "Field `OVFF` writer - Overflow Flag"]
pub type OvffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPDF` reader - RTC update finish"]
pub type UpdfR = crate::BitReader;
#[doc = "Field `UPDF` writer - RTC update finish"]
pub type UpdfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGEN` reader - RTC configuration enable"]
pub type CfgenR = crate::BitReader;
#[doc = "Field `CFGEN` writer - RTC configuration enable"]
pub type CfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFGF` reader - RTC configuration finish"]
pub type CfgfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    pub fn taf(&self) -> TafR {
        TafR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    pub fn ovff(&self) -> OvffR {
        OvffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    pub fn updf(&self) -> UpdfR {
        UpdfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    pub fn cfgen(&self) -> CfgenR {
        CfgenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC configuration finish"]
    #[inline(always)]
    pub fn cfgf(&self) -> CfgfR {
        CfgfR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLL")
            .field("tsf", &self.tsf())
            .field("taf", &self.taf())
            .field("ovff", &self.ovff())
            .field("updf", &self.updf())
            .field("cfgen", &self.cfgen())
            .field("cfgf", &self.cfgf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Time second flag"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<CtrllSpec> {
        TsfW::new(self, 0)
    }
    #[doc = "Bit 1 - Time alarm flag"]
    #[inline(always)]
    #[must_use]
    pub fn taf(&mut self) -> TafW<CtrllSpec> {
        TafW::new(self, 1)
    }
    #[doc = "Bit 2 - Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovff(&mut self) -> OvffW<CtrllSpec> {
        OvffW::new(self, 2)
    }
    #[doc = "Bit 3 - RTC update finish"]
    #[inline(always)]
    #[must_use]
    pub fn updf(&mut self) -> UpdfW<CtrllSpec> {
        UpdfW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC configuration enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfgen(&mut self) -> CfgenW<CtrllSpec> {
        CfgenW::new(self, 4)
    }
}
#[doc = "RTC Control Register Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrllSpec;
impl crate::RegisterSpec for CtrllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrll::R`](R) reader structure"]
impl crate::Readable for CtrllSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrll::W`](W) writer structure"]
impl crate::Writable for CtrllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLL to value 0x20"]
impl crate::Resettable for CtrllSpec {
    const RESET_VALUE: u32 = 0x20;
}
