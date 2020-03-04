# ! [ doc = "General-purpose I/Os" ]
# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Gpioa {
    # [ doc = "0x00 - GPIO port mode register" ]
    pub moder: Moder,
    # [ doc = "0x04 - GPIO port output type register" ]
    pub otyper: Otyper,
    # [ doc = "0x08 - GPIO port output speed register" ]
    pub ospeedr: Ospeedr,
    # [ doc = "0x0c - GPIO port pull-up/pull-down register" ]
    pub pupdr: Pupdr,
    # [ doc = "0x10 - GPIO port input data register" ]
    pub idr: Idr,
    # [ doc = "0x14 - GPIO port output data register" ]
    pub odr: Odr,
    # [ doc = "0x18 - GPIO port bit set/reset register" ]
    pub bsrr: Bsrr,
    # [ doc = "0x1c - GPIO port configuration lock register" ]
    pub lckr: Lckr,
    # [ doc = "0x20 - GPIO alternate function low register" ]
    pub afrl: Afrl,
    # [ doc = "0x24 - GPIO alternate function high register" ]
    pub afrh: Afrh,
}

# [ doc = "GPIO port mode register" ]
# [ repr ( C ) ]
pub struct Moder {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port mode register" ]
pub mod moder {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Moder {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `moder15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum ModerR {
        # [ doc = "00: Input (reset state)" ]
        InputMode,
        # [ doc = "01: General purpose output mode" ]
        OutputMode,
        # [ doc = "10: Alternate function mode" ]
        AlternateMode,
        # [ doc = "11: Analog mode" ]
        AnalogMode,
    }
    impl ModerR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                ModerR::InputMode => 0,
                ModerR::OutputMode => 1,
                ModerR::AlternateMode => 2,
                ModerR::AnalogMode => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> ModerR {
            match bits {
                0 => ModerR::InputMode,
                1 => ModerR::OutputMode,
                2 => ModerR::AlternateMode,
                3 => ModerR::AnalogMode,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `InputMode`" ]
        pub fn is_input_mode(&self) -> bool {
            *self == ModerR::InputMode
        }
        # [ doc = "Check if the value of the field is `OutputMode`" ]
        pub fn is_output_mode(&self) -> bool {
            *self == ModerR::OutputMode
        }
        # [ doc = "Check if the value of the field is `AlternateMode`" ]
        pub fn is_alternate_mode(&self) -> bool {
            *self == ModerR::AlternateMode
        }
        # [ doc = "Check if the value of the field is `AnalogMode`" ]
        pub fn is_analog_mode(&self) -> bool {
            *self == ModerR::AnalogMode
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `moder15`" ]
    pub enum ModerW {
        # [ doc = "00: Input (reset state)" ]
        InputMode,
        # [ doc = "01: General purpose output mode" ]
        OutputMode,
        # [ doc = "10: Alternate function mode" ]
        AlternateMode,
        # [ doc = "11: Analog mode" ]
        AnalogMode,
    }
    impl ModerW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                ModerW::InputMode => 0,
                ModerW::OutputMode => 1,
                ModerW::AlternateMode => 2,
                ModerW::AnalogMode => 3,
            }
        }
    }
    impl<'a> _Moder15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: ModerW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Input (reset state)" ]
        pub fn input_mode(self) -> &'a mut W {
            self.variant(ModerW::InputMode)
        }
        # [ doc = "01: General purpose output mode" ]
        pub fn output_mode(self) -> &'a mut W {
            self.variant(ModerW::OutputMode)
        }
        # [ doc = "10: Alternate function mode" ]
        pub fn alternate_mode(self) -> &'a mut W {
            self.variant(ModerW::AlternateMode)
        }
        # [ doc = "11: Analog mode" ]
        pub fn analog_mode(self) -> &'a mut W {
            self.variant(ModerW::AnalogMode)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _moder15(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn moder15(&self) -> ModerR {
            ModerR::_from(self._moder15())
        }
        fn _moder14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder14(&self) -> ModerR {
            ModerR::_from(self._moder14())
        }
        fn _moder13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder13(&self) -> ModerR {
            ModerR::_from(self._moder13())
        }
        fn _moder12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder12(&self) -> ModerR {
            ModerR::_from(self._moder12())
        }
        fn _moder11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder11(&self) -> ModerR {
            ModerR::_from(self._moder11())
        }
        fn _moder10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder10(&self) -> ModerR {
            ModerR::_from(self._moder10())
        }
        fn _moder9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder9(&self) -> ModerR {
            ModerR::_from(self._moder9())
        }
        fn _moder8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder8(&self) -> ModerR {
            ModerR::_from(self._moder8())
        }
        fn _moder7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder7(&self) -> ModerR {
            ModerR::_from(self._moder7())
        }
        fn _moder6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder6(&self) -> ModerR {
            ModerR::_from(self._moder6())
        }
        fn _moder5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder5(&self) -> ModerR {
            ModerR::_from(self._moder5())
        }
        fn _moder4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder4(&self) -> ModerR {
            ModerR::_from(self._moder4())
        }
        fn _moder3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder3(&self) -> ModerR {
            ModerR::_from(self._moder3())
        }
        fn _moder2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder2(&self) -> ModerR {
            ModerR::_from(self._moder2())
        }
        fn _moder1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder1(&self) -> ModerR {
            ModerR::_from(self._moder1())
        }
        fn _moder0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder0(&self) -> ModerR {
            ModerR::_from(self._moder0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 2818572288 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn moder15(&mut self) -> _Moder15W {
            _Moder15W { register: self }
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder14(&mut self) -> _Moder14W {
            _Moder14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder13(&mut self) -> _Moder13W {
            _Moder13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder12(&mut self) -> _Moder12W {
            _Moder12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder11(&mut self) -> _Moder11W {
            _Moder11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder10(&mut self) -> _Moder10W {
            _Moder10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder9(&mut self) -> _Moder9W {
            _Moder9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder8(&mut self) -> _Moder8W {
            _Moder8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder7(&mut self) -> _Moder7W {
            _Moder7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder6(&mut self) -> _Moder6W {
            _Moder6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder5(&mut self) -> _Moder5W {
            _Moder5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder4(&mut self) -> _Moder4W {
            _Moder4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder3(&mut self) -> _Moder3W {
            _Moder3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder2(&mut self) -> _Moder2W {
            _Moder2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder1(&mut self) -> _Moder1W {
            _Moder1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn moder0(&mut self) -> _Moder0W {
            _Moder0W { register: self }
        }
    }
}

# [ doc = "GPIO port output type register" ]
# [ repr ( C ) ]
pub struct Otyper {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port output type register" ]
pub mod otyper {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Otyper {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `ot15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OtyperR {
        # [ doc = "0: Output push-pull (reset state)" ]
        PushPull,
        # [ doc = "1: Output open-drain" ]
        OpenDrain,
    }
    impl OtyperR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                OtyperR::PushPull => 0,
                OtyperR::OpenDrain => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> OtyperR {
            match bits {
                0 => OtyperR::PushPull,
                1 => OtyperR::OpenDrain,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `PushPull`" ]
        pub fn is_push_pull(&self) -> bool {
            *self == OtyperR::PushPull
        }
        # [ doc = "Check if the value of the field is `OpenDrain`" ]
        pub fn is_open_drain(&self) -> bool {
            *self == OtyperR::OpenDrain
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `ot15`" ]
    pub enum OtyperW {
        # [ doc = "0: Output push-pull (reset state)" ]
        PushPull,
        # [ doc = "1: Output open-drain" ]
        OpenDrain,
    }
    impl OtyperW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                OtyperW::PushPull => 0,
                OtyperW::OpenDrain => 1,
            }
        }
    }
    impl<'a> _Ot15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OtyperW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Output push-pull (reset state)" ]
        pub fn push_pull(self) -> &'a mut W {
            self.variant(OtyperW::PushPull)
        }
        # [ doc = "1: Output open-drain" ]
        pub fn open_drain(self) -> &'a mut W {
            self.variant(OtyperW::OpenDrain)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _ot15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot15(&self) -> OtyperR {
            OtyperR::_from(self._ot15())
        }
        fn _ot14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot14(&self) -> OtyperR {
            OtyperR::_from(self._ot14())
        }
        fn _ot13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot13(&self) -> OtyperR {
            OtyperR::_from(self._ot13())
        }
        fn _ot12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot12(&self) -> OtyperR {
            OtyperR::_from(self._ot12())
        }
        fn _ot11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot11(&self) -> OtyperR {
            OtyperR::_from(self._ot11())
        }
        fn _ot10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot10(&self) -> OtyperR {
            OtyperR::_from(self._ot10())
        }
        fn _ot9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot9(&self) -> OtyperR {
            OtyperR::_from(self._ot9())
        }
        fn _ot8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot8(&self) -> OtyperR {
            OtyperR::_from(self._ot8())
        }
        fn _ot7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot7(&self) -> OtyperR {
            OtyperR::_from(self._ot7())
        }
        fn _ot6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot6(&self) -> OtyperR {
            OtyperR::_from(self._ot6())
        }
        fn _ot5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot5(&self) -> OtyperR {
            OtyperR::_from(self._ot5())
        }
        fn _ot4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot4(&self) -> OtyperR {
            OtyperR::_from(self._ot4())
        }
        fn _ot3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot3(&self) -> OtyperR {
            OtyperR::_from(self._ot3())
        }
        fn _ot2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot2(&self) -> OtyperR {
            OtyperR::_from(self._ot2())
        }
        fn _ot1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot1(&self) -> OtyperR {
            OtyperR::_from(self._ot1())
        }
        fn _ot0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot0(&self) -> OtyperR {
            OtyperR::_from(self._ot0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot15(&mut self) -> _Ot15W {
            _Ot15W { register: self }
        }
        # [ doc = "Bit 14 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot14(&mut self) -> _Ot14W {
            _Ot14W { register: self }
        }
        # [ doc = "Bit 13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot13(&mut self) -> _Ot13W {
            _Ot13W { register: self }
        }
        # [ doc = "Bit 12 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot12(&mut self) -> _Ot12W {
            _Ot12W { register: self }
        }
        # [ doc = "Bit 11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot11(&mut self) -> _Ot11W {
            _Ot11W { register: self }
        }
        # [ doc = "Bit 10 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot10(&mut self) -> _Ot10W {
            _Ot10W { register: self }
        }
        # [ doc = "Bit 9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot9(&mut self) -> _Ot9W {
            _Ot9W { register: self }
        }
        # [ doc = "Bit 8 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot8(&mut self) -> _Ot8W {
            _Ot8W { register: self }
        }
        # [ doc = "Bit 7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot7(&mut self) -> _Ot7W {
            _Ot7W { register: self }
        }
        # [ doc = "Bit 6 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot6(&mut self) -> _Ot6W {
            _Ot6W { register: self }
        }
        # [ doc = "Bit 5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot5(&mut self) -> _Ot5W {
            _Ot5W { register: self }
        }
        # [ doc = "Bit 4 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot4(&mut self) -> _Ot4W {
            _Ot4W { register: self }
        }
        # [ doc = "Bit 3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot3(&mut self) -> _Ot3W {
            _Ot3W { register: self }
        }
        # [ doc = "Bit 2 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot2(&mut self) -> _Ot2W {
            _Ot2W { register: self }
        }
        # [ doc = "Bit 1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot1(&mut self) -> _Ot1W {
            _Ot1W { register: self }
        }
        # [ doc = "Bit 0 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ot0(&mut self) -> _Ot0W {
            _Ot0W { register: self }
        }
    }
}

