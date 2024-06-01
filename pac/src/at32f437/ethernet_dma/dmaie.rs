#[doc = "Register `DMAIE` reader"]
pub type R = crate::R<DmaieSpec>;
#[doc = "Register `DMAIE` writer"]
pub type W = crate::W<DmaieSpec>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - Transmit stopped enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - Transmit stopped enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUE` reader - Transmit buffer unavailable enable"]
pub type TueR = crate::BitReader;
#[doc = "Field `TUE` writer - Transmit buffer unavailable enable"]
pub type TueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TJE` reader - Transmit jabber timeout enable"]
pub type TjeR = crate::BitReader;
#[doc = "Field `TJE` writer - Transmit jabber timeout enable"]
pub type TjeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVE` reader - Overflow interrupt enable"]
pub type OveR = crate::BitReader;
#[doc = "Field `OVE` writer - Overflow interrupt enable"]
pub type OveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNE` reader - Underflow interrupt enable"]
pub type UneR = crate::BitReader;
#[doc = "Field `UNE` writer - Underflow interrupt enable"]
pub type UneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUE` reader - Receive buffer unavailable enable"]
pub type RbueR = crate::BitReader;
#[doc = "Field `RBUE` writer - Receive buffer unavailable enable"]
pub type RbueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive stopped enable"]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive stopped enable"]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTE` reader - receive watchdog timeout enable"]
pub type RwteR = crate::BitReader;
#[doc = "Field `RWTE` writer - receive watchdog timeout enable"]
pub type RwteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Early transmit interrupt enable"]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Early transmit interrupt enable"]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEE` reader - Fatal bus error enable"]
pub type FbeeR = crate::BitReader;
#[doc = "Field `FBEE` writer - Fatal bus error enable"]
pub type FbeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERE` reader - Early receive interrupt enable"]
pub type EreR = crate::BitReader;
#[doc = "Field `ERE` writer - Early receive interrupt enable"]
pub type EreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal interrupt enable"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal interrupt enable"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal interrupt enable"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal interrupt enable"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    pub fn tue(&self) -> TueR {
        TueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    pub fn tje(&self) -> TjeR {
        TjeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ove(&self) -> OveR {
        OveR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn une(&self) -> UneR {
        UneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RbueR {
        RbueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RwteR {
        RwteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FbeeR {
        FbeeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn ere(&self) -> EreR {
        EreR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAIE")
            .field("tie", &self.tie())
            .field("tse", &self.tse())
            .field("tue", &self.tue())
            .field("tje", &self.tje())
            .field("ove", &self.ove())
            .field("une", &self.une())
            .field("rie", &self.rie())
            .field("rbue", &self.rbue())
            .field("rse", &self.rse())
            .field("rwte", &self.rwte())
            .field("eie", &self.eie())
            .field("fbee", &self.fbee())
            .field("ere", &self.ere())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<DmaieSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TseW<DmaieSpec> {
        TseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    #[must_use]
    pub fn tue(&mut self) -> TueW<DmaieSpec> {
        TueW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn tje(&mut self) -> TjeW<DmaieSpec> {
        TjeW::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ove(&mut self) -> OveW<DmaieSpec> {
        OveW::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn une(&mut self) -> UneW<DmaieSpec> {
        UneW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<DmaieSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    #[must_use]
    pub fn rbue(&mut self) -> RbueW<DmaieSpec> {
        RbueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    #[must_use]
    pub fn rse(&mut self) -> RseW<DmaieSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    #[must_use]
    pub fn rwte(&mut self) -> RwteW<DmaieSpec> {
        RwteW::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EieW<DmaieSpec> {
        EieW::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    #[must_use]
    pub fn fbee(&mut self) -> FbeeW<DmaieSpec> {
        FbeeW::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ere(&mut self) -> EreW<DmaieSpec> {
        EreW::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn aie(&mut self) -> AieW<DmaieSpec> {
        AieW::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nie(&mut self) -> NieW<DmaieSpec> {
        NieW::new(self, 16)
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmaie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmaie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaieSpec;
impl crate::RegisterSpec for DmaieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaie::R`](R) reader structure"]
impl crate::Readable for DmaieSpec {}
#[doc = "`write(|w| ..)` method takes [`dmaie::W`](W) writer structure"]
impl crate::Writable for DmaieSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAIE to value 0"]
impl crate::Resettable for DmaieSpec {
    const RESET_VALUE: u32 = 0;
}
