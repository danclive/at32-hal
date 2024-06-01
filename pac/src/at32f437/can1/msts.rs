#[doc = "Register `MSTS` reader"]
pub type R = crate::R<MstsSpec>;
#[doc = "Register `MSTS` writer"]
pub type W = crate::W<MstsSpec>;
#[doc = "Field `FZC` reader - Freeze mode confirm"]
pub type FzcR = crate::BitReader;
#[doc = "Field `DZC` reader - Doze mode confirm"]
pub type DzcR = crate::BitReader;
#[doc = "Field `EOIF` reader - Error occur Interrupt flag"]
pub type EoifR = crate::BitReader;
#[doc = "Field `EOIF` writer - Error occur Interrupt flag"]
pub type EoifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QDZIF` reader - Quit doze mode interrupt flag"]
pub type QdzifR = crate::BitReader;
#[doc = "Field `QDZIF` writer - Quit doze mode interrupt flag"]
pub type QdzifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDZIF` reader - Enter doze mode interrupt flag"]
pub type EdzifR = crate::BitReader;
#[doc = "Field `EDZIF` writer - Enter doze mode interrupt flag"]
pub type EdzifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CUSS` reader - Currently sending status"]
pub type CussR = crate::BitReader;
#[doc = "Field `CURS` reader - Currently receiving status"]
pub type CursR = crate::BitReader;
#[doc = "Field `LSAMPRX` reader - Last sample level of RX pin"]
pub type LsamprxR = crate::BitReader;
#[doc = "Field `REALRX` reader - Real time level of RX pin"]
pub type RealrxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Freeze mode confirm"]
    #[inline(always)]
    pub fn fzc(&self) -> FzcR {
        FzcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Doze mode confirm"]
    #[inline(always)]
    pub fn dzc(&self) -> DzcR {
        DzcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    pub fn eoif(&self) -> EoifR {
        EoifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    pub fn qdzif(&self) -> QdzifR {
        QdzifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    pub fn edzif(&self) -> EdzifR {
        EdzifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Currently sending status"]
    #[inline(always)]
    pub fn cuss(&self) -> CussR {
        CussR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Currently receiving status"]
    #[inline(always)]
    pub fn curs(&self) -> CursR {
        CursR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Last sample level of RX pin"]
    #[inline(always)]
    pub fn lsamprx(&self) -> LsamprxR {
        LsamprxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Real time level of RX pin"]
    #[inline(always)]
    pub fn realrx(&self) -> RealrxR {
        RealrxR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSTS")
            .field("realrx", &self.realrx())
            .field("lsamprx", &self.lsamprx())
            .field("curs", &self.curs())
            .field("cuss", &self.cuss())
            .field("edzif", &self.edzif())
            .field("qdzif", &self.qdzif())
            .field("eoif", &self.eoif())
            .field("dzc", &self.dzc())
            .field("fzc", &self.fzc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 2 - Error occur Interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoif(&mut self) -> EoifW<MstsSpec> {
        EoifW::new(self, 2)
    }
    #[doc = "Bit 3 - Quit doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn qdzif(&mut self) -> QdzifW<MstsSpec> {
        QdzifW::new(self, 3)
    }
    #[doc = "Bit 4 - Enter doze mode interrupt flag"]
    #[inline(always)]
    #[must_use]
    pub fn edzif(&mut self) -> EdzifW<MstsSpec> {
        EdzifW::new(self, 4)
    }
}
#[doc = "Main status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MstsSpec;
impl crate::RegisterSpec for MstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msts::R`](R) reader structure"]
impl crate::Readable for MstsSpec {}
#[doc = "`write(|w| ..)` method takes [`msts::W`](W) writer structure"]
impl crate::Writable for MstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MSTS to value 0x0c02"]
impl crate::Resettable for MstsSpec {
    const RESET_VALUE: u32 = 0x0c02;
}