# [ doc = "GPIO port output speed register" ]
# [ repr ( C ) ]
pub struct Ospeedr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port output speed register" ]
pub mod ospeedr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ospeedr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `ospeedr15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum OspeedrR {
        # [ doc = "00: Low speed" ]
        LowSpeed,
        # [ doc = "01: Medium speed" ]
        MediumSpeed,
        # [ doc = "10: Fast speed" ]
        FastSpeed,
        # [ doc = "11: High speed" ]
        HighSpeed,
    }
    impl OspeedrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                OspeedrR::LowSpeed => 0,
                OspeedrR::MediumSpeed => 1,
                OspeedrR::FastSpeed => 2,
                OspeedrR::HighSpeed => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> OspeedrR {
            match bits {
                0 => OspeedrR::LowSpeed,
                1 => OspeedrR::MediumSpeed,
                2 => OspeedrR::FastSpeed,
                3 => OspeedrR::HighSpeed,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `LowSpeed`" ]
        pub fn is_low_speed(&self) -> bool {
            *self == OspeedrR::LowSpeed
        }
        # [ doc = "Check if the value of the field is `MediumSpeed`" ]
        pub fn is_medium_speed(&self) -> bool {
            *self == OspeedrR::MediumSpeed
        }
        # [ doc = "Check if the value of the field is `FastSpeed`" ]
        pub fn is_fast_speed(&self) -> bool {
            *self == OspeedrR::FastSpeed
        }
        # [ doc = "Check if the value of the field is `HighSpeed`" ]
        pub fn is_high_speed(&self) -> bool {
            *self == OspeedrR::HighSpeed
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `ospeedr15`" ]
    pub enum OspeedrW {
        # [ doc = "00: Low speed" ]
        LowSpeed,
        # [ doc = "01: Medium speed" ]
        MediumSpeed,
        # [ doc = "10: Fast speed" ]
        FastSpeed,
        # [ doc = "11: High speed" ]
        HighSpeed,
    }
    impl OspeedrW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                OspeedrW::LowSpeed => 0,
                OspeedrW::MediumSpeed => 1,
                OspeedrW::FastSpeed => 2,
                OspeedrW::HighSpeed => 3,
            }
        }
    }
    impl<'a> _Ospeedr15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: OspeedrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: Low speed" ]
        pub fn low_speed(self) -> &'a mut W {
            self.variant(OspeedrW::LowSpeed)
        }
        # [ doc = "01: Medium speed" ]
        pub fn medium_speed(self) -> &'a mut W {
            self.variant(OspeedrW::MediumSpeed)
        }
        # [ doc = "10: Fast speed" ]
        pub fn fast_speed(self) -> &'a mut W {
            self.variant(OspeedrW::FastSpeed)
        }
        # [ doc = "11: High speed" ]
        pub fn high_speed(self) -> &'a mut W {
            self.variant(OspeedrW::HighSpeed)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _ospeedr15(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr15(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr15())
        }
        fn _ospeedr14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr14(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr14())
        }
        fn _ospeedr13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr13(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr13())
        }
        fn _ospeedr12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr12(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr12())
        }
        fn _ospeedr11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr11(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr11())
        }
        fn _ospeedr10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr10(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr10())
        }
        fn _ospeedr9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr9(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr9())
        }
        fn _ospeedr8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr8(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr8())
        }
        fn _ospeedr7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr7(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr7())
        }
        fn _ospeedr6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr6(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr6())
        }
        fn _ospeedr5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr5(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr5())
        }
        fn _ospeedr4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr4(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr4())
        }
        fn _ospeedr3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr3(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr3())
        }
        fn _ospeedr2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr2(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr2())
        }
        fn _ospeedr1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr1(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr1())
        }
        fn _ospeedr0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr0(&self) -> OspeedrR {
            OspeedrR::_from(self._ospeedr0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr15(&mut self) -> _Ospeedr15W {
            _Ospeedr15W { register: self }
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr14(&mut self) -> _Ospeedr14W {
            _Ospeedr14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr13(&mut self) -> _Ospeedr13W {
            _Ospeedr13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr12(&mut self) -> _Ospeedr12W {
            _Ospeedr12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr11(&mut self) -> _Ospeedr11W {
            _Ospeedr11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr10(&mut self) -> _Ospeedr10W {
            _Ospeedr10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr9(&mut self) -> _Ospeedr9W {
            _Ospeedr9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr8(&mut self) -> _Ospeedr8W {
            _Ospeedr8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr7(&mut self) -> _Ospeedr7W {
            _Ospeedr7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr6(&mut self) -> _Ospeedr6W {
            _Ospeedr6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr5(&mut self) -> _Ospeedr5W {
            _Ospeedr5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr4(&mut self) -> _Ospeedr4W {
            _Ospeedr4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr3(&mut self) -> _Ospeedr3W {
            _Ospeedr3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr2(&mut self) -> _Ospeedr2W {
            _Ospeedr2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr1(&mut self) -> _Ospeedr1W {
            _Ospeedr1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn ospeedr0(&mut self) -> _Ospeedr0W {
            _Ospeedr0W { register: self }
        }
    }
}

# [ doc = "GPIO port pull-up/pull-down register" ]
# [ repr ( C ) ]
pub struct Pupdr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port pull-up/pull-down register" ]
pub mod pupdr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Pupdr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `pupdr15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum PupdrR {
        # [ doc = "00: No pull-up, pull-down" ]
        NoPullUpPullDown,
        # [ doc = "01: Pull-up" ]
        PullUp,
        # [ doc = "10: Pull-down" ]
        PullDown,
        # [ doc = "11: Reserved" ]
        Reserved,
    }
    impl PupdrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                PupdrR::NoPullUpPullDown => 0,
                PupdrR::PullUp => 1,
                PupdrR::PullDown => 2,
                PupdrR::Reserved => 3,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> PupdrR {
            match bits {
                0 => PupdrR::NoPullUpPullDown,
                1 => PupdrR::PullUp,
                2 => PupdrR::PullDown,
                3 => PupdrR::Reserved,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `NoPullUpPullDown`" ]
        pub fn is_no_pull_up_pull_down(&self) -> bool {
            *self == PupdrR::NoPullUpPullDown
        }
        # [ doc = "Check if the value of the field is `PullUp`" ]
        pub fn is_pull_up(&self) -> bool {
            *self == PupdrR::PullUp
        }
        # [ doc = "Check if the value of the field is `PullDown`" ]
        pub fn is_pull_down(&self) -> bool {
            *self == PupdrR::PullDown
        }
        # [ doc = "Check if the value of the field is `Reserved`" ]
        pub fn is_reserved(&self) -> bool {
            *self == PupdrR::Reserved
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `pupdr15`" ]
    pub enum PupdrW {
        # [ doc = "00: No pull-up, pull-down" ]
        NoPullUpPullDown,
        # [ doc = "01: Pull-up" ]
        PullUp,
        # [ doc = "10: Pull-down" ]
        PullDown,
        # [ doc = "11: Reserved" ]
        Reserved,
    }
    impl PupdrW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                PupdrW::NoPullUpPullDown => 0,
                PupdrW::PullUp => 1,
                PupdrW::PullDown => 2,
                PupdrW::Reserved => 3,
            }
        }
    }
    impl<'a> _Pupdr15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: PupdrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "00: No pull-up, pull-down" ]
        pub fn no_pull_up_pull_down(self) -> &'a mut W {
            self.variant(PupdrW::NoPullUpPullDown)
        }
        # [ doc = "01: Pull-up" ]
        pub fn pull_up(self) -> &'a mut W {
            self.variant(PupdrW::PullUp)
        }
        # [ doc = "10: Pull-down" ]
        pub fn pull_down(self) -> &'a mut W {
            self.variant(PupdrW::PullDown)
        }
        # [ doc = "11: Reserved" ]
        pub fn reserved(self) -> &'a mut W {
            self.variant(PupdrW::Reserved)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _pupdr15(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr15(&self) -> PupdrR {
            PupdrR::_from(self._pupdr15())
        }
        fn _pupdr14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr14(&self) -> PupdrR {
            PupdrR::_from(self._pupdr14())
        }
        fn _pupdr13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr13(&self) -> PupdrR {
            PupdrR::_from(self._pupdr13())
        }
        fn _pupdr12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr12(&self) -> PupdrR {
            PupdrR::_from(self._pupdr12())
        }
        fn _pupdr11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr11(&self) -> PupdrR {
            PupdrR::_from(self._pupdr11())
        }
        fn _pupdr10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr10(&self) -> PupdrR {
            PupdrR::_from(self._pupdr10())
        }
        fn _pupdr9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr9(&self) -> PupdrR {
            PupdrR::_from(self._pupdr9())
        }
        fn _pupdr8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr8(&self) -> PupdrR {
            PupdrR::_from(self._pupdr8())
        }
        fn _pupdr7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr7(&self) -> PupdrR {
            PupdrR::_from(self._pupdr7())
        }
        fn _pupdr6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr6(&self) -> PupdrR {
            PupdrR::_from(self._pupdr6())
        }
        fn _pupdr5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr5(&self) -> PupdrR {
            PupdrR::_from(self._pupdr5())
        }
        fn _pupdr4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr4(&self) -> PupdrR {
            PupdrR::_from(self._pupdr4())
        }
        fn _pupdr3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr3(&self) -> PupdrR {
            PupdrR::_from(self._pupdr3())
        }
        fn _pupdr2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr2(&self) -> PupdrR {
            PupdrR::_from(self._pupdr2())
        }
        fn _pupdr1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr1(&self) -> PupdrR {
            PupdrR::_from(self._pupdr1())
        }
        fn _pupdr0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr0(&self) -> PupdrR {
            PupdrR::_from(self._pupdr0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 1677721600 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr15(&mut self) -> _Pupdr15W {
            _Pupdr15W { register: self }
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr14(&mut self) -> _Pupdr14W {
            _Pupdr14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr13(&mut self) -> _Pupdr13W {
            _Pupdr13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr12(&mut self) -> _Pupdr12W {
            _Pupdr12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr11(&mut self) -> _Pupdr11W {
            _Pupdr11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr10(&mut self) -> _Pupdr10W {
            _Pupdr10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr9(&mut self) -> _Pupdr9W {
            _Pupdr9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr8(&mut self) -> _Pupdr8W {
            _Pupdr8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr7(&mut self) -> _Pupdr7W {
            _Pupdr7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr6(&mut self) -> _Pupdr6W {
            _Pupdr6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr5(&mut self) -> _Pupdr5W {
            _Pupdr5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr4(&mut self) -> _Pupdr4W {
            _Pupdr4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr3(&mut self) -> _Pupdr3W {
            _Pupdr3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr2(&mut self) -> _Pupdr2W {
            _Pupdr2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr1(&mut self) -> _Pupdr1W {
            _Pupdr1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y =\n              0..15)" ]
        pub fn pupdr0(&mut self) -> _Pupdr0W {
            _Pupdr0W { register: self }
        }
    }
}

# [ doc = "GPIO port input data register" ]
# [ repr ( C ) ]
pub struct Idr {
    register: ::volatile_register::RO<u32>,
}

# [ doc = "GPIO port input data register" ]
pub mod idr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    impl super::Idr {
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
    }
    # [ doc = "Value of the field IDR15" ]
    pub struct Idr15R {
        bits: u8,
    }
    impl Idr15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR14" ]
    pub struct Idr14R {
        bits: u8,
    }
    impl Idr14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR13" ]
    pub struct Idr13R {
        bits: u8,
    }
    impl Idr13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR12" ]
    pub struct Idr12R {
        bits: u8,
    }
    impl Idr12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR11" ]
    pub struct Idr11R {
        bits: u8,
    }
    impl Idr11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR10" ]
    pub struct Idr10R {
        bits: u8,
    }
    impl Idr10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR9" ]
    pub struct Idr9R {
        bits: u8,
    }
    impl Idr9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR8" ]
    pub struct Idr8R {
        bits: u8,
    }
    impl Idr8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR7" ]
    pub struct Idr7R {
        bits: u8,
    }
    impl Idr7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR6" ]
    pub struct Idr6R {
        bits: u8,
    }
    impl Idr6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR5" ]
    pub struct Idr5R {
        bits: u8,
    }
    impl Idr5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR4" ]
    pub struct Idr4R {
        bits: u8,
    }
    impl Idr4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR3" ]
    pub struct Idr3R {
        bits: u8,
    }
    impl Idr3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR2" ]
    pub struct Idr2R {
        bits: u8,
    }
    impl Idr2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR1" ]
    pub struct Idr1R {
        bits: u8,
    }
    impl Idr1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field IDR0" ]
    pub struct Idr0R {
        bits: u8,
    }
    impl Idr0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _idr15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR15" ]
        pub fn idr15(&self) -> Idr15R {
            Idr15R { bits: self._idr15() }
        }
        fn _idr14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR14" ]
        pub fn idr14(&self) -> Idr14R {
            Idr14R { bits: self._idr14() }
        }
        fn _idr13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR13" ]
        pub fn idr13(&self) -> Idr13R {
            Idr13R { bits: self._idr13() }
        }
        fn _idr12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR12" ]
        pub fn idr12(&self) -> Idr12R {
            Idr12R { bits: self._idr12() }
        }
        fn _idr11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR11" ]
        pub fn idr11(&self) -> Idr11R {
            Idr11R { bits: self._idr11() }
        }
        fn _idr10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR10" ]
        pub fn idr10(&self) -> Idr10R {
            Idr10R { bits: self._idr10() }
        }
        fn _idr9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR9" ]
        pub fn idr9(&self) -> Idr9R {
            Idr9R { bits: self._idr9() }
        }
        fn _idr8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR8" ]
        pub fn idr8(&self) -> Idr8R {
            Idr8R { bits: self._idr8() }
        }
        fn _idr7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR7" ]
        pub fn idr7(&self) -> Idr7R {
            Idr7R { bits: self._idr7() }
        }
        fn _idr6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR6" ]
        pub fn idr6(&self) -> Idr6R {
            Idr6R { bits: self._idr6() }
        }
        fn _idr5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR5" ]
        pub fn idr5(&self) -> Idr5R {
            Idr5R { bits: self._idr5() }
        }
        fn _idr4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR4" ]
        pub fn idr4(&self) -> Idr4R {
            Idr4R { bits: self._idr4() }
        }
        fn _idr3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR3" ]
        pub fn idr3(&self) -> Idr3R {
            Idr3R { bits: self._idr3() }
        }
        fn _idr2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR2" ]
        pub fn idr2(&self) -> Idr2R {
            Idr2R { bits: self._idr2() }
        }
        fn _idr1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR1" ]
        pub fn idr1(&self) -> Idr1R {
            Idr1R { bits: self._idr1() }
        }
        fn _idr0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field IDR0" ]
        pub fn idr0(&self) -> Idr0R {
            Idr0R { bits: self._idr0() }
        }
    }
}

# [ doc = "GPIO port output data register" ]
# [ repr ( C ) ]
pub struct Odr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port output data register" ]
pub mod odr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Odr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Value of the field ODR15" ]
    pub struct Odr15R {
        bits: u8,
    }
    impl Odr15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR14" ]
    pub struct Odr14R {
        bits: u8,
    }
    impl Odr14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR13" ]
    pub struct Odr13R {
        bits: u8,
    }
    impl Odr13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR12" ]
    pub struct Odr12R {
        bits: u8,
    }
    impl Odr12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR11" ]
    pub struct Odr11R {
        bits: u8,
    }
    impl Odr11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR10" ]
    pub struct Odr10R {
        bits: u8,
    }
    impl Odr10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR9" ]
    pub struct Odr9R {
        bits: u8,
    }
    impl Odr9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR8" ]
    pub struct Odr8R {
        bits: u8,
    }
    impl Odr8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR7" ]
    pub struct Odr7R {
        bits: u8,
    }
    impl Odr7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR6" ]
    pub struct Odr6R {
        bits: u8,
    }
    impl Odr6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR5" ]
    pub struct Odr5R {
        bits: u8,
    }
    impl Odr5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR4" ]
    pub struct Odr4R {
        bits: u8,
    }
    impl Odr4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR3" ]
    pub struct Odr3R {
        bits: u8,
    }
    impl Odr3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR2" ]
    pub struct Odr2R {
        bits: u8,
    }
    impl Odr2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR1" ]
    pub struct Odr1R {
        bits: u8,
    }
    impl Odr1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ODR0" ]
    pub struct Odr0R {
        bits: u8,
    }
    impl Odr0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr14W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr13W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr12W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr11W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr10W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr9W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr8W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr7W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr6W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr5W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr4W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr3W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr2W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr1W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Odr0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Odr0W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _odr15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR15" ]
        pub fn odr15(&self) -> Odr15R {
            Odr15R { bits: self._odr15() }
        }
        fn _odr14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR14" ]
        pub fn odr14(&self) -> Odr14R {
            Odr14R { bits: self._odr14() }
        }
        fn _odr13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR13" ]
        pub fn odr13(&self) -> Odr13R {
            Odr13R { bits: self._odr13() }
        }
        fn _odr12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR12" ]
        pub fn odr12(&self) -> Odr12R {
            Odr12R { bits: self._odr12() }
        }
        fn _odr11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR11" ]
        pub fn odr11(&self) -> Odr11R {
            Odr11R { bits: self._odr11() }
        }
        fn _odr10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR10" ]
        pub fn odr10(&self) -> Odr10R {
            Odr10R { bits: self._odr10() }
        }
        fn _odr9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR9" ]
        pub fn odr9(&self) -> Odr9R {
            Odr9R { bits: self._odr9() }
        }
        fn _odr8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR8" ]
        pub fn odr8(&self) -> Odr8R {
            Odr8R { bits: self._odr8() }
        }
        fn _odr7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR7" ]
        pub fn odr7(&self) -> Odr7R {
            Odr7R { bits: self._odr7() }
        }
        fn _odr6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR6" ]
        pub fn odr6(&self) -> Odr6R {
            Odr6R { bits: self._odr6() }
        }
        fn _odr5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR5" ]
        pub fn odr5(&self) -> Odr5R {
            Odr5R { bits: self._odr5() }
        }
        fn _odr4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR4" ]
        pub fn odr4(&self) -> Odr4R {
            Odr4R { bits: self._odr4() }
        }
        fn _odr3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR3" ]
        pub fn odr3(&self) -> Odr3R {
            Odr3R { bits: self._odr3() }
        }
        fn _odr2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR2" ]
        pub fn odr2(&self) -> Odr2R {
            Odr2R { bits: self._odr2() }
        }
        fn _odr1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR1" ]
        pub fn odr1(&self) -> Odr1R {
            Odr1R { bits: self._odr1() }
        }
        fn _odr0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Returns the value of the field ODR0" ]
        pub fn odr0(&self) -> Odr0R {
            Odr0R { bits: self._odr0() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 15 - Port output data (y =\n              0..15)" ]
        pub fn odr15(&mut self) -> _Odr15W {
            _Odr15W { register: self }
        }
        # [ doc = "Bit 14 - Port output data (y =\n              0..15)" ]
        pub fn odr14(&mut self) -> _Odr14W {
            _Odr14W { register: self }
        }
        # [ doc = "Bit 13 - Port output data (y =\n              0..15)" ]
        pub fn odr13(&mut self) -> _Odr13W {
            _Odr13W { register: self }
        }
        # [ doc = "Bit 12 - Port output data (y =\n              0..15)" ]
        pub fn odr12(&mut self) -> _Odr12W {
            _Odr12W { register: self }
        }
        # [ doc = "Bit 11 - Port output data (y =\n              0..15)" ]
        pub fn odr11(&mut self) -> _Odr11W {
            _Odr11W { register: self }
        }
        # [ doc = "Bit 10 - Port output data (y =\n              0..15)" ]
        pub fn odr10(&mut self) -> _Odr10W {
            _Odr10W { register: self }
        }
        # [ doc = "Bit 9 - Port output data (y =\n              0..15)" ]
        pub fn odr9(&mut self) -> _Odr9W {
            _Odr9W { register: self }
        }
        # [ doc = "Bit 8 - Port output data (y =\n              0..15)" ]
        pub fn odr8(&mut self) -> _Odr8W {
            _Odr8W { register: self }
        }
        # [ doc = "Bit 7 - Port output data (y =\n              0..15)" ]
        pub fn odr7(&mut self) -> _Odr7W {
            _Odr7W { register: self }
        }
        # [ doc = "Bit 6 - Port output data (y =\n              0..15)" ]
        pub fn odr6(&mut self) -> _Odr6W {
            _Odr6W { register: self }
        }
        # [ doc = "Bit 5 - Port output data (y =\n              0..15)" ]
        pub fn odr5(&mut self) -> _Odr5W {
            _Odr5W { register: self }
        }
        # [ doc = "Bit 4 - Port output data (y =\n              0..15)" ]
        pub fn odr4(&mut self) -> _Odr4W {
            _Odr4W { register: self }
        }
        # [ doc = "Bit 3 - Port output data (y =\n              0..15)" ]
        pub fn odr3(&mut self) -> _Odr3W {
            _Odr3W { register: self }
        }
        # [ doc = "Bit 2 - Port output data (y =\n              0..15)" ]
        pub fn odr2(&mut self) -> _Odr2W {
            _Odr2W { register: self }
        }
        # [ doc = "Bit 1 - Port output data (y =\n              0..15)" ]
        pub fn odr1(&mut self) -> _Odr1W {
            _Odr1W { register: self }
        }
        # [ doc = "Bit 0 - Port output data (y =\n              0..15)" ]
        pub fn odr0(&mut self) -> _Odr0W {
            _Odr0W { register: self }
        }
    }
}

# [ doc = "GPIO port bit set/reset register" ]
# [ repr ( C ) ]
pub struct Bsrr {
    register: ::volatile_register::WO<u32>,
}

# [ doc = "GPIO port bit set/reset register" ]
pub mod bsrr {
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Bsrr {
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `br15`" ]
    pub enum BsrrW {
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        NoAction,
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        SetResetBit,
    }
    impl BsrrW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                BsrrW::NoAction => 0,
                BsrrW::SetResetBit => 1,
            }
        }
    }
    impl<'a> _Br15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Br0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Br0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Bs0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: BsrrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: No action on the corresponding ODRx bit" ]
        pub fn no_action(self) -> &'a mut W {
            self.variant(BsrrW::NoAction)
        }
        # [ doc = "1: Sets/Resets the corresponding ODRx bit" ]
        pub fn set_reset_bit(self) -> &'a mut W {
            self.variant(BsrrW::SetResetBit)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 31 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br15(&mut self) -> _Br15W {
            _Br15W { register: self }
        }
        # [ doc = "Bit 30 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br14(&mut self) -> _Br14W {
            _Br14W { register: self }
        }
        # [ doc = "Bit 29 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br13(&mut self) -> _Br13W {
            _Br13W { register: self }
        }
        # [ doc = "Bit 28 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br12(&mut self) -> _Br12W {
            _Br12W { register: self }
        }
        # [ doc = "Bit 27 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br11(&mut self) -> _Br11W {
            _Br11W { register: self }
        }
        # [ doc = "Bit 26 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br10(&mut self) -> _Br10W {
            _Br10W { register: self }
        }
        # [ doc = "Bit 25 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br9(&mut self) -> _Br9W {
            _Br9W { register: self }
        }
        # [ doc = "Bit 24 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br8(&mut self) -> _Br8W {
            _Br8W { register: self }
        }
        # [ doc = "Bit 23 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br7(&mut self) -> _Br7W {
            _Br7W { register: self }
        }
        # [ doc = "Bit 22 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br6(&mut self) -> _Br6W {
            _Br6W { register: self }
        }
        # [ doc = "Bit 21 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br5(&mut self) -> _Br5W {
            _Br5W { register: self }
        }
        # [ doc = "Bit 20 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br4(&mut self) -> _Br4W {
            _Br4W { register: self }
        }
        # [ doc = "Bit 19 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br3(&mut self) -> _Br3W {
            _Br3W { register: self }
        }
        # [ doc = "Bit 18 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br2(&mut self) -> _Br2W {
            _Br2W { register: self }
        }
        # [ doc = "Bit 17 - Port x reset bit y (y =\n              0..15)" ]
        pub fn br1(&mut self) -> _Br1W {
            _Br1W { register: self }
        }
        # [ doc = "Bit 16 - Port x set bit y (y=\n              0..15)" ]
        pub fn br0(&mut self) -> _Br0W {
            _Br0W { register: self }
        }
        # [ doc = "Bit 15 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs15(&mut self) -> _Bs15W {
            _Bs15W { register: self }
        }
        # [ doc = "Bit 14 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs14(&mut self) -> _Bs14W {
            _Bs14W { register: self }
        }
        # [ doc = "Bit 13 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs13(&mut self) -> _Bs13W {
            _Bs13W { register: self }
        }
        # [ doc = "Bit 12 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs12(&mut self) -> _Bs12W {
            _Bs12W { register: self }
        }
        # [ doc = "Bit 11 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs11(&mut self) -> _Bs11W {
            _Bs11W { register: self }
        }
        # [ doc = "Bit 10 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs10(&mut self) -> _Bs10W {
            _Bs10W { register: self }
        }
        # [ doc = "Bit 9 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs9(&mut self) -> _Bs9W {
            _Bs9W { register: self }
        }
        # [ doc = "Bit 8 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs8(&mut self) -> _Bs8W {
            _Bs8W { register: self }
        }
        # [ doc = "Bit 7 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs7(&mut self) -> _Bs7W {
            _Bs7W { register: self }
        }
        # [ doc = "Bit 6 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs6(&mut self) -> _Bs6W {
            _Bs6W { register: self }
        }
        # [ doc = "Bit 5 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs5(&mut self) -> _Bs5W {
            _Bs5W { register: self }
        }
        # [ doc = "Bit 4 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs4(&mut self) -> _Bs4W {
            _Bs4W { register: self }
        }
        # [ doc = "Bit 3 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs3(&mut self) -> _Bs3W {
            _Bs3W { register: self }
        }
        # [ doc = "Bit 2 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs2(&mut self) -> _Bs2W {
            _Bs2W { register: self }
        }
        # [ doc = "Bit 1 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs1(&mut self) -> _Bs1W {
            _Bs1W { register: self }
        }
        # [ doc = "Bit 0 - Port x set bit y (y=\n              0..15)" ]
        pub fn bs0(&mut self) -> _Bs0W {
            _Bs0W { register: self }
        }
    }
}

