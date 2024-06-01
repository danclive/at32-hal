#[doc = "Register `TMDTL1` reader"]
pub type R = crate::R<Tmdtl1Spec>;
#[doc = "Register `TMDTL1` writer"]
pub type W = crate::W<Tmdtl1Spec>;
#[doc = "Field `TMDT0` reader - Transmit mailbox data byte 0"]
pub type Tmdt0R = crate::FieldReader;
#[doc = "Field `TMDT0` writer - Transmit mailbox data byte 0"]
pub type Tmdt0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT1` reader - Transmit mailbox data byte 1"]
pub type Tmdt1R = crate::FieldReader;
#[doc = "Field `TMDT1` writer - Transmit mailbox data byte 1"]
pub type Tmdt1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT2` reader - Transmit mailbox data byte 2"]
pub type Tmdt2R = crate::FieldReader;
#[doc = "Field `TMDT2` writer - Transmit mailbox data byte 2"]
pub type Tmdt2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT3` reader - Transmit mailbox data byte 3"]
pub type Tmdt3R = crate::FieldReader;
#[doc = "Field `TMDT3` writer - Transmit mailbox data byte 3"]
pub type Tmdt3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    pub fn tmdt0(&self) -> Tmdt0R {
        Tmdt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    pub fn tmdt1(&self) -> Tmdt1R {
        Tmdt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    pub fn tmdt2(&self) -> Tmdt2R {
        Tmdt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    pub fn tmdt3(&self) -> Tmdt3R {
        Tmdt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTL1")
            .field("tmdt3", &self.tmdt3())
            .field("tmdt2", &self.tmdt2())
            .field("tmdt1", &self.tmdt1())
            .field("tmdt0", &self.tmdt0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 0"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt0(&mut self) -> Tmdt0W<Tmdtl1Spec> {
        Tmdt0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 1"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt1(&mut self) -> Tmdt1W<Tmdtl1Spec> {
        Tmdt1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 2"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt2(&mut self) -> Tmdt2W<Tmdtl1Spec> {
        Tmdt2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 3"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt3(&mut self) -> Tmdt3W<Tmdtl1Spec> {
        Tmdt3W::new(self, 24)
    }
}
#[doc = "Transmit mailbox 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdtl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdtl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmdtl1Spec;
impl crate::RegisterSpec for Tmdtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdtl1::R`](R) reader structure"]
impl crate::Readable for Tmdtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`tmdtl1::W`](W) writer structure"]
impl crate::Writable for Tmdtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDTL1 to value 0"]
impl crate::Resettable for Tmdtl1Spec {
    const RESET_VALUE: u32 = 0;
}
