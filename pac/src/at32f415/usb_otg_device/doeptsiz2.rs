#[doc = "Register `DOEPTSIZ2` reader"]
pub type R = crate::R<Doeptsiz2Spec>;
#[doc = "Register `DOEPTSIZ2` writer"]
pub type W = crate::W<Doeptsiz2Spec>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RXDPID` reader - Received data PID"]
pub type RxdpidR = crate::FieldReader;
#[doc = "Field `RXDPID` writer - Received data PID"]
pub type RxdpidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Received data PID"]
    #[inline(always)]
    pub fn rxdpid(&self) -> RxdpidR {
        RxdpidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ2")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("rxdpid", &self.rxdpid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XfersizeW<Doeptsiz2Spec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PktcntW<Doeptsiz2Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Received data PID"]
    #[inline(always)]
    #[must_use]
    pub fn rxdpid(&mut self) -> RxdpidW<Doeptsiz2Spec> {
        RxdpidW::new(self, 29)
    }
}
#[doc = "OTGFS device OUT endpoint-2 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz2Spec;
impl crate::RegisterSpec for Doeptsiz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz2::R`](R) reader structure"]
impl crate::Readable for Doeptsiz2Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz2::W`](W) writer structure"]
impl crate::Writable for Doeptsiz2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ2 to value 0"]
impl crate::Resettable for Doeptsiz2Spec {
    const RESET_VALUE: u32 = 0;
}