# [ doc = "GPIO port configuration lock register" ]
# [ repr ( C ) ]
pub struct Lckr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO port configuration lock register" ]
pub mod lckr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Lckr {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `lckk`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LckkR {
        # [ doc = "0: Port configuration lock key not active" ]
        LockKeyNotActive,
        # [ doc = "1: Port configuration lock key active. The GPIOx_LCKR register is locked until an MCU reset\n                  or a peripheral reset occurs." ]
        LockKeyActive,
    }
    impl LckkR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                LckkR::LockKeyNotActive => 0,
                LckkR::LockKeyActive => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> LckkR {
            match bits {
                0 => LckkR::LockKeyNotActive,
                1 => LckkR::LockKeyActive,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `LockKeyNotActive`" ]
        pub fn is_lock_key_not_active(&self) -> bool {
            *self == LckkR::LockKeyNotActive
        }
        # [ doc = "Check if the value of the field is `LockKeyActive`" ]
        pub fn is_lock_key_active(&self) -> bool {
            *self == LckkR::LockKeyActive
        }
    }
    # [ doc = "Possible values of the field `lck15`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum LckyR {
        # [ doc = "0: Port configuration not locked" ]
        LockKeyNotActive,
        # [ doc = "1: Port configuration locked" ]
        LockKeyActive,
    }
    impl LckyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                LckyR::LockKeyNotActive => 0,
                LckyR::LockKeyActive => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> LckyR {
            match bits {
                0 => LckyR::LockKeyNotActive,
                1 => LckyR::LockKeyActive,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `LockKeyNotActive`" ]
        pub fn is_lock_key_not_active(&self) -> bool {
            *self == LckyR::LockKeyNotActive
        }
        # [ doc = "Check if the value of the field is `LockKeyActive`" ]
        pub fn is_lock_key_active(&self) -> bool {
            *self == LckyR::LockKeyActive
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LckkW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `lckk`" ]
    pub enum LckkW {
        # [ doc = "0: Port configuration lock key not active" ]
        LockKeyNotActive,
        # [ doc = "1: Port configuration lock key active. The GPIOx_LCKR register is locked until an MCU reset\n                  or a peripheral reset occurs." ]
        LockKeyActive,
    }
    impl LckkW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                LckkW::LockKeyNotActive => 0,
                LckkW::LockKeyActive => 1,
            }
        }
    }
    impl<'a> _LckkW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckkW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration lock key not active" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckkW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration lock key active. The GPIOx_LCKR register is locked until an MCU reset or a peripheral reset occurs." ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckkW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck15W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `lck15`" ]
    pub enum LckyW {
        # [ doc = "0: Port configuration not locked" ]
        LockKeyNotActive,
        # [ doc = "1: Port configuration locked" ]
        LockKeyActive,
    }
    impl LckyW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                LckyW::LockKeyNotActive => 0,
                LckyW::LockKeyActive => 1,
            }
        }
    }
    impl<'a> _Lck15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Lck0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: LckyW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Port configuration not locked" ]
        pub fn lock_key_not_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyNotActive)
        }
        # [ doc = "1: Port configuration locked" ]
        pub fn lock_key_active(self) -> &'a mut W {
            self.variant(LckyW::LockKeyActive)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _lckk(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lckk(&self) -> LckkR {
            LckkR::_from(self._lckk())
        }
        fn _lck15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck15(&self) -> LckyR {
            LckyR::_from(self._lck15())
        }
        fn _lck14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck14(&self) -> LckyR {
            LckyR::_from(self._lck14())
        }
        fn _lck13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck13(&self) -> LckyR {
            LckyR::_from(self._lck13())
        }
        fn _lck12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck12(&self) -> LckyR {
            LckyR::_from(self._lck12())
        }
        fn _lck11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck11(&self) -> LckyR {
            LckyR::_from(self._lck11())
        }
        fn _lck10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck10(&self) -> LckyR {
            LckyR::_from(self._lck10())
        }
        fn _lck9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck9(&self) -> LckyR {
            LckyR::_from(self._lck9())
        }
        fn _lck8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck8(&self) -> LckyR {
            LckyR::_from(self._lck8())
        }
        fn _lck7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck7(&self) -> LckyR {
            LckyR::_from(self._lck7())
        }
        fn _lck6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck6(&self) -> LckyR {
            LckyR::_from(self._lck6())
        }
        fn _lck5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck5(&self) -> LckyR {
            LckyR::_from(self._lck5())
        }
        fn _lck4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck4(&self) -> LckyR {
            LckyR::_from(self._lck4())
        }
        fn _lck3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck3(&self) -> LckyR {
            LckyR::_from(self._lck3())
        }
        fn _lck2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck2(&self) -> LckyR {
            LckyR::_from(self._lck2())
        }
        fn _lck1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck1(&self) -> LckyR {
            LckyR::_from(self._lck1())
        }
        fn _lck0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck0(&self) -> LckyR {
            LckyR::_from(self._lck0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 16 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lckk(&mut self) -> _LckkW {
            _LckkW { register: self }
        }
        # [ doc = "Bit 15 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck15(&mut self) -> _Lck15W {
            _Lck15W { register: self }
        }
        # [ doc = "Bit 14 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck14(&mut self) -> _Lck14W {
            _Lck14W { register: self }
        }
        # [ doc = "Bit 13 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck13(&mut self) -> _Lck13W {
            _Lck13W { register: self }
        }
        # [ doc = "Bit 12 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck12(&mut self) -> _Lck12W {
            _Lck12W { register: self }
        }
        # [ doc = "Bit 11 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck11(&mut self) -> _Lck11W {
            _Lck11W { register: self }
        }
        # [ doc = "Bit 10 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck10(&mut self) -> _Lck10W {
            _Lck10W { register: self }
        }
        # [ doc = "Bit 9 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck9(&mut self) -> _Lck9W {
            _Lck9W { register: self }
        }
        # [ doc = "Bit 8 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck8(&mut self) -> _Lck8W {
            _Lck8W { register: self }
        }
        # [ doc = "Bit 7 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck7(&mut self) -> _Lck7W {
            _Lck7W { register: self }
        }
        # [ doc = "Bit 6 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck6(&mut self) -> _Lck6W {
            _Lck6W { register: self }
        }
        # [ doc = "Bit 5 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck5(&mut self) -> _Lck5W {
            _Lck5W { register: self }
        }
        # [ doc = "Bit 4 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck4(&mut self) -> _Lck4W {
            _Lck4W { register: self }
        }
        # [ doc = "Bit 3 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck3(&mut self) -> _Lck3W {
            _Lck3W { register: self }
        }
        # [ doc = "Bit 2 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck2(&mut self) -> _Lck2W {
            _Lck2W { register: self }
        }
        # [ doc = "Bit 1 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck1(&mut self) -> _Lck1W {
            _Lck1W { register: self }
        }
        # [ doc = "Bit 0 - Port x lock bit y (y=\n              0..15)" ]
        pub fn lck0(&mut self) -> _Lck0W {
            _Lck0W { register: self }
        }
    }
}

