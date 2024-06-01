#[doc = "Register `BK1TMGWR4` reader"]
pub type R = crate::R<Bk1tmgwr4Spec>;
#[doc = "Register `BK1TMGWR4` writer"]
pub type W = crate::W<Bk1tmgwr4Spec>;
#[doc = "Field `ADDRST` reader - Address setup time"]
pub type AddrstR = crate::FieldReader;
#[doc = "Field `ADDRST` writer - Address setup time"]
pub type AddrstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADDRHT` reader - Address-hold time"]
pub type AddrhtR = crate::FieldReader;
#[doc = "Field `ADDRHT` writer - Address-hold time"]
pub type AddrhtW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DTST` reader - Asynchronous data setup time"]
pub type DtstR = crate::FieldReader;
#[doc = "Field `DTST` writer - Asynchronous data setup time"]
pub type DtstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BUSLAT` reader - Bus latency"]
pub type BuslatR = crate::FieldReader;
#[doc = "Field `BUSLAT` writer - Bus latency"]
pub type BuslatW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ASYNCM` reader - Asynchronous mode"]
pub type AsyncmR = crate::FieldReader;
#[doc = "Field `ASYNCM` writer - Asynchronous mode"]
pub type AsyncmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    pub fn addrst(&self) -> AddrstR {
        AddrstR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    pub fn addrht(&self) -> AddrhtR {
        AddrhtR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    pub fn dtst(&self) -> DtstR {
        DtstR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    pub fn buslat(&self) -> BuslatR {
        BuslatR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    pub fn asyncm(&self) -> AsyncmR {
        AsyncmR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BK1TMGWR4")
            .field("asyncm", &self.asyncm())
            .field("buslat", &self.buslat())
            .field("dtst", &self.dtst())
            .field("addrht", &self.addrht())
            .field("addrst", &self.addrst())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn addrst(&mut self) -> AddrstW<Bk1tmgwr4Spec> {
        AddrstW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address-hold time"]
    #[inline(always)]
    #[must_use]
    pub fn addrht(&mut self) -> AddrhtW<Bk1tmgwr4Spec> {
        AddrhtW::new(self, 4)
    }
    #[doc = "Bits 8:15 - Asynchronous data setup time"]
    #[inline(always)]
    #[must_use]
    pub fn dtst(&mut self) -> DtstW<Bk1tmgwr4Spec> {
        DtstW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Bus latency"]
    #[inline(always)]
    #[must_use]
    pub fn buslat(&mut self) -> BuslatW<Bk1tmgwr4Spec> {
        BuslatW::new(self, 16)
    }
    #[doc = "Bits 28:29 - Asynchronous mode"]
    #[inline(always)]
    #[must_use]
    pub fn asyncm(&mut self) -> AsyncmW<Bk1tmgwr4Spec> {
        AsyncmW::new(self, 28)
    }
}
#[doc = "SRAM/NOR-Flash write timing registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bk1tmgwr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bk1tmgwr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bk1tmgwr4Spec;
impl crate::RegisterSpec for Bk1tmgwr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bk1tmgwr4::R`](R) reader structure"]
impl crate::Readable for Bk1tmgwr4Spec {}
#[doc = "`write(|w| ..)` method takes [`bk1tmgwr4::W`](W) writer structure"]
impl crate::Writable for Bk1tmgwr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BK1TMGWR4 to value 0x0fff_ffff"]
impl crate::Resettable for Bk1tmgwr4Spec {
    const RESET_VALUE: u32 = 0x0fff_ffff;
}
