#[doc = "Register `HCINTMSK9` reader"]
pub type R = crate::R<Hcintmsk9Spec>;
#[doc = "Register `HCINTMSK9` writer"]
pub type W = crate::W<Hcintmsk9Spec>;
#[doc = "Field `XFERCMSK` reader - Transfer completed mask"]
pub type XfercmskR = crate::BitReader;
#[doc = "Field `XFERCMSK` writer - Transfer completed mask"]
pub type XfercmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTDMSK` reader - Channel halted mask"]
pub type ChhltdmskR = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - Channel halted mask"]
pub type ChhltdmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLMSK` reader - STALL response received interrupt mask"]
pub type StallmskR = crate::BitReader;
#[doc = "Field `STALLMSK` writer - STALL response received interrupt mask"]
pub type StallmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK response received interrupt mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK response received interrupt mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKMSK` reader - ACK response received/transmitted interrupt mask"]
pub type AckmskR = crate::BitReader;
#[doc = "Field `ACKMSK` writer - ACK response received/transmitted interrupt mask"]
pub type AckmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERRMSK` reader - Transaction error mask"]
pub type XacterrmskR = crate::BitReader;
#[doc = "Field `XACTERRMSK` writer - Transaction error mask"]
pub type XacterrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERRMSK` reader - Babble error mask"]
pub type BblerrmskR = crate::BitReader;
#[doc = "Field `BBLERRMSK` writer - Babble error mask"]
pub type BblerrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUNMSK` reader - Frame overrun mask"]
pub type FrmovrunmskR = crate::BitReader;
#[doc = "Field `FRMOVRUNMSK` writer - Frame overrun mask"]
pub type FrmovrunmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTGLERRMSK` reader - Data toggle error mask"]
pub type DtglerrmskR = crate::BitReader;
#[doc = "Field `DTGLERRMSK` writer - Data toggle error mask"]
pub type DtglerrmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    pub fn xfercmsk(&self) -> XfercmskR {
        XfercmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> ChhltdmskR {
        ChhltdmskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    pub fn stallmsk(&self) -> StallmskR {
        StallmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    pub fn ackmsk(&self) -> AckmskR {
        AckmskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XacterrmskR {
        XacterrmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BblerrmskR {
        BblerrmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FrmovrunmskR {
        FrmovrunmskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    pub fn dtglerrmsk(&self) -> DtglerrmskR {
        DtglerrmskR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCINTMSK9")
            .field("xfercmsk", &self.xfercmsk())
            .field("chhltdmsk", &self.chhltdmsk())
            .field("stallmsk", &self.stallmsk())
            .field("nakmsk", &self.nakmsk())
            .field("ackmsk", &self.ackmsk())
            .field("xacterrmsk", &self.xacterrmsk())
            .field("bblerrmsk", &self.bblerrmsk())
            .field("frmovrunmsk", &self.frmovrunmsk())
            .field("dtglerrmsk", &self.dtglerrmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercmsk(&mut self) -> XfercmskW<Hcintmsk9Spec> {
        XfercmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel halted mask"]
    #[inline(always)]
    #[must_use]
    pub fn chhltdmsk(&mut self) -> ChhltdmskW<Hcintmsk9Spec> {
        ChhltdmskW::new(self, 1)
    }
    #[doc = "Bit 3 - STALL response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn stallmsk(&mut self) -> StallmskW<Hcintmsk9Spec> {
        StallmskW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK response received interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NakmskW<Hcintmsk9Spec> {
        NakmskW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK response received/transmitted interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn ackmsk(&mut self) -> AckmskW<Hcintmsk9Spec> {
        AckmskW::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction error mask"]
    #[inline(always)]
    #[must_use]
    pub fn xacterrmsk(&mut self) -> XacterrmskW<Hcintmsk9Spec> {
        XacterrmskW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble error mask"]
    #[inline(always)]
    #[must_use]
    pub fn bblerrmsk(&mut self) -> BblerrmskW<Hcintmsk9Spec> {
        BblerrmskW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame overrun mask"]
    #[inline(always)]
    #[must_use]
    pub fn frmovrunmsk(&mut self) -> FrmovrunmskW<Hcintmsk9Spec> {
        FrmovrunmskW::new(self, 9)
    }
    #[doc = "Bit 10 - Data toggle error mask"]
    #[inline(always)]
    #[must_use]
    pub fn dtglerrmsk(&mut self) -> DtglerrmskW<Hcintmsk9Spec> {
        DtglerrmskW::new(self, 10)
    }
}
#[doc = "OTGFS host channel-9 mask register (OTGFS_HCINTMSK9)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcintmsk9::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hcintmsk9::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcintmsk9Spec;
impl crate::RegisterSpec for Hcintmsk9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcintmsk9::R`](R) reader structure"]
impl crate::Readable for Hcintmsk9Spec {}
#[doc = "`write(|w| ..)` method takes [`hcintmsk9::W`](W) writer structure"]
impl crate::Writable for Hcintmsk9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HCINTMSK9 to value 0"]
impl crate::Resettable for Hcintmsk9Spec {
    const RESET_VALUE: u32 = 0;
}
