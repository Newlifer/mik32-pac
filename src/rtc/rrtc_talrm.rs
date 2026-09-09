#[doc = "Register `RRTC_TALRM` reader"]
pub type R = crate::R<RrtcTalrmSpec>;
#[doc = "Register `RRTC_TALRM` writer"]
pub type W = crate::W<RrtcTalrmSpec>;
#[doc = "Field `TOS` reader - Поле десятых долей секунды. Допустимые значения от 0 до 9"]
pub type TosR = crate::FieldReader;
#[doc = "Field `TOS` writer - Поле десятых долей секунды. Допустимые значения от 0 до 9"]
pub type TosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `S` reader - Поле единиц секунд. Допустимые значения от 0 до 9"]
pub type SR = crate::FieldReader;
#[doc = "Field `S` writer - Поле единиц секунд. Допустимые значения от 0 до 9"]
pub type SW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS` reader - Поле десятков секунд. Допустимые значения от 0 до 5"]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - Поле десятков секунд. Допустимые значения от 0 до 5"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `M` reader - Поле единиц минут. Допустимые значения от 0 до 9"]
pub type MR = crate::FieldReader;
#[doc = "Field `M` writer - Поле единиц минут. Допустимые значения от 0 до 9"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TM` reader - Поле десятков минут. Допустимые значения от 0 до 5"]
pub type TmR = crate::FieldReader;
#[doc = "Field `TM` writer - Поле десятков минут. Допустимые значения от 0 до 5"]
pub type TmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `H` reader - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
pub type HR = crate::FieldReader;
#[doc = "Field `H` writer - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
pub type HW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TH` reader - Поле десятков часов. Доступные значения от 0 до 2"]
pub type ThR = crate::FieldReader;
#[doc = "Field `TH` writer - Поле десятков часов. Доступные значения от 0 до 2"]
pub type ThW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "День недели\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DayOfWeek {
    #[doc = "1: Понедельник"]
    Monday = 1,
    #[doc = "2: Вторник"]
    Tuesday = 2,
    #[doc = "3: Среда"]
    Wednesday = 3,
    #[doc = "4: Четверг"]
    Thursday = 4,
    #[doc = "5: Пятница"]
    Friday = 5,
    #[doc = "6: Суббота"]
    Saturday = 6,
    #[doc = "7: Воскресенье"]
    Sunday = 7,
}
impl From<DayOfWeek> for u8 {
    #[inline(always)]
    fn from(variant: DayOfWeek) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DayOfWeek {
    type Ux = u8;
}
impl crate::IsEnum for DayOfWeek {}
#[doc = "Field `DOW` reader - День недели"]
pub type DowR = crate::FieldReader<DayOfWeek>;
impl DowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DayOfWeek> {
        match self.bits {
            1 => Some(DayOfWeek::Monday),
            2 => Some(DayOfWeek::Tuesday),
            3 => Some(DayOfWeek::Wednesday),
            4 => Some(DayOfWeek::Thursday),
            5 => Some(DayOfWeek::Friday),
            6 => Some(DayOfWeek::Saturday),
            7 => Some(DayOfWeek::Sunday),
            _ => None,
        }
    }
    #[doc = "Понедельник"]
    #[inline(always)]
    pub fn is_monday(&self) -> bool {
        *self == DayOfWeek::Monday
    }
    #[doc = "Вторник"]
    #[inline(always)]
    pub fn is_tuesday(&self) -> bool {
        *self == DayOfWeek::Tuesday
    }
    #[doc = "Среда"]
    #[inline(always)]
    pub fn is_wednesday(&self) -> bool {
        *self == DayOfWeek::Wednesday
    }
    #[doc = "Четверг"]
    #[inline(always)]
    pub fn is_thursday(&self) -> bool {
        *self == DayOfWeek::Thursday
    }
    #[doc = "Пятница"]
    #[inline(always)]
    pub fn is_friday(&self) -> bool {
        *self == DayOfWeek::Friday
    }
    #[doc = "Суббота"]
    #[inline(always)]
    pub fn is_saturday(&self) -> bool {
        *self == DayOfWeek::Saturday
    }
    #[doc = "Воскресенье"]
    #[inline(always)]
    pub fn is_sunday(&self) -> bool {
        *self == DayOfWeek::Sunday
    }
}
#[doc = "Field `DOW` writer - День недели"]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3, DayOfWeek>;
impl<'a, REG> DowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Понедельник"]
    #[inline(always)]
    pub fn monday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Monday)
    }
    #[doc = "Вторник"]
    #[inline(always)]
    pub fn tuesday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Tuesday)
    }
    #[doc = "Среда"]
    #[inline(always)]
    pub fn wednesday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Wednesday)
    }
    #[doc = "Четверг"]
    #[inline(always)]
    pub fn thursday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Thursday)
    }
    #[doc = "Пятница"]
    #[inline(always)]
    pub fn friday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Friday)
    }
    #[doc = "Суббота"]
    #[inline(always)]
    pub fn saturday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Saturday)
    }
    #[doc = "Воскресенье"]
    #[inline(always)]
    pub fn sunday(self) -> &'a mut crate::W<REG> {
        self.variant(DayOfWeek::Sunday)
    }
}
#[doc = "Разрешает сравнение десятых долей секунды, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctos {
    #[doc = "0: Сравнение десятых долей секунды отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение десятых долей секунды включено"]
    Enable = 1,
}
impl From<Ctos> for bool {
    #[inline(always)]
    fn from(variant: Ctos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTOS` reader - Разрешает сравнение десятых долей секунды, когда установлен"]
