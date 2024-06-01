#[doc = "Register `CMD_W1` reader"]
pub type R = crate::R<CmdW1Spec>;
#[doc = "Register `CMD_W1` writer"]
pub type W = crate::W<CmdW1Spec>;
#[doc = "Field `ADRLEN` reader - SPI address length"]
pub type AdrlenR = crate::FieldReader;
#[doc = "Field `ADRLEN` writer - SPI address length"]
pub type AdrlenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DUM2` reader - Second dummy state cycle"]
pub type Dum2R = crate::FieldReader;
#[doc = "Field `DUM2` writer - Second dummy state cycle"]
pub type Dum2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `INSLEN` reader - Instruction code length"]
pub type InslenR = crate::FieldReader;
#[doc = "Field `INSLEN` writer - Instruction code length"]
pub type InslenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PEMEN` reader - Perfrmance enhance mode enable"]
pub type PemenR = crate::BitReader;
#[doc = "Field `PEMEN` writer - Perfrmance enhance mode enable"]
pub type PemenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SPI address length"]
    #[inline(always)]
    pub fn adrlen(&self) -> AdrlenR {
        AdrlenR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:23 - Second dummy state cycle"]
    #[inline(always)]
    pub fn dum2(&self) -> Dum2R {
        Dum2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - Instruction code length"]
    #[inline(always)]
    pub fn inslen(&self) -> InslenR {
        InslenR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Perfrmance enhance mode enable"]
    #[inline(always)]
    pub fn pemen(&self) -> PemenR {
        PemenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_W1")
            .field("adrlen", &self.adrlen())
            .field("dum2", &self.dum2())
            .field("inslen", &self.inslen())
            .field("pemen", &self.pemen())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI address length"]
    #[inline(always)]
    #[must_use]
    pub fn adrlen(&mut self) -> AdrlenW<CmdW1Spec> {
        AdrlenW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Second dummy state cycle"]
    #[inline(always)]
    #[must_use]
    pub fn dum2(&mut self) -> Dum2W<CmdW1Spec> {
        Dum2W::new(self, 16)
    }
    #[doc = "Bits 24:25 - Instruction code length"]
    #[inline(always)]
    #[must_use]
    pub fn inslen(&mut self) -> InslenW<CmdW1Spec> {
        InslenW::new(self, 24)
    }
    #[doc = "Bit 28 - Perfrmance enhance mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pemen(&mut self) -> PemenW<CmdW1Spec> {
        PemenW::new(self, 28)
    }
}
#[doc = "Command word 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_w1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_w1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdW1Spec;
impl crate::RegisterSpec for CmdW1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd_w1::R`](R) reader structure"]
impl crate::Readable for CmdW1Spec {}
#[doc = "`write(|w| ..)` method takes [`cmd_w1::W`](W) writer structure"]
impl crate::Writable for CmdW1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD_W1 to value 0x0100_0003"]
impl crate::Resettable for CmdW1Spec {
    const RESET_VALUE: u32 = 0x0100_0003;
}
