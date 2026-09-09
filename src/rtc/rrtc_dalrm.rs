#[doc = "Register `RRTC_DALRM` reader"]
pub type R = crate::R<RrtcDalrmSpec>;
#[doc = "Register `RRTC_DALRM` writer"]
pub type W = crate::W<RrtcDalrmSpec>;
#[doc = "Field `D` reader - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
pub type DR = crate::FieldReader;
#[doc = "Field `D` writer - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
pub type DW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TD` reader - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
pub type TdR = crate::FieldReader;
#[doc = "Field `TD` writer - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
pub type TdW<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
#[doc = "Field `M` reader - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TM` reader - Поле десятков месяцев. Допустимые значения от 0 до 1"]
pub type TmR = crate::BitReader;
#[doc = "Field `TM` writer - Поле десятков месяцев. Допустимые значения от 0 до 1"]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Y` reader - Поле единиц годов. Допустимые значения от 0 до 9"]
pub type YR = crate::FieldReader;
#[doc = "Field `Y` writer - Поле единиц годов. Допустимые значения от 0 до 9"]
pub type YW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TY` reader - Поле десятков годов. Допустимые значения от 0 до 9"]
pub type TyR = crate::FieldReader;
#[doc = "Field `TY` writer - Поле десятков годов. Допустимые значения от 0 до 9"]
pub type TyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C` reader - Поле единиц веков. Допустимые значения от 0 до 9"]
pub type CR = crate::FieldReader;
#[doc = "Field `C` writer - Поле единиц веков. Допустимые значения от 0 до 9"]
pub type CW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TC` reader - Поле десятков веков. Допустимые значения от 0 до 9"]
pub type TcR = crate::FieldReader;
#[doc = "Field `TC` writer - Поле десятков веков. Допустимые значения от 0 до 9"]
pub type TcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Разрешает сравнения дней, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cd {
    #[doc = "0: Сравнение дней отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение дней включено"]
    Enable = 1,
}
impl From<Cd> for bool {
    #[inline(always)]
    fn from(variant: Cd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CD` reader - Разрешает сравнения дней, когда установлен"]
pub type CdR = crate::BitReader<Cd>;
impl CdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cd {
        match self.bits {
            false => Cd::Disabled,
            true => Cd::Enable,
        }
    }
    #[doc = "Сравнение дней отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cd::Disabled
    }
    #[doc = "Сравнение дней включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cd::Enable
    }
}
#[doc = "Field `CD` writer - Разрешает сравнения дней, когда установлен"]
pub type CdW<'a, REG> = crate::BitWriter<'a, REG, Cd>;
impl<'a, REG> CdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение дней отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::Disabled)
    }
    #[doc = "Сравнение дней включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cd::Enable)
    }
}
#[doc = "Разрешает сравнения месяцев, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    #[doc = "0: Сравнение месяцев отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение месяцев включено"]
    Enable = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Разрешает сравнения месяцев, когда установлен"]
