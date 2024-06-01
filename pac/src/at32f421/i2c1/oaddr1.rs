#[doc = "Register `OADDR1` reader"]
pub type R = crate::R<Oaddr1Spec>;
#[doc = "Register `OADDR1` writer"]
pub type W = crate::W<Oaddr1Spec>;
#[doc = "Field `ADDR1` reader - Own address 1"]
pub type Addr1R = crate::FieldReader<u16>;
#[doc = "Field `ADDR1` writer - Own address 1"]
pub type Addr1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `ADDR1MODE` reader - Address mode"]
pub type Addr1modeR = crate::BitReader;
#[doc = "Field `ADDR1MODE` writer - Address mode"]
pub type Addr1modeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    pub fn addr1mode(&self) -> Addr1modeR {
        Addr1modeR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR1")
            .field("addr1mode", &self.addr1mode())
            .field("addr1", &self.addr1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - Own address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> Addr1W<Oaddr1Spec> {
        Addr1W::new(self, 0)
    }
    #[doc = "Bit 15 - Address mode"]
    #[inline(always)]
    #[must_use]
    pub fn addr1mode(&mut self) -> Addr1modeW<Oaddr1Spec> {
        Addr1modeW::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oaddr1Spec;
impl crate::RegisterSpec for Oaddr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr1::R`](R) reader structure"]
impl crate::Readable for Oaddr1Spec {}
#[doc = "`write(|w| ..)` method takes [`oaddr1::W`](W) writer structure"]
impl crate::Writable for Oaddr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OADDR1 to value 0"]
impl crate::Resettable for Oaddr1Spec {
    const RESET_VALUE: u32 = 0;
}
