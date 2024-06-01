#[doc = "Register `OADDR2` reader"]
pub type R = crate::R<Oaddr2Spec>;
#[doc = "Register `OADDR2` writer"]
pub type W = crate::W<Oaddr2Spec>;
#[doc = "Field `ADDR2EN` reader - Own address 2 enable"]
pub type Addr2enR = crate::BitReader;
#[doc = "Field `ADDR2EN` writer - Own address 2 enable"]
pub type Addr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR2` reader - Own address 2"]
pub type Addr2R = crate::FieldReader;
#[doc = "Field `ADDR2` writer - Own address 2"]
pub type Addr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Own address 2 enable"]
    #[inline(always)]
    pub fn addr2en(&self) -> Addr2enR {
        Addr2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OADDR2")
            .field("addr2", &self.addr2())
            .field("addr2en", &self.addr2en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Own address 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn addr2en(&mut self) -> Addr2enW<Oaddr2Spec> {
        Addr2enW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Own address 2"]
    #[inline(always)]
    #[must_use]
    pub fn addr2(&mut self) -> Addr2W<Oaddr2Spec> {
        Addr2W::new(self, 1)
    }
}
#[doc = "Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oaddr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oaddr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oaddr2Spec;
impl crate::RegisterSpec for Oaddr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oaddr2::R`](R) reader structure"]
impl crate::Readable for Oaddr2Spec {}
#[doc = "`write(|w| ..)` method takes [`oaddr2::W`](W) writer structure"]
impl crate::Writable for Oaddr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OADDR2 to value 0"]
impl crate::Resettable for Oaddr2Spec {
    const RESET_VALUE: u32 = 0;
}
