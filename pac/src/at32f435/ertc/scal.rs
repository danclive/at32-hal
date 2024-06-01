#[doc = "Register `SCAL` reader"]
pub type R = crate::R<ScalSpec>;
#[doc = "Register `SCAL` writer"]
pub type W = crate::W<ScalSpec>;
#[doc = "Field `DEC` reader - Decrease ERTC clock"]
pub type DecR = crate::FieldReader<u16>;
#[doc = "Field `DEC` writer - Decrease ERTC clock"]
pub type DecW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `CAL16` reader - 16 second calibration period"]
pub type Cal16R = crate::BitReader;
#[doc = "Field `CAL16` writer - 16 second calibration period"]
pub type Cal16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAL8` reader - 8-second calibration period"]
pub type Cal8R = crate::BitReader;
#[doc = "Field `CAL8` writer - 8-second calibration period"]
pub type Cal8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD` reader - Add ERTC clock"]
pub type AddR = crate::BitReader;
#[doc = "Field `ADD` writer - Add ERTC clock"]
pub type AddW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    pub fn dec(&self) -> DecR {
        DecR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    pub fn cal16(&self) -> Cal16R {
        Cal16R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    pub fn cal8(&self) -> Cal8R {
        Cal8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    pub fn add(&self) -> AddR {
        AddR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCAL")
            .field("add", &self.add())
            .field("cal8", &self.cal8())
            .field("cal16", &self.cal16())
            .field("dec", &self.dec())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Decrease ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DecW<ScalSpec> {
        DecW::new(self, 0)
    }
    #[doc = "Bit 13 - 16 second calibration period"]
    #[inline(always)]
    #[must_use]
    pub fn cal16(&mut self) -> Cal16W<ScalSpec> {
        Cal16W::new(self, 13)
    }
    #[doc = "Bit 14 - 8-second calibration period"]
    #[inline(always)]
    #[must_use]
    pub fn cal8(&mut self) -> Cal8W<ScalSpec> {
        Cal8W::new(self, 14)
    }
    #[doc = "Bit 15 - Add ERTC clock"]
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> AddW<ScalSpec> {
        AddW::new(self, 15)
    }
}
#[doc = "calibration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScalSpec;
impl crate::RegisterSpec for ScalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scal::R`](R) reader structure"]
impl crate::Readable for ScalSpec {}
#[doc = "`write(|w| ..)` method takes [`scal::W`](W) writer structure"]
impl crate::Writable for ScalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCAL to value 0"]
impl crate::Resettable for ScalSpec {
    const RESET_VALUE: u32 = 0;
}
