#[doc = "Register `TMI2` reader"]
pub type R = crate::R<Tmi2Spec>;
#[doc = "Register `TMI2` writer"]
pub type W = crate::W<Tmi2Spec>;
#[doc = "Field `TMSR` reader - Transmit mailbox send request"]
pub type TmsrR = crate::BitReader;
#[doc = "Field `TMSR` writer - Transmit mailbox send request"]
pub type TmsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFRSEL` reader - Transmit mailbox frame type select"]
pub type TmfrselR = crate::BitReader;
#[doc = "Field `TMFRSEL` writer - Transmit mailbox frame type select"]
pub type TmfrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMIDSEL` reader - Transmit mailbox identifier type select"]
pub type TmidselR = crate::BitReader;
#[doc = "Field `TMIDSEL` writer - Transmit mailbox identifier type select"]
pub type TmidselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMEID` reader - Ttransmit mailbox extended identifier"]
pub type TmeidR = crate::FieldReader<u32>;
#[doc = "Field `TMEID` writer - Ttransmit mailbox extended identifier"]
pub type TmeidW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `TMSID` reader - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TmsidR = crate::FieldReader<u16>;
#[doc = "Field `TMSID` writer - Transmit mailbox standard identifier or extended identifier high bytes"]
pub type TmsidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    pub fn tmsr(&self) -> TmsrR {
        TmsrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    pub fn tmfrsel(&self) -> TmfrselR {
        TmfrselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    pub fn tmidsel(&self) -> TmidselR {
        TmidselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    pub fn tmeid(&self) -> TmeidR {
        TmeidR::new((self.bits >> 3) & 0x0003_ffff)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    pub fn tmsid(&self) -> TmsidR {
        TmsidR::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMI2")
            .field("tmsid", &self.tmsid())
            .field("tmeid", &self.tmeid())
            .field("tmidsel", &self.tmidsel())
            .field("tmfrsel", &self.tmfrsel())
            .field("tmsr", &self.tmsr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit mailbox send request"]
    #[inline(always)]
    #[must_use]
    pub fn tmsr(&mut self) -> TmsrW<Tmi2Spec> {
        TmsrW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit mailbox frame type select"]
    #[inline(always)]
    #[must_use]
    pub fn tmfrsel(&mut self) -> TmfrselW<Tmi2Spec> {
        TmfrselW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit mailbox identifier type select"]
    #[inline(always)]
    #[must_use]
    pub fn tmidsel(&mut self) -> TmidselW<Tmi2Spec> {
        TmidselW::new(self, 2)
    }
    #[doc = "Bits 3:20 - Ttransmit mailbox extended identifier"]
    #[inline(always)]
    #[must_use]
    pub fn tmeid(&mut self) -> TmeidW<Tmi2Spec> {
        TmeidW::new(self, 3)
    }
    #[doc = "Bits 21:31 - Transmit mailbox standard identifier or extended identifier high bytes"]
    #[inline(always)]
    #[must_use]
    pub fn tmsid(&mut self) -> TmsidW<Tmi2Spec> {
        TmsidW::new(self, 21)
    }
}
#[doc = "Transmit mailbox 2 identifier register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmi2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmi2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmi2Spec;
impl crate::RegisterSpec for Tmi2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmi2::R`](R) reader structure"]
impl crate::Readable for Tmi2Spec {}
#[doc = "`write(|w| ..)` method takes [`tmi2::W`](W) writer structure"]
impl crate::Writable for Tmi2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMI2 to value 0"]
impl crate::Resettable for Tmi2Spec {
    const RESET_VALUE: u32 = 0;
}
