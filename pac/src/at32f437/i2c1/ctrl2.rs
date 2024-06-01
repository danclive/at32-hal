#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Field `SADDR` reader - Slave address"]
pub type SaddrR = crate::FieldReader<u16>;
#[doc = "Field `SADDR` writer - Slave address"]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DIR` reader - Master data transmission direction"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Master data transmission direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR10` reader - Host send 10-bit address mode enable"]
pub type Addr10R = crate::BitReader;
#[doc = "Field `ADDR10` writer - Host send 10-bit address mode enable"]
pub type Addr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READH10` reader - 10-bit address header read enable"]
pub type Readh10R = crate::BitReader;
#[doc = "Field `READH10` writer - 10-bit address header read enable"]
pub type Readh10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTART` reader - Generate start condition"]
pub type GenstartR = crate::BitReader;
#[doc = "Field `GENSTART` writer - Generate start condition"]
pub type GenstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GENSTOP` reader - Generate stop condition"]
pub type GenstopR = crate::BitReader;
#[doc = "Field `GENSTOP` writer - Generate stop condition"]
pub type GenstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACKEN` reader - Not acknowledge enable"]
pub type NackenR = crate::BitReader;
#[doc = "Field `NACKEN` writer - Not acknowledge enable"]
pub type NackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT` reader - Transmit data counter"]
pub type CntR = crate::FieldReader;
#[doc = "Field `CNT` writer - Transmit data counter"]
pub type CntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RLDEN` reader - Send data reload mode enable"]
pub type RldenR = crate::BitReader;
#[doc = "Field `RLDEN` writer - Send data reload mode enable"]
pub type RldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASTOPEN` reader - Automatically send stop condition enable"]
pub type AstopenR = crate::BitReader;
#[doc = "Field `ASTOPEN` writer - Automatically send stop condition enable"]
pub type AstopenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PECTEN` reader - Request PEC transmission enable"]
pub type PectenR = crate::BitReader;
#[doc = "Field `PECTEN` writer - Request PEC transmission enable"]
pub type PectenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    pub fn addr10(&self) -> Addr10R {
        Addr10R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    pub fn readh10(&self) -> Readh10R {
        Readh10R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    pub fn genstart(&self) -> GenstartR {
        GenstartR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    pub fn genstop(&self) -> GenstopR {
        GenstopR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    pub fn nacken(&self) -> NackenR {
        NackenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    pub fn cnt(&self) -> CntR {
        CntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    pub fn rlden(&self) -> RldenR {
        RldenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    pub fn astopen(&self) -> AstopenR {
        AstopenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    pub fn pecten(&self) -> PectenR {
        PectenR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL2")
            .field("pecten", &self.pecten())
            .field("astopen", &self.astopen())
            .field("rlden", &self.rlden())
            .field("cnt", &self.cnt())
            .field("nacken", &self.nacken())
            .field("genstop", &self.genstop())
            .field("genstart", &self.genstart())
            .field("readh10", &self.readh10())
            .field("addr10", &self.addr10())
            .field("dir", &self.dir())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address"]
    #[inline(always)]
    #[must_use]
    pub fn saddr(&mut self) -> SaddrW<Ctrl2Spec> {
        SaddrW::new(self, 0)
    }
    #[doc = "Bit 10 - Master data transmission direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<Ctrl2Spec> {
        DirW::new(self, 10)
    }
    #[doc = "Bit 11 - Host send 10-bit address mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr10(&mut self) -> Addr10W<Ctrl2Spec> {
        Addr10W::new(self, 11)
    }
    #[doc = "Bit 12 - 10-bit address header read enable"]
    #[inline(always)]
    #[must_use]
    pub fn readh10(&mut self) -> Readh10W<Ctrl2Spec> {
        Readh10W::new(self, 12)
    }
    #[doc = "Bit 13 - Generate start condition"]
    #[inline(always)]
    #[must_use]
    pub fn genstart(&mut self) -> GenstartW<Ctrl2Spec> {
        GenstartW::new(self, 13)
    }
    #[doc = "Bit 14 - Generate stop condition"]
    #[inline(always)]
    #[must_use]
    pub fn genstop(&mut self) -> GenstopW<Ctrl2Spec> {
        GenstopW::new(self, 14)
    }
    #[doc = "Bit 15 - Not acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn nacken(&mut self) -> NackenW<Ctrl2Spec> {
        NackenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Transmit data counter"]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CntW<Ctrl2Spec> {
        CntW::new(self, 16)
    }
    #[doc = "Bit 24 - Send data reload mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn rlden(&mut self) -> RldenW<Ctrl2Spec> {
        RldenW::new(self, 24)
    }
    #[doc = "Bit 25 - Automatically send stop condition enable"]
    #[inline(always)]
    #[must_use]
    pub fn astopen(&mut self) -> AstopenW<Ctrl2Spec> {
        AstopenW::new(self, 25)
    }
    #[doc = "Bit 26 - Request PEC transmission enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecten(&mut self) -> PectenW<Ctrl2Spec> {
        PectenW::new(self, 26)
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
