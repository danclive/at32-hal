#[doc = "Register `S3CTRL` reader"]
pub type R = crate::R<S3ctrlSpec>;
#[doc = "Register `S3CTRL` writer"]
pub type W = crate::W<S3ctrlSpec>;
#[doc = "Field `SEN` reader - Stream enable / flag stream ready when read low"]
pub type SenR = crate::BitReader;
#[doc = "Field `SEN` writer - Stream enable / flag stream ready when read low"]
pub type SenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMERRIEN` reader - Direct mode error interrupt enable"]
pub type DmerrienR = crate::BitReader;
#[doc = "Field `DMERRIEN` writer - Direct mode error interrupt enable"]
pub type DmerrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTERRIEN` reader - Transfer error interrupt enable"]
pub type DterrienR = crate::BitReader;
#[doc = "Field `DTERRIEN` writer - Transfer error interrupt enable"]
pub type DterrienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDTIEN` reader - Half data transfer interrupt enable"]
pub type HdtienR = crate::BitReader;
#[doc = "Field `HDTIEN` writer - Half data transfer interrupt enable"]
pub type HdtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDTIEN` reader - Full data transfer complete interrupt enable"]
pub type FdtienR = crate::BitReader;
#[doc = "Field `FDTIEN` writer - Full data transfer complete interrupt enable"]
pub type FdtienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PfctrlR = crate::BitReader;
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PfctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTD` reader - Data transfer direction"]
pub type DtdR = crate::FieldReader;
#[doc = "Field `DTD` writer - Data transfer direction"]
pub type DtdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LM` reader - Loop mode"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - Loop mode"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINCM` reader - Peripheral increment mode"]
pub type PincmR = crate::BitReader;
#[doc = "Field `PINCM` writer - Peripheral increment mode"]
pub type PincmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINCM` reader - Memory increment mode"]
pub type MincmR = crate::BitReader;
#[doc = "Field `MINCM` writer - Memory increment mode"]
pub type MincmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWIDTH` reader - Peripheral data width"]
pub type PwidthR = crate::FieldReader;
#[doc = "Field `PWIDTH` writer - Peripheral data width"]
pub type PwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MWIDTH` reader - Memory data width"]
pub type MwidthR = crate::FieldReader;
#[doc = "Field `MWIDTH` writer - Memory data width"]
pub type MwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PincosR = crate::BitReader;
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PincosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` reader - Stream priority level"]
pub type SplR = crate::FieldReader;
#[doc = "Field `SPL` writer - Stream priority level"]
pub type SplW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMM` reader - Double memory mode"]
pub type DmmR = crate::BitReader;
#[doc = "Field `DMM` writer - Double memory mode"]
pub type DmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM` reader - Current memory (only in double buffer mode)"]
pub type CmR = crate::BitReader;
#[doc = "Field `CM` writer - Current memory (only in double buffer mode)"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBURST` reader - Peripheral burst transmission"]
pub type PburstR = crate::FieldReader;
#[doc = "Field `PBURST` writer - Peripheral burst transmission"]
pub type PburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBURST` reader - Memory burst transmission"]
pub type MburstR = crate::FieldReader;
#[doc = "Field `MBURST` writer - Memory burst transmission"]
pub type MburstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn sen(&self) -> SenR {
        SenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmerrien(&self) -> DmerrienR {
        DmerrienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn dterrien(&self) -> DterrienR {
        DterrienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    pub fn hdtien(&self) -> HdtienR {
        HdtienR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    pub fn fdtien(&self) -> FdtienR {
        FdtienR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PfctrlR {
        PfctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dtd(&self) -> DtdR {
        DtdR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pincm(&self) -> PincmR {
        PincmR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn mincm(&self) -> MincmR {
        MincmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    pub fn pwidth(&self) -> PwidthR {
        PwidthR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    pub fn mwidth(&self) -> MwidthR {
        MwidthR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PincosR {
        PincosR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    pub fn spl(&self) -> SplR {
        SplR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    pub fn dmm(&self) -> DmmR {
        DmmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    pub fn pburst(&self) -> PburstR {
        PburstR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    pub fn mburst(&self) -> MburstR {
        MburstR::new(((self.bits >> 23) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S3CTRL")
            .field("mburst", &self.mburst())
            .field("pburst", &self.pburst())
            .field("cm", &self.cm())
            .field("dmm", &self.dmm())
            .field("spl", &self.spl())
            .field("pincos", &self.pincos())
            .field("mwidth", &self.mwidth())
            .field("pwidth", &self.pwidth())
            .field("mincm", &self.mincm())
            .field("pincm", &self.pincm())
            .field("lm", &self.lm())
            .field("dtd", &self.dtd())
            .field("pfctrl", &self.pfctrl())
            .field("fdtien", &self.fdtien())
            .field("hdtien", &self.hdtien())
            .field("dterrien", &self.dterrien())
            .field("dmerrien", &self.dmerrien())
            .field("sen", &self.sen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SenW<S3ctrlSpec> {
        SenW::new(self, 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmerrien(&mut self) -> DmerrienW<S3ctrlSpec> {
        DmerrienW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dterrien(&mut self) -> DterrienW<S3ctrlSpec> {
        DterrienW::new(self, 2)
    }
    #[doc = "Bit 3 - Half data transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn hdtien(&mut self) -> HdtienW<S3ctrlSpec> {
        HdtienW::new(self, 3)
    }
    #[doc = "Bit 4 - Full data transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdtien(&mut self) -> FdtienW<S3ctrlSpec> {
        FdtienW::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PfctrlW<S3ctrlSpec> {
        PfctrlW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dtd(&mut self) -> DtdW<S3ctrlSpec> {
        DtdW::new(self, 6)
    }
    #[doc = "Bit 8 - Loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn lm(&mut self) -> LmW<S3ctrlSpec> {
        LmW::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pincm(&mut self) -> PincmW<S3ctrlSpec> {
        PincmW::new(self, 9)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn mincm(&mut self) -> MincmW<S3ctrlSpec> {
        MincmW::new(self, 10)
    }
    #[doc = "Bits 11:12 - Peripheral data width"]
    #[inline(always)]
    #[must_use]
    pub fn pwidth(&mut self) -> PwidthW<S3ctrlSpec> {
        PwidthW::new(self, 11)
    }
    #[doc = "Bits 13:14 - Memory data width"]
    #[inline(always)]
    #[must_use]
    pub fn mwidth(&mut self) -> MwidthW<S3ctrlSpec> {
        MwidthW::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PincosW<S3ctrlSpec> {
        PincosW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Stream priority level"]
    #[inline(always)]
    #[must_use]
    pub fn spl(&mut self) -> SplW<S3ctrlSpec> {
        SplW::new(self, 16)
    }
    #[doc = "Bit 18 - Double memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn dmm(&mut self) -> DmmW<S3ctrlSpec> {
        DmmW::new(self, 18)
    }
    #[doc = "Bit 19 - Current memory (only in double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CmW<S3ctrlSpec> {
        CmW::new(self, 19)
    }
    #[doc = "Bits 21:22 - Peripheral burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PburstW<S3ctrlSpec> {
        PburstW::new(self, 21)
    }
    #[doc = "Bits 23:24 - Memory burst transmission"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MburstW<S3ctrlSpec> {
        MburstW::new(self, 23)
    }
}
#[doc = "stream 3 control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s3ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s3ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S3ctrlSpec;
impl crate::RegisterSpec for S3ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s3ctrl::R`](R) reader structure"]
impl crate::Readable for S3ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`s3ctrl::W`](W) writer structure"]
impl crate::Writable for S3ctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S3CTRL to value 0"]
impl crate::Resettable for S3ctrlSpec {
    const RESET_VALUE: u32 = 0;
}
