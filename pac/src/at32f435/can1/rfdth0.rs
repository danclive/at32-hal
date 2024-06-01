#[doc = "Register `RFDTH0` reader"]
pub type R = crate::R<Rfdth0Spec>;
#[doc = "Field `RFDT4` reader - Receive FIFO data byte 4"]
pub type Rfdt4R = crate::FieldReader;
#[doc = "Field `RFDT5` reader - Receive FIFO data byte 5"]
pub type Rfdt5R = crate::FieldReader;
#[doc = "Field `RFDT6` reader - Receive FIFO data byte 6"]
pub type Rfdt6R = crate::FieldReader;
#[doc = "Field `RFDT7` reader - Receive FIFO data byte 7"]
pub type Rfdt7R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO data byte 4"]
    #[inline(always)]
    pub fn rfdt4(&self) -> Rfdt4R {
        Rfdt4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 5"]
    #[inline(always)]
    pub fn rfdt5(&self) -> Rfdt5R {
        Rfdt5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 6"]
    #[inline(always)]
    pub fn rfdt6(&self) -> Rfdt6R {
        Rfdt6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 7"]
    #[inline(always)]
    pub fn rfdt7(&self) -> Rfdt7R {
        Rfdt7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFDTH0")
            .field("rfdt7", &self.rfdt7())
            .field("rfdt6", &self.rfdt6())
            .field("rfdt5", &self.rfdt5())
            .field("rfdt4", &self.rfdt4())
            .finish()
    }
}
#[doc = "Receive FIFO 0 high byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdth0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfdth0Spec;
impl crate::RegisterSpec for Rfdth0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdth0::R`](R) reader structure"]
impl crate::Readable for Rfdth0Spec {}
#[doc = "`reset()` method sets RFDTH0 to value 0"]
impl crate::Resettable for Rfdth0Spec {
    const RESET_VALUE: u32 = 0;
}
