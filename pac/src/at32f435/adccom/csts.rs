#[doc = "Register `CSTS` reader"]
pub type R = crate::R<CstsSpec>;
#[doc = "Field `VMOR1` reader - Voltage monitoring out of range flag of ADC1"]
pub type Vmor1R = crate::BitReader;
#[doc = "Field `OCCE1` reader - Ordinary channels conversion end flag of ADC1"]
pub type Occe1R = crate::BitReader;
#[doc = "Field `PCCE1` reader - Preempted channels conversion end flag of ADC1"]
pub type Pcce1R = crate::BitReader;
#[doc = "Field `PCCS1` reader - Preempted channel conversion start flag of ADC1"]
pub type Pccs1R = crate::BitReader;
#[doc = "Field `OCCS1` reader - Ordinary channel conversion start flag of ADC1"]
pub type Occs1R = crate::BitReader;
#[doc = "Field `OCCO1` reader - Ordinary channel conversion overflow flag of ADC1"]
pub type Occo1R = crate::BitReader;
#[doc = "Field `RDY1` reader - ADC ready to conversion flag of ADC1"]
pub type Rdy1R = crate::BitReader;
#[doc = "Field `VMOR2` reader - Voltage monitoring out of range flag of ADC2"]
pub type Vmor2R = crate::BitReader;
#[doc = "Field `OCCE2` reader - Ordinary channels conversion end flag of ADC2"]
pub type Occe2R = crate::BitReader;
#[doc = "Field `PCCE2` reader - Preempted channels conversion end flag of ADC2"]
pub type Pcce2R = crate::BitReader;
#[doc = "Field `PCCS2` reader - Preempted channel conversion start flag of ADC2"]
pub type Pccs2R = crate::BitReader;
#[doc = "Field `OCCS2` reader - Ordinary channel conversion start flag of ADC2"]
pub type Occs2R = crate::BitReader;
#[doc = "Field `OCCO2` reader - Ordinary channel conversion overflow flag of ADC2"]
pub type Occo2R = crate::BitReader;
#[doc = "Field `RDY2` reader - ADC ready to conversion flag of ADC2"]
pub type Rdy2R = crate::BitReader;
#[doc = "Field `VMOR3` reader - Voltage monitoring out of range flag of ADC3"]
pub type Vmor3R = crate::BitReader;
#[doc = "Field `OCCE3` reader - Ordinary channels conversion end flag of ADC3"]
pub type Occe3R = crate::BitReader;
#[doc = "Field `PCCE3` reader - Preempted channels conversion end flag of ADC3"]
pub type Pcce3R = crate::BitReader;
#[doc = "Field `PCCS3` reader - Preempted channel conversion start flag of ADC3"]
pub type Pccs3R = crate::BitReader;
#[doc = "Field `OCCS3` reader - Ordinary channel conversion start flag of ADC3"]
pub type Occs3R = crate::BitReader;
#[doc = "Field `OCCO3` reader - Ordinary channel conversion overflow flag of ADC3"]
pub type Occo3R = crate::BitReader;
#[doc = "Field `RDY3` reader - ADC ready to conversion flag of ADC3"]
pub type Rdy3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Voltage monitoring out of range flag of ADC1"]
    #[inline(always)]
    pub fn vmor1(&self) -> Vmor1R {
        Vmor1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ordinary channels conversion end flag of ADC1"]
    #[inline(always)]
    pub fn occe1(&self) -> Occe1R {
        Occe1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Preempted channels conversion end flag of ADC1"]
    #[inline(always)]
    pub fn pcce1(&self) -> Pcce1R {
        Pcce1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Preempted channel conversion start flag of ADC1"]
    #[inline(always)]
    pub fn pccs1(&self) -> Pccs1R {
        Pccs1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Ordinary channel conversion start flag of ADC1"]
    #[inline(always)]
    pub fn occs1(&self) -> Occs1R {
        Occs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Ordinary channel conversion overflow flag of ADC1"]
    #[inline(always)]
    pub fn occo1(&self) -> Occo1R {
        Occo1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC ready to conversion flag of ADC1"]
    #[inline(always)]
    pub fn rdy1(&self) -> Rdy1R {
        Rdy1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Voltage monitoring out of range flag of ADC2"]
    #[inline(always)]
    pub fn vmor2(&self) -> Vmor2R {
        Vmor2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Ordinary channels conversion end flag of ADC2"]
    #[inline(always)]
    pub fn occe2(&self) -> Occe2R {
        Occe2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Preempted channels conversion end flag of ADC2"]
    #[inline(always)]
    pub fn pcce2(&self) -> Pcce2R {
        Pcce2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Preempted channel conversion start flag of ADC2"]
    #[inline(always)]
    pub fn pccs2(&self) -> Pccs2R {
        Pccs2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Ordinary channel conversion start flag of ADC2"]
    #[inline(always)]
    pub fn occs2(&self) -> Occs2R {
        Occs2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Ordinary channel conversion overflow flag of ADC2"]
    #[inline(always)]
    pub fn occo2(&self) -> Occo2R {
        Occo2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC ready to conversion flag of ADC2"]
    #[inline(always)]
    pub fn rdy2(&self) -> Rdy2R {
        Rdy2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage monitoring out of range flag of ADC3"]
    #[inline(always)]
    pub fn vmor3(&self) -> Vmor3R {
        Vmor3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ordinary channels conversion end flag of ADC3"]
    #[inline(always)]
    pub fn occe3(&self) -> Occe3R {
        Occe3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Preempted channels conversion end flag of ADC3"]
    #[inline(always)]
    pub fn pcce3(&self) -> Pcce3R {
        Pcce3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Preempted channel conversion start flag of ADC3"]
    #[inline(always)]
    pub fn pccs3(&self) -> Pccs3R {
        Pccs3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Ordinary channel conversion start flag of ADC3"]
    #[inline(always)]
    pub fn occs3(&self) -> Occs3R {
        Occs3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ordinary channel conversion overflow flag of ADC3"]
    #[inline(always)]
    pub fn occo3(&self) -> Occo3R {
        Occo3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC ready to conversion flag of ADC3"]
    #[inline(always)]
    pub fn rdy3(&self) -> Rdy3R {
        Rdy3R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSTS")
            .field("rdy3", &self.rdy3())
            .field("occo3", &self.occo3())
            .field("occs3", &self.occs3())
            .field("pccs3", &self.pccs3())
            .field("pcce3", &self.pcce3())
            .field("occe3", &self.occe3())
            .field("vmor3", &self.vmor3())
            .field("rdy2", &self.rdy2())
            .field("occo2", &self.occo2())
            .field("occs2", &self.occs2())
            .field("pccs2", &self.pccs2())
            .field("pcce2", &self.pcce2())
            .field("occe2", &self.occe2())
            .field("vmor2", &self.vmor2())
            .field("rdy1", &self.rdy1())
            .field("occo1", &self.occo1())
            .field("occs1", &self.occs1())
            .field("pccs1", &self.pccs1())
            .field("pcce1", &self.pcce1())
            .field("occe1", &self.occe1())
            .field("vmor1", &self.vmor1())
            .finish()
    }
}
#[doc = "Common status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstsSpec;
impl crate::RegisterSpec for CstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csts::R`](R) reader structure"]
impl crate::Readable for CstsSpec {}
#[doc = "`reset()` method sets CSTS to value 0"]
impl crate::Resettable for CstsSpec {
    const RESET_VALUE: u32 = 0;
}
