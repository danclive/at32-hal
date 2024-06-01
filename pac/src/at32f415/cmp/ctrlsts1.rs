#[doc = "Register `CTRLSTS1` reader"]
pub type R = crate::R<Ctrlsts1Spec>;
#[doc = "Register `CTRLSTS1` writer"]
pub type W = crate::W<Ctrlsts1Spec>;
#[doc = "Field `CMP1EN` reader - Comparator1 enable bit"]
pub type Cmp1enR = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Comparator1 enable bit"]
pub type Cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1IS` reader - Comparator1 input shift"]
pub type Cmp1isR = crate::BitReader;
#[doc = "Field `CMP1IS` writer - Comparator1 input shift"]
pub type Cmp1isW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1SSEL` reader - Comparator1 speed selection"]
pub type Cmp1sselR = crate::FieldReader;
#[doc = "Field `CMP1SSEL` writer - Comparator1 speed selection"]
pub type Cmp1sselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1INVSEL` reader - Comparator1 inverting selection"]
pub type Cmp1invselR = crate::FieldReader;
#[doc = "Field `CMP1INVSEL` writer - Comparator1 inverting selection"]
pub type Cmp1invselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1TAG` reader - Comparator1 output target"]
pub type Cmp1tagR = crate::FieldReader;
#[doc = "Field `CMP1TAG` writer - Comparator1 output target"]
pub type Cmp1tagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP1P` reader - Comparator1 polarity"]
pub type Cmp1pR = crate::BitReader;
#[doc = "Field `CMP1P` writer - Comparator1 polarity"]
pub type Cmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1HYST` reader - Comparator1 hysteresis"]
pub type Cmp1hystR = crate::FieldReader;
#[doc = "Field `CMP1HYST` writer - Comparator1 hysteresis"]
pub type Cmp1hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP1VALUE` reader - Comparator1 output value"]
pub type Cmp1valueR = crate::BitReader;
#[doc = "Field `CMP1WP` reader - Comparator1 write protect"]
pub type Cmp1wpR = crate::BitReader;
#[doc = "Field `CMP1WP` writer - Comparator1 write protect"]
pub type Cmp1wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2EN` reader - Comparator2 enable bit"]
pub type Cmp2enR = crate::BitReader;
#[doc = "Field `CMP2EN` writer - Comparator2 enable bit"]
pub type Cmp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2SSEL` reader - Comparator2 speed selection"]
pub type Cmp2sselR = crate::FieldReader;
#[doc = "Field `CMP2SSEL` writer - Comparator2 speed selection"]
pub type Cmp2sselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP2INVSEL` reader - Comparator2 inverting selection"]
pub type Cmp2invselR = crate::FieldReader;
#[doc = "Field `CMP2INVSEL` writer - Comparator2 inverting selection"]
pub type Cmp2invselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DCMPEN` reader - Double comparator mode enable"]
pub type DcmpenR = crate::BitReader;
#[doc = "Field `DCMPEN` writer - Double comparator mode enable"]
pub type DcmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2TAG` reader - Comparator2 output target"]
pub type Cmp2tagR = crate::FieldReader;
#[doc = "Field `CMP2TAG` writer - Comparator2 output target"]
pub type Cmp2tagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP2P` reader - Comparator2 polarity"]
pub type Cmp2pR = crate::BitReader;
#[doc = "Field `CMP2P` writer - Comparator2 polarity"]
pub type Cmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2HYST` reader - Comparator2 hysteresis"]
pub type Cmp2hystR = crate::FieldReader;
#[doc = "Field `CMP2HYST` writer - Comparator2 hysteresis"]
pub type Cmp2hystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP2VALUE` reader - Comparator2 output value"]
pub type Cmp2valueR = crate::BitReader;
#[doc = "Field `CMP2WP` reader - Comparator2 write protect"]
pub type Cmp2wpR = crate::BitReader;
#[doc = "Field `CMP2WP` writer - Comparator2 write protect"]
pub type Cmp2wpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator1 enable bit"]
    #[inline(always)]
    pub fn cmp1en(&self) -> Cmp1enR {
        Cmp1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator1 input shift"]
    #[inline(always)]
    pub fn cmp1is(&self) -> Cmp1isR {
        Cmp1isR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator1 speed selection"]
    #[inline(always)]
    pub fn cmp1ssel(&self) -> Cmp1sselR {
        Cmp1sselR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator1 inverting selection"]
    #[inline(always)]
    pub fn cmp1invsel(&self) -> Cmp1invselR {
        Cmp1invselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Comparator1 output target"]
    #[inline(always)]
    pub fn cmp1tag(&self) -> Cmp1tagR {
        Cmp1tagR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Comparator1 polarity"]
    #[inline(always)]
    pub fn cmp1p(&self) -> Cmp1pR {
        Cmp1pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Comparator1 hysteresis"]
    #[inline(always)]
    pub fn cmp1hyst(&self) -> Cmp1hystR {
        Cmp1hystR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Comparator1 output value"]
    #[inline(always)]
    pub fn cmp1value(&self) -> Cmp1valueR {
        Cmp1valueR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator1 write protect"]
    #[inline(always)]
    pub fn cmp1wp(&self) -> Cmp1wpR {
        Cmp1wpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator2 enable bit"]
    #[inline(always)]
    pub fn cmp2en(&self) -> Cmp2enR {
        Cmp2enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Comparator2 speed selection"]
    #[inline(always)]
    pub fn cmp2ssel(&self) -> Cmp2sselR {
        Cmp2sselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Comparator2 inverting selection"]
    #[inline(always)]
    pub fn cmp2invsel(&self) -> Cmp2invselR {
        Cmp2invselR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Double comparator mode enable"]
    #[inline(always)]
    pub fn dcmpen(&self) -> DcmpenR {
        DcmpenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator2 output target"]
    #[inline(always)]
    pub fn cmp2tag(&self) -> Cmp2tagR {
        Cmp2tagR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Comparator2 polarity"]
    #[inline(always)]
    pub fn cmp2p(&self) -> Cmp2pR {
        Cmp2pR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Comparator2 hysteresis"]
    #[inline(always)]
    pub fn cmp2hyst(&self) -> Cmp2hystR {
        Cmp2hystR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Comparator2 output value"]
    #[inline(always)]
    pub fn cmp2value(&self) -> Cmp2valueR {
        Cmp2valueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator2 write protect"]
    #[inline(always)]
    pub fn cmp2wp(&self) -> Cmp2wpR {
        Cmp2wpR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRLSTS1")
            .field("cmp1en", &self.cmp1en())
            .field("cmp1is", &self.cmp1is())
            .field("cmp1ssel", &self.cmp1ssel())
            .field("cmp1invsel", &self.cmp1invsel())
            .field("cmp1tag", &self.cmp1tag())
            .field("cmp1p", &self.cmp1p())
            .field("cmp1hyst", &self.cmp1hyst())
            .field("cmp1value", &self.cmp1value())
            .field("cmp1wp", &self.cmp1wp())
            .field("cmp2en", &self.cmp2en())
            .field("cmp2ssel", &self.cmp2ssel())
            .field("cmp2invsel", &self.cmp2invsel())
            .field("dcmpen", &self.dcmpen())
            .field("cmp2tag", &self.cmp2tag())
            .field("cmp2p", &self.cmp2p())
            .field("cmp2hyst", &self.cmp2hyst())
            .field("cmp2value", &self.cmp2value())
            .field("cmp2wp", &self.cmp2wp())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Comparator1 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1en(&mut self) -> Cmp1enW<Ctrlsts1Spec> {
        Cmp1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator1 input shift"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1is(&mut self) -> Cmp1isW<Ctrlsts1Spec> {
        Cmp1isW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Comparator1 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1ssel(&mut self) -> Cmp1sselW<Ctrlsts1Spec> {
        Cmp1sselW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Comparator1 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1invsel(&mut self) -> Cmp1invselW<Ctrlsts1Spec> {
        Cmp1invselW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Comparator1 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1tag(&mut self) -> Cmp1tagW<Ctrlsts1Spec> {
        Cmp1tagW::new(self, 8)
    }
    #[doc = "Bit 11 - Comparator1 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1p(&mut self) -> Cmp1pW<Ctrlsts1Spec> {
        Cmp1pW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Comparator1 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1hyst(&mut self) -> Cmp1hystW<Ctrlsts1Spec> {
        Cmp1hystW::new(self, 12)
    }
    #[doc = "Bit 15 - Comparator1 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1wp(&mut self) -> Cmp1wpW<Ctrlsts1Spec> {
        Cmp1wpW::new(self, 15)
    }
    #[doc = "Bit 16 - Comparator2 enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2en(&mut self) -> Cmp2enW<Ctrlsts1Spec> {
        Cmp2enW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Comparator2 speed selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2ssel(&mut self) -> Cmp2sselW<Ctrlsts1Spec> {
        Cmp2sselW::new(self, 18)
    }
    #[doc = "Bits 20:22 - Comparator2 inverting selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2invsel(&mut self) -> Cmp2invselW<Ctrlsts1Spec> {
        Cmp2invselW::new(self, 20)
    }
    #[doc = "Bit 23 - Double comparator mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcmpen(&mut self) -> DcmpenW<Ctrlsts1Spec> {
        DcmpenW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Comparator2 output target"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2tag(&mut self) -> Cmp2tagW<Ctrlsts1Spec> {
        Cmp2tagW::new(self, 24)
    }
    #[doc = "Bit 27 - Comparator2 polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2p(&mut self) -> Cmp2pW<Ctrlsts1Spec> {
        Cmp2pW::new(self, 27)
    }
    #[doc = "Bits 28:29 - Comparator2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2hyst(&mut self) -> Cmp2hystW<Ctrlsts1Spec> {
        Cmp2hystW::new(self, 28)
    }
    #[doc = "Bit 31 - Comparator2 write protect"]
    #[inline(always)]
    #[must_use]
    pub fn cmp2wp(&mut self) -> Cmp2wpW<Ctrlsts1Spec> {
        Cmp2wpW::new(self, 31)
    }
}
#[doc = "CMP control/status register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlsts1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlsts1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrlsts1Spec;
impl crate::RegisterSpec for Ctrlsts1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlsts1::R`](R) reader structure"]
impl crate::Readable for Ctrlsts1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrlsts1::W`](W) writer structure"]
impl crate::Writable for Ctrlsts1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLSTS1 to value 0"]
impl crate::Resettable for Ctrlsts1Spec {
    const RESET_VALUE: u32 = 0;
}
