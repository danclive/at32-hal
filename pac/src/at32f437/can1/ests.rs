#[doc = "Register `ESTS` reader"]
pub type R = crate::R<EstsSpec>;
#[doc = "Register `ESTS` writer"]
pub type W = crate::W<EstsSpec>;
#[doc = "Field `EAF` reader - Error active flag"]
pub type EafR = crate::BitReader;
#[doc = "Field `EPF` reader - Error passive flag"]
pub type EpfR = crate::BitReader;
#[doc = "Field `BOF` reader - Bus-off flag"]
pub type BofR = crate::BitReader;
#[doc = "Field `ETR` reader - Error type record"]
pub type EtrR = crate::FieldReader;
#[doc = "Field `ETR` writer - Error type record"]
pub type EtrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TEC` reader - Transmit error counter"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - Receive error counter"]
pub type RecR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Error active flag"]
    #[inline(always)]
    pub fn eaf(&self) -> EafR {
        EafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error passive flag"]
    #[inline(always)]
    pub fn epf(&self) -> EpfR {
        EpfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-off flag"]
    #[inline(always)]
    pub fn bof(&self) -> BofR {
        BofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    pub fn etr(&self) -> EtrR {
        EtrR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - Transmit error counter"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Receive error counter"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ESTS")
            .field("rec", &self.rec())
            .field("tec", &self.tec())
            .field("etr", &self.etr())
            .field("bof", &self.bof())
            .field("epf", &self.epf())
            .field("eaf", &self.eaf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 4:6 - Error type record"]
    #[inline(always)]
    #[must_use]
    pub fn etr(&mut self) -> EtrW<EstsSpec> {
        EtrW::new(self, 4)
    }
}
#[doc = "Error status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ests::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ests::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EstsSpec;
impl crate::RegisterSpec for EstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ests::R`](R) reader structure"]
impl crate::Readable for EstsSpec {}
#[doc = "`write(|w| ..)` method takes [`ests::W`](W) writer structure"]
impl crate::Writable for EstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ESTS to value 0"]
impl crate::Resettable for EstsSpec {
    const RESET_VALUE: u32 = 0;
}
