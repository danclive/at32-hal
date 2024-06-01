#[doc = "Register `MACPMTCTRLSTS` reader"]
pub type R = crate::R<MacpmtctrlstsSpec>;
#[doc = "Register `MACPMTCTRLSTS` writer"]
pub type W = crate::W<MacpmtctrlstsSpec>;
#[doc = "Field `PD` reader - Power down"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - Power down"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMP` reader - Enable magic packet"]
pub type EmpR = crate::BitReader;
#[doc = "Field `EMP` writer - Enable magic packet"]
pub type EmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERWF` reader - Enable remote wakeup frame"]
pub type ErwfR = crate::BitReader;
#[doc = "Field `ERWF` writer - Enable remote wakeup frame"]
pub type ErwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMP` reader - Received magic packet"]
pub type RmpR = crate::BitReader;
#[doc = "Field `RMP` writer - Received magic packet"]
pub type RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRWF` reader - Recevied remote wakeup frame"]
pub type RrwfR = crate::BitReader;
#[doc = "Field `RRWF` writer - Recevied remote wakeup frame"]
pub type RrwfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GUC` reader - Global unicast"]
pub type GucR = crate::BitReader;
#[doc = "Field `GUC` writer - Global unicast"]
pub type GucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWFFPR` reader - Remote wakeup frame filter register pointer reset"]
pub type RwffprR = crate::BitReader;
#[doc = "Field `RWFFPR` writer - Remote wakeup frame filter register pointer reset"]
pub type RwffprW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    pub fn emp(&self) -> EmpR {
        EmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    pub fn erwf(&self) -> ErwfR {
        ErwfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Received magic packet"]
    #[inline(always)]
    pub fn rmp(&self) -> RmpR {
        RmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Recevied remote wakeup frame"]
    #[inline(always)]
    pub fn rrwf(&self) -> RrwfR {
        RrwfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    pub fn guc(&self) -> GucR {
        GucR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    pub fn rwffpr(&self) -> RwffprR {
        RwffprR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACPMTCTRLSTS")
            .field("pd", &self.pd())
            .field("emp", &self.emp())
            .field("erwf", &self.erwf())
            .field("rmp", &self.rmp())
            .field("rrwf", &self.rrwf())
            .field("guc", &self.guc())
            .field("rwffpr", &self.rwffpr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Power down"]
    #[inline(always)]
    #[must_use]
    pub fn pd(&mut self) -> PdW<MacpmtctrlstsSpec> {
        PdW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable magic packet"]
    #[inline(always)]
    #[must_use]
    pub fn emp(&mut self) -> EmpW<MacpmtctrlstsSpec> {
        EmpW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable remote wakeup frame"]
    #[inline(always)]
    #[must_use]
    pub fn erwf(&mut self) -> ErwfW<MacpmtctrlstsSpec> {
        ErwfW::new(self, 2)
    }
    #[doc = "Bit 5 - Received magic packet"]
    #[inline(always)]
    #[must_use]
    pub fn rmp(&mut self) -> RmpW<MacpmtctrlstsSpec> {
        RmpW::new(self, 5)
    }
    #[doc = "Bit 6 - Recevied remote wakeup frame"]
    #[inline(always)]
    #[must_use]
    pub fn rrwf(&mut self) -> RrwfW<MacpmtctrlstsSpec> {
        RrwfW::new(self, 6)
    }
    #[doc = "Bit 9 - Global unicast"]
    #[inline(always)]
    #[must_use]
    pub fn guc(&mut self) -> GucW<MacpmtctrlstsSpec> {
        GucW::new(self, 9)
    }
    #[doc = "Bit 31 - Remote wakeup frame filter register pointer reset"]
    #[inline(always)]
    #[must_use]
    pub fn rwffpr(&mut self) -> RwffprW<MacpmtctrlstsSpec> {
        RwffprW::new(self, 31)
    }
}
#[doc = "Ethernet MAC PMT control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macpmtctrlsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macpmtctrlsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacpmtctrlstsSpec;
impl crate::RegisterSpec for MacpmtctrlstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpmtctrlsts::R`](R) reader structure"]
impl crate::Readable for MacpmtctrlstsSpec {}
#[doc = "`write(|w| ..)` method takes [`macpmtctrlsts::W`](W) writer structure"]
impl crate::Writable for MacpmtctrlstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACPMTCTRLSTS to value 0"]
impl crate::Resettable for MacpmtctrlstsSpec {
    const RESET_VALUE: u32 = 0;
}
