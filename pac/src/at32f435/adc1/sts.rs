#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `VMOR` reader - Voltage monitoring out of range flag"]
pub type VmorR = crate::BitReader;
#[doc = "Field `VMOR` writer - Voltage monitoring out of range flag"]
pub type VmorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCCE` reader - Ordinary channels conversion end flag"]
pub type OcceR = crate::BitReader;
#[doc = "Field `OCCE` writer - Ordinary channels conversion end flag"]
pub type OcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCE` reader - Preempted channels conversion end flag"]
pub type PcceR = crate::BitReader;
#[doc = "Field `PCCE` writer - Preempted channels conversion end flag"]
pub type PcceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCS` reader - Preempted channel conversion start flag"]
pub type PccsR = crate::BitReader;
#[doc = "Field `PCCS` writer - Preempted channel conversion start flag"]
pub type PccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCCS` reader - Ordinary channel conversion start flag"]
pub type OccsR = crate::BitReader;
#[doc = "Field `OCCS` writer - Ordinary channel conversion start flag"]
pub type OccsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCCO` reader - Ordinary channel conversion overflow flag"]
pub type OccoR = crate::BitReader;
#[doc = "Field `OCCO` writer - Ordinary channel conversion overflow flag"]
pub type OccoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDY` reader - ADC ready to conversion flag"]
pub type RdyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    pub fn vmor(&self) -> VmorR {
        VmorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    pub fn occe(&self) -> OcceR {
        OcceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    pub fn pcce(&self) -> PcceR {
        PcceR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    pub fn pccs(&self) -> PccsR {
        PccsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    pub fn occs(&self) -> OccsR {
        OccsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    pub fn occo(&self) -> OccoR {
        OccoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC ready to conversion flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("rdy", &self.rdy())
            .field("occo", &self.occo())
            .field("occs", &self.occs())
            .field("pccs", &self.pccs())
            .field("pcce", &self.pcce())
            .field("occe", &self.occe())
            .field("vmor", &self.vmor())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Voltage monitoring out of range flag"]
    #[inline(always)]
    #[must_use]
    pub fn vmor(&mut self) -> VmorW<StsSpec> {
        VmorW::new(self, 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn occe(&mut self) -> OcceW<StsSpec> {
        OcceW::new(self, 1)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag"]
    #[inline(always)]
    #[must_use]
    pub fn pcce(&mut self) -> PcceW<StsSpec> {
        PcceW::new(self, 2)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn pccs(&mut self) -> PccsW<StsSpec> {
        PccsW::new(self, 3)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OccsW<StsSpec> {
        OccsW::new(self, 4)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag"]
    #[inline(always)]
    #[must_use]
    pub fn occo(&mut self) -> OccoW<StsSpec> {
        OccoW::new(self, 5)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