pub type CtosR = crate::BitReader<Ctos>;
impl CtosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctos {
        match self.bits {
            false => Ctos::Disabled,
            true => Ctos::Enable,
        }
    }
    #[doc = "Сравнение десятых долей секунды отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ctos::Disabled
    }
    #[doc = "Сравнение десятых долей секунды включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctos::Enable
    }
}
#[doc = "Field `CTOS` writer - Разрешает сравнение десятых долей секунды, когда установлен"]
pub type CtosW<'a, REG> = crate::BitWriter<'a, REG, Ctos>;
impl<'a, REG> CtosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение десятых долей секунды отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ctos::Disabled)
    }
    #[doc = "Сравнение десятых долей секунды включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctos::Enable)
    }
}
#[doc = "Разрешает сравнения секунд, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs {
    #[doc = "0: Сравнение секунд отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение секунд включено"]
    Enable = 1,
}
impl From<Cs> for bool {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Разрешает сравнения секунд, когда установлен"]
pub type CsR = crate::BitReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            false => Cs::Disabled,
            true => Cs::Enable,
        }
    }
    #[doc = "Сравнение секунд отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cs::Disabled
    }
    #[doc = "Сравнение секунд включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cs::Enable
    }
}
#[doc = "Field `CS` writer - Разрешает сравнения секунд, когда установлен"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение секунд отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Disabled)
    }
    #[doc = "Сравнение секунд включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Enable)
    }
}
#[doc = "Разрешает сравнения минут, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cm {
    #[doc = "0: Сравнение минут отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение минут включено"]
    Enable = 1,
}
impl From<Cm> for bool {
    #[inline(always)]
    fn from(variant: Cm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CM` reader - Разрешает сравнения минут, когда установлен"]
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
    #[doc = "Сравнение минут отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cm::Disabled
    }
    #[doc = "Сравнение минут включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cm::Enable
    }
}
#[doc = "Field `CM` writer - Разрешает сравнения минут, когда установлен"]
pub type CmW<'a, REG> = crate::BitWriter<'a, REG, Cm>;
impl<'a, REG> CmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение минут отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Disabled)
    }
    #[doc = "Сравнение минут включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cm::Enable)
    }
}
#[doc = "Разрешает сравнения часов, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch {
    #[doc = "0: Сравнение часов отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение часов включено"]
    Enable = 1,
}
impl From<Ch> for bool {
    #[inline(always)]
    fn from(variant: Ch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH` reader - Разрешает сравнения часов, когда установлен"]
pub type ChR = crate::BitReader<Ch>;
impl ChR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch {
        match self.bits {
            false => Ch::Disabled,
            true => Ch::Enable,
        }
    }
    #[doc = "Сравнение часов отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ch::Disabled
    }
    #[doc = "Сравнение часов включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ch::Enable
    }
}
#[doc = "Field `CH` writer - Разрешает сравнения часов, когда установлен"]
pub type ChW<'a, REG> = crate::BitWriter<'a, REG, Ch>;
impl<'a, REG> ChW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение часов отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Disabled)
    }
    #[doc = "Сравнение часов включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ch::Enable)
    }
}
#[doc = "Разрешает сравнения дней недели, когда установлен\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdow {
    #[doc = "0: Сравнение дней недели отключено"]
    Disabled = 0,
    #[doc = "1: Сравнение дней недели включено"]
    Enable = 1,
}
impl From<Cdow> for bool {
    #[inline(always)]
    fn from(variant: Cdow) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDOW` reader - Разрешает сравнения дней недели, когда установлен"]
pub type CdowR = crate::BitReader<Cdow>;
impl CdowR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cdow {
        match self.bits {
            false => Cdow::Disabled,
            true => Cdow::Enable,
        }
    }
    #[doc = "Сравнение дней недели отключено"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Cdow::Disabled
    }
    #[doc = "Сравнение дней недели включено"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cdow::Enable
    }
}
#[doc = "Field `CDOW` writer - Разрешает сравнения дней недели, когда установлен"]
pub type CdowW<'a, REG> = crate::BitWriter<'a, REG, Cdow>;
impl<'a, REG> CdowW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Сравнение дней недели отключено"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Cdow::Disabled)
    }
    #[doc = "Сравнение дней недели включено"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cdow::Enable)
    }
}
impl R {
    #[doc = "Bits 0:3 - Поле десятых долей секунды. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tos(&self) -> TosR {
        TosR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Поле единиц секунд. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Поле десятков секунд. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:14 - Поле единиц минут. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 15:17 - Поле десятков минут. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:21 - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23 - Поле десятков часов. Доступные значения от 0 до 2"]
    #[inline(always)]
    pub fn th(&self) -> ThR {
        ThR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:26 - День недели"]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Разрешает сравнение десятых долей секунды, когда установлен"]
    #[inline(always)]
    pub fn ctos(&self) -> CtosR {
        CtosR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Разрешает сравнения секунд, когда установлен"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Разрешает сравнения минут, когда установлен"]
    #[inline(always)]
    pub fn cm(&self) -> CmR {
        CmR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Разрешает сравнения часов, когда установлен"]
    #[inline(always)]
    pub fn ch(&self) -> ChR {
        ChR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Разрешает сравнения дней недели, когда установлен"]
    #[inline(always)]
    pub fn cdow(&self) -> CdowR {
        CdowR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Поле десятых долей секунды. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn tos(&mut self) -> TosW<'_, RrtcTalrmSpec> {
        TosW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Поле единиц секунд. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn s(&mut self) -> SW<'_, RrtcTalrmSpec> {
        SW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Поле десятков секунд. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, RrtcTalrmSpec> {
        TsW::new(self, 8)
    }
    #[doc = "Bits 11:14 - Поле единиц минут. Допустимые значения от 0 до 9"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, RrtcTalrmSpec> {
        MW::new(self, 11)
    }
    #[doc = "Bits 15:17 - Поле десятков минут. Допустимые значения от 0 до 5"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<'_, RrtcTalrmSpec> {
        TmW::new(self, 15)
    }
    #[doc = "Bits 18:21 - Поле единиц часов. Допустимые значения: - TH = 0 - От 0 до 9; - TH = 2 - От 0 до 3"]
    #[inline(always)]
    pub fn h(&mut self) -> HW<'_, RrtcTalrmSpec> {
        HW::new(self, 18)
    }
    #[doc = "Bits 22:23 - Поле десятков часов. Доступные значения от 0 до 2"]
    #[inline(always)]
    pub fn th(&mut self) -> ThW<'_, RrtcTalrmSpec> {
        ThW::new(self, 22)
    }
    #[doc = "Bits 24:26 - День недели"]
    #[inline(always)]
    pub fn dow(&mut self) -> DowW<'_, RrtcTalrmSpec> {
        DowW::new(self, 24)
    }
    #[doc = "Bit 27 - Разрешает сравнение десятых долей секунды, когда установлен"]
    #[inline(always)]
    pub fn ctos(&mut self) -> CtosW<'_, RrtcTalrmSpec> {
        CtosW::new(self, 27)
    }
    #[doc = "Bit 28 - Разрешает сравнения секунд, когда установлен"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<'_, RrtcTalrmSpec> {
        CsW::new(self, 28)
    }
    #[doc = "Bit 29 - Разрешает сравнения минут, когда установлен"]
    #[inline(always)]
    pub fn cm(&mut self) -> CmW<'_, RrtcTalrmSpec> {
        CmW::new(self, 29)
    }
    #[doc = "Bit 30 - Разрешает сравнения часов, когда установлен"]
    #[inline(always)]
    pub fn ch(&mut self) -> ChW<'_, RrtcTalrmSpec> {
        ChW::new(self, 30)
    }
    #[doc = "Bit 31 - Разрешает сравнения дней недели, когда установлен"]
    #[inline(always)]
    pub fn cdow(&mut self) -> CdowW<'_, RrtcTalrmSpec> {
        CdowW::new(self, 31)
    }
}
#[doc = "регистр хранит время, при совпадении которого со значением регистра RRTC_TIME, будет сгенерировано соответствующее прерывание\n\nYou can [`read`](crate::Reg::read) this register and get [`rrtc_talrm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rrtc_talrm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RrtcTalrmSpec;
impl crate::RegisterSpec for RrtcTalrmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rrtc_talrm::R`](R) reader structure"]
impl crate::Readable for RrtcTalrmSpec {}
#[doc = "`write(|w| ..)` method takes [`rrtc_talrm::W`](W) writer structure"]
impl crate::Writable for RrtcTalrmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RRTC_TALRM to value 0"]
impl crate::Resettable for RrtcTalrmSpec {}
