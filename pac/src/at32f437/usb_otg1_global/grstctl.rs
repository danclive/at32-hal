#[doc = "Register `GRSTCTL` reader"]
pub type R = crate::R<GrstctlSpec>;
#[doc = "Register `GRSTCTL` writer"]
pub type W = crate::W<GrstctlSpec>;
#[doc = "Field `CSFTRST` reader - Core soft reset"]
pub type CsftrstR = crate::BitReader;
#[doc = "Field `CSFTRST` writer - Core soft reset"]
pub type CsftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIUSFTRST` reader - PIU FS Dedicated Controller Soft Reset"]
pub type PiusftrstR = crate::BitReader;
#[doc = "Field `PIUSFTRST` writer - PIU FS Dedicated Controller Soft Reset"]
pub type PiusftrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMCNTRST` reader - Host frame counter reset"]
pub type FrmcntrstR = crate::BitReader;
#[doc = "Field `FRMCNTRST` writer - Host frame counter reset"]
pub type FrmcntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFFLSH` reader - RxFIFO flush"]
pub type RxfflshR = crate::BitReader;
#[doc = "Field `RXFFLSH` writer - RxFIFO flush"]
pub type RxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFFLSH` reader - TxFIFO flush"]
pub type TxfflshR = crate::BitReader;
#[doc = "Field `TXFFLSH` writer - TxFIFO flush"]
pub type TxfflshW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `AHBIDLE` reader - AHB master idle"]
pub type AhbidleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    pub fn csftrst(&self) -> CsftrstR {
        CsftrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    pub fn piusftrst(&self) -> PiusftrstR {
        PiusftrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    pub fn frmcntrst(&self) -> FrmcntrstR {
        FrmcntrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    pub fn rxfflsh(&self) -> RxfflshR {
        RxfflshR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    pub fn txfflsh(&self) -> TxfflshR {
        TxfflshR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - AHB master idle"]
    #[inline(always)]
    pub fn ahbidle(&self) -> AhbidleR {
        AhbidleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GRSTCTL")
            .field("csftrst", &self.csftrst())
            .field("piusftrst", &self.piusftrst())
            .field("frmcntrst", &self.frmcntrst())
            .field("rxfflsh", &self.rxfflsh())
            .field("txfflsh", &self.txfflsh())
            .field("txfnum", &self.txfnum())
            .field("ahbidle", &self.ahbidle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Core soft reset"]
    #[inline(always)]
    #[must_use]
    pub fn csftrst(&mut self) -> CsftrstW<GrstctlSpec> {
        CsftrstW::new(self, 0)
    }
    #[doc = "Bit 1 - PIU FS Dedicated Controller Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn piusftrst(&mut self) -> PiusftrstW<GrstctlSpec> {
        PiusftrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Host frame counter reset"]
    #[inline(always)]
    #[must_use]
    pub fn frmcntrst(&mut self) -> FrmcntrstW<GrstctlSpec> {
        FrmcntrstW::new(self, 2)
    }
    #[doc = "Bit 4 - RxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn rxfflsh(&mut self) -> RxfflshW<GrstctlSpec> {
        RxfflshW::new(self, 4)
    }
    #[doc = "Bit 5 - TxFIFO flush"]
    #[inline(always)]
    #[must_use]
    pub fn txfflsh(&mut self) -> TxfflshW<GrstctlSpec> {
        TxfflshW::new(self, 5)
    }
    #[doc = "Bits 6:10 - TxFIFO number"]
    #[inline(always)]
    #[must_use]
    pub fn txfnum(&mut self) -> TxfnumW<GrstctlSpec> {
        TxfnumW::new(self, 6)
    }
}
#[doc = "OTGFS reset register (OTGFS_GRSTCTL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grstctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grstctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrstctlSpec;
impl crate::RegisterSpec for GrstctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grstctl::R`](R) reader structure"]
impl crate::Readable for GrstctlSpec {}
#[doc = "`write(|w| ..)` method takes [`grstctl::W`](W) writer structure"]
impl crate::Writable for GrstctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRSTCTL to value 0x2000_0000"]
impl crate::Resettable for GrstctlSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
