#[doc = "Register `UHDRV` reader"]
pub type R = crate::R<UhdrvSpec>;
#[doc = "Register `UHDRV` writer"]
pub type W = crate::W<UhdrvSpec>;
#[doc = "Field `PB3_UH` reader - PB3 ultra high sourcing/sinking strength"]
pub type Pb3UhR = crate::BitReader;
#[doc = "Field `PB3_UH` writer - PB3 ultra high sourcing/sinking strength"]
pub type Pb3UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9_UH` reader - PB9 ultra high sourcing/sinking strength"]
pub type Pb9UhR = crate::BitReader;
#[doc = "Field `PB9_UH` writer - PB9 ultra high sourcing/sinking strength"]
pub type Pb9UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB10_UH` reader - PB10 ultra high sourcing/sinking strength"]
pub type Pb10UhR = crate::BitReader;
#[doc = "Field `PB10_UH` writer - PB10 ultra high sourcing/sinking strength"]
pub type Pb10UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12_UH` reader - PD12 ultra high sourcing/sinking strength"]
pub type Pd12UhR = crate::BitReader;
#[doc = "Field `PD12_UH` writer - PD12 ultra high sourcing/sinking strength"]
pub type Pd12UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13_UH` reader - PD13 ultra high sourcing/sinking strength"]
pub type Pd13UhR = crate::BitReader;
#[doc = "Field `PD13_UH` writer - PD13 ultra high sourcing/sinking strength"]
pub type Pd13UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14_UH` reader - PD14 ultra high sourcing/sinking strength"]
pub type Pd14UhR = crate::BitReader;
#[doc = "Field `PD14_UH` writer - PD14 ultra high sourcing/sinking strength"]
pub type Pd14UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15_UH` reader - PD15 ultra high sourcing/sinking strength"]
pub type Pd15UhR = crate::BitReader;
#[doc = "Field `PD15_UH` writer - PD15 ultra high sourcing/sinking strength"]
pub type Pd15UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF14_UH` reader - PF14 ultra high sourcing/sinking strength"]
pub type Pf14UhR = crate::BitReader;
#[doc = "Field `PF14_UH` writer - PF14 ultra high sourcing/sinking strength"]
pub type Pf14UhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PF15_UH` reader - PF15 ultra high sourcing/sinking strength"]
pub type Pf15UhR = crate::BitReader;
#[doc = "Field `PF15_UH` writer - PF15 ultra high sourcing/sinking strength"]
pub type Pf15UhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PB3 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb3_uh(&self) -> Pb3UhR {
        Pb3UhR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb9_uh(&self) -> Pb9UhR {
        Pb9UhR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PB10 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pb10_uh(&self) -> Pb10UhR {
        Pb10UhR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - PD12 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd12_uh(&self) -> Pd12UhR {
        Pd12UhR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PD13 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd13_uh(&self) -> Pd13UhR {
        Pd13UhR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PD14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd14_uh(&self) -> Pd14UhR {
        Pd14UhR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PD15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pd15_uh(&self) -> Pd15UhR {
        Pd15UhR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PF14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pf14_uh(&self) -> Pf14UhR {
        Pf14UhR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PF15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    pub fn pf15_uh(&self) -> Pf15UhR {
        Pf15UhR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHDRV")
            .field("pf15_uh", &self.pf15_uh())
            .field("pf14_uh", &self.pf14_uh())
            .field("pd15_uh", &self.pd15_uh())
            .field("pd14_uh", &self.pd14_uh())
            .field("pd13_uh", &self.pd13_uh())
            .field("pd12_uh", &self.pd12_uh())
            .field("pb10_uh", &self.pb10_uh())
            .field("pb9_uh", &self.pb9_uh())
            .field("pb3_uh", &self.pb3_uh())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PB3 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb3_uh(&mut self) -> Pb3UhW<UhdrvSpec> {
        Pb3UhW::new(self, 0)
    }
    #[doc = "Bit 1 - PB9 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_uh(&mut self) -> Pb9UhW<UhdrvSpec> {
        Pb9UhW::new(self, 1)
    }
    #[doc = "Bit 2 - PB10 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pb10_uh(&mut self) -> Pb10UhW<UhdrvSpec> {
        Pb10UhW::new(self, 2)
    }
    #[doc = "Bit 5 - PD12 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd12_uh(&mut self) -> Pd12UhW<UhdrvSpec> {
        Pd12UhW::new(self, 5)
    }
    #[doc = "Bit 6 - PD13 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd13_uh(&mut self) -> Pd13UhW<UhdrvSpec> {
        Pd13UhW::new(self, 6)
    }
    #[doc = "Bit 7 - PD14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd14_uh(&mut self) -> Pd14UhW<UhdrvSpec> {
        Pd14UhW::new(self, 7)
    }
    #[doc = "Bit 8 - PD15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pd15_uh(&mut self) -> Pd15UhW<UhdrvSpec> {
        Pd15UhW::new(self, 8)
    }
    #[doc = "Bit 9 - PF14 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf14_uh(&mut self) -> Pf14UhW<UhdrvSpec> {
        Pf14UhW::new(self, 9)
    }
    #[doc = "Bit 10 - PF15 ultra high sourcing/sinking strength"]
    #[inline(always)]
    #[must_use]
    pub fn pf15_uh(&mut self) -> Pf15UhW<UhdrvSpec> {
        Pf15UhW::new(self, 10)
    }
}
#[doc = "Ultra high drive register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uhdrv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhdrv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UhdrvSpec;
impl crate::RegisterSpec for UhdrvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhdrv::R`](R) reader structure"]
impl crate::Readable for UhdrvSpec {}
#[doc = "`write(|w| ..)` method takes [`uhdrv::W`](W) writer structure"]
impl crate::Writable for UhdrvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHDRV to value 0"]
impl crate::Resettable for UhdrvSpec {
    const RESET_VALUE: u32 = 0;
}
