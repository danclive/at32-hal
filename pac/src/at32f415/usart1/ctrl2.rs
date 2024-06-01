#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `ID` reader - USART identification"]
pub type IdR = crate::FieldReader;
#[doc = "Field `ID` writer - USART identification"]
pub type IdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BFBN` reader - Break frame bit num"]
pub type BfbnR = crate::BitReader;
#[doc = "Field `BFBN` writer - Break frame bit num"]
pub type BfbnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFIEN` reader - Break frame interrupt enable"]
pub type BfienR = crate::BitReader;
#[doc = "Field `BFIEN` writer - Break frame interrupt enable"]
pub type BfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBCP` reader - Last bit clock pulse"]
pub type LbcpR = crate::BitReader;
#[doc = "Field `LBCP` writer - Last bit clock pulse"]
pub type LbcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPHA` reader - Clock phase"]
pub type ClkphaR = crate::BitReader;
#[doc = "Field `CLKPHA` writer - Clock phase"]
pub type ClkphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPOL` reader - Clock polarity"]
pub type ClkpolR = crate::BitReader;
#[doc = "Field `CLKPOL` writer - Clock polarity"]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPBN` reader - STOP bit num"]
pub type StopbnR = crate::FieldReader;
#[doc = "Field `STOPBN` writer - STOP bit num"]
pub type StopbnW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LINEN` reader - LIN mode enable"]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - LIN mode enable"]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    pub fn id(&self) -> IdR {
        IdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    pub fn bfbn(&self) -> BfbnR {
        BfbnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    pub fn bfien(&self) -> BfienR {
        BfienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    pub fn lbcp(&self) -> LbcpR {
        LbcpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    pub fn clkpha(&self) -> ClkphaR {
        ClkphaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    pub fn stopbn(&self) -> StopbnR {
        StopbnR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("linen", &self.linen())
            .field("stopbn", &self.stopbn())
            .field("clken", &self.clken())
            .field("clkpol", &self.clkpol())
            .field("clkpha", &self.clkpha())
            .field("lbcp", &self.lbcp())
            .field("bfien", &self.bfien())
            .field("bfbn", &self.bfbn())
            .field("id", &self.id())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - USART identification"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> IdW<Ctrl2Spec> {
        IdW::new(self, 0)
    }
    #[doc = "Bit 5 - Break frame bit num"]
    #[inline(always)]
    #[must_use]
    pub fn bfbn(&mut self) -> BfbnW<Ctrl2Spec> {
        BfbnW::new(self, 5)
    }
    #[doc = "Bit 6 - Break frame interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bfien(&mut self) -> BfienW<Ctrl2Spec> {
        BfienW::new(self, 6)
    }
    #[doc = "Bit 8 - Last bit clock pulse"]
    #[inline(always)]
    #[must_use]
    pub fn lbcp(&mut self) -> LbcpW<Ctrl2Spec> {
        LbcpW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn clkpha(&mut self) -> ClkphaW<Ctrl2Spec> {
        ClkphaW::new(self, 9)
    }
    #[doc = "Bit 10 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> ClkpolW<Ctrl2Spec> {
        ClkpolW::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn clken(&mut self) -> ClkenW<Ctrl2Spec> {
        ClkenW::new(self, 11)
    }
    #[doc = "Bits 12:13 - STOP bit num"]
    #[inline(always)]
    #[must_use]
    pub fn stopbn(&mut self) -> StopbnW<Ctrl2Spec> {
        StopbnW::new(self, 12)
    }
    #[doc = "Bit 14 - LIN mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn linen(&mut self) -> LinenW<Ctrl2Spec> {
        LinenW::new(self, 14)
    }
}
#[doc = "Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0;
}
