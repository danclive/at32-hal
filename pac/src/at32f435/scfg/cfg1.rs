#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `MEM_MAP_SEL` reader - Memory address mapping selection bits"]
pub type MemMapSelR = crate::FieldReader;
#[doc = "Field `MEM_MAP_SEL` writer - Memory address mapping selection bits"]
pub type MemMapSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IrPolR = crate::BitReader;
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IrPolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IR_SRC_SEL` reader - IR signal source selection"]
pub type IrSrcSelR = crate::FieldReader;
#[doc = "Field `IR_SRC_SEL` writer - IR signal source selection"]
pub type IrSrcSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWAP_XMC` reader - XMC address mapping swap"]
pub type SwapXmcR = crate::FieldReader;
#[doc = "Field `SWAP_XMC` writer - XMC address mapping swap"]
pub type SwapXmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&self) -> MemMapSelR {
        MemMapSelR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IrPolR {
        IrPolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&self) -> IrSrcSelR {
        IrSrcSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - XMC address mapping swap"]
    #[inline(always)]
    pub fn swap_xmc(&self) -> SwapXmcR {
        SwapXmcR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("mem_map_sel", &self.mem_map_sel())
            .field("ir_pol", &self.ir_pol())
            .field("ir_src_sel", &self.ir_src_sel())
            .field("swap_xmc", &self.swap_xmc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_map_sel(&mut self) -> MemMapSelW<Cfg1Spec> {
        MemMapSelW::new(self, 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_pol(&mut self) -> IrPolW<Cfg1Spec> {
        IrPolW::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    #[must_use]
    pub fn ir_src_sel(&mut self) -> IrSrcSelW<Cfg1Spec> {
        IrSrcSelW::new(self, 6)
    }
    #[doc = "Bits 10:11 - XMC address mapping swap"]
    #[inline(always)]
    #[must_use]
    pub fn swap_xmc(&mut self) -> SwapXmcW<Cfg1Spec> {
        SwapXmcW::new(self, 10)
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
