#[doc = "Register `HCTSIZ1` reader"]
pub type R = crate::R<Hctsiz1Spec>;
#[doc = "Register `HCTSIZ1` writer"]
pub type W = crate::W<Hctsiz1Spec>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XfersizeR = crate::FieldReader<u32>;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PID` reader - PID"]
pub type PidR = crate::FieldReader;
#[doc = "Field `PID` writer - PID"]
pub type PidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    pub fn pid(&self) -> PidR {
        PidR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCTSIZ1")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .field("pid", &self.pid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XfersizeW<Hctsiz1Spec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PktcntW<Hctsiz1Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - PID"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PidW<Hctsiz1Spec> {
        PidW::new(self, 29)
    }
}
#[doc = "OTGFS host channel-1 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hctsiz1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hctsiz1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hctsiz1Spec;
impl crate::RegisterSpec for Hctsiz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hctsiz1::R`](R) reader structure"]
impl crate::Readable for Hctsiz1Spec {}
#[doc = "`write(|w| ..)` method takes [`hctsiz1::W`](W) writer structure"]
impl crate::Writable for Hctsiz1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCTSIZ1 to value 0"]
impl crate::Resettable for Hctsiz1Spec {
    const RESET_VALUE: u32 = 0;
}
