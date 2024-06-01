#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CctrlSpec>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CctrlSpec>;
#[doc = "Field `MSSEL` reader - Master slave mode select"]
pub type MsselR = crate::FieldReader;
#[doc = "Field `MSSEL` writer - Master slave mode select"]
pub type MsselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ASISEL` reader - Adjacent ADC sampling interval select for ordinary shifting mode"]
pub type AsiselR = crate::FieldReader;
#[doc = "Field `ASISEL` writer - Adjacent ADC sampling interval select for ordinary shifting mode"]
pub type AsiselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MSDRCEN` reader - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MsdrcenR = crate::BitReader;
#[doc = "Field `MSDRCEN` writer - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MsdrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDMASEL_L` reader - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MsdmaselLR = crate::FieldReader;
#[doc = "Field `MSDMASEL_L` writer - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MsdmaselLW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCDIV` reader - ADC division"]
pub type AdcdivR = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - ADC division"]
pub type AdcdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VbatenR = crate::BitReader;
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VbatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSRVEN` reader - Internal temperature sensor and VINTRV enable"]
pub type ItsrvenR = crate::BitReader;
#[doc = "Field `ITSRVEN` writer - Internal temperature sensor and VINTRV enable"]
pub type ItsrvenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSDMASEL_H` reader - High bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MsdmaselHR = crate::BitReader;
#[doc = "Field `MSDMASEL_H` writer - High bit of ordinary channel DMA transfer mode select for master slave mode"]
pub type MsdmaselHW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&self) -> MsselR {
        MsselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select for ordinary shifting mode"]
    #[inline(always)]
    pub fn asisel(&self) -> AsiselR {
        AsiselR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    pub fn msdrcen(&self) -> MsdrcenR {
        MsdrcenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    pub fn msdmasel_l(&self) -> MsdmaselLR {
        MsdmaselLR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC division"]
    #[inline(always)]
    pub fn adcdiv(&self) -> AdcdivR {
        AdcdivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VbatenR {
        VbatenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    pub fn itsrven(&self) -> ItsrvenR {
        ItsrvenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - High bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    pub fn msdmasel_h(&self) -> MsdmaselHR {
        MsdmaselHR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("msdmasel_h", &self.msdmasel_h())
            .field("itsrven", &self.itsrven())
            .field("vbaten", &self.vbaten())
            .field("adcdiv", &self.adcdiv())
            .field("msdmasel_l", &self.msdmasel_l())
            .field("msdrcen", &self.msdrcen())
            .field("asisel", &self.asisel())
            .field("mssel", &self.mssel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    #[must_use]
    pub fn mssel(&mut self) -> MsselW<CctrlSpec> {
        MsselW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select for ordinary shifting mode"]
    #[inline(always)]
    #[must_use]
    pub fn asisel(&mut self) -> AsiselW<CctrlSpec> {
        AsiselW::new(self, 8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdrcen(&mut self) -> MsdrcenW<CctrlSpec> {
        MsdrcenW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Low bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdmasel_l(&mut self) -> MsdmaselLW<CctrlSpec> {
        MsdmaselLW::new(self, 14)
    }
    #[doc = "Bits 16:19 - ADC division"]
    #[inline(always)]
    #[must_use]
    pub fn adcdiv(&mut self) -> AdcdivW<CctrlSpec> {
        AdcdivW::new(self, 16)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VbatenW<CctrlSpec> {
        VbatenW::new(self, 22)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    #[must_use]
    pub fn itsrven(&mut self) -> ItsrvenW<CctrlSpec> {
        ItsrvenW::new(self, 23)
    }
    #[doc = "Bit 28 - High bit of ordinary channel DMA transfer mode select for master slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msdmasel_h(&mut self) -> MsdmaselHW<CctrlSpec> {
        MsdmaselHW::new(self, 28)
    }
}
#[doc = "Common control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CctrlSpec;
impl crate::RegisterSpec for CctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CctrlSpec {
    const RESET_VALUE: u32 = 0;
}
