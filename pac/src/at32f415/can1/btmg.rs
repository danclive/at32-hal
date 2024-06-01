#[doc = "Register `BTMG` reader"]
pub type R = crate::R<BtmgSpec>;
#[doc = "Register `BTMG` writer"]
pub type W = crate::W<BtmgSpec>;
#[doc = "Field `BRDIV` reader - Baud rate division"]
pub type BrdivR = crate::FieldReader<u16>;
#[doc = "Field `BRDIV` writer - Baud rate division"]
pub type BrdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BTS1` reader - Bit time segment 1"]
pub type Bts1R = crate::FieldReader;
#[doc = "Field `BTS1` writer - Bit time segment 1"]
pub type Bts1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BTS2` reader - Bit time segment 2"]
pub type Bts2R = crate::FieldReader;
#[doc = "Field `BTS2` writer - Bit time segment 2"]
pub type Bts2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSAW` reader - Resynchronization adjust width"]
pub type RsawR = crate::FieldReader;
#[doc = "Field `RSAW` writer - Resynchronization adjust width"]
pub type RsawW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LBEN` reader - Loop back mode"]
pub type LbenR = crate::BitReader;
#[doc = "Field `LBEN` writer - Loop back mode"]
pub type LbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOEN` reader - Listen-Only mode"]
pub type LoenR = crate::BitReader;
#[doc = "Field `LOEN` writer - Listen-Only mode"]
pub type LoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    pub fn brdiv(&self) -> BrdivR {
        BrdivR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    pub fn bts1(&self) -> Bts1R {
        Bts1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    pub fn bts2(&self) -> Bts2R {
        Bts2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    pub fn rsaw(&self) -> RsawR {
        RsawR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    pub fn lben(&self) -> LbenR {
        LbenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    pub fn loen(&self) -> LoenR {
        LoenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BTMG")
            .field("loen", &self.loen())
            .field("lben", &self.lben())
            .field("rsaw", &self.rsaw())
            .field("bts2", &self.bts2())
            .field("bts1", &self.bts1())
            .field("brdiv", &self.brdiv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Baud rate division"]
    #[inline(always)]
    #[must_use]
    pub fn brdiv(&mut self) -> BrdivW<BtmgSpec> {
        BrdivW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Bit time segment 1"]
    #[inline(always)]
    #[must_use]
    pub fn bts1(&mut self) -> Bts1W<BtmgSpec> {
        Bts1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Bit time segment 2"]
    #[inline(always)]
    #[must_use]
    pub fn bts2(&mut self) -> Bts2W<BtmgSpec> {
        Bts2W::new(self, 20)
    }
    #[doc = "Bits 24:25 - Resynchronization adjust width"]
    #[inline(always)]
    #[must_use]
    pub fn rsaw(&mut self) -> RsawW<BtmgSpec> {
        RsawW::new(self, 24)
    }
    #[doc = "Bit 30 - Loop back mode"]
    #[inline(always)]
    #[must_use]
    pub fn lben(&mut self) -> LbenW<BtmgSpec> {
        LbenW::new(self, 30)
    }
    #[doc = "Bit 31 - Listen-Only mode"]
    #[inline(always)]
    #[must_use]
    pub fn loen(&mut self) -> LoenW<BtmgSpec> {
        LoenW::new(self, 31)
    }
}
#[doc = "Bit timing register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btmg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btmg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtmgSpec;
impl crate::RegisterSpec for BtmgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btmg::R`](R) reader structure"]
impl crate::Readable for BtmgSpec {}
#[doc = "`write(|w| ..)` method takes [`btmg::W`](W) writer structure"]
impl crate::Writable for BtmgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BTMG to value 0"]
impl crate::Resettable for BtmgSpec {
    const RESET_VALUE: u32 = 0;
}
