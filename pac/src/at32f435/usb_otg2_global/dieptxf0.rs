#[doc = "Register `DIEPTXF0` reader"]
pub type R = crate::R<Dieptxf0Spec>;
#[doc = "Register `DIEPTXF0` writer"]
pub type W = crate::W<Dieptxf0Spec>;
#[doc = "Field `INEPT0TXSTADDR` reader - Endpoint 0 transmit RAM start address"]
pub type Inept0txstaddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXSTADDR` writer - Endpoint 0 transmit RAM start address"]
pub type Inept0txstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPT0TXDEP` reader - Endpoint 0 TxFIFO depth"]
pub type Inept0txdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPT0TXDEP` writer - Endpoint 0 TxFIFO depth"]
pub type Inept0txdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn inept0txstaddr(&self) -> Inept0txstaddrR {
        Inept0txstaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn inept0txdep(&self) -> Inept0txdepR {
        Inept0txdepR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF0")
            .field("inept0txstaddr", &self.inept0txstaddr())
            .field("inept0txdep", &self.inept0txdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txstaddr(&mut self) -> Inept0txstaddrW<Dieptxf0Spec> {
        Inept0txstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn inept0txdep(&mut self) -> Inept0txdepW<Dieptxf0Spec> {
        Inept0txdepW::new(self, 16)
    }
}
#[doc = "IN Endpoint TxFIFO 0 transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf0Spec;
impl crate::RegisterSpec for Dieptxf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf0::R`](R) reader structure"]
impl crate::Readable for Dieptxf0Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf0::W`](W) writer structure"]
impl crate::Writable for Dieptxf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF0 to value 0x0200"]
impl crate::Resettable for Dieptxf0Spec {
    const RESET_VALUE: u32 = 0x0200;
}