pub type CmR = crate::BitReader<Cm>;
impl CmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cm {
        match self.bits {
            false => Cm::Disabled,
            true => Cm::Enable,
        }
    }
    #[doc = "Сравнение месяцев отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cm::Disabled
    }
    #[doc = "Сравнение месяцев включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cm::Enable
    }
}
#[doc = "Field `CM` writer - Разрешает сравнения месяцев, когда установлен"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение месяцев отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Disabled)
    }
    #[doc = "Сравнение месяцев включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Enable)
    }
}
#[doc = "Разрешает сравнения годов, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cy {
    #[doc = "0: Сравнение годов отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение годов включено"]
    Enable = 1,
}
impl From<Cy> for bool {
    #[inline(always)]
    fn from(variant: Cy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CY` reader - Разрешает сравнения годов, когда установлен"]
pub type CyR = crate::BitReader<Cy>;
impl CyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cy {
        match self.bits {
            false => Cy::Disabled,
            true => Cy::Enable,
        }
    }
    #[doc = "Сравнение годов отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cy::Disabled
    }
    #[doc = "Сравнение годов включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cy::Enable
    }
}
#[doc = "Field `CY` writer - Разрешает сравнения годов, когда установлен"]
pub type CyW<'a, REG> = crate::BitWriter<'a, REG, Cy>;
impl<'a, REG> CyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение годов отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cy::Disabled)
    }
    #[doc = "Сравнение годов включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cy::Enable)
    }
}
#[doc = "Рразрешает сравнения веков, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cc {
    #[doc = "0: Сравнение веков отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение веков включено"]
    Enable = 1,
}
impl From<Cc> for bool {
    #[inline(always)]
    fn from(variant: Cc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CC` reader - Рразрешает сравнения веков, когда установлен"]
pub type CcR = crate::BitReader<Cc>;
impl CcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cc {
        match self.bits {
            false => Cc::Disabled,
            true => Cc::Enable,
        }
    }
    #[doc = "Сравнение веков отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cc::Disabled
    }
    #[doc = "Сравнение веков включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cc::Enable
    }
}
#[doc = "Field `CC` writer - Рразрешает сравнения веков, когда установлен"]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG, Cc>;
impl<'a, REG> CcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение веков отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Disabled)
    }
    #[doc = "Сравнение веков включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cc::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
    #[inline(always)]
    pub fn d(&self) -> DR {
        DR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
    #[inline(always)]
    pub fn td(&self) -> TdR {
        TdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Поле десятков месяцев. Допустимые значения от 0 до 1"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:14 - Поле единиц годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn y(&self) -> YR {
        YR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18 - Поле десятков годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn ty(&self) -> TyR {
        TyR::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - Поле единиц веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn c(&self) -> CR {
        CR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:26 - Поле десятков веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Разрешает сравнения дней, когда установлен"]
    #[inline(always)]
    pub fn cd(&self) -> CdR {
        CdR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Разрешает сравнения месяцев, когда установлен"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Разрешает сравнения годов, когда установлен"]
    #[inline(always)]
    pub fn cy(&self) -> CyR {
        CyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Рразрешает сравнения веков, когда установлен"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Поле единиц дней. Допустимые значения: - TD = 2 - от 0 до 3; - TD = 3 - от 0 до 1"]
    #[inline(always)]
    pub fn d(&mut self) -> DW<'_, RrtcDalrmSpec> {
        DW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Поле десятков дней. Допустимые значения: - {TM,M} != 03 - от 0 до 3; - {TM,M} == 03 - от 0 до 2"]
    #[inline(always)]
    pub fn td(&mut self) -> TdW<'_, RrtcDalrmSpec> {
        TdW::new(self, 4)
    }
    #[doc = "Bits 6:9 - Поле единиц месяцев. Допустимые значения: - TM = 0 - от 0 до 9; - TM = 1 - от 0 до 2"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, RrtcDalrmSpec> {
        MW::new(self, 6)
    }
    #[doc = "Bit 10 - Поле десятков месяцев. Допустимые значения от 0 до 1"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<'_, RrtcDalrmSpec> {
        TmW::new(self, 10)
    }
    #[doc = "Bits 11:14 - Поле единиц годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn y(&mut self) -> YW<'_, RrtcDalrmSpec> {
        YW::new(self, 11)
    }
    #[doc = "Bits 15:18 - Поле десятков годов. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn ty(&mut self) -> TyW<'_, RrtcDalrmSpec> {
        TyW::new(self, 15)
    }
    #[doc = "Bits 19:22 - Поле единиц веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn c(&mut self) -> CW<'_, RrtcDalrmSpec> {
        CW::new(self, 19)
    }
    #[doc = "Bits 23:26 - Поле десятков веков. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, RrtcDalrmSpec> {
        TcW::new(self, 23)
    }
    #[doc = "Bit 27 - Разрешает сравнения дней, когда установлен"]
    #[inline(always)]
    pub fn cd(&mut self) -> CdW<'_, RrtcDalrmSpec> {
        CdW::new(self, 27)
    }
    #[doc = "Bit 28 - Разрешает сравнения месяцев, когда установлен"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, RrtcDalrmSpec> {
        CmW::new(self, 28)
    }
    #[doc = "Bit 29 - Разрешает сравнения годов, когда установлен"]
    #[inline(always)]
    pub fn cy(&mut self) -> CyW<'_, RrtcDalrmSpec> {
        CyW::new(self, 29)
    }
    #[doc = "Bit 30 - Рразрешает сравнения веков, когда установлен"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, RrtcDalrmSpec> {
        CcW::new(self, 30)
    }
}
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание.\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_dalrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_dalrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcDalrmSpec;
impl crate::RegisterSpec for RrtcDalrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_dalrm::R`](R) reader structure"]
impl crate::Readable for RrtcDalrmSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_dalrm::W`](W) writer structure"]
impl crate::Writable for RrtcDalrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRTC_DALRM to value 0"]
impl crate::Resettable for RrtcDalrmSpec {}
