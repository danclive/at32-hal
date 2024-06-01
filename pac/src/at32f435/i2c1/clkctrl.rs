#[doc = "Register `CLKCTRL` reader"]
pub type R = crate::R<ClkctrlSpec>;
#[doc = "Register `CLKCTRL` writer"]
pub type W = crate::W<ClkctrlSpec>;
#[doc = "Field `SCLL` reader - SCL low level"]
pub type ScllR = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low level"]
pub type ScllW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high level"]
pub type SclhR = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high level"]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDAD` reader - SDA output delay"]
pub type SdadR = crate::FieldReader;
#[doc = "Field `SDAD` writer - SDA output delay"]
pub type SdadW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLD` reader - SCL output delay"]
pub type ScldR = crate::FieldReader;
#[doc = "Field `SCLD` writer - SCL output delay"]
pub type ScldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVH` reader - High 4 bits of clock divider value"]
pub type DivhR = crate::FieldReader;
#[doc = "Field `DIVH` writer - High 4 bits of clock divider value"]
pub type DivhW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DIVL` reader - Low 4 bits of clock divider value"]
pub type DivlR = crate::FieldReader;
#[doc = "Field `DIVL` writer - Low 4 bits of clock divider value"]
pub type DivlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCL low level"]
    #[inline(always)]
    pub fn scll(&self) -> ScllR {
        ScllR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high level"]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - SDA output delay"]
    #[inline(always)]
    pub fn sdad(&self) -> SdadR {
        SdadR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SCL output delay"]
    #[inline(always)]
    pub fn scld(&self) -> ScldR {
        ScldR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - High 4 bits of clock divider value"]
    #[inline(always)]
    pub fn divh(&self) -> DivhR {
        DivhR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Low 4 bits of clock divider value"]
    #[inline(always)]
    pub fn divl(&self) -> DivlR {
        DivlR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKCTRL")
            .field("scll", &self.scll())
            .field("sclh", &self.sclh())
            .field("sdad", &self.sdad())
            .field("scld", &self.scld())
            .field("divh", &self.divh())
            .field("divl", &self.divl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low level"]
    #[inline(always)]
    #[must_use]
    pub fn scll(&mut self) -> ScllW<ClkctrlSpec> {
        ScllW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high level"]
    #[inline(always)]
    #[must_use]
    pub fn sclh(&mut self) -> SclhW<ClkctrlSpec> {
        SclhW::new(self, 8)
    }
    #[doc = "Bits 16:19 - SDA output delay"]
    #[inline(always)]
    #[must_use]
    pub fn sdad(&mut self) -> SdadW<ClkctrlSpec> {
        SdadW::new(self, 16)
    }
    #[doc = "Bits 20:23 - SCL output delay"]
    #[inline(always)]
    #[must_use]
    pub fn scld(&mut self) -> ScldW<ClkctrlSpec> {
        ScldW::new(self, 20)
    }
    #[doc = "Bits 24:27 - High 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divh(&mut self) -> DivhW<ClkctrlSpec> {
        DivhW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Low 4 bits of clock divider value"]
    #[inline(always)]
    #[must_use]
    pub fn divl(&mut self) -> DivlW<ClkctrlSpec> {
        DivlW::new(self, 28)
    }
}
#[doc = "Clock contorl register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctrlSpec;
impl crate::RegisterSpec for ClkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctrl::R`](R) reader structure"]
impl crate::Readable for ClkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctrl::W`](W) writer structure"]
impl crate::Writable for ClkctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for ClkctrlSpec {
    const RESET_VALUE: u32 = 0;
}
