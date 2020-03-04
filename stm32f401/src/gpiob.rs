# ! [ doc = "General-purpose I/Os" ]# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Gpiob {
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
    # [ doc = "Value of the field MODER15" ]
    pub struct Moder15R {
        bits: u8,
    }
    impl Moder15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER14" ]
    pub struct Moder14R {
        bits: u8,
    }
    impl Moder14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER13" ]
    pub struct Moder13R {
        bits: u8,
    }
    impl Moder13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER12" ]
    pub struct Moder12R {
        bits: u8,
    }
    impl Moder12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER11" ]
    pub struct Moder11R {
        bits: u8,
    }
    impl Moder11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER10" ]
    pub struct Moder10R {
        bits: u8,
    }
    impl Moder10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER9" ]
    pub struct Moder9R {
        bits: u8,
    }
    impl Moder9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER8" ]
    pub struct Moder8R {
        bits: u8,
    }
    impl Moder8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER7" ]
    pub struct Moder7R {
        bits: u8,
    }
    impl Moder7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER6" ]
    pub struct Moder6R {
        bits: u8,
    }
    impl Moder6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER5" ]
    pub struct Moder5R {
        bits: u8,
    }
    impl Moder5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER4" ]
    pub struct Moder4R {
        bits: u8,
    }
    impl Moder4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER3" ]
    pub struct Moder3R {
        bits: u8,
    }
    impl Moder3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER2" ]
    pub struct Moder2R {
        bits: u8,
    }
    impl Moder2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER1" ]
    pub struct Moder1R {
        bits: u8,
    }
    impl Moder1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODER0" ]
    pub struct Moder0R {
        bits: u8,
    }
    impl Moder0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Moder15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Moder15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        pub fn moder15(&self) -> Moder15R {
            Moder15R { bits: self._moder15() }
        }
        fn _moder14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn moder14(&self) -> Moder14R {
            Moder14R { bits: self._moder14() }
        }
        fn _moder13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn moder13(&self) -> Moder13R {
            Moder13R { bits: self._moder13() }
        }
        fn _moder12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn moder12(&self) -> Moder12R {
            Moder12R { bits: self._moder12() }
        }
        fn _moder11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn moder11(&self) -> Moder11R {
            Moder11R { bits: self._moder11() }
        }
        fn _moder10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn moder10(&self) -> Moder10R {
            Moder10R { bits: self._moder10() }
        }
        fn _moder9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn moder9(&self) -> Moder9R {
            Moder9R { bits: self._moder9() }
        }
        fn _moder8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn moder8(&self) -> Moder8R {
            Moder8R { bits: self._moder8() }
        }
        fn _moder7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn moder7(&self) -> Moder7R {
            Moder7R { bits: self._moder7() }
        }
        fn _moder6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn moder6(&self) -> Moder6R {
            Moder6R { bits: self._moder6() }
        }
        fn _moder5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn moder5(&self) -> Moder5R {
            Moder5R { bits: self._moder5() }
        }
        fn _moder4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn moder4(&self) -> Moder4R {
            Moder4R { bits: self._moder4() }
        }
        fn _moder3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn moder3(&self) -> Moder3R {
            Moder3R { bits: self._moder3() }
        }
        fn _moder2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn moder2(&self) -> Moder2R {
            Moder2R { bits: self._moder2() }
        }
        fn _moder1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn moder1(&self) -> Moder1R {
            Moder1R { bits: self._moder1() }
        }
        fn _moder0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
        pub fn moder0(&self) -> Moder0R {
            Moder0R { bits: self._moder0() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 640 }
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
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn moder14(&mut self) -> _Moder14W {
            _Moder14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn moder13(&mut self) -> _Moder13W {
            _Moder13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn moder12(&mut self) -> _Moder12W {
            _Moder12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn moder11(&mut self) -> _Moder11W {
            _Moder11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn moder10(&mut self) -> _Moder10W {
            _Moder10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn moder9(&mut self) -> _Moder9W {
            _Moder9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn moder8(&mut self) -> _Moder8W {
            _Moder8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn moder7(&mut self) -> _Moder7W {
            _Moder7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn moder6(&mut self) -> _Moder6W {
            _Moder6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn moder5(&mut self) -> _Moder5W {
            _Moder5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn moder4(&mut self) -> _Moder4W {
            _Moder4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn moder3(&mut self) -> _Moder3W {
            _Moder3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn moder2(&mut self) -> _Moder2W {
            _Moder2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn moder1(&mut self) -> _Moder1W {
            _Moder1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
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
    # [ doc = "Value of the field OT15" ]
    pub struct Ot15R {
        bits: u8,
    }
    impl Ot15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT14" ]
    pub struct Ot14R {
        bits: u8,
    }
    impl Ot14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT13" ]
    pub struct Ot13R {
        bits: u8,
    }
    impl Ot13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT12" ]
    pub struct Ot12R {
        bits: u8,
    }
    impl Ot12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT11" ]
    pub struct Ot11R {
        bits: u8,
    }
    impl Ot11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT10" ]
    pub struct Ot10R {
        bits: u8,
    }
    impl Ot10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT9" ]
    pub struct Ot9R {
        bits: u8,
    }
    impl Ot9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT8" ]
    pub struct Ot8R {
        bits: u8,
    }
    impl Ot8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT7" ]
    pub struct Ot7R {
        bits: u8,
    }
    impl Ot7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT6" ]
    pub struct Ot6R {
        bits: u8,
    }
    impl Ot6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT5" ]
    pub struct Ot5R {
        bits: u8,
    }
    impl Ot5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT4" ]
    pub struct Ot4R {
        bits: u8,
    }
    impl Ot4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT3" ]
    pub struct Ot3R {
        bits: u8,
    }
    impl Ot3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT2" ]
    pub struct Ot2R {
        bits: u8,
    }
    impl Ot2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT1" ]
    pub struct Ot1R {
        bits: u8,
    }
    impl Ot1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OT0" ]
    pub struct Ot0R {
        bits: u8,
    }
    impl Ot0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ot15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot15W<'a> {
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
    pub struct _Ot14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot14W<'a> {
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
    pub struct _Ot13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot13W<'a> {
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
    pub struct _Ot12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot12W<'a> {
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
    pub struct _Ot11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot11W<'a> {
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
    pub struct _Ot10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot10W<'a> {
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
    pub struct _Ot9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot9W<'a> {
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
    pub struct _Ot8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot8W<'a> {
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
    pub struct _Ot7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot7W<'a> {
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
    pub struct _Ot6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot6W<'a> {
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
    pub struct _Ot5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot5W<'a> {
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
    pub struct _Ot4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot4W<'a> {
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
    pub struct _Ot3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot3W<'a> {
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
    pub struct _Ot2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot2W<'a> {
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
    pub struct _Ot1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot1W<'a> {
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
    pub struct _Ot0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ot0W<'a> {
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
        fn _ot15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Port x configuration bits (y = 0..15)" ]
        pub fn ot15(&self) -> Ot15R {
            Ot15R { bits: self._ot15() }
        }
        fn _ot14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port x configuration bits (y = 0..15)" ]
        pub fn ot14(&self) -> Ot14R {
            Ot14R { bits: self._ot14() }
        }
        fn _ot13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port x configuration bits (y = 0..15)" ]
        pub fn ot13(&self) -> Ot13R {
            Ot13R { bits: self._ot13() }
        }
        fn _ot12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port x configuration bits (y = 0..15)" ]
        pub fn ot12(&self) -> Ot12R {
            Ot12R { bits: self._ot12() }
        }
        fn _ot11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port x configuration bits (y = 0..15)" ]
        pub fn ot11(&self) -> Ot11R {
            Ot11R { bits: self._ot11() }
        }
        fn _ot10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port x configuration bits (y = 0..15)" ]
        pub fn ot10(&self) -> Ot10R {
            Ot10R { bits: self._ot10() }
        }
        fn _ot9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port x configuration bits (y = 0..15)" ]
        pub fn ot9(&self) -> Ot9R {
            Ot9R { bits: self._ot9() }
        }
        fn _ot8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port x configuration bits (y = 0..15)" ]
        pub fn ot8(&self) -> Ot8R {
            Ot8R { bits: self._ot8() }
        }
        fn _ot7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port x configuration bits (y = 0..15)" ]
        pub fn ot7(&self) -> Ot7R {
            Ot7R { bits: self._ot7() }
        }
        fn _ot6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port x configuration bits (y = 0..15)" ]
        pub fn ot6(&self) -> Ot6R {
            Ot6R { bits: self._ot6() }
        }
        fn _ot5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port x configuration bits (y = 0..15)" ]
        pub fn ot5(&self) -> Ot5R {
            Ot5R { bits: self._ot5() }
        }
        fn _ot4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port x configuration bits (y = 0..15)" ]
        pub fn ot4(&self) -> Ot4R {
            Ot4R { bits: self._ot4() }
        }
        fn _ot3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port x configuration bits (y = 0..15)" ]
        pub fn ot3(&self) -> Ot3R {
            Ot3R { bits: self._ot3() }
        }
        fn _ot2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port x configuration bits (y = 0..15)" ]
        pub fn ot2(&self) -> Ot2R {
            Ot2R { bits: self._ot2() }
        }
        fn _ot1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port x configuration bits (y = 0..15)" ]
        pub fn ot1(&self) -> Ot1R {
            Ot1R { bits: self._ot1() }
        }
        fn _ot0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port x configuration bits (y = 0..15)" ]
        pub fn ot0(&self) -> Ot0R {
            Ot0R { bits: self._ot0() }
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
        # [ doc = "Bit 15 - Port x configuration bits (y = 0..15)" ]
        pub fn ot15(&mut self) -> _Ot15W {
            _Ot15W { register: self }
        }
        # [ doc = "Bit 14 - Port x configuration bits (y = 0..15)" ]
        pub fn ot14(&mut self) -> _Ot14W {
            _Ot14W { register: self }
        }
        # [ doc = "Bit 13 - Port x configuration bits (y = 0..15)" ]
        pub fn ot13(&mut self) -> _Ot13W {
            _Ot13W { register: self }
        }
        # [ doc = "Bit 12 - Port x configuration bits (y = 0..15)" ]
        pub fn ot12(&mut self) -> _Ot12W {
            _Ot12W { register: self }
        }
        # [ doc = "Bit 11 - Port x configuration bits (y = 0..15)" ]
        pub fn ot11(&mut self) -> _Ot11W {
            _Ot11W { register: self }
        }
        # [ doc = "Bit 10 - Port x configuration bits (y = 0..15)" ]
        pub fn ot10(&mut self) -> _Ot10W {
            _Ot10W { register: self }
        }
        # [ doc = "Bit 9 - Port x configuration bits (y = 0..15)" ]
        pub fn ot9(&mut self) -> _Ot9W {
            _Ot9W { register: self }
        }
        # [ doc = "Bit 8 - Port x configuration bits (y = 0..15)" ]
        pub fn ot8(&mut self) -> _Ot8W {
            _Ot8W { register: self }
        }
        # [ doc = "Bit 7 - Port x configuration bits (y = 0..15)" ]
        pub fn ot7(&mut self) -> _Ot7W {
            _Ot7W { register: self }
        }
        # [ doc = "Bit 6 - Port x configuration bits (y = 0..15)" ]
        pub fn ot6(&mut self) -> _Ot6W {
            _Ot6W { register: self }
        }
        # [ doc = "Bit 5 - Port x configuration bits (y = 0..15)" ]
        pub fn ot5(&mut self) -> _Ot5W {
            _Ot5W { register: self }
        }
        # [ doc = "Bit 4 - Port x configuration bits (y = 0..15)" ]
        pub fn ot4(&mut self) -> _Ot4W {
            _Ot4W { register: self }
        }
        # [ doc = "Bit 3 - Port x configuration bits (y = 0..15)" ]
        pub fn ot3(&mut self) -> _Ot3W {
            _Ot3W { register: self }
        }
        # [ doc = "Bit 2 - Port x configuration bits (y = 0..15)" ]
        pub fn ot2(&mut self) -> _Ot2W {
            _Ot2W { register: self }
        }
        # [ doc = "Bit 1 - Port x configuration bits (y = 0..15)" ]
        pub fn ot1(&mut self) -> _Ot1W {
            _Ot1W { register: self }
        }
        # [ doc = "Bit 0 - Port x configuration bits (y = 0..15)" ]
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
    # [ doc = "Value of the field OSPEEDR15" ]
    pub struct Ospeedr15R {
        bits: u8,
    }
    impl Ospeedr15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR14" ]
    pub struct Ospeedr14R {
        bits: u8,
    }
    impl Ospeedr14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR13" ]
    pub struct Ospeedr13R {
        bits: u8,
    }
    impl Ospeedr13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR12" ]
    pub struct Ospeedr12R {
        bits: u8,
    }
    impl Ospeedr12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR11" ]
    pub struct Ospeedr11R {
        bits: u8,
    }
    impl Ospeedr11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR10" ]
    pub struct Ospeedr10R {
        bits: u8,
    }
    impl Ospeedr10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR9" ]
    pub struct Ospeedr9R {
        bits: u8,
    }
    impl Ospeedr9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR8" ]
    pub struct Ospeedr8R {
        bits: u8,
    }
    impl Ospeedr8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR7" ]
    pub struct Ospeedr7R {
        bits: u8,
    }
    impl Ospeedr7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR6" ]
    pub struct Ospeedr6R {
        bits: u8,
    }
    impl Ospeedr6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR5" ]
    pub struct Ospeedr5R {
        bits: u8,
    }
    impl Ospeedr5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR4" ]
    pub struct Ospeedr4R {
        bits: u8,
    }
    impl Ospeedr4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR3" ]
    pub struct Ospeedr3R {
        bits: u8,
    }
    impl Ospeedr3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR2" ]
    pub struct Ospeedr2R {
        bits: u8,
    }
    impl Ospeedr2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR1" ]
    pub struct Ospeedr1R {
        bits: u8,
    }
    impl Ospeedr1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OSPEEDR0" ]
    pub struct Ospeedr0R {
        bits: u8,
    }
    impl Ospeedr0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ospeedr15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ospeedr15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr15(&self) -> Ospeedr15R {
            Ospeedr15R { bits: self._ospeedr15() }
        }
        fn _ospeedr14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr14(&self) -> Ospeedr14R {
            Ospeedr14R { bits: self._ospeedr14() }
        }
        fn _ospeedr13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr13(&self) -> Ospeedr13R {
            Ospeedr13R { bits: self._ospeedr13() }
        }
        fn _ospeedr12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr12(&self) -> Ospeedr12R {
            Ospeedr12R { bits: self._ospeedr12() }
        }
        fn _ospeedr11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr11(&self) -> Ospeedr11R {
            Ospeedr11R { bits: self._ospeedr11() }
        }
        fn _ospeedr10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr10(&self) -> Ospeedr10R {
            Ospeedr10R { bits: self._ospeedr10() }
        }
        fn _ospeedr9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr9(&self) -> Ospeedr9R {
            Ospeedr9R { bits: self._ospeedr9() }
        }
        fn _ospeedr8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr8(&self) -> Ospeedr8R {
            Ospeedr8R { bits: self._ospeedr8() }
        }
        fn _ospeedr7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr7(&self) -> Ospeedr7R {
            Ospeedr7R { bits: self._ospeedr7() }
        }
        fn _ospeedr6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr6(&self) -> Ospeedr6R {
            Ospeedr6R { bits: self._ospeedr6() }
        }
        fn _ospeedr5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr5(&self) -> Ospeedr5R {
            Ospeedr5R { bits: self._ospeedr5() }
        }
        fn _ospeedr4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr4(&self) -> Ospeedr4R {
            Ospeedr4R { bits: self._ospeedr4() }
        }
        fn _ospeedr3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr3(&self) -> Ospeedr3R {
            Ospeedr3R { bits: self._ospeedr3() }
        }
        fn _ospeedr2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr2(&self) -> Ospeedr2R {
            Ospeedr2R { bits: self._ospeedr2() }
        }
        fn _ospeedr1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr1(&self) -> Ospeedr1R {
            Ospeedr1R { bits: self._ospeedr1() }
        }
        fn _ospeedr0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr0(&self) -> Ospeedr0R {
            Ospeedr0R { bits: self._ospeedr0() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 192 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr15(&mut self) -> _Ospeedr15W {
            _Ospeedr15W { register: self }
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr14(&mut self) -> _Ospeedr14W {
            _Ospeedr14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr13(&mut self) -> _Ospeedr13W {
            _Ospeedr13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr12(&mut self) -> _Ospeedr12W {
            _Ospeedr12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr11(&mut self) -> _Ospeedr11W {
            _Ospeedr11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr10(&mut self) -> _Ospeedr10W {
            _Ospeedr10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr9(&mut self) -> _Ospeedr9W {
            _Ospeedr9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr8(&mut self) -> _Ospeedr8W {
            _Ospeedr8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr7(&mut self) -> _Ospeedr7W {
            _Ospeedr7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr6(&mut self) -> _Ospeedr6W {
            _Ospeedr6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr5(&mut self) -> _Ospeedr5W {
            _Ospeedr5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr4(&mut self) -> _Ospeedr4W {
            _Ospeedr4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr3(&mut self) -> _Ospeedr3W {
            _Ospeedr3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr2(&mut self) -> _Ospeedr2W {
            _Ospeedr2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn ospeedr1(&mut self) -> _Ospeedr1W {
            _Ospeedr1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
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
    # [ doc = "Value of the field PUPDR15" ]
    pub struct Pupdr15R {
        bits: u8,
    }
    impl Pupdr15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR14" ]
    pub struct Pupdr14R {
        bits: u8,
    }
    impl Pupdr14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR13" ]
    pub struct Pupdr13R {
        bits: u8,
    }
    impl Pupdr13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR12" ]
    pub struct Pupdr12R {
        bits: u8,
    }
    impl Pupdr12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR11" ]
    pub struct Pupdr11R {
        bits: u8,
    }
    impl Pupdr11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR10" ]
    pub struct Pupdr10R {
        bits: u8,
    }
    impl Pupdr10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR9" ]
    pub struct Pupdr9R {
        bits: u8,
    }
    impl Pupdr9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR8" ]
    pub struct Pupdr8R {
        bits: u8,
    }
    impl Pupdr8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR7" ]
    pub struct Pupdr7R {
        bits: u8,
    }
    impl Pupdr7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR6" ]
    pub struct Pupdr6R {
        bits: u8,
    }
    impl Pupdr6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR5" ]
    pub struct Pupdr5R {
        bits: u8,
    }
    impl Pupdr5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR4" ]
    pub struct Pupdr4R {
        bits: u8,
    }
    impl Pupdr4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR3" ]
    pub struct Pupdr3R {
        bits: u8,
    }
    impl Pupdr3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR2" ]
    pub struct Pupdr2R {
        bits: u8,
    }
    impl Pupdr2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR1" ]
    pub struct Pupdr1R {
        bits: u8,
    }
    impl Pupdr1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PUPDR0" ]
    pub struct Pupdr0R {
        bits: u8,
    }
    impl Pupdr0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pupdr15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pupdr15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr15(&self) -> Pupdr15R {
            Pupdr15R { bits: self._pupdr15() }
        }
        fn _pupdr14(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr14(&self) -> Pupdr14R {
            Pupdr14R { bits: self._pupdr14() }
        }
        fn _pupdr13(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr13(&self) -> Pupdr13R {
            Pupdr13R { bits: self._pupdr13() }
        }
        fn _pupdr12(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr12(&self) -> Pupdr12R {
            Pupdr12R { bits: self._pupdr12() }
        }
        fn _pupdr11(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr11(&self) -> Pupdr11R {
            Pupdr11R { bits: self._pupdr11() }
        }
        fn _pupdr10(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr10(&self) -> Pupdr10R {
            Pupdr10R { bits: self._pupdr10() }
        }
        fn _pupdr9(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr9(&self) -> Pupdr9R {
            Pupdr9R { bits: self._pupdr9() }
        }
        fn _pupdr8(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr8(&self) -> Pupdr8R {
            Pupdr8R { bits: self._pupdr8() }
        }
        fn _pupdr7(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr7(&self) -> Pupdr7R {
            Pupdr7R { bits: self._pupdr7() }
        }
        fn _pupdr6(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr6(&self) -> Pupdr6R {
            Pupdr6R { bits: self._pupdr6() }
        }
        fn _pupdr5(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr5(&self) -> Pupdr5R {
            Pupdr5R { bits: self._pupdr5() }
        }
        fn _pupdr4(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr4(&self) -> Pupdr4R {
            Pupdr4R { bits: self._pupdr4() }
        }
        fn _pupdr3(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr3(&self) -> Pupdr3R {
            Pupdr3R { bits: self._pupdr3() }
        }
        fn _pupdr2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr2(&self) -> Pupdr2R {
            Pupdr2R { bits: self._pupdr2() }
        }
        fn _pupdr1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr1(&self) -> Pupdr1R {
            Pupdr1R { bits: self._pupdr1() }
        }
        fn _pupdr0(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr0(&self) -> Pupdr0R {
            Pupdr0R { bits: self._pupdr0() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 256 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 30:31 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr15(&mut self) -> _Pupdr15W {
            _Pupdr15W { register: self }
        }
        # [ doc = "Bits 28:29 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr14(&mut self) -> _Pupdr14W {
            _Pupdr14W { register: self }
        }
        # [ doc = "Bits 26:27 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr13(&mut self) -> _Pupdr13W {
            _Pupdr13W { register: self }
        }
        # [ doc = "Bits 24:25 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr12(&mut self) -> _Pupdr12W {
            _Pupdr12W { register: self }
        }
        # [ doc = "Bits 22:23 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr11(&mut self) -> _Pupdr11W {
            _Pupdr11W { register: self }
        }
        # [ doc = "Bits 20:21 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr10(&mut self) -> _Pupdr10W {
            _Pupdr10W { register: self }
        }
        # [ doc = "Bits 18:19 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr9(&mut self) -> _Pupdr9W {
            _Pupdr9W { register: self }
        }
        # [ doc = "Bits 16:17 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr8(&mut self) -> _Pupdr8W {
            _Pupdr8W { register: self }
        }
        # [ doc = "Bits 14:15 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr7(&mut self) -> _Pupdr7W {
            _Pupdr7W { register: self }
        }
        # [ doc = "Bits 12:13 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr6(&mut self) -> _Pupdr6W {
            _Pupdr6W { register: self }
        }
        # [ doc = "Bits 10:11 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr5(&mut self) -> _Pupdr5W {
            _Pupdr5W { register: self }
        }
        # [ doc = "Bits 8:9 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr4(&mut self) -> _Pupdr4W {
            _Pupdr4W { register: self }
        }
        # [ doc = "Bits 6:7 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr3(&mut self) -> _Pupdr3W {
            _Pupdr3W { register: self }
        }
        # [ doc = "Bits 4:5 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr2(&mut self) -> _Pupdr2W {
            _Pupdr2W { register: self }
        }
        # [ doc = "Bits 2:3 - Port x configuration bits (y = 0..15)" ]
        pub fn pupdr1(&mut self) -> _Pupdr1W {
            _Pupdr1W { register: self }
        }
        # [ doc = "Bits 0:1 - Port x configuration bits (y = 0..15)" ]
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
        # [ doc = "Bit 15 - Port input data (y = 0..15)" ]
        pub fn idr15(&self) -> Idr15R {
            Idr15R { bits: self._idr15() }
        }
        fn _idr14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port input data (y = 0..15)" ]
        pub fn idr14(&self) -> Idr14R {
            Idr14R { bits: self._idr14() }
        }
        fn _idr13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port input data (y = 0..15)" ]
        pub fn idr13(&self) -> Idr13R {
            Idr13R { bits: self._idr13() }
        }
        fn _idr12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port input data (y = 0..15)" ]
        pub fn idr12(&self) -> Idr12R {
            Idr12R { bits: self._idr12() }
        }
        fn _idr11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port input data (y = 0..15)" ]
        pub fn idr11(&self) -> Idr11R {
            Idr11R { bits: self._idr11() }
        }
        fn _idr10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port input data (y = 0..15)" ]
        pub fn idr10(&self) -> Idr10R {
            Idr10R { bits: self._idr10() }
        }
        fn _idr9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port input data (y = 0..15)" ]
        pub fn idr9(&self) -> Idr9R {
            Idr9R { bits: self._idr9() }
        }
        fn _idr8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port input data (y = 0..15)" ]
        pub fn idr8(&self) -> Idr8R {
            Idr8R { bits: self._idr8() }
        }
        fn _idr7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port input data (y = 0..15)" ]
        pub fn idr7(&self) -> Idr7R {
            Idr7R { bits: self._idr7() }
        }
        fn _idr6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port input data (y = 0..15)" ]
        pub fn idr6(&self) -> Idr6R {
            Idr6R { bits: self._idr6() }
        }
        fn _idr5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port input data (y = 0..15)" ]
        pub fn idr5(&self) -> Idr5R {
            Idr5R { bits: self._idr5() }
        }
        fn _idr4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port input data (y = 0..15)" ]
        pub fn idr4(&self) -> Idr4R {
            Idr4R { bits: self._idr4() }
        }
        fn _idr3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port input data (y = 0..15)" ]
        pub fn idr3(&self) -> Idr3R {
            Idr3R { bits: self._idr3() }
        }
        fn _idr2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port input data (y = 0..15)" ]
        pub fn idr2(&self) -> Idr2R {
            Idr2R { bits: self._idr2() }
        }
        fn _idr1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port input data (y = 0..15)" ]
        pub fn idr1(&self) -> Idr1R {
            Idr1R { bits: self._idr1() }
        }
        fn _idr0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port input data (y = 0..15)" ]
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
        # [ doc = "Bit 15 - Port output data (y = 0..15)" ]
        pub fn odr15(&self) -> Odr15R {
            Odr15R { bits: self._odr15() }
        }
        fn _odr14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port output data (y = 0..15)" ]
        pub fn odr14(&self) -> Odr14R {
            Odr14R { bits: self._odr14() }
        }
        fn _odr13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port output data (y = 0..15)" ]
        pub fn odr13(&self) -> Odr13R {
            Odr13R { bits: self._odr13() }
        }
        fn _odr12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port output data (y = 0..15)" ]
        pub fn odr12(&self) -> Odr12R {
            Odr12R { bits: self._odr12() }
        }
        fn _odr11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port output data (y = 0..15)" ]
        pub fn odr11(&self) -> Odr11R {
            Odr11R { bits: self._odr11() }
        }
        fn _odr10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port output data (y = 0..15)" ]
        pub fn odr10(&self) -> Odr10R {
            Odr10R { bits: self._odr10() }
        }
        fn _odr9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port output data (y = 0..15)" ]
        pub fn odr9(&self) -> Odr9R {
            Odr9R { bits: self._odr9() }
        }
        fn _odr8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port output data (y = 0..15)" ]
        pub fn odr8(&self) -> Odr8R {
            Odr8R { bits: self._odr8() }
        }
        fn _odr7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port output data (y = 0..15)" ]
        pub fn odr7(&self) -> Odr7R {
            Odr7R { bits: self._odr7() }
        }
        fn _odr6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port output data (y = 0..15)" ]
        pub fn odr6(&self) -> Odr6R {
            Odr6R { bits: self._odr6() }
        }
        fn _odr5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port output data (y = 0..15)" ]
        pub fn odr5(&self) -> Odr5R {
            Odr5R { bits: self._odr5() }
        }
        fn _odr4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port output data (y = 0..15)" ]
        pub fn odr4(&self) -> Odr4R {
            Odr4R { bits: self._odr4() }
        }
        fn _odr3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port output data (y = 0..15)" ]
        pub fn odr3(&self) -> Odr3R {
            Odr3R { bits: self._odr3() }
        }
        fn _odr2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port output data (y = 0..15)" ]
        pub fn odr2(&self) -> Odr2R {
            Odr2R { bits: self._odr2() }
        }
        fn _odr1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port output data (y = 0..15)" ]
        pub fn odr1(&self) -> Odr1R {
            Odr1R { bits: self._odr1() }
        }
        fn _odr0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port output data (y = 0..15)" ]
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
        # [ doc = "Bit 15 - Port output data (y = 0..15)" ]
        pub fn odr15(&mut self) -> _Odr15W {
            _Odr15W { register: self }
        }
        # [ doc = "Bit 14 - Port output data (y = 0..15)" ]
        pub fn odr14(&mut self) -> _Odr14W {
            _Odr14W { register: self }
        }
        # [ doc = "Bit 13 - Port output data (y = 0..15)" ]
        pub fn odr13(&mut self) -> _Odr13W {
            _Odr13W { register: self }
        }
        # [ doc = "Bit 12 - Port output data (y = 0..15)" ]
        pub fn odr12(&mut self) -> _Odr12W {
            _Odr12W { register: self }
        }
        # [ doc = "Bit 11 - Port output data (y = 0..15)" ]
        pub fn odr11(&mut self) -> _Odr11W {
            _Odr11W { register: self }
        }
        # [ doc = "Bit 10 - Port output data (y = 0..15)" ]
        pub fn odr10(&mut self) -> _Odr10W {
            _Odr10W { register: self }
        }
        # [ doc = "Bit 9 - Port output data (y = 0..15)" ]
        pub fn odr9(&mut self) -> _Odr9W {
            _Odr9W { register: self }
        }
        # [ doc = "Bit 8 - Port output data (y = 0..15)" ]
        pub fn odr8(&mut self) -> _Odr8W {
            _Odr8W { register: self }
        }
        # [ doc = "Bit 7 - Port output data (y = 0..15)" ]
        pub fn odr7(&mut self) -> _Odr7W {
            _Odr7W { register: self }
        }
        # [ doc = "Bit 6 - Port output data (y = 0..15)" ]
        pub fn odr6(&mut self) -> _Odr6W {
            _Odr6W { register: self }
        }
        # [ doc = "Bit 5 - Port output data (y = 0..15)" ]
        pub fn odr5(&mut self) -> _Odr5W {
            _Odr5W { register: self }
        }
        # [ doc = "Bit 4 - Port output data (y = 0..15)" ]
        pub fn odr4(&mut self) -> _Odr4W {
            _Odr4W { register: self }
        }
        # [ doc = "Bit 3 - Port output data (y = 0..15)" ]
        pub fn odr3(&mut self) -> _Odr3W {
            _Odr3W { register: self }
        }
        # [ doc = "Bit 2 - Port output data (y = 0..15)" ]
        pub fn odr2(&mut self) -> _Odr2W {
            _Odr2W { register: self }
        }
        # [ doc = "Bit 1 - Port output data (y = 0..15)" ]
        pub fn odr1(&mut self) -> _Odr1W {
            _Odr1W { register: self }
        }
        # [ doc = "Bit 0 - Port output data (y = 0..15)" ]
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
    impl<'a> _Br15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
    pub struct _Bs14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs14W<'a> {
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
    pub struct _Bs13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs13W<'a> {
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
    pub struct _Bs12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs12W<'a> {
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
    pub struct _Bs11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs11W<'a> {
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
    pub struct _Bs10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs10W<'a> {
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
    pub struct _Bs9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs9W<'a> {
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
    pub struct _Bs8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs8W<'a> {
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
    pub struct _Bs7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs7W<'a> {
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
    pub struct _Bs6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs6W<'a> {
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
    pub struct _Bs5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs5W<'a> {
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
    pub struct _Bs4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs4W<'a> {
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
    pub struct _Bs3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs3W<'a> {
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
    pub struct _Bs2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs2W<'a> {
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
    pub struct _Bs1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs1W<'a> {
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
    pub struct _Bs0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Bs0W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = "Bit 31 - Port x reset bit y (y = 0..15)" ]
        pub fn br15(&mut self) -> _Br15W {
            _Br15W { register: self }
        }
        # [ doc = "Bit 30 - Port x reset bit y (y = 0..15)" ]
        pub fn br14(&mut self) -> _Br14W {
            _Br14W { register: self }
        }
        # [ doc = "Bit 29 - Port x reset bit y (y = 0..15)" ]
        pub fn br13(&mut self) -> _Br13W {
            _Br13W { register: self }
        }
        # [ doc = "Bit 28 - Port x reset bit y (y = 0..15)" ]
        pub fn br12(&mut self) -> _Br12W {
            _Br12W { register: self }
        }
        # [ doc = "Bit 27 - Port x reset bit y (y = 0..15)" ]
        pub fn br11(&mut self) -> _Br11W {
            _Br11W { register: self }
        }
        # [ doc = "Bit 26 - Port x reset bit y (y = 0..15)" ]
        pub fn br10(&mut self) -> _Br10W {
            _Br10W { register: self }
        }
        # [ doc = "Bit 25 - Port x reset bit y (y = 0..15)" ]
        pub fn br9(&mut self) -> _Br9W {
            _Br9W { register: self }
        }
        # [ doc = "Bit 24 - Port x reset bit y (y = 0..15)" ]
        pub fn br8(&mut self) -> _Br8W {
            _Br8W { register: self }
        }
        # [ doc = "Bit 23 - Port x reset bit y (y = 0..15)" ]
        pub fn br7(&mut self) -> _Br7W {
            _Br7W { register: self }
        }
        # [ doc = "Bit 22 - Port x reset bit y (y = 0..15)" ]
        pub fn br6(&mut self) -> _Br6W {
            _Br6W { register: self }
        }
        # [ doc = "Bit 21 - Port x reset bit y (y = 0..15)" ]
        pub fn br5(&mut self) -> _Br5W {
            _Br5W { register: self }
        }
        # [ doc = "Bit 20 - Port x reset bit y (y = 0..15)" ]
        pub fn br4(&mut self) -> _Br4W {
            _Br4W { register: self }
        }
        # [ doc = "Bit 19 - Port x reset bit y (y = 0..15)" ]
        pub fn br3(&mut self) -> _Br3W {
            _Br3W { register: self }
        }
        # [ doc = "Bit 18 - Port x reset bit y (y = 0..15)" ]
        pub fn br2(&mut self) -> _Br2W {
            _Br2W { register: self }
        }
        # [ doc = "Bit 17 - Port x reset bit y (y = 0..15)" ]
        pub fn br1(&mut self) -> _Br1W {
            _Br1W { register: self }
        }
        # [ doc = "Bit 16 - Port x set bit y (y= 0..15)" ]
        pub fn br0(&mut self) -> _Br0W {
            _Br0W { register: self }
        }
        # [ doc = "Bit 15 - Port x set bit y (y= 0..15)" ]
        pub fn bs15(&mut self) -> _Bs15W {
            _Bs15W { register: self }
        }
        # [ doc = "Bit 14 - Port x set bit y (y= 0..15)" ]
        pub fn bs14(&mut self) -> _Bs14W {
            _Bs14W { register: self }
        }
        # [ doc = "Bit 13 - Port x set bit y (y= 0..15)" ]
        pub fn bs13(&mut self) -> _Bs13W {
            _Bs13W { register: self }
        }
        # [ doc = "Bit 12 - Port x set bit y (y= 0..15)" ]
        pub fn bs12(&mut self) -> _Bs12W {
            _Bs12W { register: self }
        }
        # [ doc = "Bit 11 - Port x set bit y (y= 0..15)" ]
        pub fn bs11(&mut self) -> _Bs11W {
            _Bs11W { register: self }
        }
        # [ doc = "Bit 10 - Port x set bit y (y= 0..15)" ]
        pub fn bs10(&mut self) -> _Bs10W {
            _Bs10W { register: self }
        }
        # [ doc = "Bit 9 - Port x set bit y (y= 0..15)" ]
        pub fn bs9(&mut self) -> _Bs9W {
            _Bs9W { register: self }
        }
        # [ doc = "Bit 8 - Port x set bit y (y= 0..15)" ]
        pub fn bs8(&mut self) -> _Bs8W {
            _Bs8W { register: self }
        }
        # [ doc = "Bit 7 - Port x set bit y (y= 0..15)" ]
        pub fn bs7(&mut self) -> _Bs7W {
            _Bs7W { register: self }
        }
        # [ doc = "Bit 6 - Port x set bit y (y= 0..15)" ]
        pub fn bs6(&mut self) -> _Bs6W {
            _Bs6W { register: self }
        }
        # [ doc = "Bit 5 - Port x set bit y (y= 0..15)" ]
        pub fn bs5(&mut self) -> _Bs5W {
            _Bs5W { register: self }
        }
        # [ doc = "Bit 4 - Port x set bit y (y= 0..15)" ]
        pub fn bs4(&mut self) -> _Bs4W {
            _Bs4W { register: self }
        }
        # [ doc = "Bit 3 - Port x set bit y (y= 0..15)" ]
        pub fn bs3(&mut self) -> _Bs3W {
            _Bs3W { register: self }
        }
        # [ doc = "Bit 2 - Port x set bit y (y= 0..15)" ]
        pub fn bs2(&mut self) -> _Bs2W {
            _Bs2W { register: self }
        }
        # [ doc = "Bit 1 - Port x set bit y (y= 0..15)" ]
        pub fn bs1(&mut self) -> _Bs1W {
            _Bs1W { register: self }
        }
        # [ doc = "Bit 0 - Port x set bit y (y= 0..15)" ]
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
    # [ doc = "Value of the field LCKK" ]
    pub struct LckkR {
        bits: u8,
    }
    impl LckkR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK15" ]
    pub struct Lck15R {
        bits: u8,
    }
    impl Lck15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK14" ]
    pub struct Lck14R {
        bits: u8,
    }
    impl Lck14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK13" ]
    pub struct Lck13R {
        bits: u8,
    }
    impl Lck13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK12" ]
    pub struct Lck12R {
        bits: u8,
    }
    impl Lck12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK11" ]
    pub struct Lck11R {
        bits: u8,
    }
    impl Lck11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK10" ]
    pub struct Lck10R {
        bits: u8,
    }
    impl Lck10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK9" ]
    pub struct Lck9R {
        bits: u8,
    }
    impl Lck9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK8" ]
    pub struct Lck8R {
        bits: u8,
    }
    impl Lck8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK7" ]
    pub struct Lck7R {
        bits: u8,
    }
    impl Lck7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK6" ]
    pub struct Lck6R {
        bits: u8,
    }
    impl Lck6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK5" ]
    pub struct Lck5R {
        bits: u8,
    }
    impl Lck5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK4" ]
    pub struct Lck4R {
        bits: u8,
    }
    impl Lck4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK3" ]
    pub struct Lck3R {
        bits: u8,
    }
    impl Lck3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK2" ]
    pub struct Lck2R {
        bits: u8,
    }
    impl Lck2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK1" ]
    pub struct Lck1R {
        bits: u8,
    }
    impl Lck1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LCK0" ]
    pub struct Lck0R {
        bits: u8,
    }
    impl Lck0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LckkW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LckkW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
    impl<'a> _Lck15W<'a> {
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
    pub struct _Lck14W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck14W<'a> {
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
    pub struct _Lck13W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck13W<'a> {
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
    pub struct _Lck12W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck12W<'a> {
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
    pub struct _Lck11W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck11W<'a> {
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
    pub struct _Lck10W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck10W<'a> {
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
    pub struct _Lck9W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck9W<'a> {
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
    pub struct _Lck8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck8W<'a> {
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
    pub struct _Lck7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck7W<'a> {
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
    pub struct _Lck6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck6W<'a> {
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
    pub struct _Lck5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck5W<'a> {
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
    pub struct _Lck4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck4W<'a> {
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
    pub struct _Lck3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck3W<'a> {
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
    pub struct _Lck2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck2W<'a> {
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
    pub struct _Lck1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck1W<'a> {
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
    pub struct _Lck0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Lck0W<'a> {
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
        fn _lckk(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Port x lock bit y (y= 0..15)" ]
        pub fn lckk(&self) -> LckkR {
            LckkR { bits: self._lckk() }
        }
        fn _lck15(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Port x lock bit y (y= 0..15)" ]
        pub fn lck15(&self) -> Lck15R {
            Lck15R { bits: self._lck15() }
        }
        fn _lck14(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Port x lock bit y (y= 0..15)" ]
        pub fn lck14(&self) -> Lck14R {
            Lck14R { bits: self._lck14() }
        }
        fn _lck13(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Port x lock bit y (y= 0..15)" ]
        pub fn lck13(&self) -> Lck13R {
            Lck13R { bits: self._lck13() }
        }
        fn _lck12(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Port x lock bit y (y= 0..15)" ]
        pub fn lck12(&self) -> Lck12R {
            Lck12R { bits: self._lck12() }
        }
        fn _lck11(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Port x lock bit y (y= 0..15)" ]
        pub fn lck11(&self) -> Lck11R {
            Lck11R { bits: self._lck11() }
        }
        fn _lck10(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Port x lock bit y (y= 0..15)" ]
        pub fn lck10(&self) -> Lck10R {
            Lck10R { bits: self._lck10() }
        }
        fn _lck9(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Port x lock bit y (y= 0..15)" ]
        pub fn lck9(&self) -> Lck9R {
            Lck9R { bits: self._lck9() }
        }
        fn _lck8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Port x lock bit y (y= 0..15)" ]
        pub fn lck8(&self) -> Lck8R {
            Lck8R { bits: self._lck8() }
        }
        fn _lck7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Port x lock bit y (y= 0..15)" ]
        pub fn lck7(&self) -> Lck7R {
            Lck7R { bits: self._lck7() }
        }
        fn _lck6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Port x lock bit y (y= 0..15)" ]
        pub fn lck6(&self) -> Lck6R {
            Lck6R { bits: self._lck6() }
        }
        fn _lck5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Port x lock bit y (y= 0..15)" ]
        pub fn lck5(&self) -> Lck5R {
            Lck5R { bits: self._lck5() }
        }
        fn _lck4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Port x lock bit y (y= 0..15)" ]
        pub fn lck4(&self) -> Lck4R {
            Lck4R { bits: self._lck4() }
        }
        fn _lck3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Port x lock bit y (y= 0..15)" ]
        pub fn lck3(&self) -> Lck3R {
            Lck3R { bits: self._lck3() }
        }
        fn _lck2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Port x lock bit y (y= 0..15)" ]
        pub fn lck2(&self) -> Lck2R {
            Lck2R { bits: self._lck2() }
        }
        fn _lck1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Port x lock bit y (y= 0..15)" ]
        pub fn lck1(&self) -> Lck1R {
            Lck1R { bits: self._lck1() }
        }
        fn _lck0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Port x lock bit y (y= 0..15)" ]
        pub fn lck0(&self) -> Lck0R {
            Lck0R { bits: self._lck0() }
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
        # [ doc = "Bit 16 - Port x lock bit y (y= 0..15)" ]
        pub fn lckk(&mut self) -> _LckkW {
            _LckkW { register: self }
        }
        # [ doc = "Bit 15 - Port x lock bit y (y= 0..15)" ]
        pub fn lck15(&mut self) -> _Lck15W {
            _Lck15W { register: self }
        }
        # [ doc = "Bit 14 - Port x lock bit y (y= 0..15)" ]
        pub fn lck14(&mut self) -> _Lck14W {
            _Lck14W { register: self }
        }
        # [ doc = "Bit 13 - Port x lock bit y (y= 0..15)" ]
        pub fn lck13(&mut self) -> _Lck13W {
            _Lck13W { register: self }
        }
        # [ doc = "Bit 12 - Port x lock bit y (y= 0..15)" ]
        pub fn lck12(&mut self) -> _Lck12W {
            _Lck12W { register: self }
        }
        # [ doc = "Bit 11 - Port x lock bit y (y= 0..15)" ]
        pub fn lck11(&mut self) -> _Lck11W {
            _Lck11W { register: self }
        }
        # [ doc = "Bit 10 - Port x lock bit y (y= 0..15)" ]
        pub fn lck10(&mut self) -> _Lck10W {
            _Lck10W { register: self }
        }
        # [ doc = "Bit 9 - Port x lock bit y (y= 0..15)" ]
        pub fn lck9(&mut self) -> _Lck9W {
            _Lck9W { register: self }
        }
        # [ doc = "Bit 8 - Port x lock bit y (y= 0..15)" ]
        pub fn lck8(&mut self) -> _Lck8W {
            _Lck8W { register: self }
        }
        # [ doc = "Bit 7 - Port x lock bit y (y= 0..15)" ]
        pub fn lck7(&mut self) -> _Lck7W {
            _Lck7W { register: self }
        }
        # [ doc = "Bit 6 - Port x lock bit y (y= 0..15)" ]
        pub fn lck6(&mut self) -> _Lck6W {
            _Lck6W { register: self }
        }
        # [ doc = "Bit 5 - Port x lock bit y (y= 0..15)" ]
        pub fn lck5(&mut self) -> _Lck5W {
            _Lck5W { register: self }
        }
        # [ doc = "Bit 4 - Port x lock bit y (y= 0..15)" ]
        pub fn lck4(&mut self) -> _Lck4W {
            _Lck4W { register: self }
        }
        # [ doc = "Bit 3 - Port x lock bit y (y= 0..15)" ]
        pub fn lck3(&mut self) -> _Lck3W {
            _Lck3W { register: self }
        }
        # [ doc = "Bit 2 - Port x lock bit y (y= 0..15)" ]
        pub fn lck2(&mut self) -> _Lck2W {
            _Lck2W { register: self }
        }
        # [ doc = "Bit 1 - Port x lock bit y (y= 0..15)" ]
        pub fn lck1(&mut self) -> _Lck1W {
            _Lck1W { register: self }
        }
        # [ doc = "Bit 0 - Port x lock bit y (y= 0..15)" ]
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
    # [ doc = "Value of the field AFRL7" ]
    pub struct Afrl7R {
        bits: u8,
    }
    impl Afrl7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL6" ]
    pub struct Afrl6R {
        bits: u8,
    }
    impl Afrl6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL5" ]
    pub struct Afrl5R {
        bits: u8,
    }
    impl Afrl5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL4" ]
    pub struct Afrl4R {
        bits: u8,
    }
    impl Afrl4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL3" ]
    pub struct Afrl3R {
        bits: u8,
    }
    impl Afrl3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL2" ]
    pub struct Afrl2R {
        bits: u8,
    }
    impl Afrl2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL1" ]
    pub struct Afrl1R {
        bits: u8,
    }
    impl Afrl1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRL0" ]
    pub struct Afrl0R {
        bits: u8,
    }
    impl Afrl0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrl7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrl7W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl7(&self) -> Afrl7R {
            Afrl7R { bits: self._afrl7() }
        }
        fn _afrl6(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl6(&self) -> Afrl6R {
            Afrl6R { bits: self._afrl6() }
        }
        fn _afrl5(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl5(&self) -> Afrl5R {
            Afrl5R { bits: self._afrl5() }
        }
        fn _afrl4(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl4(&self) -> Afrl4R {
            Afrl4R { bits: self._afrl4() }
        }
        fn _afrl3(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl3(&self) -> Afrl3R {
            Afrl3R { bits: self._afrl3() }
        }
        fn _afrl2(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl2(&self) -> Afrl2R {
            Afrl2R { bits: self._afrl2() }
        }
        fn _afrl1(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl1(&self) -> Afrl1R {
            Afrl1R { bits: self._afrl1() }
        }
        fn _afrl0(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl0(&self) -> Afrl0R {
            Afrl0R { bits: self._afrl0() }
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
        # [ doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl7(&mut self) -> _Afrl7W {
            _Afrl7W { register: self }
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl6(&mut self) -> _Afrl6W {
            _Afrl6W { register: self }
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl5(&mut self) -> _Afrl5W {
            _Afrl5W { register: self }
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl4(&mut self) -> _Afrl4W {
            _Afrl4W { register: self }
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl3(&mut self) -> _Afrl3W {
            _Afrl3W { register: self }
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl2(&mut self) -> _Afrl2W {
            _Afrl2W { register: self }
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)" ]
        pub fn afrl1(&mut self) -> _Afrl1W {
            _Afrl1W { register: self }
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)" ]
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
    # [ doc = "Value of the field AFRH15" ]
    pub struct Afrh15R {
        bits: u8,
    }
    impl Afrh15R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH14" ]
    pub struct Afrh14R {
        bits: u8,
    }
    impl Afrh14R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH13" ]
    pub struct Afrh13R {
        bits: u8,
    }
    impl Afrh13R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH12" ]
    pub struct Afrh12R {
        bits: u8,
    }
    impl Afrh12R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH11" ]
    pub struct Afrh11R {
        bits: u8,
    }
    impl Afrh11R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH10" ]
    pub struct Afrh10R {
        bits: u8,
    }
    impl Afrh10R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH9" ]
    pub struct Afrh9R {
        bits: u8,
    }
    impl Afrh9R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field AFRH8" ]
    pub struct Afrh8R {
        bits: u8,
    }
    impl Afrh8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Afrh15W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Afrh15W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
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
        # [ doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh15(&self) -> Afrh15R {
            Afrh15R { bits: self._afrh15() }
        }
        fn _afrh14(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh14(&self) -> Afrh14R {
            Afrh14R { bits: self._afrh14() }
        }
        fn _afrh13(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh13(&self) -> Afrh13R {
            Afrh13R { bits: self._afrh13() }
        }
        fn _afrh12(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh12(&self) -> Afrh12R {
            Afrh12R { bits: self._afrh12() }
        }
        fn _afrh11(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh11(&self) -> Afrh11R {
            Afrh11R { bits: self._afrh11() }
        }
        fn _afrh10(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh10(&self) -> Afrh10R {
            Afrh10R { bits: self._afrh10() }
        }
        fn _afrh9(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh9(&self) -> Afrh9R {
            Afrh9R { bits: self._afrh9() }
        }
        fn _afrh8(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh8(&self) -> Afrh8R {
            Afrh8R { bits: self._afrh8() }
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
        # [ doc = "Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh15(&mut self) -> _Afrh15W {
            _Afrh15W { register: self }
        }
        # [ doc = "Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh14(&mut self) -> _Afrh14W {
            _Afrh14W { register: self }
        }
        # [ doc = "Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh13(&mut self) -> _Afrh13W {
            _Afrh13W { register: self }
        }
        # [ doc = "Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh12(&mut self) -> _Afrh12W {
            _Afrh12W { register: self }
        }
        # [ doc = "Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh11(&mut self) -> _Afrh11W {
            _Afrh11W { register: self }
        }
        # [ doc = "Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh10(&mut self) -> _Afrh10W {
            _Afrh10W { register: self }
        }
        # [ doc = "Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh9(&mut self) -> _Afrh9W {
            _Afrh9W { register: self }
        }
        # [ doc = "Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)" ]
        pub fn afrh8(&mut self) -> _Afrh8W {
            _Afrh8W { register: self }
        }
    }
}
