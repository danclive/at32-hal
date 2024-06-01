#[doc = "Register `DIEPCTL4` reader"]
pub type R = crate::R<Diepctl4Spec>;
#[doc = "Register `DIEPCTL4` writer"]
pub type W = crate::W<Diepctl4Spec>;
#[doc = "Field `MPS` reader - Maximum packet size"]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - Maximum packet size"]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBACEPT` reader - USB active endpoint"]
pub type UsbaceptR = crate::BitReader;
#[doc = "Field `USBACEPT` writer - USB active endpoint"]
pub type UsbaceptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPID` reader - Endpoint Data PID"]
pub type DpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EptypeR = crate::FieldReader;
#[doc = "Field `EPTYPE` writer - Endpoint type"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD0PID` writer - Set DATA0 PID"]
pub type Setd0pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETD1PID` writer - Set DATA1 PID"]
pub type Setd1pidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTDIS` reader - Endpoint disable"]
pub type EptdisR = crate::BitReader;
#[doc = "Field `EPTDIS` writer - Endpoint disable"]
pub type EptdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTENA` reader - Endpoint enable"]
pub type EptenaR = crate::BitReader;
#[doc = "Field `EPTENA` writer - Endpoint enable"]
pub type EptenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbacept(&self) -> UsbaceptR {
        UsbaceptR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
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
        f.debug_struct("DIEPCTL4")
            .field("mps", &self.mps())
            .field("usbacept", &self.usbacept())
            .field("dpid", &self.dpid())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("stall", &self.stall())
            .field("txfnum", &self.txfnum())
            .field("eptdis", &self.eptdis())
            .field("eptena", &self.eptena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    #[must_use]
    pub fn mps(&mut self) -> MpsW<Diepctl4Spec> {
        MpsW::new(self, 0)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn usbacept(&mut self) -> UsbaceptW<Diepctl4Spec> {
        UsbaceptW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<Diepctl4Spec> {
        EptypeW::new(self, 18)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Diepctl4Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TxfnumW<Diepctl4Spec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CnakW<Diepctl4Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SnakW<Diepctl4Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    #[must_use]
    pub fn setd0pid(&mut self) -> Setd0pidW<Diepctl4Spec> {
        Setd0pidW::new(self, 28)
    }
    #[doc = "Bit 29 - Set DATA1 PID"]
    #[inline(always)]
    #[must_use]
    pub fn setd1pid(&mut self) -> Setd1pidW<Diepctl4Spec> {
        Setd1pidW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    #[must_use]
    pub fn eptdis(&mut self) -> EptdisW<Diepctl4Spec> {
        EptdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn eptena(&mut self) -> EptenaW<Diepctl4Spec> {
        EptenaW::new(self, 31)
    }
}
#[doc = "OTGFS device IN endpoint-4 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepctl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepctl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Diepctl4Spec;
impl crate::RegisterSpec for Diepctl4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepctl4::R`](R) reader structure"]
impl crate::Readable for Diepctl4Spec {}
#[doc = "`write(|w| ..)` method takes [`diepctl4::W`](W) writer structure"]
impl crate::Writable for Diepctl4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPCTL4 to value 0"]
impl crate::Resettable for Diepctl4Spec {
    const RESET_VALUE: u32 = 0;
}