# [ doc = "GPIO alternate function low register" ]
# [ repr ( C ) ]
pub struct Afrl {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO alternate function low register" ]
pub mod afrl {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Afrl {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    # [ doc = "Possible values of the field `afrl7`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum AfrR {
        # [ doc = "0000: AF0" ]
        Af0,
        # [ doc = "0001: AF1" ]
        Af1,
        # [ doc = "0010: AF2" ]
        Af2,
        # [ doc = "0011: AF3" ]
        Af3,
        # [ doc = "0100: AF4" ]
        Af4,
        # [ doc = "0101: AF5" ]
        Af5,
        # [ doc = "0110: AF6" ]
        Af6,
        # [ doc = "0111: AF7" ]
        Af7,
        # [ doc = "1000: AF8" ]
        Af8,
        # [ doc = "Reserved" ]
        _Reserved1001,
        # [ doc = "1010: AF10" ]
        Af10,
        # [ doc = "1011: AF11" ]
        Af11,
        # [ doc = "1100: AF12" ]
        Af12,
        # [ doc = "1101: AF13" ]
        Af13,
        # [ doc = "1110: AF14" ]
        Af14,
        # [ doc = "1111: AF15" ]
        Af15,
    }
    impl AfrR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                AfrR::Af0 => 0,
                AfrR::Af1 => 1,
                AfrR::Af2 => 2,
                AfrR::Af3 => 3,
                AfrR::Af4 => 4,
                AfrR::Af5 => 5,
                AfrR::Af6 => 6,
                AfrR::Af7 => 7,
                AfrR::Af8 => 8,
                AfrR::_Reserved1001 => 9,
                AfrR::Af10 => 10,
                AfrR::Af11 => 11,
                AfrR::Af12 => 12,
                AfrR::Af13 => 13,
                AfrR::Af14 => 14,
                AfrR::Af15 => 15,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> AfrR {
            match bits {
                0 => AfrR::Af0,
                1 => AfrR::Af1,
                2 => AfrR::Af2,
                3 => AfrR::Af3,
                4 => AfrR::Af4,
                5 => AfrR::Af5,
                6 => AfrR::Af6,
                7 => AfrR::Af7,
                8 => AfrR::Af8,
                9 => AfrR::_Reserved1001,
                10 => AfrR::Af10,
                11 => AfrR::Af11,
                12 => AfrR::Af12,
                13 => AfrR::Af13,
                14 => AfrR::Af14,
                15 => AfrR::Af15,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `Af0`" ]
        pub fn is_af0(&self) -> bool {
            *self == AfrR::Af0
        }
        # [ doc = "Check if the value of the field is `Af1`" ]
        pub fn is_af1(&self) -> bool {
            *self == AfrR::Af1
        }
        # [ doc = "Check if the value of the field is `Af2`" ]
        pub fn is_af2(&self) -> bool {
            *self == AfrR::Af2
        }
        # [ doc = "Check if the value of the field is `Af3`" ]
        pub fn is_af3(&self) -> bool {
            *self == AfrR::Af3
        }
        # [ doc = "Check if the value of the field is `Af4`" ]
        pub fn is_af4(&self) -> bool {
            *self == AfrR::Af4
        }
        # [ doc = "Check if the value of the field is `Af5`" ]
        pub fn is_af5(&self) -> bool {
            *self == AfrR::Af5
        }
        # [ doc = "Check if the value of the field is `Af6`" ]
        pub fn is_af6(&self) -> bool {
            *self == AfrR::Af6
        }
        # [ doc = "Check if the value of the field is `Af7`" ]
        pub fn is_af7(&self) -> bool {
            *self == AfrR::Af7
        }
        # [ doc = "Check if the value of the field is `Af8`" ]
        pub fn is_af8(&self) -> bool {
            *self == AfrR::Af8
        }
        # [ doc = "Check if the value of the field is `Af10`" ]
        pub fn is_af10(&self) -> bool {
            *self == AfrR::Af10
        }
        # [ doc = "Check if the value of the field is `Af11`" ]
        pub fn is_af11(&self) -> bool {
            *self == AfrR::Af11
        }
        # [ doc = "Check if the value of the field is `Af12`" ]
        pub fn is_af12(&self) -> bool {
            *self == AfrR::Af12
        }
        # [ doc = "Check if the value of the field is `Af13`" ]
        pub fn is_af13(&self) -> bool {
            *self == AfrR::Af13
        }
        # [ doc = "Check if the value of the field is `Af14`" ]
        pub fn is_af14(&self) -> bool {
            *self == AfrR::Af14
        }
        # [ doc = "Check if the value of the field is `Af15`" ]
        pub fn is_af15(&self) -> bool {
            *self == AfrR::Af15
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl7W<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `afrl7`" ]
    pub enum AfrW {
        # [ doc = "0000: AF0" ]
        Af0,
        # [ doc = "0001: AF1" ]
        Af1,
        # [ doc = "0010: AF2" ]
        Af2,
        # [ doc = "0011: AF3" ]
        Af3,
        # [ doc = "0100: AF4" ]
        Af4,
        # [ doc = "0101: AF5" ]
        Af5,
        # [ doc = "0110: AF6" ]
        Af6,
        # [ doc = "0111: AF7" ]
        Af7,
        # [ doc = "1000: AF8" ]
        Af8,
        # [ doc = "1001: AF9" ]
        Af9,
        # [ doc = "1010: AF10" ]
        Af10,
        # [ doc = "1011: AF11" ]
        Af11,
        # [ doc = "1100: AF12" ]
        Af12,
        # [ doc = "1101: AF13" ]
        Af13,
        # [ doc = "1110: AF14" ]
        Af14,
        # [ doc = "1111: AF15" ]
        Af15,
    }
    impl AfrW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                AfrW::Af0 => 0,
                AfrW::Af1 => 1,
                AfrW::Af2 => 2,
                AfrW::Af3 => 3,
                AfrW::Af4 => 4,
                AfrW::Af5 => 5,
                AfrW::Af6 => 6,
                AfrW::Af7 => 7,
                AfrW::Af8 => 8,
                AfrW::Af9 => 8,
                AfrW::Af10 => 10,
                AfrW::Af11 => 11,
                AfrW::Af12 => 12,
                AfrW::Af13 => 13,
                AfrW::Af14 => 14,
                AfrW::Af15 => 15,
            }
        }
    }
    impl<'a> _Afrl7W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl6W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl5W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl4W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl3W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl2W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl1W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl0W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _afrl7(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:31 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl7(&self) -> AfrR {
            AfrR::_from(self._afrl7())
        }
        fn _afrl6(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl6(&self) -> AfrR {
            AfrR::_from(self._afrl6())
        }
        fn _afrl5(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl5(&self) -> AfrR {
            AfrR::_from(self._afrl5())
        }
        fn _afrl4(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl4(&self) -> AfrR {
            AfrR::_from(self._afrl4())
        }
        fn _afrl3(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl3(&self) -> AfrR {
            AfrR::_from(self._afrl3())
        }
        fn _afrl2(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl2(&self) -> AfrR {
            AfrR::_from(self._afrl2())
        }
        fn _afrl1(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl1(&self) -> AfrR {
            AfrR::_from(self._afrl1())
        }
        fn _afrl0(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl0(&self) -> AfrR {
            AfrR::_from(self._afrl0())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:31 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl7(&mut self) -> _Afrl7W {
            _Afrl7W { register: self }
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl6(&mut self) -> _Afrl6W {
            _Afrl6W { register: self }
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl5(&mut self) -> _Afrl5W {
            _Afrl5W { register: self }
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl4(&mut self) -> _Afrl4W {
            _Afrl4W { register: self }
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl3(&mut self) -> _Afrl3W {
            _Afrl3W { register: self }
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl2(&mut self) -> _Afrl2W {
            _Afrl2W { register: self }
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl1(&mut self) -> _Afrl1W {
            _Afrl1W { register: self }
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x\n              bit y (y = 0..7)" ]
        pub fn afrl0(&mut self) -> _Afrl0W {
            _Afrl0W { register: self }
        }
    }
}

# [ doc = "GPIO alternate function high register" ]
# [ repr ( C ) ]
pub struct Afrh {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "GPIO alternate function high register" ]
pub mod afrh {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Afrh {
        # [ doc = r" Modifies the contents of the register" ]
        pub fn modify<F>(&mut self, f: F)
            where for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W
        {
            let bits = self.register.read();
            let r = R { bits: bits };
            let mut w = W { bits: bits };
            f(&r, &mut w);
            self.register.write(w.bits);
        }
        # [ doc = r" Reads the contents of the register" ]
        pub fn read(&self) -> R {
            R { bits: self.register.read() }
        }
        # [ doc = r" Writes to the register" ]
        pub fn write<F>(&mut self, f: F)
            where F: FnOnce(&mut W) -> &mut W
        {
            let mut w = W::reset_value();
            f(&mut w);
            self.register.write(w.bits);
        }
    }
    pub use super::afrl::AfrR;
    # [ doc = r" Proxy" ]
    pub struct _Afrh15W<'a> {
        register: &'a mut W,
    }
    pub use super::afrl::AfrW;
    impl<'a> _Afrh15W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh14W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh13W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh12W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh11W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh10W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh9W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh8W<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: AfrW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0000: AF0" ]
        pub fn af0(self) -> &'a mut W {
            self.variant(AfrW::Af0)
        }
        # [ doc = "0001: AF1" ]
        pub fn af1(self) -> &'a mut W {
            self.variant(AfrW::Af1)
        }
        # [ doc = "0010: AF2" ]
        pub fn af2(self) -> &'a mut W {
            self.variant(AfrW::Af2)
        }
        # [ doc = "0011: AF3" ]
        pub fn af3(self) -> &'a mut W {
            self.variant(AfrW::Af3)
        }
        # [ doc = "0100: AF4" ]
        pub fn af4(self) -> &'a mut W {
            self.variant(AfrW::Af4)
        }
        # [ doc = "0101: AF5" ]
        pub fn af5(self) -> &'a mut W {
            self.variant(AfrW::Af5)
        }
        # [ doc = "0110: AF6" ]
        pub fn af6(self) -> &'a mut W {
            self.variant(AfrW::Af6)
        }
        # [ doc = "0111: AF7" ]
        pub fn af7(self) -> &'a mut W {
            self.variant(AfrW::Af7)
        }
        # [ doc = "1000: AF8" ]
        pub fn af8(self) -> &'a mut W {
            self.variant(AfrW::Af8)
        }
        # [ doc = "1001: AF9" ]
        pub fn af9(self) -> &'a mut W {
            self.variant(AfrW::Af9)
        }
        # [ doc = "1010: AF10" ]
        pub fn af10(self) -> &'a mut W {
            self.variant(AfrW::Af10)
        }
        # [ doc = "1011: AF11" ]
        pub fn af11(self) -> &'a mut W {
            self.variant(AfrW::Af11)
        }
        # [ doc = "1100: AF12" ]
        pub fn af12(self) -> &'a mut W {
            self.variant(AfrW::Af12)
        }
        # [ doc = "1101: AF13" ]
        pub fn af13(self) -> &'a mut W {
            self.variant(AfrW::Af13)
        }
        # [ doc = "1110: AF14" ]
        pub fn af14(self) -> &'a mut W {
            self.variant(AfrW::Af14)
        }
        # [ doc = "1111: AF15" ]
        pub fn af15(self) -> &'a mut W {
            self.variant(AfrW::Af15)
        }
        # [ doc = r" Writes raw `bits` to the field" ]
        pub fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    impl R {
        # [ doc = r" Value of the register as raw bits" ]
        pub fn bits(&self) -> u32 {
            self.bits
        }
        fn _afrh15(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:31 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh15(&self) -> AfrR {
            AfrR::_from(self._afrh15())
        }
        fn _afrh14(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh14(&self) -> AfrR {
            AfrR::_from(self._afrh14())
        }
        fn _afrh13(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh13(&self) -> AfrR {
            AfrR::_from(self._afrh13())
        }
        fn _afrh12(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh12(&self) -> AfrR {
            AfrR::_from(self._afrh12())
        }
        fn _afrh11(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh11(&self) -> AfrR {
            AfrR::_from(self._afrh11())
        }
        fn _afrh10(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh10(&self) -> AfrR {
            AfrR::_from(self._afrh10())
        }
        fn _afrh9(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh9(&self) -> AfrR {
            AfrR::_from(self._afrh9())
        }
        fn _afrh8(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh8(&self) -> AfrR {
            AfrR::_from(self._afrh8())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 0 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:31 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh15(&mut self) -> _Afrh15W {
            _Afrh15W { register: self }
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh14(&mut self) -> _Afrh14W {
            _Afrh14W { register: self }
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh13(&mut self) -> _Afrh13W {
            _Afrh13W { register: self }
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh12(&mut self) -> _Afrh12W {
            _Afrh12W { register: self }
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh11(&mut self) -> _Afrh11W {
            _Afrh11W { register: self }
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh10(&mut self) -> _Afrh10W {
            _Afrh10W { register: self }
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh9(&mut self) -> _Afrh9W {
            _Afrh9W { register: self }
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x\n              bit y (y = 8..15)" ]
        pub fn afrh8(&mut self) -> _Afrh8W {
            _Afrh8W { register: self }
        }
    }
}
