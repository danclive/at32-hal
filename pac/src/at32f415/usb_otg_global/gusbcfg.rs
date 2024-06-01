#[doc = "Register `GUSBCFG` reader"]
pub type R = crate::R<GusbcfgSpec>;
#[doc = "Register `GUSBCFG` writer"]
pub type W = crate::W<GusbcfgSpec>;
#[doc = "Field `TOUTCAL` reader - FS timeout calibration"]
pub type ToutcalR = crate::FieldReader;
#[doc = "Field `TOUTCAL` writer - FS timeout calibration"]
pub type ToutcalW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USBTRDTIM` reader - USB turnaround time"]
pub type UsbtrdtimR = crate::FieldReader;
#[doc = "Field `USBTRDTIM` writer - USB turnaround time"]
pub type UsbtrdtimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FHSTMODE` reader - Force host mode"]
pub type FhstmodeR = crate::BitReader;
#[doc = "Field `FHSTMODE` writer - Force host mode"]
pub type FhstmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDEVMODE` reader - Force device mode"]
pub type FdevmodeR = crate::BitReader;
#[doc = "Field `FDEVMODE` writer - Force device mode"]
pub type FdevmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COTXPKT` reader - Corrupt Tx packet"]
pub type CotxpktR = crate::BitReader;
#[doc = "Field `COTXPKT` writer - Corrupt Tx packet"]
pub type CotxpktW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    pub fn toutcal(&self) -> ToutcalR {
        ToutcalR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    pub fn usbtrdtim(&self) -> UsbtrdtimR {
        UsbtrdtimR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    pub fn fhstmode(&self) -> FhstmodeR {
        FhstmodeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    pub fn fdevmode(&self) -> FdevmodeR {
        FdevmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    pub fn cotxpkt(&self) -> CotxpktR {
        CotxpktR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GUSBCFG")
            .field("toutcal", &self.toutcal())
            .field("usbtrdtim", &self.usbtrdtim())
            .field("fhstmode", &self.fhstmode())
            .field("fdevmode", &self.fdevmode())
            .field("cotxpkt", &self.cotxpkt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - FS timeout calibration"]
    #[inline(always)]
    #[must_use]
    pub fn toutcal(&mut self) -> ToutcalW<GusbcfgSpec> {
        ToutcalW::new(self, 0)
    }
    #[doc = "Bits 10:13 - USB turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn usbtrdtim(&mut self) -> UsbtrdtimW<GusbcfgSpec> {
        UsbtrdtimW::new(self, 10)
    }
    #[doc = "Bit 29 - Force host mode"]
    #[inline(always)]
    #[must_use]
    pub fn fhstmode(&mut self) -> FhstmodeW<GusbcfgSpec> {
        FhstmodeW::new(self, 29)
    }
    #[doc = "Bit 30 - Force device mode"]
    #[inline(always)]
    #[must_use]
    pub fn fdevmode(&mut self) -> FdevmodeW<GusbcfgSpec> {
        FdevmodeW::new(self, 30)
    }
    #[doc = "Bit 31 - Corrupt Tx packet"]
    #[inline(always)]
    #[must_use]
    pub fn cotxpkt(&mut self) -> CotxpktW<GusbcfgSpec> {
        CotxpktW::new(self, 31)
    }
}
#[doc = "USB configuration register (OTGFS_GUSBCFG)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gusbcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gusbcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GusbcfgSpec;
impl crate::RegisterSpec for GusbcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gusbcfg::R`](R) reader structure"]
impl crate::Readable for GusbcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`gusbcfg::W`](W) writer structure"]
impl crate::Writable for GusbcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GUSBCFG to value 0x0a00"]
impl crate::Resettable for GusbcfgSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
