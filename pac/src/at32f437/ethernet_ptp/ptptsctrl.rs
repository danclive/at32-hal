#[doc = "Register `PTPTSCTRL` reader"]
pub type R = crate::R<PtptsctrlSpec>;
#[doc = "Register `PTPTSCTRL` writer"]
pub type W = crate::W<PtptsctrlSpec>;
#[doc = "Field `TE` reader - Timestamp enable"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - Timestamp enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFCU` reader - Timestamp fine or coarse update"]
pub type TfcuR = crate::BitReader;
#[doc = "Field `TFCU` writer - Timestamp fine or coarse update"]
pub type TfcuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TI` reader - Timestamp initialize"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Timestamp initialize"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TU` reader - Timestamp update"]
pub type TuR = crate::BitReader;
#[doc = "Field `TU` writer - Timestamp update"]
pub type TuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TITE` reader - Timestamp interrupt trigger enable"]
pub type TiteR = crate::BitReader;
#[doc = "Field `TITE` writer - Timestamp interrupt trigger enable"]
pub type TiteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARU` reader - Addend register update"]
pub type AruR = crate::BitReader;
#[doc = "Field `ARU` writer - Addend register update"]
pub type AruW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETAF` reader - Enable timestamp for all frames"]
pub type EtafR = crate::BitReader;
#[doc = "Field `ETAF` writer - Enable timestamp for all frames"]
pub type EtafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBRC` reader - Timestamp digital or binary rollover control"]
pub type TdbrcR = crate::BitReader;
#[doc = "Field `TDBRC` writer - Timestamp digital or binary rollover control"]
pub type TdbrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPV2F` reader - Enable PTP packet processing for version2 format"]
pub type Eppv2fR = crate::BitReader;
#[doc = "Field `EPPV2F` writer - Enable PTP packet processing for version2 format"]
pub type Eppv2fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPEF` reader - Enable processing of PTP over EMAC frames"]
pub type EppefR = crate::BitReader;
#[doc = "Field `EPPEF` writer - Enable processing of PTP over EMAC frames"]
pub type EppefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPFSIP6U` reader - Enable processing of PTP frames sent over IPv6-UDP"]
pub type Eppfsip6uR = crate::BitReader;
#[doc = "Field `EPPFSIP6U` writer - Enable processing of PTP frames sent over IPv6-UDP"]
pub type Eppfsip6uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPPFSIP4U` reader - Enable processing of PTP frames sent over IPv4-UDP"]
pub type Eppfsip4uR = crate::BitReader;
#[doc = "Field `EPPFSIP4U` writer - Enable processing of PTP frames sent over IPv4-UDP"]
pub type Eppfsip4uW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETSFEM` reader - Enable timestamp snapshot for event message"]
pub type EtsfemR = crate::BitReader;
#[doc = "Field `ETSFEM` writer - Enable timestamp snapshot for event message"]
pub type EtsfemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESFMRTM` reader - Enable snapshot for message relevant to master"]
pub type EsfmrtmR = crate::BitReader;
#[doc = "Field `ESFMRTM` writer - Enable snapshot for message relevant to master"]
pub type EsfmrtmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPPFTS` reader - Select PTP packet for taking snapshot"]
pub type SppftsR = crate::FieldReader;
#[doc = "Field `SPPFTS` writer - Select PTP packet for taking snapshot"]
pub type SppftsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EMAFPFF` reader - Enable MAC address for PTP frame filtering"]
pub type EmafpffR = crate::BitReader;
#[doc = "Field `EMAFPFF` writer - Enable MAC address for PTP frame filtering"]
pub type EmafpffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Timestamp enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timestamp fine or coarse update"]
    #[inline(always)]
    pub fn tfcu(&self) -> TfcuR {
        TfcuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timestamp initialize"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timestamp update"]
    #[inline(always)]
    pub fn tu(&self) -> TuR {
        TuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp interrupt trigger enable"]
    #[inline(always)]
    pub fn tite(&self) -> TiteR {
        TiteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Addend register update"]
    #[inline(always)]
    pub fn aru(&self) -> AruR {
        AruR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable timestamp for all frames"]
    #[inline(always)]
    pub fn etaf(&self) -> EtafR {
        EtafR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timestamp digital or binary rollover control"]
    #[inline(always)]
    pub fn tdbrc(&self) -> TdbrcR {
        TdbrcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable PTP packet processing for version2 format"]
    #[inline(always)]
    pub fn eppv2f(&self) -> Eppv2fR {
        Eppv2fR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable processing of PTP over EMAC frames"]
    #[inline(always)]
    pub fn eppef(&self) -> EppefR {
        EppefR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable processing of PTP frames sent over IPv6-UDP"]
    #[inline(always)]
    pub fn eppfsip6u(&self) -> Eppfsip6uR {
        Eppfsip6uR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable processing of PTP frames sent over IPv4-UDP"]
    #[inline(always)]
    pub fn eppfsip4u(&self) -> Eppfsip4uR {
        Eppfsip4uR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable timestamp snapshot for event message"]
    #[inline(always)]
    pub fn etsfem(&self) -> EtsfemR {
        EtsfemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable snapshot for message relevant to master"]
    #[inline(always)]
    pub fn esfmrtm(&self) -> EsfmrtmR {
        EsfmrtmR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Select PTP packet for taking snapshot"]
    #[inline(always)]
    pub fn sppfts(&self) -> SppftsR {
        SppftsR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    pub fn emafpff(&self) -> EmafpffR {
        EmafpffR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PTPTSCTRL")
            .field("te", &self.te())
            .field("tfcu", &self.tfcu())
            .field("ti", &self.ti())
            .field("tu", &self.tu())
            .field("tite", &self.tite())
            .field("aru", &self.aru())
            .field("etaf", &self.etaf())
            .field("tdbrc", &self.tdbrc())
            .field("eppv2f", &self.eppv2f())
            .field("eppef", &self.eppef())
            .field("eppfsip6u", &self.eppfsip6u())
            .field("eppfsip4u", &self.eppfsip4u())
            .field("etsfem", &self.etsfem())
            .field("esfmrtm", &self.esfmrtm())
            .field("sppfts", &self.sppfts())
            .field("emafpff", &self.emafpff())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Timestamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<PtptsctrlSpec> {
        TeW::new(self, 0)
    }
    #[doc = "Bit 1 - Timestamp fine or coarse update"]
    #[inline(always)]
    #[must_use]
    pub fn tfcu(&mut self) -> TfcuW<PtptsctrlSpec> {
        TfcuW::new(self, 1)
    }
    #[doc = "Bit 2 - Timestamp initialize"]
    #[inline(always)]
    #[must_use]
    pub fn ti(&mut self) -> TiW<PtptsctrlSpec> {
        TiW::new(self, 2)
    }
    #[doc = "Bit 3 - Timestamp update"]
    #[inline(always)]
    #[must_use]
    pub fn tu(&mut self) -> TuW<PtptsctrlSpec> {
        TuW::new(self, 3)
    }
    #[doc = "Bit 4 - Timestamp interrupt trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn tite(&mut self) -> TiteW<PtptsctrlSpec> {
        TiteW::new(self, 4)
    }
    #[doc = "Bit 5 - Addend register update"]
    #[inline(always)]
    #[must_use]
    pub fn aru(&mut self) -> AruW<PtptsctrlSpec> {
        AruW::new(self, 5)
    }
    #[doc = "Bit 8 - Enable timestamp for all frames"]
    #[inline(always)]
    #[must_use]
    pub fn etaf(&mut self) -> EtafW<PtptsctrlSpec> {
        EtafW::new(self, 8)
    }
    #[doc = "Bit 9 - Timestamp digital or binary rollover control"]
    #[inline(always)]
    #[must_use]
    pub fn tdbrc(&mut self) -> TdbrcW<PtptsctrlSpec> {
        TdbrcW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable PTP packet processing for version2 format"]
    #[inline(always)]
    #[must_use]
    pub fn eppv2f(&mut self) -> Eppv2fW<PtptsctrlSpec> {
        Eppv2fW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable processing of PTP over EMAC frames"]
    #[inline(always)]
    #[must_use]
    pub fn eppef(&mut self) -> EppefW<PtptsctrlSpec> {
        EppefW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable processing of PTP frames sent over IPv6-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn eppfsip6u(&mut self) -> Eppfsip6uW<PtptsctrlSpec> {
        Eppfsip6uW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable processing of PTP frames sent over IPv4-UDP"]
    #[inline(always)]
    #[must_use]
    pub fn eppfsip4u(&mut self) -> Eppfsip4uW<PtptsctrlSpec> {
        Eppfsip4uW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable timestamp snapshot for event message"]
    #[inline(always)]
    #[must_use]
    pub fn etsfem(&mut self) -> EtsfemW<PtptsctrlSpec> {
        EtsfemW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable snapshot for message relevant to master"]
    #[inline(always)]
    #[must_use]
    pub fn esfmrtm(&mut self) -> EsfmrtmW<PtptsctrlSpec> {
        EsfmrtmW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Select PTP packet for taking snapshot"]
    #[inline(always)]
    #[must_use]
    pub fn sppfts(&mut self) -> SppftsW<PtptsctrlSpec> {
        SppftsW::new(self, 16)
    }
    #[doc = "Bit 18 - Enable MAC address for PTP frame filtering"]
    #[inline(always)]
    #[must_use]
    pub fn emafpff(&mut self) -> EmafpffW<PtptsctrlSpec> {
        EmafpffW::new(self, 18)
    }
}
#[doc = "Ethernet PTP time stamp control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptptsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptptsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtptsctrlSpec;
impl crate::RegisterSpec for PtptsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptptsctrl::R`](R) reader structure"]
impl crate::Readable for PtptsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ptptsctrl::W`](W) writer structure"]
impl crate::Writable for PtptsctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTPTSCTRL to value 0x2000"]
impl crate::Resettable for PtptsctrlSpec {
    const RESET_VALUE: u32 = 0x2000;
}
