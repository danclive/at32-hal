#[doc = "Register `TMDTH2` reader"]
pub type R = crate::R<Tmdth2Spec>;
#[doc = "Register `TMDTH2` writer"]
pub type W = crate::W<Tmdth2Spec>;
#[doc = "Field `TMDT4` reader - Transmit mailbox data byte 4"]
pub type Tmdt4R = crate::FieldReader;
#[doc = "Field `TMDT4` writer - Transmit mailbox data byte 4"]
pub type Tmdt4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT5` reader - Transmit mailbox data byte 5"]
pub type Tmdt5R = crate::FieldReader;
#[doc = "Field `TMDT5` writer - Transmit mailbox data byte 5"]
pub type Tmdt5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT6` reader - Transmit mailbox data byte 6"]
pub type Tmdt6R = crate::FieldReader;
#[doc = "Field `TMDT6` writer - Transmit mailbox data byte 6"]
pub type Tmdt6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TMDT7` reader - Transmit mailbox data byte 7"]
pub type Tmdt7R = crate::FieldReader;
#[doc = "Field `TMDT7` writer - Transmit mailbox data byte 7"]
pub type Tmdt7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    pub fn tmdt4(&self) -> Tmdt4R {
        Tmdt4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    pub fn tmdt5(&self) -> Tmdt5R {
        Tmdt5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    pub fn tmdt6(&self) -> Tmdt6R {
        Tmdt6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    pub fn tmdt7(&self) -> Tmdt7R {
        Tmdt7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMDTH2")
            .field("tmdt7", &self.tmdt7())
            .field("tmdt6", &self.tmdt6())
            .field("tmdt5", &self.tmdt5())
            .field("tmdt4", &self.tmdt4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit mailbox data byte 4"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt4(&mut self) -> Tmdt4W<Tmdth2Spec> {
        Tmdt4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Transmit mailbox data byte 5"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt5(&mut self) -> Tmdt5W<Tmdth2Spec> {
        Tmdt5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Transmit mailbox data byte 6"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt6(&mut self) -> Tmdt6W<Tmdth2Spec> {
        Tmdt6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Transmit mailbox data byte 7"]
    #[inline(always)]
    #[must_use]
    pub fn tmdt7(&mut self) -> Tmdt7W<Tmdth2Spec> {
        Tmdt7W::new(self, 24)
    }
}
#[doc = "Transmit mailbox 2 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmdth2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmdth2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tmdth2Spec;
impl crate::RegisterSpec for Tmdth2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmdth2::R`](R) reader structure"]
impl crate::Readable for Tmdth2Spec {}
#[doc = "`write(|w| ..)` method takes [`tmdth2::W`](W) writer structure"]
impl crate::Writable for Tmdth2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMDTH2 to value 0"]
impl crate::Resettable for Tmdth2Spec {
    const RESET_VALUE: u32 = 0;
}
