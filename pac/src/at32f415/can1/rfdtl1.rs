#[doc = "Register `RFDTL1` reader"]
pub type R = crate::R<Rfdtl1Spec>;
#[doc = "Field `RFDT0` reader - Receive FIFO data byte 0"]
pub type Rfdt0R = crate::FieldReader;
#[doc = "Field `RFDT1` reader - Receive FIFO data byte 1"]
pub type Rfdt1R = crate::FieldReader;
#[doc = "Field `RFDT2` reader - Receive FIFO data byte 2"]
pub type Rfdt2R = crate::FieldReader;
#[doc = "Field `RFDT3` reader - Receive FIFO data byte 3"]
pub type Rfdt3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive FIFO data byte 0"]
    #[inline(always)]
    pub fn rfdt0(&self) -> Rfdt0R {
        Rfdt0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive FIFO data byte 1"]
    #[inline(always)]
    pub fn rfdt1(&self) -> Rfdt1R {
        Rfdt1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Receive FIFO data byte 2"]
    #[inline(always)]
    pub fn rfdt2(&self) -> Rfdt2R {
        Rfdt2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive FIFO data byte 3"]
    #[inline(always)]
    pub fn rfdt3(&self) -> Rfdt3R {
        Rfdt3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RFDTL1")
            .field("rfdt3", &self.rfdt3())
            .field("rfdt2", &self.rfdt2())
            .field("rfdt1", &self.rfdt1())
            .field("rfdt0", &self.rfdt0())
            .finish()
    }
}
#[doc = "Receive FIFO 1 low byte data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfdtl1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rfdtl1Spec;
impl crate::RegisterSpec for Rfdtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfdtl1::R`](R) reader structure"]
impl crate::Readable for Rfdtl1Spec {}
#[doc = "`reset()` method sets RFDTL1 to value 0"]
impl crate::Resettable for Rfdtl1Spec {
    const RESET_VALUE: u32 = 0;
}
