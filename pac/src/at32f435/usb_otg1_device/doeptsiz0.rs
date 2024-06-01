#[doc = "Register `DOEPTSIZ0` reader"]
pub type R = crate::R<Doeptsiz0Spec>;
#[doc = "Register `DOEPTSIZ0` writer"]
pub type W = crate::W<Doeptsiz0Spec>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XfersizeR = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::BitReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUPCNT` reader - SETUP packet count"]
pub type SetupcntR = crate::FieldReader;
#[doc = "Field `SETUPCNT` writer - SETUP packet count"]
pub type SetupcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    pub fn setupcnt(&self) -> SetupcntR {
        SetupcntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPTSIZ0")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("setupcnt", &self.setupcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XfersizeW<Doeptsiz0Spec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bit 19 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PktcntW<Doeptsiz0Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - SETUP packet count"]
    #[inline(always)]
    #[must_use]
    pub fn setupcnt(&mut self) -> SetupcntW<Doeptsiz0Spec> {
        SetupcntW::new(self, 29)
    }
}
#[doc = "OTGFS device OUT endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doeptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doeptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doeptsiz0Spec;
impl crate::RegisterSpec for Doeptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doeptsiz0::R`](R) reader structure"]
impl crate::Readable for Doeptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`doeptsiz0::W`](W) writer structure"]
impl crate::Writable for Doeptsiz0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOEPTSIZ0 to value 0"]
impl crate::Resettable for Doeptsiz0Spec {
    const RESET_VALUE: u32 = 0;
}
