#[doc = "Register `STS` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `STS` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `PERR` reader - Parity error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Framing error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `NERR` reader - Noise error"]
pub type NerrR = crate::BitReader;
#[doc = "Field `ROERR` reader - Receiver overflow error"]
pub type RoerrR = crate::BitReader;
#[doc = "Field `IDLEF` reader - IDLE flag"]
pub type IdlefR = crate::BitReader;
#[doc = "Field `RDBF` reader - Receive data buffer full"]
pub type RdbfR = crate::BitReader;
#[doc = "Field `RDBF` writer - Receive data buffer full"]
pub type RdbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDC` reader - Transmit data complete"]
pub type TdcR = crate::BitReader;
#[doc = "Field `TDC` writer - Transmit data complete"]
pub type TdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBE` reader - Transmit data buffer empty"]
pub type TdbeR = crate::BitReader;
#[doc = "Field `BFF` reader - Break frame flag"]
pub type BffR = crate::BitReader;
#[doc = "Field `BFF` writer - Break frame flag"]
pub type BffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSCF` reader - CTS change flag"]
pub type CtscfR = crate::BitReader;
#[doc = "Field `CTSCF` writer - CTS change flag"]
pub type CtscfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise error"]
    #[inline(always)]
    pub fn nerr(&self) -> NerrR {
        NerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver overflow error"]
    #[inline(always)]
    pub fn roerr(&self) -> RoerrR {
        RoerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE flag"]
    #[inline(always)]
    pub fn idlef(&self) -> IdlefR {
        IdlefR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    pub fn rdbf(&self) -> RdbfR {
        RdbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit data buffer empty"]
    #[inline(always)]
    pub fn tdbe(&self) -> TdbeR {
        TdbeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    pub fn bff(&self) -> BffR {
        BffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    pub fn ctscf(&self) -> CtscfR {
        CtscfR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STS")
            .field("ctscf", &self.ctscf())
            .field("bff", &self.bff())
            .field("tdbe", &self.tdbe())
            .field("tdc", &self.tdc())
            .field("rdbf", &self.rdbf())
            .field("idlef", &self.idlef())
            .field("roerr", &self.roerr())
            .field("nerr", &self.nerr())
            .field("ferr", &self.ferr())
            .field("perr", &self.perr())
            .finish()
    }
}
impl W {
    #[doc = "Bit 5 - Receive data buffer full"]
    #[inline(always)]
    #[must_use]
    pub fn rdbf(&mut self) -> RdbfW<StsSpec> {
        RdbfW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit data complete"]
    #[inline(always)]
    #[must_use]
    pub fn tdc(&mut self) -> TdcW<StsSpec> {
        TdcW::new(self, 6)
    }
    #[doc = "Bit 8 - Break frame flag"]
    #[inline(always)]
    #[must_use]
    pub fn bff(&mut self) -> BffW<StsSpec> {
        BffW::new(self, 8)
    }
    #[doc = "Bit 9 - CTS change flag"]
    #[inline(always)]
    #[must_use]
    pub fn ctscf(&mut self) -> CtscfW<StsSpec> {
        CtscfW::new(self, 9)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STS to value 0xc0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0xc0;
}
