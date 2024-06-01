#[doc = "Register `ACTRL` reader"]
pub type R = crate::R<ActrlSpec>;
#[doc = "Register `ACTRL` writer"]
pub type W = crate::W<ActrlSpec>;
#[doc = "Field `EISRE` reader - Enhanced image scaling resize enable"]
pub type EisreR = crate::BitReader;
#[doc = "Field `EISRE` writer - Enhanced image scaling resize enable"]
pub type EisreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFRCE` reader - Enhanced frame rate control enable"]
pub type EfrceR = crate::BitReader;
#[doc = "Field `EFRCE` writer - Enhanced frame rate control enable"]
pub type EfrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MIBE` reader - Monochrome image binarization enable"]
pub type MibeR = crate::BitReader;
#[doc = "Field `MIBE` writer - Monochrome image binarization enable"]
pub type MibeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCDES` reader - Basic pixel capture/drop extended selection"]
pub type PcdesR = crate::BitReader;
#[doc = "Field `PCDES` writer - Basic pixel capture/drop extended selection"]
pub type PcdesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFDF` reader - Enhanced function data format"]
pub type EfdfR = crate::FieldReader;
#[doc = "Field `EFDF` writer - Enhanced function data format"]
pub type EfdfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFDM` reader - Enhanced function data format management"]
pub type EfdmR = crate::BitReader;
#[doc = "Field `EFDM` writer - Enhanced function data format management"]
pub type EfdmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDUN` reader - Input data un-used number"]
pub type IdunR = crate::FieldReader;
#[doc = "Field `IDUN` writer - Input data un-used number"]
pub type IdunW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDUS` reader - Input data un-used setting"]
pub type IdusR = crate::BitReader;
#[doc = "Field `IDUS` writer - Input data un-used setting"]
pub type IdusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABT` reader - DMA burst transfer configuration"]
pub type DmabtR = crate::BitReader;
#[doc = "Field `DMABT` writer - DMA burst transfer configuration"]
pub type DmabtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEID` reader - Horizontal synchonization event and interrupt definition"]
pub type HseidR = crate::BitReader;
#[doc = "Field `HSEID` writer - Horizontal synchonization event and interrupt definition"]
pub type HseidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSEID` reader - Vertical synchonization event and interrupt definition"]
pub type VseidR = crate::BitReader;
#[doc = "Field `VSEID` writer - Vertical synchonization event and interrupt definition"]
pub type VseidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enhanced image scaling resize enable"]
    #[inline(always)]
    pub fn eisre(&self) -> EisreR {
        EisreR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enhanced frame rate control enable"]
    #[inline(always)]
    pub fn efrce(&self) -> EfrceR {
        EfrceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Monochrome image binarization enable"]
    #[inline(always)]
    pub fn mibe(&self) -> MibeR {
        MibeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Basic pixel capture/drop extended selection"]
    #[inline(always)]
    pub fn pcdes(&self) -> PcdesR {
        PcdesR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Enhanced function data format"]
    #[inline(always)]
    pub fn efdf(&self) -> EfdfR {
        EfdfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Enhanced function data format management"]
    #[inline(always)]
    pub fn efdm(&self) -> EfdmR {
        EfdmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Input data un-used number"]
    #[inline(always)]
    pub fn idun(&self) -> IdunR {
        IdunR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Input data un-used setting"]
    #[inline(always)]
    pub fn idus(&self) -> IdusR {
        IdusR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA burst transfer configuration"]
    #[inline(always)]
    pub fn dmabt(&self) -> DmabtR {
        DmabtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Horizontal synchonization event and interrupt definition"]
    #[inline(always)]
    pub fn hseid(&self) -> HseidR {
        HseidR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Vertical synchonization event and interrupt definition"]
    #[inline(always)]
    pub fn vseid(&self) -> VseidR {
        VseidR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACTRL")
            .field("vseid", &self.vseid())
            .field("hseid", &self.hseid())
            .field("dmabt", &self.dmabt())
            .field("idus", &self.idus())
            .field("idun", &self.idun())
            .field("efdm", &self.efdm())
            .field("efdf", &self.efdf())
            .field("pcdes", &self.pcdes())
            .field("mibe", &self.mibe())
            .field("efrce", &self.efrce())
            .field("eisre", &self.eisre())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enhanced image scaling resize enable"]
    #[inline(always)]
    #[must_use]
    pub fn eisre(&mut self) -> EisreW<ActrlSpec> {
        EisreW::new(self, 0)
    }
    #[doc = "Bit 1 - Enhanced frame rate control enable"]
    #[inline(always)]
    #[must_use]
    pub fn efrce(&mut self) -> EfrceW<ActrlSpec> {
        EfrceW::new(self, 1)
    }
    #[doc = "Bit 2 - Monochrome image binarization enable"]
    #[inline(always)]
    #[must_use]
    pub fn mibe(&mut self) -> MibeW<ActrlSpec> {
        MibeW::new(self, 2)
    }
    #[doc = "Bit 3 - Basic pixel capture/drop extended selection"]
    #[inline(always)]
    #[must_use]
    pub fn pcdes(&mut self) -> PcdesW<ActrlSpec> {
        PcdesW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Enhanced function data format"]
    #[inline(always)]
    #[must_use]
    pub fn efdf(&mut self) -> EfdfW<ActrlSpec> {
        EfdfW::new(self, 4)
    }
    #[doc = "Bit 6 - Enhanced function data format management"]
    #[inline(always)]
    #[must_use]
    pub fn efdm(&mut self) -> EfdmW<ActrlSpec> {
        EfdmW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Input data un-used number"]
    #[inline(always)]
    #[must_use]
    pub fn idun(&mut self) -> IdunW<ActrlSpec> {
        IdunW::new(self, 8)
    }
    #[doc = "Bit 10 - Input data un-used setting"]
    #[inline(always)]
    #[must_use]
    pub fn idus(&mut self) -> IdusW<ActrlSpec> {
        IdusW::new(self, 10)
    }
    #[doc = "Bit 12 - DMA burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn dmabt(&mut self) -> DmabtW<ActrlSpec> {
        DmabtW::new(self, 12)
    }
    #[doc = "Bit 16 - Horizontal synchonization event and interrupt definition"]
    #[inline(always)]
    #[must_use]
    pub fn hseid(&mut self) -> HseidW<ActrlSpec> {
        HseidW::new(self, 16)
    }
    #[doc = "Bit 17 - Vertical synchonization event and interrupt definition"]
    #[inline(always)]
    #[must_use]
    pub fn vseid(&mut self) -> VseidW<ActrlSpec> {
        VseidW::new(self, 17)
    }
}
#[doc = "Advanced Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`actrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`actrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ActrlSpec;
impl crate::RegisterSpec for ActrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`actrl::R`](R) reader structure"]
impl crate::Readable for ActrlSpec {}
#[doc = "`write(|w| ..)` method takes [`actrl::W`](W) writer structure"]
impl crate::Writable for ActrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACTRL to value 0"]
impl crate::Resettable for ActrlSpec {
    const RESET_VALUE: u32 = 0;
}
