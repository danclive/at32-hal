#[doc = "Register `IENA` reader"]
pub type R = crate::R<IenaSpec>;
#[doc = "Register `IENA` writer"]
pub type W = crate::W<IenaSpec>;
#[doc = "Field `CFDIE` reader - Capture frame done interrupt enable"]
pub type CfdieR = crate::BitReader;
#[doc = "Field `CFDIE` writer - Capture frame done interrupt enable"]
pub type CfdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Data FIFO overrun interrupt enable"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - Data FIFO overrun interrupt enable"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESEIE` reader - Embedded synchronization error interrupt enable"]
pub type EseieR = crate::BitReader;
#[doc = "Field `ESEIE` writer - Embedded synchronization error interrupt enable"]
pub type EseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSIE` reader - Vertical synchronization interrupt enablee"]
pub type VsieR = crate::BitReader;
#[doc = "Field `VSIE` writer - Vertical synchronization interrupt enablee"]
pub type VsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIE` reader - Horizontal synchronization interrupt enable"]
pub type HsieR = crate::BitReader;
#[doc = "Field `HSIE` writer - Horizontal synchronization interrupt enable"]
pub type HsieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture frame done interrupt enable"]
    #[inline(always)]
    pub fn cfdie(&self) -> CfdieR {
        CfdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt enable"]
    #[inline(always)]
    pub fn eseie(&self) -> EseieR {
        EseieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt enablee"]
    #[inline(always)]
    pub fn vsie(&self) -> VsieR {
        VsieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt enable"]
    #[inline(always)]
    pub fn hsie(&self) -> HsieR {
        HsieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IENA")
            .field("hsie", &self.hsie())
            .field("vsie", &self.vsie())
            .field("eseie", &self.eseie())
            .field("ovrie", &self.ovrie())
            .field("cfdie", &self.cfdie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Capture frame done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfdie(&mut self) -> CfdieW<IenaSpec> {
        CfdieW::new(self, 0)
    }
    #[doc = "Bit 1 - Data FIFO overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OvrieW<IenaSpec> {
        OvrieW::new(self, 1)
    }
    #[doc = "Bit 2 - Embedded synchronization error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eseie(&mut self) -> EseieW<IenaSpec> {
        EseieW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical synchronization interrupt enablee"]
    #[inline(always)]
    #[must_use]
    pub fn vsie(&mut self) -> VsieW<IenaSpec> {
        VsieW::new(self, 3)
    }
    #[doc = "Bit 4 - Horizontal synchronization interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hsie(&mut self) -> HsieW<IenaSpec> {
        HsieW::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenaSpec;
impl crate::RegisterSpec for IenaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iena::R`](R) reader structure"]
impl crate::Readable for IenaSpec {}
#[doc = "`write(|w| ..)` method takes [`iena::W`](W) writer structure"]
impl crate::Writable for IenaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IENA to value 0"]
impl crate::Resettable for IenaSpec {
    const RESET_VALUE: u32 = 0;
}
