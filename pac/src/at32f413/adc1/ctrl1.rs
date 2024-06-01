#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `VMCSEL` reader - Voltage monitoring channel select"]
pub type VmcselR = crate::FieldReader;
#[doc = "Field `VMCSEL` writer - Voltage monitoring channel select"]
pub type VmcselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCEIEN` reader - Channel conversion end interrupt enable"]
pub type CceienR = crate::BitReader;
#[doc = "Field `CCEIEN` writer - Channel conversion end interrupt enable"]
pub type CceienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMORIEN` reader - Voltage monitoring out of range interrupt enable"]
pub type VmorienR = crate::BitReader;
#[doc = "Field `VMORIEN` writer - Voltage monitoring out of range interrupt enable"]
pub type VmorienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCCEIEN` reader - Conversion end interrupt enable for preempted channels"]
pub type PcceienR = crate::BitReader;
#[doc = "Field `PCCEIEN` writer - Conversion end interrupt enable for preempted channels"]
pub type PcceienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQEN` reader - Sequence mode enable"]
pub type SqenR = crate::BitReader;
#[doc = "Field `SQEN` writer - Sequence mode enable"]
pub type SqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VMSGEN` reader - Voltage monitoring enable on a single channel"]
pub type VmsgenR = crate::BitReader;
#[doc = "Field `VMSGEN` writer - Voltage monitoring enable on a single channel"]
pub type VmsgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCAUTOEN` reader - Preempted group automatic conversion enable after ordinary group"]
pub type PcautoenR = crate::BitReader;
#[doc = "Field `PCAUTOEN` writer - Preempted group automatic conversion enable after ordinary group"]
pub type PcautoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPEN` reader - Partitioned mode enable on ordinary channels"]
pub type OcpenR = crate::BitReader;
#[doc = "Field `OCPEN` writer - Partitioned mode enable on ordinary channels"]
pub type OcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPEN` reader - Partitioned mode enable on preempted channels"]
pub type PcpenR = crate::BitReader;
#[doc = "Field `PCPEN` writer - Partitioned mode enable on preempted channels"]
pub type PcpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPCNT` reader - Partitioned mode conversion count of ordinary channels"]
pub type OcpcntR = crate::FieldReader;
#[doc = "Field `OCPCNT` writer - Partitioned mode conversion count of ordinary channels"]
pub type OcpcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSSEL` reader - Master slave mode select"]
pub type MsselR = crate::FieldReader;
#[doc = "Field `MSSEL` writer - Master slave mode select"]
pub type MsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PCVMEN` reader - Voltage monitoring enable on preempted channels"]
pub type PcvmenR = crate::BitReader;
#[doc = "Field `PCVMEN` writer - Voltage monitoring enable on preempted channels"]
pub type PcvmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCVMEN` reader - Voltage monitoring enable on ordinary channels"]
pub type OcvmenR = crate::BitReader;
#[doc = "Field `OCVMEN` writer - Voltage monitoring enable on ordinary channels"]
pub type OcvmenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    pub fn vmcsel(&self) -> VmcselR {
        VmcselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CceienR {
        CceienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    pub fn vmorien(&self) -> VmorienR {
        VmorienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    pub fn pcceien(&self) -> PcceienR {
        PcceienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    pub fn sqen(&self) -> SqenR {
        SqenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    pub fn vmsgen(&self) -> VmsgenR {
        VmsgenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    pub fn pcautoen(&self) -> PcautoenR {
        PcautoenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    pub fn ocpen(&self) -> OcpenR {
        OcpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    pub fn pcpen(&self) -> PcpenR {
        PcpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    pub fn ocpcnt(&self) -> OcpcntR {
        OcpcntR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&self) -> MsselR {
        MsselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    pub fn pcvmen(&self) -> PcvmenR {
        PcvmenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    pub fn ocvmen(&self) -> OcvmenR {
        OcvmenR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("ocvmen", &self.ocvmen())
            .field("pcvmen", &self.pcvmen())
            .field("mssel", &self.mssel())
            .field("ocpcnt", &self.ocpcnt())
            .field("pcpen", &self.pcpen())
            .field("ocpen", &self.ocpen())
            .field("pcautoen", &self.pcautoen())
            .field("vmsgen", &self.vmsgen())
            .field("sqen", &self.sqen())
            .field("pcceien", &self.pcceien())
            .field("vmorien", &self.vmorien())
            .field("cceien", &self.cceien())
            .field("vmcsel", &self.vmcsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Voltage monitoring channel select"]
    #[inline(always)]
    #[must_use]
    pub fn vmcsel(&mut self) -> VmcselW<Ctrl1Spec> {
        VmcselW::new(self, 0)
    }
    #[doc = "Bit 5 - Channel conversion end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CceienW<Ctrl1Spec> {
        CceienW::new(self, 5)
    }
    #[doc = "Bit 6 - Voltage monitoring out of range interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmorien(&mut self) -> VmorienW<Ctrl1Spec> {
        VmorienW::new(self, 6)
    }
    #[doc = "Bit 7 - Conversion end interrupt enable for preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcceien(&mut self) -> PcceienW<Ctrl1Spec> {
        PcceienW::new(self, 7)
    }
    #[doc = "Bit 8 - Sequence mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn sqen(&mut self) -> SqenW<Ctrl1Spec> {
        SqenW::new(self, 8)
    }
    #[doc = "Bit 9 - Voltage monitoring enable on a single channel"]
    #[inline(always)]
    #[must_use]
    pub fn vmsgen(&mut self) -> VmsgenW<Ctrl1Spec> {
        VmsgenW::new(self, 9)
    }
    #[doc = "Bit 10 - Preempted group automatic conversion enable after ordinary group"]
    #[inline(always)]
    #[must_use]
    pub fn pcautoen(&mut self) -> PcautoenW<Ctrl1Spec> {
        PcautoenW::new(self, 10)
    }
    #[doc = "Bit 11 - Partitioned mode enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpen(&mut self) -> OcpenW<Ctrl1Spec> {
        OcpenW::new(self, 11)
    }
    #[doc = "Bit 12 - Partitioned mode enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcpen(&mut self) -> PcpenW<Ctrl1Spec> {
        PcpenW::new(self, 12)
    }
    #[doc = "Bits 13:15 - Partitioned mode conversion count of ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocpcnt(&mut self) -> OcpcntW<Ctrl1Spec> {
        OcpcntW::new(self, 13)
    }
    #[doc = "Bits 16:19 - Master slave mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mssel(&mut self) -> MsselW<Ctrl1Spec> {
        MsselW::new(self, 16)
    }
    #[doc = "Bit 22 - Voltage monitoring enable on preempted channels"]
    #[inline(always)]
    #[must_use]
    pub fn pcvmen(&mut self) -> PcvmenW<Ctrl1Spec> {
        PcvmenW::new(self, 22)
    }
    #[doc = "Bit 23 - Voltage monitoring enable on ordinary channels"]
    #[inline(always)]
    #[must_use]
    pub fn ocvmen(&mut self) -> OcvmenW<Ctrl1Spec> {
        OcvmenW::new(self, 23)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
