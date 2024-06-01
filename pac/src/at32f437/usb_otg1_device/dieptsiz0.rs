#[doc = "Register `DIEPTSIZ0` reader"]
pub type R = crate::R<Dieptsiz0Spec>;
#[doc = "Register `DIEPTSIZ0` writer"]
pub type W = crate::W<Dieptsiz0Spec>;
#[doc = "Field `XFERSIZE` reader - Transfer size"]
pub type XfersizeR = crate::FieldReader;
#[doc = "Field `XFERSIZE` writer - Transfer size"]
pub type XfersizeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XfersizeR {
        XfersizeR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTSIZ0")
            .field("xfersize", &self.xfersize())
            .field("pktcnt", &self.pktcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn xfersize(&mut self) -> XfersizeW<Dieptsiz0Spec> {
        XfersizeW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PktcntW<Dieptsiz0Spec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "OTGFS device IN endpoint-0 transfer size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptsiz0Spec;
impl crate::RegisterSpec for Dieptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz0::R`](R) reader structure"]
impl crate::Readable for Dieptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz0::W`](W) writer structure"]
impl crate::Writable for Dieptsiz0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ0 to value 0"]
impl crate::Resettable for Dieptsiz0Spec {
    const RESET_VALUE: u32 = 0;
}
