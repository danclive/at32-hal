#[doc = "Register `S3FCTRL` reader"]
pub type R = crate::R<S3fctrlSpec>;
#[doc = "Register `S3FCTRL` writer"]
pub type W = crate::W<S3fctrlSpec>;
#[doc = "Field `FTHSEL` reader - FIFO threshold selection"]
pub type FthselR = crate::FieldReader;
#[doc = "Field `FTHSEL` writer - FIFO threshold selection"]
pub type FthselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FEN` reader - FIFO mode enable"]
pub type FenR = crate::BitReader;
#[doc = "Field `FEN` writer - FIFO mode enable"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSTS` reader - FIFO status"]
pub type FstsR = crate::FieldReader;
#[doc = "Field `FERRIEN` reader - FIFO error interrupt enable"]
pub type FerrienR = crate::BitReader;
#[doc = "Field `FERRIEN` writer - FIFO error interrupt enable"]
pub type FerrienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    pub fn fthsel(&self) -> FthselR {
        FthselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FIFO status"]
    #[inline(always)]
    pub fn fsts(&self) -> FstsR {
        FstsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    pub fn ferrien(&self) -> FerrienR {
        FerrienR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S3FCTRL")
            .field("ferrien", &self.ferrien())
            .field("fsts", &self.fsts())
            .field("fen", &self.fen())
            .field("fthsel", &self.fthsel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - FIFO threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn fthsel(&mut self) -> FthselW<S3fctrlSpec> {
        FthselW::new(self, 0)
    }
    #[doc = "Bit 2 - FIFO mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<S3fctrlSpec> {
        FenW::new(self, 2)
    }
    #[doc = "Bit 7 - FIFO error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferrien(&mut self) -> FerrienW<S3fctrlSpec> {
        FerrienW::new(self, 7)
    }
}
#[doc = "stream 3 FIFO control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3fctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3fctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3fctrlSpec;
impl crate::RegisterSpec for S3fctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3fctrl::R`](R) reader structure"]
impl crate::Readable for S3fctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`s3fctrl::W`](W) writer structure"]
impl crate::Writable for S3fctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3FCTRL to value 0x21"]
impl crate::Resettable for S3fctrlSpec {
    const RESET_VALUE: u32 = 0x21;
}
