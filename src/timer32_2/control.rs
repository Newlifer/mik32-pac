#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Режим счёта таймера\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CountMode {
    #[doc = "0: Прямой режим"]
    Direct = 0,
    #[doc = "1: Обратный режим"]
    Reverse = 1,
    #[doc = "2: Двунаправленный режим"]
    Bidirectional = 2,
}
impl From<CountMode> for u8 {
    #[inline(always)]
    fn from(variant: CountMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CountMode {
    type Ux = u8;
}
impl crate::IsEnum for CountMode {}
#[doc = "Field `COUNT_MODE` reader - Режим счёта таймера"]
pub type CountModeR = crate::FieldReader<CountMode>;
impl CountModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CountMode> {
        match self.bits {
            0 => Some(CountMode::Direct),
            1 => Some(CountMode::Reverse),
            2 => Some(CountMode::Bidirectional),
            _ => None,
        }
    }
    #[doc = "Прямой режим"]
    #[inline(always)]
    pub fn is_direct(&self) -> bool {
        *self == CountMode::Direct
    }
    #[doc = "Обратный режим"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == CountMode::Reverse
    }
    #[doc = "Двунаправленный режим"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == CountMode::Bidirectional
    }
}
#[doc = "Field `COUNT_MODE` writer - Режим счёта таймера"]
pub type CountModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, CountMode>;
impl<'a, REG> CountModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Прямой режим"]
    #[inline(always)]
    pub fn direct(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Direct)
    }
    #[doc = "Обратный режим"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Reverse)
    }
    #[doc = "Двунаправленный режим"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(CountMode::Bidirectional)
    }
}
#[doc = "Выбор источника тактового сигнала для счёта\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Source {
    #[doc = "0: Выход предделителя"]
    Prescaler = 0,
    #[doc = "1: Вход модуля TIM1"]
    Tim1 = 1,
    #[doc = "2: Внешний вход TX"]
    TxPin = 2,
    #[doc = "3: Вход модуля TIM2"]
    Tim2 = 3,
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(variant: Source) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Source {
    type Ux = u8;
}
impl crate::IsEnum for Source {}
#[doc = "Field `SOURCE` reader - Выбор источника тактового сигнала для счёта"]
pub type SourceR = crate::FieldReader<Source>;
impl SourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Source {
        match self.bits {
            0 => Source::Prescaler,
            1 => Source::Tim1,
            2 => Source::TxPin,
            3 => Source::Tim2,
            _ => unreachable!(),
        }
    }
    #[doc = "Выход предделителя"]
    #[inline(always)]
    pub fn is_prescaler(&self) -> bool {
        *self == Source::Prescaler
    }
    #[doc = "Вход модуля TIM1"]
    #[inline(always)]
    pub fn is_tim1(&self) -> bool {
        *self == Source::Tim1
    }
    #[doc = "Внешний вход TX"]
    #[inline(always)]
    pub fn is_tx_pin(&self) -> bool {
        *self == Source::TxPin
    }
    #[doc = "Вход модуля TIM2"]
    #[inline(always)]
    pub fn is_tim2(&self) -> bool {
        *self == Source::Tim2
    }
}
#[doc = "Field `SOURCE` writer - Выбор источника тактового сигнала для счёта"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 2, Source, crate::Safe>;
impl<'a, REG> SourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Выход предделителя"]
    #[inline(always)]
    pub fn prescaler(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Prescaler)
    }
    #[doc = "Вход модуля TIM1"]
    #[inline(always)]
    pub fn tim1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Tim1)
    }
    #[doc = "Внешний вход TX"]
    #[inline(always)]
    pub fn tx_pin(self) -> &'a mut crate::W<REG> {
        self.variant(Source::TxPin)
    }
    #[doc = "Вход модуля TIM2"]
    #[inline(always)]
    pub fn tim2(self) -> &'a mut crate::W<REG> {
        self.variant(Source::Tim2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Режим счёта таймера"]
    #[inline(always)]
    pub fn count_mode(&self) -> CountModeR {
        CountModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Выбор источника тактового сигнала для счёта"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Режим счёта таймера"]
    #[inline(always)]
    pub fn count_mode(&mut self) -> CountModeW<'_, ControlSpec> {
        CountModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Выбор источника тактового сигнала для счёта"]
    #[inline(always)]
    pub fn source(&mut self) -> SourceW<'_, ControlSpec> {
        SourceW::new(self, 2)
    }
}
#[doc = "Конфигурационный регистр основного таймера\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
