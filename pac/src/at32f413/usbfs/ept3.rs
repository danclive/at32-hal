#[doc = "Register `EPT3` reader"]
pub type R = crate::R<Ept3Spec>;
#[doc = "Register `EPT3` writer"]
pub type W = crate::W<Ept3Spec>;
#[doc = "Field `EPTADDR` reader - Endpoint address"]
pub type EptaddrR = crate::FieldReader;
#[doc = "Field `EPTADDR` writer - Endpoint address"]
pub type EptaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXSTS` reader - Tx status"]
pub type TxstsR = crate::FieldReader;
#[doc = "Field `TXSTS` writer - Tx status"]
pub type TxstsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TXDTS` reader - Tx data toggle synchronization"]
pub type TxdtsR = crate::BitReader;
#[doc = "Field `TXDTS` writer - Tx data toggle synchronization"]
pub type TxdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTC` reader - Tx transaction completed"]
pub type TxtcR = crate::BitReader;
#[doc = "Field `TXTC` writer - Tx transaction completed"]
pub type TxtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXF` reader - Endpoint extend function"]
pub type ExfR = crate::BitReader;
#[doc = "Field `EXF` writer - Endpoint extend function"]
pub type ExfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_TYPE` reader - Transfer type"]
pub type TransTypeR = crate::FieldReader;
#[doc = "Field `TRANS_TYPE` writer - Transfer type"]
pub type TransTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SETUPTC` reader - Setup transaction completed"]
pub type SetuptcR = crate::BitReader;
#[doc = "Field `SETUPTC` writer - Setup transaction completed"]
pub type SetuptcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSTS` reader - Rx Status"]
pub type RxstsR = crate::FieldReader;
#[doc = "Field `RXSTS` writer - Rx Status"]
pub type RxstsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDTS` reader - Rx data toggle synchronization"]
pub type RxdtsR = crate::BitReader;
#[doc = "Field `RXDTS` writer - Rx data toggle synchronization"]
pub type RxdtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXTC` reader - Rx transaction completed"]
pub type RxtcR = crate::BitReader;
#[doc = "Field `RXTC` writer - Rx transaction completed"]
pub type RxtcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    pub fn eptaddr(&self) -> EptaddrR {
        EptaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    pub fn txsts(&self) -> TxstsR {
        TxstsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    pub fn txdts(&self) -> TxdtsR {
        TxdtsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    pub fn txtc(&self) -> TxtcR {
        TxtcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    pub fn exf(&self) -> ExfR {
        ExfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    pub fn trans_type(&self) -> TransTypeR {
        TransTypeR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    pub fn setuptc(&self) -> SetuptcR {
        SetuptcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    pub fn rxsts(&self) -> RxstsR {
        RxstsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    pub fn rxdts(&self) -> RxdtsR {
        RxdtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    pub fn rxtc(&self) -> RxtcR {
        RxtcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPT3")
            .field("eptaddr", &self.eptaddr())
            .field("txsts", &self.txsts())
            .field("txdts", &self.txdts())
            .field("txtc", &self.txtc())
            .field("exf", &self.exf())
            .field("trans_type", &self.trans_type())
            .field("setuptc", &self.setuptc())
            .field("rxsts", &self.rxsts())
            .field("rxdts", &self.rxdts())
            .field("rxtc", &self.rxtc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Endpoint address"]
    #[inline(always)]
    #[must_use]
    pub fn eptaddr(&mut self) -> EptaddrW<Ept3Spec> {
        EptaddrW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Tx status"]
    #[inline(always)]
    #[must_use]
    pub fn txsts(&mut self) -> TxstsW<Ept3Spec> {
        TxstsW::new(self, 4)
    }
    #[doc = "Bit 6 - Tx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn txdts(&mut self) -> TxdtsW<Ept3Spec> {
        TxdtsW::new(self, 6)
    }
    #[doc = "Bit 7 - Tx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn txtc(&mut self) -> TxtcW<Ept3Spec> {
        TxtcW::new(self, 7)
    }
    #[doc = "Bit 8 - Endpoint extend function"]
    #[inline(always)]
    #[must_use]
    pub fn exf(&mut self) -> ExfW<Ept3Spec> {
        ExfW::new(self, 8)
    }
    #[doc = "Bits 9:10 - Transfer type"]
    #[inline(always)]
    #[must_use]
    pub fn trans_type(&mut self) -> TransTypeW<Ept3Spec> {
        TransTypeW::new(self, 9)
    }
    #[doc = "Bit 11 - Setup transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn setuptc(&mut self) -> SetuptcW<Ept3Spec> {
        SetuptcW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Rx Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxsts(&mut self) -> RxstsW<Ept3Spec> {
        RxstsW::new(self, 12)
    }
    #[doc = "Bit 14 - Rx data toggle synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn rxdts(&mut self) -> RxdtsW<Ept3Spec> {
        RxdtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Rx transaction completed"]
    #[inline(always)]
    #[must_use]
    pub fn rxtc(&mut self) -> RxtcW<Ept3Spec> {
        RxtcW::new(self, 15)
    }
}
#[doc = "endpoint 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ept3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ept3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ept3Spec;
impl crate::RegisterSpec for Ept3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ept3::R`](R) reader structure"]
impl crate::Readable for Ept3Spec {}
#[doc = "`write(|w| ..)` method takes [`ept3::W`](W) writer structure"]
impl crate::Writable for Ept3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EPT3 to value 0"]
impl crate::Resettable for Ept3Spec {
    const RESET_VALUE: u32 = 0;
}
