#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `MEM_MAP_SEL` reader - Memory address mapping selection bits"]
pub type MemMapSelR = crate::FieldReader;
#[doc = "Field `MEM_MAP_SEL` writer - Memory address mapping selection bits"]
pub type MemMapSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PA11_12_RMP` reader - PA11 PA12 Remap"]
pub type Pa11_12RmpR = crate::BitReader;
#[doc = "Field `PA11_12_RMP` writer - PA11 PA12 Remap"]
pub type Pa11_12RmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_POL` reader - IRTMR Polariyt select"]
pub type IrPolR = crate::BitReader;
#[doc = "Field `IR_POL` writer - IRTMR Polariyt select"]
pub type IrPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_SRC_SEL` reader - IRTMR source select"]
pub type IrSrcSelR = crate::FieldReader;
#[doc = "Field `IR_SRC_SEL` writer - IRTMR source select"]
pub type IrSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADC_DMA_RMP` reader - ADC DMA remap"]
pub type AdcDmaRmpR = crate::BitReader;
#[doc = "Field `ADC_DMA_RMP` writer - ADC DMA remap"]
pub type AdcDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_TX_DMA_RMP` reader - USART1 transmit DMA remap"]
pub type Usart1TxDmaRmpR = crate::BitReader;
#[doc = "Field `USART1_TX_DMA_RMP` writer - USART1 transmit DMA remap"]
pub type Usart1TxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1_RX_DMA_RMP` reader - USART1 receive DMA remap"]
pub type Usart1RxDmaRmpR = crate::BitReader;
#[doc = "Field `USART1_RX_DMA_RMP` writer - USART1 receive DMA remap"]
pub type Usart1RxDmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR16_DMA_RMP` reader - TMR16 DMA remap"]
pub type Tmr16DmaRmpR = crate::BitReader;
#[doc = "Field `TMR16_DMA_RMP` writer - TMR16 DMA remap"]
pub type Tmr16DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR17_DMA_RMP` reader - TMR17 DMA remap"]
pub type Tmr17DmaRmpR = crate::BitReader;
#[doc = "Field `TMR17_DMA_RMP` writer - TMR17 DMA remap"]
pub type Tmr17DmaRmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&self) -> MemMapSelR {
        MemMapSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - PA11 PA12 Remap"]
    #[inline(always)]
    pub fn pa11_12_rmp(&self) -> Pa11_12RmpR {
        Pa11_12RmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IRTMR Polariyt select"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IrPolR {
        IrPolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IRTMR source select"]
    #[inline(always)]
    pub fn ir_src_sel(&self) -> IrSrcSelR {
        IrSrcSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - ADC DMA remap"]
    #[inline(always)]
    pub fn adc_dma_rmp(&self) -> AdcDmaRmpR {
        AdcDmaRmpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USART1 transmit DMA remap"]
    #[inline(always)]
    pub fn usart1_tx_dma_rmp(&self) -> Usart1TxDmaRmpR {
        Usart1TxDmaRmpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART1 receive DMA remap"]
    #[inline(always)]
    pub fn usart1_rx_dma_rmp(&self) -> Usart1RxDmaRmpR {
        Usart1RxDmaRmpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TMR16 DMA remap"]
    #[inline(always)]
    pub fn tmr16_dma_rmp(&self) -> Tmr16DmaRmpR {
        Tmr16DmaRmpR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TMR17 DMA remap"]
    #[inline(always)]
    pub fn tmr17_dma_rmp(&self) -> Tmr17DmaRmpR {
        Tmr17DmaRmpR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("tmr17_dma_rmp", &self.tmr17_dma_rmp())
            .field("tmr16_dma_rmp", &self.tmr16_dma_rmp())
            .field("usart1_rx_dma_rmp", &self.usart1_rx_dma_rmp())
            .field("usart1_tx_dma_rmp", &self.usart1_tx_dma_rmp())
            .field("adc_dma_rmp", &self.adc_dma_rmp())
            .field("ir_src_sel", &self.ir_src_sel())
            .field("ir_pol", &self.ir_pol())
            .field("pa11_12_rmp", &self.pa11_12_rmp())
            .field("mem_map_sel", &self.mem_map_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory address mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_map_sel(&mut self) -> MemMapSelW<Cfg1Spec> {
        MemMapSelW::new(self, 0)
    }
    #[doc = "Bit 4 - PA11 PA12 Remap"]
    #[inline(always)]
    #[must_use]
    pub fn pa11_12_rmp(&mut self) -> Pa11_12RmpW<Cfg1Spec> {
        Pa11_12RmpW::new(self, 4)
    }
    #[doc = "Bit 5 - IRTMR Polariyt select"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IrPolW<Cfg1Spec> {
        IrPolW::new(self, 5)
    }
    #[doc = "Bits 6:7 - IRTMR source select"]
    #[inline(always)]
    #[must_use]
    pub fn ir_src_sel(&mut self) -> IrSrcSelW<Cfg1Spec> {
        IrSrcSelW::new(self, 6)
    }
    #[doc = "Bit 8 - ADC DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn adc_dma_rmp(&mut self) -> AdcDmaRmpW<Cfg1Spec> {
        AdcDmaRmpW::new(self, 8)
    }
    #[doc = "Bit 9 - USART1 transmit DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_tx_dma_rmp(&mut self) -> Usart1TxDmaRmpW<Cfg1Spec> {
        Usart1TxDmaRmpW::new(self, 9)
    }
    #[doc = "Bit 10 - USART1 receive DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn usart1_rx_dma_rmp(&mut self) -> Usart1RxDmaRmpW<Cfg1Spec> {
        Usart1RxDmaRmpW::new(self, 10)
    }
    #[doc = "Bit 11 - TMR16 DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr16_dma_rmp(&mut self) -> Tmr16DmaRmpW<Cfg1Spec> {
        Tmr16DmaRmpW::new(self, 11)
    }
    #[doc = "Bit 12 - TMR17 DMA remap"]
    #[inline(always)]
    #[must_use]
    pub fn tmr17_dma_rmp(&mut self) -> Tmr17DmaRmpW<Cfg1Spec> {
        Tmr17DmaRmpW::new(self, 12)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
