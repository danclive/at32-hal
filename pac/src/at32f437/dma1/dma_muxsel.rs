#[doc = "Register `DMA_MUXSEL` reader"]
pub type R = crate::R<DmaMuxselSpec>;
#[doc = "Register `DMA_MUXSEL` writer"]
pub type W = crate::W<DmaMuxselSpec>;
#[doc = "Field `TBL_SEL` reader - Multiplexer Table Select"]
pub type TblSelR = crate::BitReader;
#[doc = "Field `TBL_SEL` writer - Multiplexer Table Select"]
pub type TblSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    pub fn tbl_sel(&self) -> TblSelR {
        TblSelR::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_MUXSEL")
            .field("tbl_sel", &self.tbl_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexer Table Select"]
    #[inline(always)]
    #[must_use]
    pub fn tbl_sel(&mut self) -> TblSelW<DmaMuxselSpec> {
        TblSelW::new(self, 0)
    }
}
#[doc = "DMAMUX Table Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_muxsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_muxsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaMuxselSpec;
impl crate::RegisterSpec for DmaMuxselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_muxsel::R`](R) reader structure"]
impl crate::Readable for DmaMuxselSpec {}
#[doc = "`write(|w| ..)` method takes [`dma_muxsel::W`](W) writer structure"]
impl crate::Writable for DmaMuxselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_MUXSEL to value 0"]
impl crate::Resettable for DmaMuxselSpec {
    const RESET_VALUE: u32 = 0;
}
