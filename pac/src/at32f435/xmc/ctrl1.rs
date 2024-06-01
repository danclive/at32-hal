#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `CA` reader - Number of column address bits"]
pub type CaR = crate::FieldReader;
#[doc = "Field `CA` writer - Number of column address bits"]
pub type CaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RA` reader - Number of row address bits"]
pub type RaR = crate::FieldReader;
#[doc = "Field `RA` writer - Number of row address bits"]
pub type RaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DB` reader - Memory data bus width"]
pub type DbR = crate::FieldReader;
#[doc = "Field `DB` writer - Memory data bus width"]
pub type DbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INBK` reader - Number of internal banks"]
pub type InbkR = crate::BitReader;
#[doc = "Field `INBK` writer - Number of internal banks"]
pub type InbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAS` reader - CAS latency"]
pub type CasR = crate::FieldReader;
#[doc = "Field `CAS` writer - CAS latency"]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WRP` reader - Write protection"]
pub type WrpR = crate::BitReader;
#[doc = "Field `WRP` writer - Write protection"]
pub type WrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDIV` reader - Clock division configuration"]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Clock division configuration"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BSTR` reader - Burst read"]
pub type BstrR = crate::BitReader;
#[doc = "Field `BSTR` writer - Burst read"]
pub type BstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Read delay"]
pub type RdR = crate::FieldReader;
#[doc = "Field `RD` writer - Read delay"]
pub type RdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    pub fn ca(&self) -> CaR {
        CaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    pub fn db(&self) -> DbR {
        DbR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    pub fn inbk(&self) -> InbkR {
        InbkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    pub fn wrp(&self) -> WrpR {
        WrpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Clock division configuration"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    pub fn bstr(&self) -> BstrR {
        BstrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Read delay"]
    #[inline(always)]
    pub fn rd(&self) -> RdR {
        RdR::new(((self.bits >> 13) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("ca", &self.ca())
            .field("ra", &self.ra())
            .field("db", &self.db())
            .field("inbk", &self.inbk())
            .field("cas", &self.cas())
            .field("wrp", &self.wrp())
            .field("clkdiv", &self.clkdiv())
            .field("bstr", &self.bstr())
            .field("rd", &self.rd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of column address bits"]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CaW<Ctrl1Spec> {
        CaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Number of row address bits"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RaW<Ctrl1Spec> {
        RaW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Memory data bus width"]
    #[inline(always)]
    #[must_use]
    pub fn db(&mut self) -> DbW<Ctrl1Spec> {
        DbW::new(self, 4)
    }
    #[doc = "Bit 6 - Number of internal banks"]
    #[inline(always)]
    #[must_use]
    pub fn inbk(&mut self) -> InbkW<Ctrl1Spec> {
        InbkW::new(self, 6)
    }
    #[doc = "Bits 7:8 - CAS latency"]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<Ctrl1Spec> {
        CasW::new(self, 7)
    }
    #[doc = "Bit 9 - Write protection"]
    #[inline(always)]
    #[must_use]
    pub fn wrp(&mut self) -> WrpW<Ctrl1Spec> {
        WrpW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Clock division configuration"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> ClkdivW<Ctrl1Spec> {
        ClkdivW::new(self, 10)
    }
    #[doc = "Bit 12 - Burst read"]
    #[inline(always)]
    #[must_use]
    pub fn bstr(&mut self) -> BstrW<Ctrl1Spec> {
        BstrW::new(self, 12)
    }
    #[doc = "Bits 13:14 - Read delay"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RdW<Ctrl1Spec> {
        RdW::new(self, 13)
    }
}
#[doc = "SDRAM Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL1 to value 0x02d0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0x02d0;
}
