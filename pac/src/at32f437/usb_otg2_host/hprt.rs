#[doc = "Register `HPRT` reader"]
pub type R = crate::R<HprtSpec>;
#[doc = "Register `HPRT` writer"]
pub type W = crate::W<HprtSpec>;
#[doc = "Field `PRTCONSTS` reader - Port connect status"]
pub type PrtconstsR = crate::BitReader;
#[doc = "Field `PRTCONDET` reader - Port connect detected"]
pub type PrtcondetR = crate::BitReader;
#[doc = "Field `PRTCONDET` writer - Port connect detected"]
pub type PrtcondetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENA` reader - Port enable"]
pub type PrtenaR = crate::BitReader;
#[doc = "Field `PRTENA` writer - Port enable"]
pub type PrtenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTENCHNG` reader - Port enable/disable change"]
pub type PrtenchngR = crate::BitReader;
#[doc = "Field `PRTENCHNG` writer - Port enable/disable change"]
pub type PrtenchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTOVRCACT` reader - Port overcurrent active"]
pub type PrtovrcactR = crate::BitReader;
#[doc = "Field `PRTOVRCCHNG` reader - Port overcurrent change"]
pub type PrtovrcchngR = crate::BitReader;
#[doc = "Field `PRTOVRCCHNG` writer - Port overcurrent change"]
pub type PrtovrcchngW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRES` reader - Port resume"]
pub type PrtresR = crate::BitReader;
#[doc = "Field `PRTRES` writer - Port resume"]
pub type PrtresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTSUSP` reader - Port suspend"]
pub type PrtsuspR = crate::BitReader;
#[doc = "Field `PRTSUSP` writer - Port suspend"]
pub type PrtsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTRST` reader - Port reset"]
pub type PrtrstR = crate::BitReader;
#[doc = "Field `PRTRST` writer - Port reset"]
pub type PrtrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTLNSTS` reader - Port line status"]
pub type PrtlnstsR = crate::FieldReader;
#[doc = "Field `PRTPWR` reader - Port power"]
pub type PrtpwrR = crate::BitReader;
#[doc = "Field `PRTPWR` writer - Port power"]
pub type PrtpwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRTTSTCTL` reader - Port test control"]
pub type PrttstctlR = crate::FieldReader;
#[doc = "Field `PRTTSTCTL` writer - Port test control"]
pub type PrttstctlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRTSPD` reader - Port speed"]
pub type PrtspdR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Port connect status"]
    #[inline(always)]
    pub fn prtconsts(&self) -> PrtconstsR {
        PrtconstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    pub fn prtcondet(&self) -> PrtcondetR {
        PrtcondetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    pub fn prtena(&self) -> PrtenaR {
        PrtenaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    pub fn prtenchng(&self) -> PrtenchngR {
        PrtenchngR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port overcurrent active"]
    #[inline(always)]
    pub fn prtovrcact(&self) -> PrtovrcactR {
        PrtovrcactR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    pub fn prtovrcchng(&self) -> PrtovrcchngR {
        PrtovrcchngR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    pub fn prtres(&self) -> PrtresR {
        PrtresR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    pub fn prtsusp(&self) -> PrtsuspR {
        PrtsuspR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    pub fn prtrst(&self) -> PrtrstR {
        PrtrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Port line status"]
    #[inline(always)]
    pub fn prtlnsts(&self) -> PrtlnstsR {
        PrtlnstsR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    pub fn prtpwr(&self) -> PrtpwrR {
        PrtpwrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    pub fn prttstctl(&self) -> PrttstctlR {
        PrttstctlR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:18 - Port speed"]
    #[inline(always)]
    pub fn prtspd(&self) -> PrtspdR {
        PrtspdR::new(((self.bits >> 17) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPRT")
            .field("prtconsts", &self.prtconsts())
            .field("prtcondet", &self.prtcondet())
            .field("prtena", &self.prtena())
            .field("prtenchng", &self.prtenchng())
            .field("prtovrcact", &self.prtovrcact())
            .field("prtovrcchng", &self.prtovrcchng())
            .field("prtres", &self.prtres())
            .field("prtsusp", &self.prtsusp())
            .field("prtrst", &self.prtrst())
            .field("prtlnsts", &self.prtlnsts())
            .field("prtpwr", &self.prtpwr())
            .field("prttstctl", &self.prttstctl())
            .field("prtspd", &self.prtspd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Port connect detected"]
    #[inline(always)]
    #[must_use]
    pub fn prtcondet(&mut self) -> PrtcondetW<HprtSpec> {
        PrtcondetW::new(self, 1)
    }
    #[doc = "Bit 2 - Port enable"]
    #[inline(always)]
    #[must_use]
    pub fn prtena(&mut self) -> PrtenaW<HprtSpec> {
        PrtenaW::new(self, 2)
    }
    #[doc = "Bit 3 - Port enable/disable change"]
    #[inline(always)]
    #[must_use]
    pub fn prtenchng(&mut self) -> PrtenchngW<HprtSpec> {
        PrtenchngW::new(self, 3)
    }
    #[doc = "Bit 5 - Port overcurrent change"]
    #[inline(always)]
    #[must_use]
    pub fn prtovrcchng(&mut self) -> PrtovrcchngW<HprtSpec> {
        PrtovrcchngW::new(self, 5)
    }
    #[doc = "Bit 6 - Port resume"]
    #[inline(always)]
    #[must_use]
    pub fn prtres(&mut self) -> PrtresW<HprtSpec> {
        PrtresW::new(self, 6)
    }
    #[doc = "Bit 7 - Port suspend"]
    #[inline(always)]
    #[must_use]
    pub fn prtsusp(&mut self) -> PrtsuspW<HprtSpec> {
        PrtsuspW::new(self, 7)
    }
    #[doc = "Bit 8 - Port reset"]
    #[inline(always)]
    #[must_use]
    pub fn prtrst(&mut self) -> PrtrstW<HprtSpec> {
        PrtrstW::new(self, 8)
    }
    #[doc = "Bit 12 - Port power"]
    #[inline(always)]
    #[must_use]
    pub fn prtpwr(&mut self) -> PrtpwrW<HprtSpec> {
        PrtpwrW::new(self, 12)
    }
    #[doc = "Bits 13:16 - Port test control"]
    #[inline(always)]
    #[must_use]
    pub fn prttstctl(&mut self) -> PrttstctlW<HprtSpec> {
        PrttstctlW::new(self, 13)
    }
}
#[doc = "OTGFS host port control and status register (OTGFS_HPRT)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hprt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hprt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HprtSpec;
impl crate::RegisterSpec for HprtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hprt::R`](R) reader structure"]
impl crate::Readable for HprtSpec {}
#[doc = "`write(|w| ..)` method takes [`hprt::W`](W) writer structure"]
impl crate::Writable for HprtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPRT to value 0"]
impl crate::Resettable for HprtSpec {
    const RESET_VALUE: u32 = 0;
}
