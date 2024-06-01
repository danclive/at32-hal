#[doc = "Register `INTSTS` reader"]
pub type R = crate::R<IntstsSpec>;
#[doc = "Register `INTSTS` writer"]
pub type W = crate::W<IntstsSpec>;
#[doc = "Field `EPT_NUM` reader - Endpoint number"]
pub type EptNumR = crate::FieldReader;
#[doc = "Field `EPT_NUM` writer - Endpoint number"]
pub type EptNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INOUT` reader - In/Out transaction"]
pub type InoutR = crate::BitReader;
#[doc = "Field `INOUT` writer - In/Out transaction"]
pub type InoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSOF` reader - Lost start of frame"]
pub type LsofR = crate::BitReader;
#[doc = "Field `LSOF` writer - Lost start of frame"]
pub type LsofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOF` reader - start of frame"]
pub type SofR = crate::BitReader;
#[doc = "Field `SOF` writer - start of frame"]
pub type SofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - Bus reset"]
pub type RstR = crate::BitReader;
#[doc = "Field `RST` writer - Bus reset"]
pub type RstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SP` reader - Bus suspend"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - Bus suspend"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WK` reader - Wakeup"]
pub type WkR = crate::BitReader;
#[doc = "Field `WK` writer - Wakeup"]
pub type WkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Bus error"]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Bus error"]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCFOR` reader - USB core fifo overrun memory"]
pub type UcforR = crate::BitReader;
#[doc = "Field `UCFOR` writer - USB core fifo overrun memory"]
pub type UcforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - transaction completed"]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - transaction completed"]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    pub fn ept_num(&self) -> EptNumR {
        EptNumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    pub fn inout(&self) -> InoutR {
        InoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    pub fn lsof(&self) -> LsofR {
        LsofR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    pub fn rst(&self) -> RstR {
        RstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    pub fn wk(&self) -> WkR {
        WkR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    pub fn ucfor(&self) -> UcforR {
        UcforR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTSTS")
            .field("ept_num", &self.ept_num())
            .field("inout", &self.inout())
            .field("lsof", &self.lsof())
            .field("sof", &self.sof())
            .field("rst", &self.rst())
            .field("sp", &self.sp())
            .field("wk", &self.wk())
            .field("be", &self.be())
            .field("ucfor", &self.ucfor())
            .field("tc", &self.tc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint number"]
    #[inline(always)]
    #[must_use]
    pub fn ept_num(&mut self) -> EptNumW<IntstsSpec> {
        EptNumW::new(self, 0)
    }
    #[doc = "Bit 4 - In/Out transaction"]
    #[inline(always)]
    #[must_use]
    pub fn inout(&mut self) -> InoutW<IntstsSpec> {
        InoutW::new(self, 4)
    }
    #[doc = "Bit 8 - Lost start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn lsof(&mut self) -> LsofW<IntstsSpec> {
        LsofW::new(self, 8)
    }
    #[doc = "Bit 9 - start of frame"]
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SofW<IntstsSpec> {
        SofW::new(self, 9)
    }
    #[doc = "Bit 10 - Bus reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RstW<IntstsSpec> {
        RstW::new(self, 10)
    }
    #[doc = "Bit 11 - Bus suspend"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SpW<IntstsSpec> {
        SpW::new(self, 11)
    }
    #[doc = "Bit 12 - Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn wk(&mut self) -> WkW<IntstsSpec> {
        WkW::new(self, 12)
    }
    #[doc = "Bit 13 - Bus error"]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<IntstsSpec> {
        BeW::new(self, 13)
    }
    #[doc = "Bit 14 - USB core fifo overrun memory"]
    #[inline(always)]
    #[must_use]
    pub fn ucfor(&mut self) -> UcforW<IntstsSpec> {
        UcforW::new(self, 14)
    }
    #[doc = "Bit 15 - transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<IntstsSpec> {
        TcW::new(self, 15)
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstsSpec;
impl crate::RegisterSpec for IntstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intsts::R`](R) reader structure"]
impl crate::Readable for IntstsSpec {}
#[doc = "`write(|w| ..)` method takes [`intsts::W`](W) writer structure"]
impl crate::Writable for IntstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTSTS to value 0"]
impl crate::Resettable for IntstsSpec {
    const RESET_VALUE: u32 = 0;
}
