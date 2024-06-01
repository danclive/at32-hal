#[doc = "Register `REMAP6` reader"]
pub type R = crate::R<Remap6Spec>;
#[doc = "Register `REMAP6` writer"]
pub type W = crate::W<Remap6Spec>;
#[doc = "Field `CAN1_GMUX` reader - CAN1 muxing"]
pub type Can1GmuxR = crate::FieldReader;
#[doc = "Field `CAN1_GMUX` writer - CAN1 muxing"]
pub type Can1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SDIO1_GMUX` reader - SDIO1 muxing"]
pub type Sdio1GmuxR = crate::FieldReader;
#[doc = "Field `SDIO1_GMUX` writer - SDIO1 muxing"]
pub type Sdio1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART1_GMUX` reader - USART1 muxing"]
pub type Usart1GmuxR = crate::FieldReader;
#[doc = "Field `USART1_GMUX` writer - USART1 muxing"]
pub type Usart1GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USART3_GMUX` reader - USART3 muxing"]
pub type Usart3GmuxR = crate::FieldReader;
#[doc = "Field `USART3_GMUX` writer - USART3 muxing"]
pub type Usart3GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UART4_GMUX` reader - UART4 muxing"]
pub type Uart4GmuxR = crate::FieldReader;
#[doc = "Field `UART4_GMUX` writer - UART4 muxing"]
pub type Uart4GmuxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    pub fn can1_gmux(&self) -> Can1GmuxR {
        Can1GmuxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    pub fn sdio1_gmux(&self) -> Sdio1GmuxR {
        Sdio1GmuxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    pub fn usart1_gmux(&self) -> Usart1GmuxR {
        Usart1GmuxR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    pub fn usart3_gmux(&self) -> Usart3GmuxR {
        Usart3GmuxR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    pub fn uart4_gmux(&self) -> Uart4GmuxR {
        Uart4GmuxR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REMAP6")
            .field("uart4_gmux", &self.uart4_gmux())
            .field("usart3_gmux", &self.usart3_gmux())
            .field("usart1_gmux", &self.usart1_gmux())
            .field("sdio1_gmux", &self.sdio1_gmux())
            .field("can1_gmux", &self.can1_gmux())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - CAN1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn can1_gmux(&mut self) -> Can1GmuxW<Remap6Spec> {
        Can1GmuxW::new(self, 0)
    }
    #[doc = "Bits 8:11 - SDIO1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn sdio1_gmux(&mut self) -> Sdio1GmuxW<Remap6Spec> {
        Sdio1GmuxW::new(self, 8)
    }
    #[doc = "Bits 16:19 - USART1 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_gmux(&mut self) -> Usart1GmuxW<Remap6Spec> {
        Usart1GmuxW::new(self, 16)
    }
    #[doc = "Bits 24:27 - USART3 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn usart3_gmux(&mut self) -> Usart3GmuxW<Remap6Spec> {
        Usart3GmuxW::new(self, 24)
    }
    #[doc = "Bits 28:31 - UART4 muxing"]
    #[inline(always)]
    #[must_use]
    pub fn uart4_gmux(&mut self) -> Uart4GmuxW<Remap6Spec> {
        Uart4GmuxW::new(self, 28)
    }
}
#[doc = "IO MUX remap register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Remap6Spec;
impl crate::RegisterSpec for Remap6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`remap6::R`](R) reader structure"]
impl crate::Readable for Remap6Spec {}
#[doc = "`write(|w| ..)` method takes [`remap6::W`](W) writer structure"]
impl crate::Writable for Remap6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REMAP6 to value 0"]
impl crate::Resettable for Remap6Spec {
    const RESET_VALUE: u32 = 0;
}
