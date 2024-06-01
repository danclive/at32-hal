#[doc = "Register `DOEPCTL0` reader"]
pub type R = crate::R<Doepctl0Spec>;
#[doc = "Register `DOEPCTL0` writer"]
pub type W = crate::W<Doepctl0Spec>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MpsR = crate::FieldReader;
#[doc = "Field `USBACEPT` reader - USB active endpoint"]
pub type UsbaceptR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `SNP` reader - Snoop mode"]
pub type SnpR = crate::BitReader;
#[doc = "Field `SNP` writer - Snoop mode"]
pub type SnpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDIS` reader - Endpoint disable"]
pub type EptdisR = crate::BitReader;
#[doc = "Field `EPTDIS` writer - Endpoint disable"]
pub type EptdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTENA` reader - Endpoint enable"]
pub type EptenaR = crate::BitReader;
#[doc = "Field `EPTENA` writer - Endpoint enable"]
pub type EptenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbacept(&self) -> UsbaceptR {
        UsbaceptR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snp(&self) -> SnpR {
        SnpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn eptdis(&self) -> EptdisR {
        EptdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn eptena(&self) -> EptenaR {
        EptenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL0")
            .field("mps", &self.mps())
            .field("usbacept", &self.usbacept())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("eptdis", &self.eptdis())
            .field("eptena", &self.eptena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SnpW<Doepctl0Spec> {
        SnpW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Doepctl0Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CnakW<Doepctl0Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SnakW<Doepctl0Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn eptdis(&mut self) -> EptdisW<Doepctl0Spec> {
        EptdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn eptena(&mut self) -> EptenaW<Doepctl0Spec> {
        EptenaW::new(self, 31)
    }
}
#[doc = "OTGFS device OUT endpoint-0 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doepctl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doepctl0Spec;
impl crate::RegisterSpec for Doepctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepctl0::R`](R) reader structure"]
impl crate::Readable for Doepctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`doepctl0::W`](W) writer structure"]
impl crate::Writable for Doepctl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPCTL0 to value 0x8000"]
impl crate::Resettable for Doepctl0Spec {
    const RESET_VALUE: u32 = 0x8000;
}
