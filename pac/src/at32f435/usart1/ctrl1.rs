#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `SBF` reader - Send break frame"]
pub type SbfR = crate::BitReader;
#[doc = "Field `SBF` writer - Send break frame"]
pub type SbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM` reader - Receiver mute"]
pub type RmR = crate::BitReader;
#[doc = "Field `RM` writer - Receiver mute"]
pub type RmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - Receiver enable"]
pub type RenR = crate::BitReader;
#[doc = "Field `REN` writer - Receiver enable"]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - Transmitter enable"]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - Transmitter enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEIEN` reader - IDLE interrupt enable"]
pub type IdleienR = crate::BitReader;
#[doc = "Field `IDLEIEN` writer - IDLE interrupt enable"]
pub type IdleienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDBFIEN` reader - RDBF interrupt enable"]
pub type RdbfienR = crate::BitReader;
#[doc = "Field `RDBFIEN` writer - RDBF interrupt enable"]
pub type RdbfienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCIEN` reader - TDC interrupt enable"]
pub type TdcienR = crate::BitReader;
#[doc = "Field `TDCIEN` writer - TDC interrupt enable"]
pub type TdcienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBEIEN` reader - TDBE interrupt enable"]
pub type TdbeienR = crate::BitReader;
#[doc = "Field `TDBEIEN` writer - TDBE interrupt enable"]
pub type TdbeienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIEN` reader - PERR interrupt enable"]
pub type PerrienR = crate::BitReader;
#[doc = "Field `PERRIEN` writer - PERR interrupt enable"]
pub type PerrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEL` reader - Parity selection"]
pub type PselR = crate::BitReader;
#[doc = "Field `PSEL` writer - Parity selection"]
pub type PselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEN` reader - Parity enable"]
pub type PenR = crate::BitReader;
#[doc = "Field `PEN` writer - Parity enable"]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUM` reader - Wake up mode"]
pub type WumR = crate::BitReader;
#[doc = "Field `WUM` writer - Wake up mode"]
pub type WumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBN0` reader - low bit for Data bit num"]
pub type Dbn0R = crate::BitReader;
#[doc = "Field `DBN0` writer - low bit for Data bit num"]
pub type Dbn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEN` reader - USART enable"]
pub type UenR = crate::BitReader;
#[doc = "Field `UEN` writer - USART enable"]
pub type UenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCDT` reader - transmit complete delay time"]
pub type TcdtR = crate::FieldReader;
#[doc = "Field `TCDT` writer - transmit complete delay time"]
pub type TcdtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TSDT` reader - transmit start delay time"]
pub type TsdtR = crate::FieldReader;
#[doc = "Field `TSDT` writer - transmit start delay time"]
pub type TsdtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DBN1` reader - high bit for Data bit num"]
pub type Dbn1R = crate::BitReader;
#[doc = "Field `DBN1` writer - high bit for Data bit num"]
pub type Dbn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleien(&self) -> IdleienR {
        IdleienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    pub fn rdbfien(&self) -> RdbfienR {
        RdbfienR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    pub fn tdcien(&self) -> TdcienR {
        TdcienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    pub fn tdbeien(&self) -> TdbeienR {
        TdbeienR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    pub fn perrien(&self) -> PerrienR {
        PerrienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    pub fn wum(&self) -> WumR {
        WumR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - low bit for Data bit num"]
    #[inline(always)]
    pub fn dbn0(&self) -> Dbn0R {
        Dbn0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    pub fn uen(&self) -> UenR {
        UenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    pub fn tcdt(&self) -> TcdtR {
        TcdtR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    pub fn tsdt(&self) -> TsdtR {
        TsdtR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 28 - high bit for Data bit num"]
    #[inline(always)]
    pub fn dbn1(&self) -> Dbn1R {
        Dbn1R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("dbn1", &self.dbn1())
            .field("tsdt", &self.tsdt())
            .field("tcdt", &self.tcdt())
            .field("uen", &self.uen())
            .field("dbn0", &self.dbn0())
            .field("wum", &self.wum())
            .field("pen", &self.pen())
            .field("psel", &self.psel())
            .field("perrien", &self.perrien())
            .field("tdbeien", &self.tdbeien())
            .field("tdcien", &self.tdcien())
            .field("rdbfien", &self.rdbfien())
            .field("idleien", &self.idleien())
            .field("ten", &self.ten())
            .field("ren", &self.ren())
            .field("rm", &self.rm())
            .field("sbf", &self.sbf())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Send break frame"]
    #[inline(always)]
    #[must_use]
    pub fn sbf(&mut self) -> SbfW<Ctrl1Spec> {
        SbfW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver mute"]
    #[inline(always)]
    #[must_use]
    pub fn rm(&mut self) -> RmW<Ctrl1Spec> {
        RmW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn ren(&mut self) -> RenW<Ctrl1Spec> {
        RenW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<Ctrl1Spec> {
        TenW::new(self, 3)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleien(&mut self) -> IdleienW<Ctrl1Spec> {
        IdleienW::new(self, 4)
    }
    #[doc = "Bit 5 - RDBF interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdbfien(&mut self) -> RdbfienW<Ctrl1Spec> {
        RdbfienW::new(self, 5)
    }
    #[doc = "Bit 6 - TDC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdcien(&mut self) -> TdcienW<Ctrl1Spec> {
        TdcienW::new(self, 6)
    }
    #[doc = "Bit 7 - TDBE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdbeien(&mut self) -> TdbeienW<Ctrl1Spec> {
        TdbeienW::new(self, 7)
    }
    #[doc = "Bit 8 - PERR interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn perrien(&mut self) -> PerrienW<Ctrl1Spec> {
        PerrienW::new(self, 8)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<Ctrl1Spec> {
        PselW::new(self, 9)
    }
    #[doc = "Bit 10 - Parity enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<Ctrl1Spec> {
        PenW::new(self, 10)
    }
    #[doc = "Bit 11 - Wake up mode"]
    #[inline(always)]
    #[must_use]
    pub fn wum(&mut self) -> WumW<Ctrl1Spec> {
        WumW::new(self, 11)
    }
    #[doc = "Bit 12 - low bit for Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn0(&mut self) -> Dbn0W<Ctrl1Spec> {
        Dbn0W::new(self, 12)
    }
    #[doc = "Bit 13 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn uen(&mut self) -> UenW<Ctrl1Spec> {
        UenW::new(self, 13)
    }
    #[doc = "Bits 16:20 - transmit complete delay time"]
    #[inline(always)]
    #[must_use]
    pub fn tcdt(&mut self) -> TcdtW<Ctrl1Spec> {
        TcdtW::new(self, 16)
    }
    #[doc = "Bits 21:25 - transmit start delay time"]
    #[inline(always)]
    #[must_use]
    pub fn tsdt(&mut self) -> TsdtW<Ctrl1Spec> {
        TsdtW::new(self, 21)
    }
    #[doc = "Bit 28 - high bit for Data bit num"]
    #[inline(always)]
    #[must_use]
    pub fn dbn1(&mut self) -> Dbn1W<Ctrl1Spec> {
        Dbn1W::new(self, 28)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
