#[doc = "Register `HCINT10` reader"]
pub type R = crate::R<Hcint10Spec>;
#[doc = "Register `HCINT10` writer"]
pub type W = crate::W<Hcint10Spec>;
#[doc = "Field `XFERC` reader - Transfer completed"]
pub type XfercR = crate::BitReader;
#[doc = "Field `XFERC` writer - Transfer completed"]
pub type XfercW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - Channel halted"]
pub type ChhltdR = crate::BitReader;
#[doc = "Field `CHHLTD` writer - Channel halted"]
pub type ChhltdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL response received interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL response received interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK response received interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK response received interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK response received/transmitted interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK response received/transmitted interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - Transaction error"]
pub type XacterrR = crate::BitReader;
#[doc = "Field `XACTERR` writer - Transaction error"]
pub type XacterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - Babble error"]
pub type BblerrR = crate::BitReader;
#[doc = "Field `BBLERR` writer - Babble error"]
pub type BblerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - Frame overrun"]
pub type FrmovrunR = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - Frame overrun"]
pub type FrmovrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGLERR` reader - Data toggle error"]
pub type DtglerrR = crate::BitReader;
#[doc = "Field `DTGLERR` writer - Data toggle error"]
pub type DtglerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    pub fn xferc(&self) -> XfercR {
        XfercR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> ChhltdR {
        ChhltdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XacterrR {
        XacterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BblerrR {
        BblerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FrmovrunR {
        FrmovrunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    pub fn dtglerr(&self) -> DtglerrR {
        DtglerrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINT10")
            .field("xferc", &self.xferc())
            .field("chhltd", &self.chhltd())
            .field("stall", &self.stall())
            .field("nak", &self.nak())
            .field("ack", &self.ack())
            .field("xacterr", &self.xacterr())
            .field("bblerr", &self.bblerr())
            .field("frmovrun", &self.frmovrun())
            .field("dtglerr", &self.dtglerr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed"]
    #[inline(always)]
    #[must_use]
    pub fn xferc(&mut self) -> XfercW<Hcint10Spec> {
        XfercW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted"]
    #[inline(always)]
    #[must_use]
    pub fn chhltd(&mut self) -> ChhltdW<Hcint10Spec> {
        ChhltdW::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<Hcint10Spec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn nak(&mut self) -> NakW<Hcint10Spec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> AckW<Hcint10Spec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error"]
    #[inline(always)]
    #[must_use]
    pub fn xacterr(&mut self) -> XacterrW<Hcint10Spec> {
        XacterrW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error"]
    #[inline(always)]
    #[must_use]
    pub fn bblerr(&mut self) -> BblerrW<Hcint10Spec> {
        BblerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrun(&mut self) -> FrmovrunW<Hcint10Spec> {
        FrmovrunW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error"]
    #[inline(always)]
    #[must_use]
    pub fn dtglerr(&mut self) -> DtglerrW<Hcint10Spec> {
        DtglerrW::new(self, 10)
    }
}
#[doc = "OTGFS host channel-10 interrupt register (OTGFS_HCINT10)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcint10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcint10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcint10Spec;
impl crate::RegisterSpec for Hcint10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcint10::R`](R) reader structure"]
impl crate::Readable for Hcint10Spec {}
#[doc = "`write(|w| ..)` method takes [`hcint10::W`](W) writer structure"]
impl crate::Writable for Hcint10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINT10 to value 0"]
impl crate::Resettable for Hcint10Spec {
    const RESET_VALUE: u32 = 0;
}
