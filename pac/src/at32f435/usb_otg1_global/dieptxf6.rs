#[doc = "Register `DIEPTXF6` reader"]
pub type R = crate::R<Dieptxf6Spec>;
#[doc = "Register `DIEPTXF6` writer"]
pub type W = crate::W<Dieptxf6Spec>;
#[doc = "Field `INEPTXFSTADDR` reader - IN endpoint FIFO6 transmit RAM start address"]
pub type IneptxfstaddrR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFSTADDR` writer - IN endpoint FIFO6 transmit RAM start address"]
pub type IneptxfstaddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `INEPTXFDEP` reader - IN endpoint TxFIFO depth"]
pub type IneptxfdepR = crate::FieldReader<u16>;
#[doc = "Field `INEPTXFDEP` writer - IN endpoint TxFIFO depth"]
pub type IneptxfdepW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
    #[inline(always)]
    pub fn ineptxfstaddr(&self) -> IneptxfstaddrR {
        IneptxfstaddrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    pub fn ineptxfdep(&self) -> IneptxfdepR {
        IneptxfdepR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPTXF6")
            .field("ineptxfstaddr", &self.ineptxfstaddr())
            .field("ineptxfdep", &self.ineptxfdep())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint FIFO6 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfstaddr(&mut self) -> IneptxfstaddrW<Dieptxf6Spec> {
        IneptxfstaddrW::new(self, 0)
    }
    #[doc = "Bits 16:31 - IN endpoint TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn ineptxfdep(&mut self) -> IneptxfdepW<Dieptxf6Spec> {
        IneptxfdepW::new(self, 16)
    }
}
#[doc = "OTGFS device IN endpoint transmit FIFO size register (OTGFS_DIEPTXF6)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptxf6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptxf6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dieptxf6Spec;
impl crate::RegisterSpec for Dieptxf6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptxf6::R`](R) reader structure"]
impl crate::Readable for Dieptxf6Spec {}
#[doc = "`write(|w| ..)` method takes [`dieptxf6::W`](W) writer structure"]
impl crate::Writable for Dieptxf6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTXF6 to value 0x0200_0400"]
impl crate::Resettable for Dieptxf6Spec {
    const RESET_VALUE: u32 = 0x0200_0400;
}
