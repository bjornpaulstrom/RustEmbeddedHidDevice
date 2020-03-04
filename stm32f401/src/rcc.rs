# ! [ doc = "Reset and clock control" ]# [ doc = r" Register block" ]
# [ repr ( C ) ]
pub struct Rcc {
    # [ doc = "0x00 - clock control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - PLL configuration register" ]
    pub pllcfgr: Pllcfgr,
    # [ doc = "0x08 - clock configuration register" ]
    pub cfgr: Cfgr,
    # [ doc = "0x0c - clock interrupt register" ]
    pub cir: Cir,
    # [ doc = "0x10 - AHB1 peripheral reset register" ]
    pub ahb1rstr: Ahb1rstr,
    # [ doc = "0x14 - AHB2 peripheral reset register" ]
    pub ahb2rstr: Ahb2rstr,
    # [ doc = "0x18 - AHB3 peripheral reset register" ]
    pub ahb3rstr: Ahb3rstr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x20 - APB1 peripheral reset register" ]
    pub apb1rstr: Apb1rstr,
    # [ doc = "0x24 - APB2 peripheral reset register" ]
    pub apb2rstr: Apb2rstr,
    _reserved1: [u8; 8usize],
    # [ doc = "0x30 - AHB1 peripheral clock register" ]
    pub ahb1enr: Ahb1enr,
    # [ doc = "0x34 - AHB2 peripheral clock enable register" ]
    pub ahb2enr: Ahb2enr,
    # [ doc = "0x38 - AHB3 peripheral clock enable register" ]
    pub ahb3enr: Ahb3enr,
    _reserved2: [u8; 4usize],
    # [ doc = "0x40 - APB1 peripheral clock enable register" ]
    pub apb1enr: Apb1enr,
    # [ doc = "0x44 - APB2 peripheral clock enable register" ]
    pub apb2enr: Apb2enr,
    _reserved3: [u8; 8usize],
    # [ doc = "0x50 - AHB1 peripheral clock enable in low power mode register" ]
    pub ahb1lpenr: Ahb1lpenr,
    # [ doc = "0x54 - AHB2 peripheral clock enable in low power mode register" ]
    pub ahb2lpenr: Ahb2lpenr,
    # [ doc = "0x58 - AHB3 peripheral clock enable in low power mode register" ]
    pub ahb3lpenr: Ahb3lpenr,
    _reserved4: [u8; 4usize],
    # [ doc = "0x60 - APB1 peripheral clock enable in low power mode register" ]
    pub apb1lpenr: Apb1lpenr,
    # [ doc = "0x64 - APB2 peripheral clock enabled in low power mode register" ]
    pub apb2lpenr: Apb2lpenr,
    _reserved5: [u8; 8usize],
    # [ doc = "0x70 - Backup domain control register" ]
    pub bdcr: Bdcr,
    # [ doc = "0x74 - clock control & status register" ]
    pub csr: Csr,
    _reserved6: [u8; 8usize],
    # [ doc = "0x80 - spread spectrum clock generation register" ]
    pub sscgr: Sscgr,
    # [ doc = "0x84 - PLLI2S configuration register" ]
    pub plli2scfgr: Plli2scfgr,
}

# [ doc = "clock control register" ]
# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "clock control register" ]
pub mod cr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cr {
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
    # [ doc = "Value of the field PLLI2SRDY" ]
    pub struct Plli2srdyR {
        bits: u8,
    }
    impl Plli2srdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLI2SON" ]
    pub struct Plli2sonR {
        bits: u8,
    }
    impl Plli2sonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDY" ]
    pub struct PllrdyR {
        bits: u8,
    }
    impl PllrdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLON" ]
    pub struct PllonR {
        bits: u8,
    }
    impl PllonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CSSON" ]
    pub struct CssonR {
        bits: u8,
    }
    impl CssonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSEBYP" ]
    pub struct HsebypR {
        bits: u8,
    }
    impl HsebypR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDY" ]
    pub struct HserdyR {
        bits: u8,
    }
    impl HserdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSEON" ]
    pub struct HseonR {
        bits: u8,
    }
    impl HseonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSICAL" ]
    pub struct HsicalR {
        bits: u8,
    }
    impl HsicalR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSITRIM" ]
    pub struct HsitrimR {
        bits: u8,
    }
    impl HsitrimR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDY" ]
    pub struct HsirdyR {
        bits: u8,
    }
    impl HsirdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSION" ]
    pub struct HsionR {
        bits: u8,
    }
    impl HsionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Plli2sonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plli2sonW<'a> {
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
    pub struct _PllonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllonW<'a> {
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
    pub struct _CssonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CssonW<'a> {
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
    pub struct _HsebypW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsebypW<'a> {
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
    pub struct _HseonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HseonW<'a> {
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
    pub struct _HsitrimW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsitrimW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HsionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsionW<'a> {
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
        fn _plli2srdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - PLLI2S clock ready flag" ]
        pub fn plli2srdy(&self) -> Plli2srdyR {
            Plli2srdyR { bits: self._plli2srdy() }
        }
        fn _plli2son(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - PLLI2S enable" ]
        pub fn plli2son(&self) -> Plli2sonR {
            Plli2sonR { bits: self._plli2son() }
        }
        fn _pllrdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Main PLL (PLL) clock ready flag" ]
        pub fn pllrdy(&self) -> PllrdyR {
            PllrdyR { bits: self._pllrdy() }
        }
        fn _pllon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Main PLL (PLL) enable" ]
        pub fn pllon(&self) -> PllonR {
            PllonR { bits: self._pllon() }
        }
        fn _csson(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - Clock security system enable" ]
        pub fn csson(&self) -> CssonR {
            CssonR { bits: self._csson() }
        }
        fn _hsebyp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - HSE clock bypass" ]
        pub fn hsebyp(&self) -> HsebypR {
            HsebypR { bits: self._hsebyp() }
        }
        fn _hserdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - HSE clock ready flag" ]
        pub fn hserdy(&self) -> HserdyR {
            HserdyR { bits: self._hserdy() }
        }
        fn _hseon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - HSE clock enable" ]
        pub fn hseon(&self) -> HseonR {
            HseonR { bits: self._hseon() }
        }
        fn _hsical(&self) -> u8 {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 8:15 - Internal high-speed clock calibration" ]
        pub fn hsical(&self) -> HsicalR {
            HsicalR { bits: self._hsical() }
        }
        fn _hsitrim(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 3:7 - Internal high-speed clock trimming" ]
        pub fn hsitrim(&self) -> HsitrimR {
            HsitrimR { bits: self._hsitrim() }
        }
        fn _hsirdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Internal high-speed clock ready flag" ]
        pub fn hsirdy(&self) -> HsirdyR {
            HsirdyR { bits: self._hsirdy() }
        }
        fn _hsion(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Internal high-speed clock enable" ]
        pub fn hsion(&self) -> HsionR {
            HsionR { bits: self._hsion() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 131 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 26 - PLLI2S enable" ]
        pub fn plli2son(&mut self) -> _Plli2sonW {
            _Plli2sonW { register: self }
        }
        # [ doc = "Bit 24 - Main PLL (PLL) enable" ]
        pub fn pllon(&mut self) -> _PllonW {
            _PllonW { register: self }
        }
        # [ doc = "Bit 19 - Clock security system enable" ]
        pub fn csson(&mut self) -> _CssonW {
            _CssonW { register: self }
        }
        # [ doc = "Bit 18 - HSE clock bypass" ]
        pub fn hsebyp(&mut self) -> _HsebypW {
            _HsebypW { register: self }
        }
        # [ doc = "Bit 16 - HSE clock enable" ]
        pub fn hseon(&mut self) -> _HseonW {
            _HseonW { register: self }
        }
        # [ doc = "Bits 3:7 - Internal high-speed clock trimming" ]
        pub fn hsitrim(&mut self) -> _HsitrimW {
            _HsitrimW { register: self }
        }
        # [ doc = "Bit 0 - Internal high-speed clock enable" ]
        pub fn hsion(&mut self) -> _HsionW {
            _HsionW { register: self }
        }
    }
}

# [ doc = "PLL configuration register" ]
# [ repr ( C ) ]
pub struct Pllcfgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "PLL configuration register" ]
pub mod pllcfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Pllcfgr {
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
    # [ doc = "Value of the field PLLQ3" ]
    pub struct Pllq3R {
        bits: u8,
    }
    impl Pllq3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLQ2" ]
    pub struct Pllq2R {
        bits: u8,
    }
    impl Pllq2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLQ1" ]
    pub struct Pllq1R {
        bits: u8,
    }
    impl Pllq1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLQ0" ]
    pub struct Pllq0R {
        bits: u8,
    }
    impl Pllq0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLSRC" ]
    pub struct PllsrcR {
        bits: u8,
    }
    impl PllsrcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLP1" ]
    pub struct Pllp1R {
        bits: u8,
    }
    impl Pllp1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLP0" ]
    pub struct Pllp0R {
        bits: u8,
    }
    impl Pllp0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN8" ]
    pub struct Plln8R {
        bits: u8,
    }
    impl Plln8R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN7" ]
    pub struct Plln7R {
        bits: u8,
    }
    impl Plln7R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN6" ]
    pub struct Plln6R {
        bits: u8,
    }
    impl Plln6R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN5" ]
    pub struct Plln5R {
        bits: u8,
    }
    impl Plln5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN4" ]
    pub struct Plln4R {
        bits: u8,
    }
    impl Plln4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN3" ]
    pub struct Plln3R {
        bits: u8,
    }
    impl Plln3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN2" ]
    pub struct Plln2R {
        bits: u8,
    }
    impl Plln2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN1" ]
    pub struct Plln1R {
        bits: u8,
    }
    impl Plln1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLN0" ]
    pub struct Plln0R {
        bits: u8,
    }
    impl Plln0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM5" ]
    pub struct Pllm5R {
        bits: u8,
    }
    impl Pllm5R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM4" ]
    pub struct Pllm4R {
        bits: u8,
    }
    impl Pllm4R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM3" ]
    pub struct Pllm3R {
        bits: u8,
    }
    impl Pllm3R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM2" ]
    pub struct Pllm2R {
        bits: u8,
    }
    impl Pllm2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM1" ]
    pub struct Pllm1R {
        bits: u8,
    }
    impl Pllm1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLM0" ]
    pub struct Pllm0R {
        bits: u8,
    }
    impl Pllm0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Pllq3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllq3W<'a> {
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
    pub struct _Pllq2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllq2W<'a> {
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
    pub struct _Pllq1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllq1W<'a> {
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
    pub struct _Pllq0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllq0W<'a> {
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
    pub struct _PllsrcW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllsrcW<'a> {
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
    pub struct _Pllp1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllp1W<'a> {
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
    pub struct _Pllp0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllp0W<'a> {
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
    pub struct _Plln8W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln8W<'a> {
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
    pub struct _Plln7W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln7W<'a> {
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
    pub struct _Plln6W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln6W<'a> {
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
    pub struct _Plln5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln5W<'a> {
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
    pub struct _Plln4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln4W<'a> {
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
    pub struct _Plln3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln3W<'a> {
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
    pub struct _Plln2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln2W<'a> {
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
    pub struct _Plln1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln1W<'a> {
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
    pub struct _Plln0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plln0W<'a> {
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
    pub struct _Pllm5W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm5W<'a> {
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
    pub struct _Pllm4W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm4W<'a> {
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
    pub struct _Pllm3W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm3W<'a> {
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
    pub struct _Pllm2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm2W<'a> {
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
    pub struct _Pllm1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm1W<'a> {
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
    pub struct _Pllm0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Pllm0W<'a> {
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
        fn _pllq3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq3(&self) -> Pllq3R {
            Pllq3R { bits: self._pllq3() }
        }
        fn _pllq2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq2(&self) -> Pllq2R {
            Pllq2R { bits: self._pllq2() }
        }
        fn _pllq1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq1(&self) -> Pllq1R {
            Pllq1R { bits: self._pllq1() }
        }
        fn _pllq0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq0(&self) -> Pllq0R {
            Pllq0R { bits: self._pllq0() }
        }
        fn _pllsrc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source" ]
        pub fn pllsrc(&self) -> PllsrcR {
            PllsrcR { bits: self._pllsrc() }
        }
        fn _pllp1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - Main PLL (PLL) division factor for main system clock" ]
        pub fn pllp1(&self) -> Pllp1R {
            Pllp1R { bits: self._pllp1() }
        }
        fn _pllp0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Main PLL (PLL) division factor for main system clock" ]
        pub fn pllp0(&self) -> Pllp0R {
            Pllp0R { bits: self._pllp0() }
        }
        fn _plln8(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln8(&self) -> Plln8R {
            Plln8R { bits: self._plln8() }
        }
        fn _plln7(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln7(&self) -> Plln7R {
            Plln7R { bits: self._plln7() }
        }
        fn _plln6(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln6(&self) -> Plln6R {
            Plln6R { bits: self._plln6() }
        }
        fn _plln5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln5(&self) -> Plln5R {
            Plln5R { bits: self._plln5() }
        }
        fn _plln4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln4(&self) -> Plln4R {
            Plln4R { bits: self._plln4() }
        }
        fn _plln3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln3(&self) -> Plln3R {
            Plln3R { bits: self._plln3() }
        }
        fn _plln2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln2(&self) -> Plln2R {
            Plln2R { bits: self._plln2() }
        }
        fn _plln1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln1(&self) -> Plln1R {
            Plln1R { bits: self._plln1() }
        }
        fn _plln0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln0(&self) -> Plln0R {
            Plln0R { bits: self._plln0() }
        }
        fn _pllm5(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm5(&self) -> Pllm5R {
            Pllm5R { bits: self._pllm5() }
        }
        fn _pllm4(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm4(&self) -> Pllm4R {
            Pllm4R { bits: self._pllm4() }
        }
        fn _pllm3(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm3(&self) -> Pllm3R {
            Pllm3R { bits: self._pllm3() }
        }
        fn _pllm2(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm2(&self) -> Pllm2R {
            Pllm2R { bits: self._pllm2() }
        }
        fn _pllm1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm1(&self) -> Pllm1R {
            Pllm1R { bits: self._pllm1() }
        }
        fn _pllm0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm0(&self) -> Pllm0R {
            Pllm0R { bits: self._pllm0() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 603992080 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 27 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq3(&mut self) -> _Pllq3W {
            _Pllq3W { register: self }
        }
        # [ doc = "Bit 26 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq2(&mut self) -> _Pllq2W {
            _Pllq2W { register: self }
        }
        # [ doc = "Bit 25 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq1(&mut self) -> _Pllq1W {
            _Pllq1W { register: self }
        }
        # [ doc = "Bit 24 - Main PLL (PLL) division factor for USB OTG FS, SDIO and random number generator clocks" ]
        pub fn pllq0(&mut self) -> _Pllq0W {
            _Pllq0W { register: self }
        }
        # [ doc = "Bit 22 - Main PLL(PLL) and audio PLL (PLLI2S) entry clock source" ]
        pub fn pllsrc(&mut self) -> _PllsrcW {
            _PllsrcW { register: self }
        }
        # [ doc = "Bit 17 - Main PLL (PLL) division factor for main system clock" ]
        pub fn pllp1(&mut self) -> _Pllp1W {
            _Pllp1W { register: self }
        }
        # [ doc = "Bit 16 - Main PLL (PLL) division factor for main system clock" ]
        pub fn pllp0(&mut self) -> _Pllp0W {
            _Pllp0W { register: self }
        }
        # [ doc = "Bit 14 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln8(&mut self) -> _Plln8W {
            _Plln8W { register: self }
        }
        # [ doc = "Bit 13 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln7(&mut self) -> _Plln7W {
            _Plln7W { register: self }
        }
        # [ doc = "Bit 12 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln6(&mut self) -> _Plln6W {
            _Plln6W { register: self }
        }
        # [ doc = "Bit 11 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln5(&mut self) -> _Plln5W {
            _Plln5W { register: self }
        }
        # [ doc = "Bit 10 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln4(&mut self) -> _Plln4W {
            _Plln4W { register: self }
        }
        # [ doc = "Bit 9 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln3(&mut self) -> _Plln3W {
            _Plln3W { register: self }
        }
        # [ doc = "Bit 8 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln2(&mut self) -> _Plln2W {
            _Plln2W { register: self }
        }
        # [ doc = "Bit 7 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln1(&mut self) -> _Plln1W {
            _Plln1W { register: self }
        }
        # [ doc = "Bit 6 - Main PLL (PLL) multiplication factor for VCO" ]
        pub fn plln0(&mut self) -> _Plln0W {
            _Plln0W { register: self }
        }
        # [ doc = "Bit 5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm5(&mut self) -> _Pllm5W {
            _Pllm5W { register: self }
        }
        # [ doc = "Bit 4 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm4(&mut self) -> _Pllm4W {
            _Pllm4W { register: self }
        }
        # [ doc = "Bit 3 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm3(&mut self) -> _Pllm3W {
            _Pllm3W { register: self }
        }
        # [ doc = "Bit 2 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm2(&mut self) -> _Pllm2W {
            _Pllm2W { register: self }
        }
        # [ doc = "Bit 1 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm1(&mut self) -> _Pllm1W {
            _Pllm1W { register: self }
        }
        # [ doc = "Bit 0 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock" ]
        pub fn pllm0(&mut self) -> _Pllm0W {
            _Pllm0W { register: self }
        }
    }
}

# [ doc = "clock configuration register" ]
# [ repr ( C ) ]
pub struct Cfgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "clock configuration register" ]
pub mod cfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cfgr {
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
    # [ doc = "Value of the field MCO2" ]
    pub struct Mco2R {
        bits: u8,
    }
    impl Mco2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MCO2PRE" ]
    pub struct Mco2preR {
        bits: u8,
    }
    impl Mco2preR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MCO1PRE" ]
    pub struct Mco1preR {
        bits: u8,
    }
    impl Mco1preR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2SSRC" ]
    pub struct I2ssrcR {
        bits: u8,
    }
    impl I2ssrcR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field MCO1" ]
    pub struct Mco1R {
        bits: u8,
    }
    impl Mco1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCPRE" ]
    pub struct RtcpreR {
        bits: u8,
    }
    impl RtcpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PPRE2" ]
    pub struct Ppre2R {
        bits: u8,
    }
    impl Ppre2R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PPRE1" ]
    pub struct Ppre1R {
        bits: u8,
    }
    impl Ppre1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HPRE" ]
    pub struct HpreR {
        bits: u8,
    }
    impl HpreR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SWS1" ]
    pub struct Sws1R {
        bits: u8,
    }
    impl Sws1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SWS0" ]
    pub struct Sws0R {
        bits: u8,
    }
    impl Sws0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SW1" ]
    pub struct Sw1R {
        bits: u8,
    }
    impl Sw1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SW0" ]
    pub struct Sw0R {
        bits: u8,
    }
    impl Sw0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Mco2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Mco2W<'a> {
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
    pub struct _Mco2preW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Mco2preW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Mco1preW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Mco1preW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _I2ssrcW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2ssrcW<'a> {
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
    pub struct _Mco1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Mco1W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _RtcpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtcpreW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ppre2W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ppre2W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Ppre1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Ppre1W<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _HpreW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HpreW<'a> {
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
    pub struct _Sw1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sw1W<'a> {
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
    pub struct _Sw0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sw0W<'a> {
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
        fn _mco2(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 30:31 - Microcontroller clock output 2" ]
        pub fn mco2(&self) -> Mco2R {
            Mco2R { bits: self._mco2() }
        }
        fn _mco2pre(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 27:29 - MCO2 prescaler" ]
        pub fn mco2pre(&self) -> Mco2preR {
            Mco2preR { bits: self._mco2pre() }
        }
        fn _mco1pre(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 24:26 - MCO1 prescaler" ]
        pub fn mco1pre(&self) -> Mco1preR {
            Mco1preR { bits: self._mco1pre() }
        }
        fn _i2ssrc(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - I2S clock selection" ]
        pub fn i2ssrc(&self) -> I2ssrcR {
            I2ssrcR { bits: self._i2ssrc() }
        }
        fn _mco1(&self) -> u8 {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 21:22 - Microcontroller clock output 1" ]
        pub fn mco1(&self) -> Mco1R {
            Mco1R { bits: self._mco1() }
        }
        fn _rtcpre(&self) -> u8 {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 16:20 - HSE division factor for RTC clock" ]
        pub fn rtcpre(&self) -> RtcpreR {
            RtcpreR { bits: self._rtcpre() }
        }
        fn _ppre2(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 13:15 - APB high-speed prescaler (APB2)" ]
        pub fn ppre2(&self) -> Ppre2R {
            Ppre2R { bits: self._ppre2() }
        }
        fn _ppre1(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 10:12 - APB Low speed prescaler (APB1)" ]
        pub fn ppre1(&self) -> Ppre1R {
            Ppre1R { bits: self._ppre1() }
        }
        fn _hpre(&self) -> u8 {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        pub fn hpre(&self) -> HpreR {
            HpreR { bits: self._hpre() }
        }
        fn _sws1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - System clock switch status" ]
        pub fn sws1(&self) -> Sws1R {
            Sws1R { bits: self._sws1() }
        }
        fn _sws0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - System clock switch status" ]
        pub fn sws0(&self) -> Sws0R {
            Sws0R { bits: self._sws0() }
        }
        fn _sw1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - System clock switch" ]
        pub fn sw1(&self) -> Sw1R {
            Sw1R { bits: self._sw1() }
        }
        fn _sw0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - System clock switch" ]
        pub fn sw0(&self) -> Sw0R {
            Sw0R { bits: self._sw0() }
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
        # [ doc = "Bits 30:31 - Microcontroller clock output 2" ]
        pub fn mco2(&mut self) -> _Mco2W {
            _Mco2W { register: self }
        }
        # [ doc = "Bits 27:29 - MCO2 prescaler" ]
        pub fn mco2pre(&mut self) -> _Mco2preW {
            _Mco2preW { register: self }
        }
        # [ doc = "Bits 24:26 - MCO1 prescaler" ]
        pub fn mco1pre(&mut self) -> _Mco1preW {
            _Mco1preW { register: self }
        }
        # [ doc = "Bit 23 - I2S clock selection" ]
        pub fn i2ssrc(&mut self) -> _I2ssrcW {
            _I2ssrcW { register: self }
        }
        # [ doc = "Bits 21:22 - Microcontroller clock output 1" ]
        pub fn mco1(&mut self) -> _Mco1W {
            _Mco1W { register: self }
        }
        # [ doc = "Bits 16:20 - HSE division factor for RTC clock" ]
        pub fn rtcpre(&mut self) -> _RtcpreW {
            _RtcpreW { register: self }
        }
        # [ doc = "Bits 13:15 - APB high-speed prescaler (APB2)" ]
        pub fn ppre2(&mut self) -> _Ppre2W {
            _Ppre2W { register: self }
        }
        # [ doc = "Bits 10:12 - APB Low speed prescaler (APB1)" ]
        pub fn ppre1(&mut self) -> _Ppre1W {
            _Ppre1W { register: self }
        }
        # [ doc = "Bits 4:7 - AHB prescaler" ]
        pub fn hpre(&mut self) -> _HpreW {
            _HpreW { register: self }
        }
        # [ doc = "Bit 1 - System clock switch" ]
        pub fn sw1(&mut self) -> _Sw1W {
            _Sw1W { register: self }
        }
        # [ doc = "Bit 0 - System clock switch" ]
        pub fn sw0(&mut self) -> _Sw0W {
            _Sw0W { register: self }
        }
    }
}

# [ doc = "clock interrupt register" ]
# [ repr ( C ) ]
pub struct Cir {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "clock interrupt register" ]
pub mod cir {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Cir {
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
    # [ doc = "Value of the field PLLI2SRDYIE" ]
    pub struct Plli2srdyieR {
        bits: u8,
    }
    impl Plli2srdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDYIE" ]
    pub struct PllrdyieR {
        bits: u8,
    }
    impl PllrdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDYIE" ]
    pub struct HserdyieR {
        bits: u8,
    }
    impl HserdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDYIE" ]
    pub struct HsirdyieR {
        bits: u8,
    }
    impl HsirdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDYIE" ]
    pub struct LserdyieR {
        bits: u8,
    }
    impl LserdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSIRDYIE" ]
    pub struct LsirdyieR {
        bits: u8,
    }
    impl LsirdyieR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CSSF" ]
    pub struct CssfR {
        bits: u8,
    }
    impl CssfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLI2SRDYF" ]
    pub struct Plli2srdyfR {
        bits: u8,
    }
    impl Plli2srdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLRDYF" ]
    pub struct PllrdyfR {
        bits: u8,
    }
    impl PllrdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSERDYF" ]
    pub struct HserdyfR {
        bits: u8,
    }
    impl HserdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field HSIRDYF" ]
    pub struct HsirdyfR {
        bits: u8,
    }
    impl HsirdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDYF" ]
    pub struct LserdyfR {
        bits: u8,
    }
    impl LserdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSIRDYF" ]
    pub struct LsirdyfR {
        bits: u8,
    }
    impl LsirdyfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _CsscW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CsscW<'a> {
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
    pub struct _Plli2srdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plli2srdycW<'a> {
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
    pub struct _PllrdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllrdycW<'a> {
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
    pub struct _HserdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HserdycW<'a> {
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
    pub struct _HsirdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsirdycW<'a> {
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
    pub struct _LserdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LserdycW<'a> {
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
    pub struct _LsirdycW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsirdycW<'a> {
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
    pub struct _Plli2srdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plli2srdyieW<'a> {
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
    pub struct _PllrdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PllrdyieW<'a> {
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
    pub struct _HserdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HserdyieW<'a> {
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
    pub struct _HsirdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _HsirdyieW<'a> {
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
    pub struct _LserdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LserdyieW<'a> {
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
    pub struct _LsirdyieW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsirdyieW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
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
        fn _plli2srdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 13 - PLLI2S ready interrupt enable" ]
        pub fn plli2srdyie(&self) -> Plli2srdyieR {
            Plli2srdyieR { bits: self._plli2srdyie() }
        }
        fn _pllrdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - Main PLL (PLL) ready interrupt enable" ]
        pub fn pllrdyie(&self) -> PllrdyieR {
            PllrdyieR { bits: self._pllrdyie() }
        }
        fn _hserdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - HSE ready interrupt enable" ]
        pub fn hserdyie(&self) -> HserdyieR {
            HserdyieR { bits: self._hserdyie() }
        }
        fn _hsirdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - HSI ready interrupt enable" ]
        pub fn hsirdyie(&self) -> HsirdyieR {
            HsirdyieR { bits: self._hsirdyie() }
        }
        fn _lserdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - LSE ready interrupt enable" ]
        pub fn lserdyie(&self) -> LserdyieR {
            LserdyieR { bits: self._lserdyie() }
        }
        fn _lsirdyie(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - LSI ready interrupt enable" ]
        pub fn lsirdyie(&self) -> LsirdyieR {
            LsirdyieR { bits: self._lsirdyie() }
        }
        fn _cssf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - Clock security system interrupt flag" ]
        pub fn cssf(&self) -> CssfR {
            CssfR { bits: self._cssf() }
        }
        fn _plli2srdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - PLLI2S ready interrupt flag" ]
        pub fn plli2srdyf(&self) -> Plli2srdyfR {
            Plli2srdyfR { bits: self._plli2srdyf() }
        }
        fn _pllrdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - Main PLL (PLL) ready interrupt flag" ]
        pub fn pllrdyf(&self) -> PllrdyfR {
            PllrdyfR { bits: self._pllrdyf() }
        }
        fn _hserdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - HSE ready interrupt flag" ]
        pub fn hserdyf(&self) -> HserdyfR {
            HserdyfR { bits: self._hserdyf() }
        }
        fn _hsirdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - HSI ready interrupt flag" ]
        pub fn hsirdyf(&self) -> HsirdyfR {
            HsirdyfR { bits: self._hsirdyf() }
        }
        fn _lserdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - LSE ready interrupt flag" ]
        pub fn lserdyf(&self) -> LserdyfR {
            LserdyfR { bits: self._lserdyf() }
        }
        fn _lsirdyf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - LSI ready interrupt flag" ]
        pub fn lsirdyf(&self) -> LsirdyfR {
            LsirdyfR { bits: self._lsirdyf() }
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
        # [ doc = "Bit 23 - Clock security system interrupt clear" ]
        pub fn cssc(&mut self) -> _CsscW {
            _CsscW { register: self }
        }
        # [ doc = "Bit 21 - PLLI2S ready interrupt clear" ]
        pub fn plli2srdyc(&mut self) -> _Plli2srdycW {
            _Plli2srdycW { register: self }
        }
        # [ doc = "Bit 20 - Main PLL(PLL) ready interrupt clear" ]
        pub fn pllrdyc(&mut self) -> _PllrdycW {
            _PllrdycW { register: self }
        }
        # [ doc = "Bit 19 - HSE ready interrupt clear" ]
        pub fn hserdyc(&mut self) -> _HserdycW {
            _HserdycW { register: self }
        }
        # [ doc = "Bit 18 - HSI ready interrupt clear" ]
        pub fn hsirdyc(&mut self) -> _HsirdycW {
            _HsirdycW { register: self }
        }
        # [ doc = "Bit 17 - LSE ready interrupt clear" ]
        pub fn lserdyc(&mut self) -> _LserdycW {
            _LserdycW { register: self }
        }
        # [ doc = "Bit 16 - LSI ready interrupt clear" ]
        pub fn lsirdyc(&mut self) -> _LsirdycW {
            _LsirdycW { register: self }
        }
        # [ doc = "Bit 13 - PLLI2S ready interrupt enable" ]
        pub fn plli2srdyie(&mut self) -> _Plli2srdyieW {
            _Plli2srdyieW { register: self }
        }
        # [ doc = "Bit 12 - Main PLL (PLL) ready interrupt enable" ]
        pub fn pllrdyie(&mut self) -> _PllrdyieW {
            _PllrdyieW { register: self }
        }
        # [ doc = "Bit 11 - HSE ready interrupt enable" ]
        pub fn hserdyie(&mut self) -> _HserdyieW {
            _HserdyieW { register: self }
        }
        # [ doc = "Bit 10 - HSI ready interrupt enable" ]
        pub fn hsirdyie(&mut self) -> _HsirdyieW {
            _HsirdyieW { register: self }
        }
        # [ doc = "Bit 9 - LSE ready interrupt enable" ]
        pub fn lserdyie(&mut self) -> _LserdyieW {
            _LserdyieW { register: self }
        }
        # [ doc = "Bit 8 - LSI ready interrupt enable" ]
        pub fn lsirdyie(&mut self) -> _LsirdyieW {
            _LsirdyieW { register: self }
        }
    }
}

# [ doc = "AHB1 peripheral reset register" ]
# [ repr ( C ) ]
pub struct Ahb1rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB1 peripheral reset register" ]
pub mod ahb1rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb1rstr {
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
    # [ doc = "Value of the field OTGHSRST" ]
    pub struct OtghsrstR {
        bits: u8,
    }
    impl OtghsrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACRST" ]
    pub struct EthmacrstR {
        bits: u8,
    }
    impl EthmacrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA2RST" ]
    pub struct Dma2rstR {
        bits: u8,
    }
    impl Dma2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA1RST" ]
    pub struct Dma1rstR {
        bits: u8,
    }
    impl Dma1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCRST" ]
    pub struct CrcrstR {
        bits: u8,
    }
    impl CrcrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOIRST" ]
    pub struct GpioirstR {
        bits: u8,
    }
    impl GpioirstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOHRST" ]
    pub struct GpiohrstR {
        bits: u8,
    }
    impl GpiohrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOGRST" ]
    pub struct GpiogrstR {
        bits: u8,
    }
    impl GpiogrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOFRST" ]
    pub struct GpiofrstR {
        bits: u8,
    }
    impl GpiofrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOERST" ]
    pub struct GpioerstR {
        bits: u8,
    }
    impl GpioerstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIODRST" ]
    pub struct GpiodrstR {
        bits: u8,
    }
    impl GpiodrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOCRST" ]
    pub struct GpiocrstR {
        bits: u8,
    }
    impl GpiocrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOBRST" ]
    pub struct GpiobrstR {
        bits: u8,
    }
    impl GpiobrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOARST" ]
    pub struct GpioarstR {
        bits: u8,
    }
    impl GpioarstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtghsrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtghsrstW<'a> {
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
    pub struct _EthmacrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacrstW<'a> {
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
    pub struct _Dma2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma2rstW<'a> {
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
    pub struct _Dma1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma1rstW<'a> {
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
    pub struct _CrcrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcrstW<'a> {
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
    pub struct _GpioirstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioirstW<'a> {
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
    pub struct _GpiohrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiohrstW<'a> {
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
    pub struct _GpiogrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiogrstW<'a> {
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
    pub struct _GpiofrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiofrstW<'a> {
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
    pub struct _GpioerstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioerstW<'a> {
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
    pub struct _GpiodrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiodrstW<'a> {
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
    pub struct _GpiocrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiocrstW<'a> {
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
    pub struct _GpiobrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiobrstW<'a> {
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
    pub struct _GpioarstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioarstW<'a> {
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
        fn _otghsrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - USB OTG HS module reset" ]
        pub fn otghsrst(&self) -> OtghsrstR {
            OtghsrstR { bits: self._otghsrst() }
        }
        fn _ethmacrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Ethernet MAC reset" ]
        pub fn ethmacrst(&self) -> EthmacrstR {
            EthmacrstR { bits: self._ethmacrst() }
        }
        fn _dma2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - DMA2 reset" ]
        pub fn dma2rst(&self) -> Dma2rstR {
            Dma2rstR { bits: self._dma2rst() }
        }
        fn _dma1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - DMA2 reset" ]
        pub fn dma1rst(&self) -> Dma1rstR {
            Dma1rstR { bits: self._dma1rst() }
        }
        fn _crcrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - CRC reset" ]
        pub fn crcrst(&self) -> CrcrstR {
            CrcrstR { bits: self._crcrst() }
        }
        fn _gpioirst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - IO port I reset" ]
        pub fn gpioirst(&self) -> GpioirstR {
            GpioirstR { bits: self._gpioirst() }
        }
        fn _gpiohrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - IO port H reset" ]
        pub fn gpiohrst(&self) -> GpiohrstR {
            GpiohrstR { bits: self._gpiohrst() }
        }
        fn _gpiogrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - IO port G reset" ]
        pub fn gpiogrst(&self) -> GpiogrstR {
            GpiogrstR { bits: self._gpiogrst() }
        }
        fn _gpiofrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - IO port F reset" ]
        pub fn gpiofrst(&self) -> GpiofrstR {
            GpiofrstR { bits: self._gpiofrst() }
        }
        fn _gpioerst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IO port E reset" ]
        pub fn gpioerst(&self) -> GpioerstR {
            GpioerstR { bits: self._gpioerst() }
        }
        fn _gpiodrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - IO port D reset" ]
        pub fn gpiodrst(&self) -> GpiodrstR {
            GpiodrstR { bits: self._gpiodrst() }
        }
        fn _gpiocrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - IO port C reset" ]
        pub fn gpiocrst(&self) -> GpiocrstR {
            GpiocrstR { bits: self._gpiocrst() }
        }
        fn _gpiobrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - IO port B reset" ]
        pub fn gpiobrst(&self) -> GpiobrstR {
            GpiobrstR { bits: self._gpiobrst() }
        }
        fn _gpioarst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - IO port A reset" ]
        pub fn gpioarst(&self) -> GpioarstR {
            GpioarstR { bits: self._gpioarst() }
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
        # [ doc = "Bit 29 - USB OTG HS module reset" ]
        pub fn otghsrst(&mut self) -> _OtghsrstW {
            _OtghsrstW { register: self }
        }
        # [ doc = "Bit 25 - Ethernet MAC reset" ]
        pub fn ethmacrst(&mut self) -> _EthmacrstW {
            _EthmacrstW { register: self }
        }
        # [ doc = "Bit 22 - DMA2 reset" ]
        pub fn dma2rst(&mut self) -> _Dma2rstW {
            _Dma2rstW { register: self }
        }
        # [ doc = "Bit 21 - DMA2 reset" ]
        pub fn dma1rst(&mut self) -> _Dma1rstW {
            _Dma1rstW { register: self }
        }
        # [ doc = "Bit 12 - CRC reset" ]
        pub fn crcrst(&mut self) -> _CrcrstW {
            _CrcrstW { register: self }
        }
        # [ doc = "Bit 8 - IO port I reset" ]
        pub fn gpioirst(&mut self) -> _GpioirstW {
            _GpioirstW { register: self }
        }
        # [ doc = "Bit 7 - IO port H reset" ]
        pub fn gpiohrst(&mut self) -> _GpiohrstW {
            _GpiohrstW { register: self }
        }
        # [ doc = "Bit 6 - IO port G reset" ]
        pub fn gpiogrst(&mut self) -> _GpiogrstW {
            _GpiogrstW { register: self }
        }
        # [ doc = "Bit 5 - IO port F reset" ]
        pub fn gpiofrst(&mut self) -> _GpiofrstW {
            _GpiofrstW { register: self }
        }
        # [ doc = "Bit 4 - IO port E reset" ]
        pub fn gpioerst(&mut self) -> _GpioerstW {
            _GpioerstW { register: self }
        }
        # [ doc = "Bit 3 - IO port D reset" ]
        pub fn gpiodrst(&mut self) -> _GpiodrstW {
            _GpiodrstW { register: self }
        }
        # [ doc = "Bit 2 - IO port C reset" ]
        pub fn gpiocrst(&mut self) -> _GpiocrstW {
            _GpiocrstW { register: self }
        }
        # [ doc = "Bit 1 - IO port B reset" ]
        pub fn gpiobrst(&mut self) -> _GpiobrstW {
            _GpiobrstW { register: self }
        }
        # [ doc = "Bit 0 - IO port A reset" ]
        pub fn gpioarst(&mut self) -> _GpioarstW {
            _GpioarstW { register: self }
        }
    }
}

# [ doc = "AHB2 peripheral reset register" ]
# [ repr ( C ) ]
pub struct Ahb2rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB2 peripheral reset register" ]
pub mod ahb2rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb2rstr {
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
    # [ doc = "Value of the field OTGFSRST" ]
    pub struct OtgfsrstR {
        bits: u8,
    }
    impl OtgfsrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RNGRST" ]
    pub struct RngrstR {
        bits: u8,
    }
    impl RngrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DCMIRST" ]
    pub struct DcmirstR {
        bits: u8,
    }
    impl DcmirstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtgfsrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtgfsrstW<'a> {
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
    pub struct _RngrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RngrstW<'a> {
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
    pub struct _DcmirstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DcmirstW<'a> {
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
        fn _otgfsrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - USB OTG FS module reset" ]
        pub fn otgfsrst(&self) -> OtgfsrstR {
            OtgfsrstR { bits: self._otgfsrst() }
        }
        fn _rngrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Random number generator module reset" ]
        pub fn rngrst(&self) -> RngrstR {
            RngrstR { bits: self._rngrst() }
        }
        fn _dcmirst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Camera interface reset" ]
        pub fn dcmirst(&self) -> DcmirstR {
            DcmirstR { bits: self._dcmirst() }
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
        # [ doc = "Bit 7 - USB OTG FS module reset" ]
        pub fn otgfsrst(&mut self) -> _OtgfsrstW {
            _OtgfsrstW { register: self }
        }
        # [ doc = "Bit 6 - Random number generator module reset" ]
        pub fn rngrst(&mut self) -> _RngrstW {
            _RngrstW { register: self }
        }
        # [ doc = "Bit 0 - Camera interface reset" ]
        pub fn dcmirst(&mut self) -> _DcmirstW {
            _DcmirstW { register: self }
        }
    }
}

# [ doc = "AHB3 peripheral reset register" ]
# [ repr ( C ) ]
pub struct Ahb3rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB3 peripheral reset register" ]
pub mod ahb3rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb3rstr {
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
    # [ doc = "Value of the field FSMCRST" ]
    pub struct FsmcrstR {
        bits: u8,
    }
    impl FsmcrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FsmcrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FsmcrstW<'a> {
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
        fn _fsmcrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Flexible static memory controller module reset" ]
        pub fn fsmcrst(&self) -> FsmcrstR {
            FsmcrstR { bits: self._fsmcrst() }
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
        # [ doc = "Bit 0 - Flexible static memory controller module reset" ]
        pub fn fsmcrst(&mut self) -> _FsmcrstW {
            _FsmcrstW { register: self }
        }
    }
}

# [ doc = "APB1 peripheral reset register" ]
# [ repr ( C ) ]
pub struct Apb1rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB1 peripheral reset register" ]
pub mod apb1rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb1rstr {
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
    # [ doc = "Value of the field DACRST" ]
    pub struct DacrstR {
        bits: u8,
    }
    impl DacrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PWRRST" ]
    pub struct PwrrstR {
        bits: u8,
    }
    impl PwrrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN2RST" ]
    pub struct Can2rstR {
        bits: u8,
    }
    impl Can2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN1RST" ]
    pub struct Can1rstR {
        bits: u8,
    }
    impl Can1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C3RST" ]
    pub struct I2c3rstR {
        bits: u8,
    }
    impl I2c3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C2RST" ]
    pub struct I2c2rstR {
        bits: u8,
    }
    impl I2c2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1RST" ]
    pub struct I2c1rstR {
        bits: u8,
    }
    impl I2c1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART5RST" ]
    pub struct Uart5rstR {
        bits: u8,
    }
    impl Uart5rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART4RST" ]
    pub struct Uart4rstR {
        bits: u8,
    }
    impl Uart4rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART3RST" ]
    pub struct Uart3rstR {
        bits: u8,
    }
    impl Uart3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART2RST" ]
    pub struct Uart2rstR {
        bits: u8,
    }
    impl Uart2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI3RST" ]
    pub struct Spi3rstR {
        bits: u8,
    }
    impl Spi3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI2RST" ]
    pub struct Spi2rstR {
        bits: u8,
    }
    impl Spi2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGRST" ]
    pub struct WwdgrstR {
        bits: u8,
    }
    impl WwdgrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM14RST" ]
    pub struct Tim14rstR {
        bits: u8,
    }
    impl Tim14rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM13RST" ]
    pub struct Tim13rstR {
        bits: u8,
    }
    impl Tim13rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM12RST" ]
    pub struct Tim12rstR {
        bits: u8,
    }
    impl Tim12rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM7RST" ]
    pub struct Tim7rstR {
        bits: u8,
    }
    impl Tim7rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM6RST" ]
    pub struct Tim6rstR {
        bits: u8,
    }
    impl Tim6rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM5RST" ]
    pub struct Tim5rstR {
        bits: u8,
    }
    impl Tim5rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM4RST" ]
    pub struct Tim4rstR {
        bits: u8,
    }
    impl Tim4rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM3RST" ]
    pub struct Tim3rstR {
        bits: u8,
    }
    impl Tim3rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM2RST" ]
    pub struct Tim2rstR {
        bits: u8,
    }
    impl Tim2rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DacrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DacrstW<'a> {
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
    pub struct _PwrrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PwrrstW<'a> {
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
    pub struct _Can2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can2rstW<'a> {
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
    pub struct _Can1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can1rstW<'a> {
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
    pub struct _I2c3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c3rstW<'a> {
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
    pub struct _I2c2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c2rstW<'a> {
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
    pub struct _I2c1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1rstW<'a> {
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
    pub struct _Uart5rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart5rstW<'a> {
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
    pub struct _Uart4rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart4rstW<'a> {
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
    pub struct _Uart3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart3rstW<'a> {
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
    pub struct _Uart2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart2rstW<'a> {
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
    pub struct _Spi3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi3rstW<'a> {
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
    pub struct _Spi2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi2rstW<'a> {
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
    pub struct _WwdgrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgrstW<'a> {
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
    pub struct _Tim14rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim14rstW<'a> {
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
    pub struct _Tim13rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim13rstW<'a> {
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
    pub struct _Tim12rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim12rstW<'a> {
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
    pub struct _Tim7rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim7rstW<'a> {
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
    pub struct _Tim6rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim6rstW<'a> {
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
    pub struct _Tim5rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim5rstW<'a> {
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
    pub struct _Tim4rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim4rstW<'a> {
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
    pub struct _Tim3rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim3rstW<'a> {
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
    pub struct _Tim2rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim2rstW<'a> {
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
        fn _dacrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - DAC reset" ]
        pub fn dacrst(&self) -> DacrstR {
            DacrstR { bits: self._dacrst() }
        }
        fn _pwrrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        pub fn pwrrst(&self) -> PwrrstR {
            PwrrstR { bits: self._pwrrst() }
        }
        fn _can2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - CAN2 reset" ]
        pub fn can2rst(&self) -> Can2rstR {
            Can2rstR { bits: self._can2rst() }
        }
        fn _can1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - CAN1 reset" ]
        pub fn can1rst(&self) -> Can1rstR {
            Can1rstR { bits: self._can1rst() }
        }
        fn _i2c3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - I2C3 reset" ]
        pub fn i2c3rst(&self) -> I2c3rstR {
            I2c3rstR { bits: self._i2c3rst() }
        }
        fn _i2c2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I2C 2 reset" ]
        pub fn i2c2rst(&self) -> I2c2rstR {
            I2c2rstR { bits: self._i2c2rst() }
        }
        fn _i2c1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - I2C 1 reset" ]
        pub fn i2c1rst(&self) -> I2c1rstR {
            I2c1rstR { bits: self._i2c1rst() }
        }
        fn _uart5rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - USART 5 reset" ]
        pub fn uart5rst(&self) -> Uart5rstR {
            Uart5rstR { bits: self._uart5rst() }
        }
        fn _uart4rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - USART 4 reset" ]
        pub fn uart4rst(&self) -> Uart4rstR {
            Uart4rstR { bits: self._uart4rst() }
        }
        fn _uart3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - USART 3 reset" ]
        pub fn uart3rst(&self) -> Uart3rstR {
            Uart3rstR { bits: self._uart3rst() }
        }
        fn _uart2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        pub fn uart2rst(&self) -> Uart2rstR {
            Uart2rstR { bits: self._uart2rst() }
        }
        fn _spi3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - SPI 3 reset" ]
        pub fn spi3rst(&self) -> Spi3rstR {
            Spi3rstR { bits: self._spi3rst() }
        }
        fn _spi2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - SPI 2 reset" ]
        pub fn spi2rst(&self) -> Spi2rstR {
            Spi2rstR { bits: self._spi2rst() }
        }
        fn _wwdgrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        pub fn wwdgrst(&self) -> WwdgrstR {
            WwdgrstR { bits: self._wwdgrst() }
        }
        fn _tim14rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - TIM14 reset" ]
        pub fn tim14rst(&self) -> Tim14rstR {
            Tim14rstR { bits: self._tim14rst() }
        }
        fn _tim13rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - TIM13 reset" ]
        pub fn tim13rst(&self) -> Tim13rstR {
            Tim13rstR { bits: self._tim13rst() }
        }
        fn _tim12rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - TIM12 reset" ]
        pub fn tim12rst(&self) -> Tim12rstR {
            Tim12rstR { bits: self._tim12rst() }
        }
        fn _tim7rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - TIM7 reset" ]
        pub fn tim7rst(&self) -> Tim7rstR {
            Tim7rstR { bits: self._tim7rst() }
        }
        fn _tim6rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - TIM6 reset" ]
        pub fn tim6rst(&self) -> Tim6rstR {
            Tim6rstR { bits: self._tim6rst() }
        }
        fn _tim5rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - TIM5 reset" ]
        pub fn tim5rst(&self) -> Tim5rstR {
            Tim5rstR { bits: self._tim5rst() }
        }
        fn _tim4rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - TIM4 reset" ]
        pub fn tim4rst(&self) -> Tim4rstR {
            Tim4rstR { bits: self._tim4rst() }
        }
        fn _tim3rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM3 reset" ]
        pub fn tim3rst(&self) -> Tim3rstR {
            Tim3rstR { bits: self._tim3rst() }
        }
        fn _tim2rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM2 reset" ]
        pub fn tim2rst(&self) -> Tim2rstR {
            Tim2rstR { bits: self._tim2rst() }
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
        # [ doc = "Bit 29 - DAC reset" ]
        pub fn dacrst(&mut self) -> _DacrstW {
            _DacrstW { register: self }
        }
        # [ doc = "Bit 28 - Power interface reset" ]
        pub fn pwrrst(&mut self) -> _PwrrstW {
            _PwrrstW { register: self }
        }
        # [ doc = "Bit 26 - CAN2 reset" ]
        pub fn can2rst(&mut self) -> _Can2rstW {
            _Can2rstW { register: self }
        }
        # [ doc = "Bit 25 - CAN1 reset" ]
        pub fn can1rst(&mut self) -> _Can1rstW {
            _Can1rstW { register: self }
        }
        # [ doc = "Bit 23 - I2C3 reset" ]
        pub fn i2c3rst(&mut self) -> _I2c3rstW {
            _I2c3rstW { register: self }
        }
        # [ doc = "Bit 22 - I2C 2 reset" ]
        pub fn i2c2rst(&mut self) -> _I2c2rstW {
            _I2c2rstW { register: self }
        }
        # [ doc = "Bit 21 - I2C 1 reset" ]
        pub fn i2c1rst(&mut self) -> _I2c1rstW {
            _I2c1rstW { register: self }
        }
        # [ doc = "Bit 20 - USART 5 reset" ]
        pub fn uart5rst(&mut self) -> _Uart5rstW {
            _Uart5rstW { register: self }
        }
        # [ doc = "Bit 19 - USART 4 reset" ]
        pub fn uart4rst(&mut self) -> _Uart4rstW {
            _Uart4rstW { register: self }
        }
        # [ doc = "Bit 18 - USART 3 reset" ]
        pub fn uart3rst(&mut self) -> _Uart3rstW {
            _Uart3rstW { register: self }
        }
        # [ doc = "Bit 17 - USART 2 reset" ]
        pub fn uart2rst(&mut self) -> _Uart2rstW {
            _Uart2rstW { register: self }
        }
        # [ doc = "Bit 15 - SPI 3 reset" ]
        pub fn spi3rst(&mut self) -> _Spi3rstW {
            _Spi3rstW { register: self }
        }
        # [ doc = "Bit 14 - SPI 2 reset" ]
        pub fn spi2rst(&mut self) -> _Spi2rstW {
            _Spi2rstW { register: self }
        }
        # [ doc = "Bit 11 - Window watchdog reset" ]
        pub fn wwdgrst(&mut self) -> _WwdgrstW {
            _WwdgrstW { register: self }
        }
        # [ doc = "Bit 8 - TIM14 reset" ]
        pub fn tim14rst(&mut self) -> _Tim14rstW {
            _Tim14rstW { register: self }
        }
        # [ doc = "Bit 7 - TIM13 reset" ]
        pub fn tim13rst(&mut self) -> _Tim13rstW {
            _Tim13rstW { register: self }
        }
        # [ doc = "Bit 6 - TIM12 reset" ]
        pub fn tim12rst(&mut self) -> _Tim12rstW {
            _Tim12rstW { register: self }
        }
        # [ doc = "Bit 5 - TIM7 reset" ]
        pub fn tim7rst(&mut self) -> _Tim7rstW {
            _Tim7rstW { register: self }
        }
        # [ doc = "Bit 4 - TIM6 reset" ]
        pub fn tim6rst(&mut self) -> _Tim6rstW {
            _Tim6rstW { register: self }
        }
        # [ doc = "Bit 3 - TIM5 reset" ]
        pub fn tim5rst(&mut self) -> _Tim5rstW {
            _Tim5rstW { register: self }
        }
        # [ doc = "Bit 2 - TIM4 reset" ]
        pub fn tim4rst(&mut self) -> _Tim4rstW {
            _Tim4rstW { register: self }
        }
        # [ doc = "Bit 1 - TIM3 reset" ]
        pub fn tim3rst(&mut self) -> _Tim3rstW {
            _Tim3rstW { register: self }
        }
        # [ doc = "Bit 0 - TIM2 reset" ]
        pub fn tim2rst(&mut self) -> _Tim2rstW {
            _Tim2rstW { register: self }
        }
    }
}

# [ doc = "APB2 peripheral reset register" ]
# [ repr ( C ) ]
pub struct Apb2rstr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB2 peripheral reset register" ]
pub mod apb2rstr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb2rstr {
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
    # [ doc = "Value of the field TIM11RST" ]
    pub struct Tim11rstR {
        bits: u8,
    }
    impl Tim11rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM10RST" ]
    pub struct Tim10rstR {
        bits: u8,
    }
    impl Tim10rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM9RST" ]
    pub struct Tim9rstR {
        bits: u8,
    }
    impl Tim9rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SYSCFGRST" ]
    pub struct SyscfgrstR {
        bits: u8,
    }
    impl SyscfgrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI1RST" ]
    pub struct Spi1rstR {
        bits: u8,
    }
    impl Spi1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SDIORST" ]
    pub struct SdiorstR {
        bits: u8,
    }
    impl SdiorstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADCRST" ]
    pub struct AdcrstR {
        bits: u8,
    }
    impl AdcrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART6RST" ]
    pub struct Usart6rstR {
        bits: u8,
    }
    impl Usart6rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART1RST" ]
    pub struct Usart1rstR {
        bits: u8,
    }
    impl Usart1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM8RST" ]
    pub struct Tim8rstR {
        bits: u8,
    }
    impl Tim8rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM1RST" ]
    pub struct Tim1rstR {
        bits: u8,
    }
    impl Tim1rstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Tim11rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim11rstW<'a> {
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
    pub struct _Tim10rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim10rstW<'a> {
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
    pub struct _Tim9rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim9rstW<'a> {
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
    pub struct _SyscfgrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SyscfgrstW<'a> {
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
    pub struct _Spi1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi1rstW<'a> {
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
    pub struct _SdiorstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SdiorstW<'a> {
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
    pub struct _AdcrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _AdcrstW<'a> {
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
    pub struct _Usart6rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart6rstW<'a> {
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
    pub struct _Usart1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1rstW<'a> {
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
    pub struct _Tim8rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim8rstW<'a> {
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
    pub struct _Tim1rstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim1rstW<'a> {
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
        fn _tim11rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - TIM11 reset" ]
        pub fn tim11rst(&self) -> Tim11rstR {
            Tim11rstR { bits: self._tim11rst() }
        }
        fn _tim10rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TIM10 reset" ]
        pub fn tim10rst(&self) -> Tim10rstR {
            Tim10rstR { bits: self._tim10rst() }
        }
        fn _tim9rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - TIM9 reset" ]
        pub fn tim9rst(&self) -> Tim9rstR {
            Tim9rstR { bits: self._tim9rst() }
        }
        fn _syscfgrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - System configuration controller reset" ]
        pub fn syscfgrst(&self) -> SyscfgrstR {
            SyscfgrstR { bits: self._syscfgrst() }
        }
        fn _spi1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        pub fn spi1rst(&self) -> Spi1rstR {
            Spi1rstR { bits: self._spi1rst() }
        }
        fn _sdiorst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - SDIO reset" ]
        pub fn sdiorst(&self) -> SdiorstR {
            SdiorstR { bits: self._sdiorst() }
        }
        fn _adcrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - ADC interface reset (common to all ADCs)" ]
        pub fn adcrst(&self) -> AdcrstR {
            AdcrstR { bits: self._adcrst() }
        }
        fn _usart6rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - USART6 reset" ]
        pub fn usart6rst(&self) -> Usart6rstR {
            Usart6rstR { bits: self._usart6rst() }
        }
        fn _usart1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - USART1 reset" ]
        pub fn usart1rst(&self) -> Usart1rstR {
            Usart1rstR { bits: self._usart1rst() }
        }
        fn _tim8rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM8 reset" ]
        pub fn tim8rst(&self) -> Tim8rstR {
            Tim8rstR { bits: self._tim8rst() }
        }
        fn _tim1rst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM1 reset" ]
        pub fn tim1rst(&self) -> Tim1rstR {
            Tim1rstR { bits: self._tim1rst() }
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
        # [ doc = "Bit 18 - TIM11 reset" ]
        pub fn tim11rst(&mut self) -> _Tim11rstW {
            _Tim11rstW { register: self }
        }
        # [ doc = "Bit 17 - TIM10 reset" ]
        pub fn tim10rst(&mut self) -> _Tim10rstW {
            _Tim10rstW { register: self }
        }
        # [ doc = "Bit 16 - TIM9 reset" ]
        pub fn tim9rst(&mut self) -> _Tim9rstW {
            _Tim9rstW { register: self }
        }
        # [ doc = "Bit 14 - System configuration controller reset" ]
        pub fn syscfgrst(&mut self) -> _SyscfgrstW {
            _SyscfgrstW { register: self }
        }
        # [ doc = "Bit 12 - SPI 1 reset" ]
        pub fn spi1rst(&mut self) -> _Spi1rstW {
            _Spi1rstW { register: self }
        }
        # [ doc = "Bit 11 - SDIO reset" ]
        pub fn sdiorst(&mut self) -> _SdiorstW {
            _SdiorstW { register: self }
        }
        # [ doc = "Bit 8 - ADC interface reset (common to all ADCs)" ]
        pub fn adcrst(&mut self) -> _AdcrstW {
            _AdcrstW { register: self }
        }
        # [ doc = "Bit 5 - USART6 reset" ]
        pub fn usart6rst(&mut self) -> _Usart6rstW {
            _Usart6rstW { register: self }
        }
        # [ doc = "Bit 4 - USART1 reset" ]
        pub fn usart1rst(&mut self) -> _Usart1rstW {
            _Usart1rstW { register: self }
        }
        # [ doc = "Bit 1 - TIM8 reset" ]
        pub fn tim8rst(&mut self) -> _Tim8rstW {
            _Tim8rstW { register: self }
        }
        # [ doc = "Bit 0 - TIM1 reset" ]
        pub fn tim1rst(&mut self) -> _Tim1rstW {
            _Tim1rstW { register: self }
        }
    }
}

# [ doc = "AHB1 peripheral clock register" ]
# [ repr ( C ) ]
pub struct Ahb1enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB1 peripheral clock register" ]
pub mod ahb1enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb1enr {
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
    # [ doc = "Value of the field OTGHSULPIEN" ]
    pub struct OtghsulpienR {
        bits: u8,
    }
    impl OtghsulpienR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OTGHSEN" ]
    pub struct OtghsenR {
        bits: u8,
    }
    impl OtghsenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACPTPEN" ]
    pub struct EthmacptpenR {
        bits: u8,
    }
    impl EthmacptpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACRXEN" ]
    pub struct EthmacrxenR {
        bits: u8,
    }
    impl EthmacrxenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACTXEN" ]
    pub struct EthmactxenR {
        bits: u8,
    }
    impl EthmactxenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACEN" ]
    pub struct EthmacenR {
        bits: u8,
    }
    impl EthmacenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA2EN" ]
    pub struct Dma2enR {
        bits: u8,
    }
    impl Dma2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA1EN" ]
    pub struct Dma1enR {
        bits: u8,
    }
    impl Dma1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BKPSRAMEN" ]
    pub struct BkpsramenR {
        bits: u8,
    }
    impl BkpsramenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCEN" ]
    pub struct CrcenR {
        bits: u8,
    }
    impl CrcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOIEN" ]
    pub struct GpioienR {
        bits: u8,
    }
    impl GpioienR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOHEN" ]
    pub struct GpiohenR {
        bits: u8,
    }
    impl GpiohenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOGEN" ]
    pub struct GpiogenR {
        bits: u8,
    }
    impl GpiogenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOFEN" ]
    pub struct GpiofenR {
        bits: u8,
    }
    impl GpiofenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOEEN" ]
    pub struct GpioeenR {
        bits: u8,
    }
    impl GpioeenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIODEN" ]
    pub struct GpiodenR {
        bits: u8,
    }
    impl GpiodenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOCEN" ]
    pub struct GpiocenR {
        bits: u8,
    }
    impl GpiocenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Possible values of the field `gpioben`" ]
    # [ derive ( Clone , Copy , Debug , PartialEq ) ]
    pub enum EnableR {
        # [ doc = "0: Disable" ]
        Disable,
        # [ doc = "1: Enable" ]
        Enable,
    }
    impl EnableR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            match *self {
                EnableR::Disable => 0,
                EnableR::Enable => 1,
            }
        }
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _from(bits: u8) -> EnableR {
            match bits {
                0 => EnableR::Disable,
                1 => EnableR::Enable,
                _ => unreachable!(),
            }
        }
        # [ doc = "Check if the value of the field is `Disable`" ]
        pub fn is_disable(&self) -> bool {
            *self == EnableR::Disable
        }
        # [ doc = "Check if the value of the field is `Enable`" ]
        pub fn is_enable(&self) -> bool {
            *self == EnableR::Enable
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtghsulpienW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtghsulpienW<'a> {
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
    pub struct _OtghsenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtghsenW<'a> {
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
    pub struct _EthmacptpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacptpenW<'a> {
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
    pub struct _EthmacrxenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacrxenW<'a> {
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
    pub struct _EthmactxenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmactxenW<'a> {
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
    pub struct _EthmacenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacenW<'a> {
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
    pub struct _Dma2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma2enW<'a> {
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
    pub struct _Dma1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma1enW<'a> {
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
    pub struct _BkpsramenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BkpsramenW<'a> {
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
    pub struct _CrcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrcenW<'a> {
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
    pub struct _GpioienW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioienW<'a> {
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
    pub struct _GpiohenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiohenW<'a> {
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
    pub struct _GpiogenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiogenW<'a> {
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
    pub struct _GpiofenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiofenW<'a> {
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
    pub struct _GpioeenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioeenW<'a> {
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
    pub struct _GpiodenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiodenW<'a> {
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
    pub struct _GpiocenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiocenW<'a> {
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
    pub struct _GpiobenW<'a> {
        register: &'a mut W,
    }
    # [ doc = "Values that can be written to the field `gpioben`" ]
    pub enum EnableW {
        # [ doc = "0: Disable" ]
        Disable,
        # [ doc = "1: Enable" ]
        Enable,
    }
    impl EnableW {
        # [ allow ( missing_docs ) ]
        # [ doc ( hidden ) ]
        # [ inline ( always ) ]
        pub fn _bits(&self) -> u8 {
            match *self {
                EnableW::Disable => 0,
                EnableW::Enable => 1,
            }
        }
    }
    impl<'a> _GpiobenW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: EnableW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Disable" ]
        pub fn disable(self) -> &'a mut W {
            self.variant(EnableW::Disable)
        }
        # [ doc = "1: Enable" ]
        pub fn enable(self) -> &'a mut W {
            self.variant(EnableW::Enable)
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
    pub struct _GpioaenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioaenW<'a> {
        # [ doc = r" Writes `variant` to the field" ]
        pub fn variant(self, variant: EnableW) -> &'a mut W {
            self.bits(variant._bits())
        }
        # [ doc = "0: Disable" ]
        pub fn disable(self) -> &'a mut W {
            self.variant(EnableW::Disable)
        }
        # [ doc = "1: Enable" ]
        pub fn enable(self) -> &'a mut W {
            self.variant(EnableW::Enable)
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
        fn _otghsulpien(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - USB OTG HSULPI clock enable" ]
        pub fn otghsulpien(&self) -> OtghsulpienR {
            OtghsulpienR { bits: self._otghsulpien() }
        }
        fn _otghsen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - USB OTG HS clock enable" ]
        pub fn otghsen(&self) -> OtghsenR {
            OtghsenR { bits: self._otghsen() }
        }
        fn _ethmacptpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Ethernet PTP clock enable" ]
        pub fn ethmacptpen(&self) -> EthmacptpenR {
            EthmacptpenR { bits: self._ethmacptpen() }
        }
        fn _ethmacrxen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - Ethernet Reception clock enable" ]
        pub fn ethmacrxen(&self) -> EthmacrxenR {
            EthmacrxenR { bits: self._ethmacrxen() }
        }
        fn _ethmactxen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - Ethernet Transmission clock enable" ]
        pub fn ethmactxen(&self) -> EthmactxenR {
            EthmactxenR { bits: self._ethmactxen() }
        }
        fn _ethmacen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Ethernet MAC clock enable" ]
        pub fn ethmacen(&self) -> EthmacenR {
            EthmacenR { bits: self._ethmacen() }
        }
        fn _dma2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - DMA2 clock enable" ]
        pub fn dma2en(&self) -> Dma2enR {
            Dma2enR { bits: self._dma2en() }
        }
        fn _dma1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - DMA1 clock enable" ]
        pub fn dma1en(&self) -> Dma1enR {
            Dma1enR { bits: self._dma1en() }
        }
        fn _bkpsramen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Backup SRAM interface clock enable" ]
        pub fn bkpsramen(&self) -> BkpsramenR {
            BkpsramenR { bits: self._bkpsramen() }
        }
        fn _crcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - CRC clock enable" ]
        pub fn crcen(&self) -> CrcenR {
            CrcenR { bits: self._crcen() }
        }
        fn _gpioien(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - IO port I clock enable" ]
        pub fn gpioien(&self) -> GpioienR {
            GpioienR { bits: self._gpioien() }
        }
        fn _gpiohen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - IO port H clock enable" ]
        pub fn gpiohen(&self) -> GpiohenR {
            GpiohenR { bits: self._gpiohen() }
        }
        fn _gpiogen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - IO port G clock enable" ]
        pub fn gpiogen(&self) -> GpiogenR {
            GpiogenR { bits: self._gpiogen() }
        }
        fn _gpiofen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - IO port F clock enable" ]
        pub fn gpiofen(&self) -> GpiofenR {
            GpiofenR { bits: self._gpiofen() }
        }
        fn _gpioeen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IO port E clock enable" ]
        pub fn gpioeen(&self) -> GpioeenR {
            GpioeenR { bits: self._gpioeen() }
        }
        fn _gpioden(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - IO port D clock enable" ]
        pub fn gpioden(&self) -> GpiodenR {
            GpiodenR { bits: self._gpioden() }
        }
        fn _gpiocen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - IO port C clock enable" ]
        pub fn gpiocen(&self) -> GpiocenR {
            GpiocenR { bits: self._gpiocen() }
        }
        fn _gpioben(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - IO port B clock enable" ]
        pub fn gpioben(&self) -> EnableR {
            EnableR::_from(self._gpioben())
        }
        fn _gpioaen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - IO port A clock enable" ]
        pub fn gpioaen(&self) -> EnableR {
            EnableR::_from(self._gpioaen())
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 1048576 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 30 - USB OTG HSULPI clock enable" ]
        pub fn otghsulpien(&mut self) -> _OtghsulpienW {
            _OtghsulpienW { register: self }
        }
        # [ doc = "Bit 29 - USB OTG HS clock enable" ]
        pub fn otghsen(&mut self) -> _OtghsenW {
            _OtghsenW { register: self }
        }
        # [ doc = "Bit 28 - Ethernet PTP clock enable" ]
        pub fn ethmacptpen(&mut self) -> _EthmacptpenW {
            _EthmacptpenW { register: self }
        }
        # [ doc = "Bit 27 - Ethernet Reception clock enable" ]
        pub fn ethmacrxen(&mut self) -> _EthmacrxenW {
            _EthmacrxenW { register: self }
        }
        # [ doc = "Bit 26 - Ethernet Transmission clock enable" ]
        pub fn ethmactxen(&mut self) -> _EthmactxenW {
            _EthmactxenW { register: self }
        }
        # [ doc = "Bit 25 - Ethernet MAC clock enable" ]
        pub fn ethmacen(&mut self) -> _EthmacenW {
            _EthmacenW { register: self }
        }
        # [ doc = "Bit 22 - DMA2 clock enable" ]
        pub fn dma2en(&mut self) -> _Dma2enW {
            _Dma2enW { register: self }
        }
        # [ doc = "Bit 21 - DMA1 clock enable" ]
        pub fn dma1en(&mut self) -> _Dma1enW {
            _Dma1enW { register: self }
        }
        # [ doc = "Bit 18 - Backup SRAM interface clock enable" ]
        pub fn bkpsramen(&mut self) -> _BkpsramenW {
            _BkpsramenW { register: self }
        }
        # [ doc = "Bit 12 - CRC clock enable" ]
        pub fn crcen(&mut self) -> _CrcenW {
            _CrcenW { register: self }
        }
        # [ doc = "Bit 8 - IO port I clock enable" ]
        pub fn gpioien(&mut self) -> _GpioienW {
            _GpioienW { register: self }
        }
        # [ doc = "Bit 7 - IO port H clock enable" ]
        pub fn gpiohen(&mut self) -> _GpiohenW {
            _GpiohenW { register: self }
        }
        # [ doc = "Bit 6 - IO port G clock enable" ]
        pub fn gpiogen(&mut self) -> _GpiogenW {
            _GpiogenW { register: self }
        }
        # [ doc = "Bit 5 - IO port F clock enable" ]
        pub fn gpiofen(&mut self) -> _GpiofenW {
            _GpiofenW { register: self }
        }
        # [ doc = "Bit 4 - IO port E clock enable" ]
        pub fn gpioeen(&mut self) -> _GpioeenW {
            _GpioeenW { register: self }
        }
        # [ doc = "Bit 3 - IO port D clock enable" ]
        pub fn gpioden(&mut self) -> _GpiodenW {
            _GpiodenW { register: self }
        }
        # [ doc = "Bit 2 - IO port C clock enable" ]
        pub fn gpiocen(&mut self) -> _GpiocenW {
            _GpiocenW { register: self }
        }
        # [ doc = "Bit 1 - IO port B clock enable" ]
        pub fn gpioben(&mut self) -> _GpiobenW {
            _GpiobenW { register: self }
        }
        # [ doc = "Bit 0 - IO port A clock enable" ]
        pub fn gpioaen(&mut self) -> _GpioaenW {
            _GpioaenW { register: self }
        }
    }
}

# [ doc = "AHB2 peripheral clock enable register" ]
# [ repr ( C ) ]
pub struct Ahb2enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB2 peripheral clock enable register" ]
pub mod ahb2enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb2enr {
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
    # [ doc = "Value of the field OTGFSEN" ]
    pub struct OtgfsenR {
        bits: u8,
    }
    impl OtgfsenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RNGEN" ]
    pub struct RngenR {
        bits: u8,
    }
    impl RngenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DCMIEN" ]
    pub struct DcmienR {
        bits: u8,
    }
    impl DcmienR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtgfsenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtgfsenW<'a> {
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
    pub struct _RngenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RngenW<'a> {
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
    pub struct _DcmienW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DcmienW<'a> {
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
        fn _otgfsen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - USB OTG FS clock enable" ]
        pub fn otgfsen(&self) -> OtgfsenR {
            OtgfsenR { bits: self._otgfsen() }
        }
        fn _rngen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Random number generator clock enable" ]
        pub fn rngen(&self) -> RngenR {
            RngenR { bits: self._rngen() }
        }
        fn _dcmien(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Camera interface enable" ]
        pub fn dcmien(&self) -> DcmienR {
            DcmienR { bits: self._dcmien() }
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
        # [ doc = "Bit 7 - USB OTG FS clock enable" ]
        pub fn otgfsen(&mut self) -> _OtgfsenW {
            _OtgfsenW { register: self }
        }
        # [ doc = "Bit 6 - Random number generator clock enable" ]
        pub fn rngen(&mut self) -> _RngenW {
            _RngenW { register: self }
        }
        # [ doc = "Bit 0 - Camera interface enable" ]
        pub fn dcmien(&mut self) -> _DcmienW {
            _DcmienW { register: self }
        }
    }
}

# [ doc = "AHB3 peripheral clock enable register" ]
# [ repr ( C ) ]
pub struct Ahb3enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB3 peripheral clock enable register" ]
pub mod ahb3enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb3enr {
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
    # [ doc = "Value of the field FSMCEN" ]
    pub struct FsmcenR {
        bits: u8,
    }
    impl FsmcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FsmcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FsmcenW<'a> {
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
        fn _fsmcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Flexible static memory controller module clock enable" ]
        pub fn fsmcen(&self) -> FsmcenR {
            FsmcenR { bits: self._fsmcen() }
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
        # [ doc = "Bit 0 - Flexible static memory controller module clock enable" ]
        pub fn fsmcen(&mut self) -> _FsmcenW {
            _FsmcenW { register: self }
        }
    }
}

# [ doc = "APB1 peripheral clock enable register" ]
# [ repr ( C ) ]
pub struct Apb1enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB1 peripheral clock enable register" ]
pub mod apb1enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb1enr {
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
    # [ doc = "Value of the field DACEN" ]
    pub struct DacenR {
        bits: u8,
    }
    impl DacenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PWREN" ]
    pub struct PwrenR {
        bits: u8,
    }
    impl PwrenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN2EN" ]
    pub struct Can2enR {
        bits: u8,
    }
    impl Can2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN1EN" ]
    pub struct Can1enR {
        bits: u8,
    }
    impl Can1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C3EN" ]
    pub struct I2c3enR {
        bits: u8,
    }
    impl I2c3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C2EN" ]
    pub struct I2c2enR {
        bits: u8,
    }
    impl I2c2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1EN" ]
    pub struct I2c1enR {
        bits: u8,
    }
    impl I2c1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART5EN" ]
    pub struct Uart5enR {
        bits: u8,
    }
    impl Uart5enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART4EN" ]
    pub struct Uart4enR {
        bits: u8,
    }
    impl Uart4enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART3EN" ]
    pub struct Usart3enR {
        bits: u8,
    }
    impl Usart3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART2EN" ]
    pub struct Usart2enR {
        bits: u8,
    }
    impl Usart2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI3EN" ]
    pub struct Spi3enR {
        bits: u8,
    }
    impl Spi3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI2EN" ]
    pub struct Spi2enR {
        bits: u8,
    }
    impl Spi2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGEN" ]
    pub struct WwdgenR {
        bits: u8,
    }
    impl WwdgenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM14EN" ]
    pub struct Tim14enR {
        bits: u8,
    }
    impl Tim14enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM13EN" ]
    pub struct Tim13enR {
        bits: u8,
    }
    impl Tim13enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM12EN" ]
    pub struct Tim12enR {
        bits: u8,
    }
    impl Tim12enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM7EN" ]
    pub struct Tim7enR {
        bits: u8,
    }
    impl Tim7enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM6EN" ]
    pub struct Tim6enR {
        bits: u8,
    }
    impl Tim6enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM5EN" ]
    pub struct Tim5enR {
        bits: u8,
    }
    impl Tim5enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM4EN" ]
    pub struct Tim4enR {
        bits: u8,
    }
    impl Tim4enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM3EN" ]
    pub struct Tim3enR {
        bits: u8,
    }
    impl Tim3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM2EN" ]
    pub struct Tim2enR {
        bits: u8,
    }
    impl Tim2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DacenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DacenW<'a> {
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
    pub struct _PwrenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PwrenW<'a> {
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
    pub struct _Can2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can2enW<'a> {
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
    pub struct _Can1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can1enW<'a> {
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
    pub struct _I2c3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c3enW<'a> {
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
    pub struct _I2c2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c2enW<'a> {
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
    pub struct _I2c1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1enW<'a> {
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
    pub struct _Uart5enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart5enW<'a> {
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
    pub struct _Uart4enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart4enW<'a> {
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
    pub struct _Usart3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart3enW<'a> {
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
    pub struct _Usart2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart2enW<'a> {
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
    pub struct _Spi3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi3enW<'a> {
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
    pub struct _Spi2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi2enW<'a> {
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
    pub struct _WwdgenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgenW<'a> {
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
    pub struct _Tim14enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim14enW<'a> {
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
    pub struct _Tim13enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim13enW<'a> {
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
    pub struct _Tim12enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim12enW<'a> {
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
    pub struct _Tim7enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim7enW<'a> {
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
    pub struct _Tim6enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim6enW<'a> {
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
    pub struct _Tim5enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim5enW<'a> {
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
    pub struct _Tim4enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim4enW<'a> {
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
    pub struct _Tim3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim3enW<'a> {
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
    pub struct _Tim2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim2enW<'a> {
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
        fn _dacen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        pub fn dacen(&self) -> DacenR {
            DacenR { bits: self._dacen() }
        }
        fn _pwren(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        pub fn pwren(&self) -> PwrenR {
            PwrenR { bits: self._pwren() }
        }
        fn _can2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - CAN 2 clock enable" ]
        pub fn can2en(&self) -> Can2enR {
            Can2enR { bits: self._can2en() }
        }
        fn _can1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - CAN 1 clock enable" ]
        pub fn can1en(&self) -> Can1enR {
            Can1enR { bits: self._can1en() }
        }
        fn _i2c3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - I2C3 clock enable" ]
        pub fn i2c3en(&self) -> I2c3enR {
            I2c3enR { bits: self._i2c3en() }
        }
        fn _i2c2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I2C2 clock enable" ]
        pub fn i2c2en(&self) -> I2c2enR {
            I2c2enR { bits: self._i2c2en() }
        }
        fn _i2c1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - I2C1 clock enable" ]
        pub fn i2c1en(&self) -> I2c1enR {
            I2c1enR { bits: self._i2c1en() }
        }
        fn _uart5en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - UART5 clock enable" ]
        pub fn uart5en(&self) -> Uart5enR {
            Uart5enR { bits: self._uart5en() }
        }
        fn _uart4en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - UART4 clock enable" ]
        pub fn uart4en(&self) -> Uart4enR {
            Uart4enR { bits: self._uart4en() }
        }
        fn _usart3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - USART3 clock enable" ]
        pub fn usart3en(&self) -> Usart3enR {
            Usart3enR { bits: self._usart3en() }
        }
        fn _usart2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        pub fn usart2en(&self) -> Usart2enR {
            Usart2enR { bits: self._usart2en() }
        }
        fn _spi3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - SPI3 clock enable" ]
        pub fn spi3en(&self) -> Spi3enR {
            Spi3enR { bits: self._spi3en() }
        }
        fn _spi2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - SPI2 clock enable" ]
        pub fn spi2en(&self) -> Spi2enR {
            Spi2enR { bits: self._spi2en() }
        }
        fn _wwdgen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        pub fn wwdgen(&self) -> WwdgenR {
            WwdgenR { bits: self._wwdgen() }
        }
        fn _tim14en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - TIM14 clock enable" ]
        pub fn tim14en(&self) -> Tim14enR {
            Tim14enR { bits: self._tim14en() }
        }
        fn _tim13en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - TIM13 clock enable" ]
        pub fn tim13en(&self) -> Tim13enR {
            Tim13enR { bits: self._tim13en() }
        }
        fn _tim12en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - TIM12 clock enable" ]
        pub fn tim12en(&self) -> Tim12enR {
            Tim12enR { bits: self._tim12en() }
        }
        fn _tim7en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - TIM7 clock enable" ]
        pub fn tim7en(&self) -> Tim7enR {
            Tim7enR { bits: self._tim7en() }
        }
        fn _tim6en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - TIM6 clock enable" ]
        pub fn tim6en(&self) -> Tim6enR {
            Tim6enR { bits: self._tim6en() }
        }
        fn _tim5en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - TIM5 clock enable" ]
        pub fn tim5en(&self) -> Tim5enR {
            Tim5enR { bits: self._tim5en() }
        }
        fn _tim4en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - TIM4 clock enable" ]
        pub fn tim4en(&self) -> Tim4enR {
            Tim4enR { bits: self._tim4en() }
        }
        fn _tim3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM3 clock enable" ]
        pub fn tim3en(&self) -> Tim3enR {
            Tim3enR { bits: self._tim3en() }
        }
        fn _tim2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM2 clock enable" ]
        pub fn tim2en(&self) -> Tim2enR {
            Tim2enR { bits: self._tim2en() }
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
        # [ doc = "Bit 29 - DAC interface clock enable" ]
        pub fn dacen(&mut self) -> _DacenW {
            _DacenW { register: self }
        }
        # [ doc = "Bit 28 - Power interface clock enable" ]
        pub fn pwren(&mut self) -> _PwrenW {
            _PwrenW { register: self }
        }
        # [ doc = "Bit 26 - CAN 2 clock enable" ]
        pub fn can2en(&mut self) -> _Can2enW {
            _Can2enW { register: self }
        }
        # [ doc = "Bit 25 - CAN 1 clock enable" ]
        pub fn can1en(&mut self) -> _Can1enW {
            _Can1enW { register: self }
        }
        # [ doc = "Bit 23 - I2C3 clock enable" ]
        pub fn i2c3en(&mut self) -> _I2c3enW {
            _I2c3enW { register: self }
        }
        # [ doc = "Bit 22 - I2C2 clock enable" ]
        pub fn i2c2en(&mut self) -> _I2c2enW {
            _I2c2enW { register: self }
        }
        # [ doc = "Bit 21 - I2C1 clock enable" ]
        pub fn i2c1en(&mut self) -> _I2c1enW {
            _I2c1enW { register: self }
        }
        # [ doc = "Bit 20 - UART5 clock enable" ]
        pub fn uart5en(&mut self) -> _Uart5enW {
            _Uart5enW { register: self }
        }
        # [ doc = "Bit 19 - UART4 clock enable" ]
        pub fn uart4en(&mut self) -> _Uart4enW {
            _Uart4enW { register: self }
        }
        # [ doc = "Bit 18 - USART3 clock enable" ]
        pub fn usart3en(&mut self) -> _Usart3enW {
            _Usart3enW { register: self }
        }
        # [ doc = "Bit 17 - USART 2 clock enable" ]
        pub fn usart2en(&mut self) -> _Usart2enW {
            _Usart2enW { register: self }
        }
        # [ doc = "Bit 15 - SPI3 clock enable" ]
        pub fn spi3en(&mut self) -> _Spi3enW {
            _Spi3enW { register: self }
        }
        # [ doc = "Bit 14 - SPI2 clock enable" ]
        pub fn spi2en(&mut self) -> _Spi2enW {
            _Spi2enW { register: self }
        }
        # [ doc = "Bit 11 - Window watchdog clock enable" ]
        pub fn wwdgen(&mut self) -> _WwdgenW {
            _WwdgenW { register: self }
        }
        # [ doc = "Bit 8 - TIM14 clock enable" ]
        pub fn tim14en(&mut self) -> _Tim14enW {
            _Tim14enW { register: self }
        }
        # [ doc = "Bit 7 - TIM13 clock enable" ]
        pub fn tim13en(&mut self) -> _Tim13enW {
            _Tim13enW { register: self }
        }
        # [ doc = "Bit 6 - TIM12 clock enable" ]
        pub fn tim12en(&mut self) -> _Tim12enW {
            _Tim12enW { register: self }
        }
        # [ doc = "Bit 5 - TIM7 clock enable" ]
        pub fn tim7en(&mut self) -> _Tim7enW {
            _Tim7enW { register: self }
        }
        # [ doc = "Bit 4 - TIM6 clock enable" ]
        pub fn tim6en(&mut self) -> _Tim6enW {
            _Tim6enW { register: self }
        }
        # [ doc = "Bit 3 - TIM5 clock enable" ]
        pub fn tim5en(&mut self) -> _Tim5enW {
            _Tim5enW { register: self }
        }
        # [ doc = "Bit 2 - TIM4 clock enable" ]
        pub fn tim4en(&mut self) -> _Tim4enW {
            _Tim4enW { register: self }
        }
        # [ doc = "Bit 1 - TIM3 clock enable" ]
        pub fn tim3en(&mut self) -> _Tim3enW {
            _Tim3enW { register: self }
        }
        # [ doc = "Bit 0 - TIM2 clock enable" ]
        pub fn tim2en(&mut self) -> _Tim2enW {
            _Tim2enW { register: self }
        }
    }
}

# [ doc = "APB2 peripheral clock enable register" ]
# [ repr ( C ) ]
pub struct Apb2enr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB2 peripheral clock enable register" ]
pub mod apb2enr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb2enr {
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
    # [ doc = "Value of the field TIM11EN" ]
    pub struct Tim11enR {
        bits: u8,
    }
    impl Tim11enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM10EN" ]
    pub struct Tim10enR {
        bits: u8,
    }
    impl Tim10enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM9EN" ]
    pub struct Tim9enR {
        bits: u8,
    }
    impl Tim9enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SYSCFGEN" ]
    pub struct SyscfgenR {
        bits: u8,
    }
    impl SyscfgenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI1EN" ]
    pub struct Spi1enR {
        bits: u8,
    }
    impl Spi1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SDIOEN" ]
    pub struct SdioenR {
        bits: u8,
    }
    impl SdioenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC3EN" ]
    pub struct Adc3enR {
        bits: u8,
    }
    impl Adc3enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC2EN" ]
    pub struct Adc2enR {
        bits: u8,
    }
    impl Adc2enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC1EN" ]
    pub struct Adc1enR {
        bits: u8,
    }
    impl Adc1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART6EN" ]
    pub struct Usart6enR {
        bits: u8,
    }
    impl Usart6enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART1EN" ]
    pub struct Usart1enR {
        bits: u8,
    }
    impl Usart1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM8EN" ]
    pub struct Tim8enR {
        bits: u8,
    }
    impl Tim8enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM1EN" ]
    pub struct Tim1enR {
        bits: u8,
    }
    impl Tim1enR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Tim11enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim11enW<'a> {
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
    pub struct _Tim10enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim10enW<'a> {
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
    pub struct _Tim9enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim9enW<'a> {
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
    pub struct _SyscfgenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SyscfgenW<'a> {
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
    pub struct _Spi1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi1enW<'a> {
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
    pub struct _SdioenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SdioenW<'a> {
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
    pub struct _Adc3enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc3enW<'a> {
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
    pub struct _Adc2enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc2enW<'a> {
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
    pub struct _Adc1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc1enW<'a> {
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
    pub struct _Usart6enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart6enW<'a> {
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
    pub struct _Usart1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1enW<'a> {
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
    pub struct _Tim8enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim8enW<'a> {
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
    pub struct _Tim1enW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim1enW<'a> {
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
        fn _tim11en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - TIM11 clock enable" ]
        pub fn tim11en(&self) -> Tim11enR {
            Tim11enR { bits: self._tim11en() }
        }
        fn _tim10en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TIM10 clock enable" ]
        pub fn tim10en(&self) -> Tim10enR {
            Tim10enR { bits: self._tim10en() }
        }
        fn _tim9en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - TIM9 clock enable" ]
        pub fn tim9en(&self) -> Tim9enR {
            Tim9enR { bits: self._tim9en() }
        }
        fn _syscfgen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - System configuration controller clock enable" ]
        pub fn syscfgen(&self) -> SyscfgenR {
            SyscfgenR { bits: self._syscfgen() }
        }
        fn _spi1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - SPI1 clock enable" ]
        pub fn spi1en(&self) -> Spi1enR {
            Spi1enR { bits: self._spi1en() }
        }
        fn _sdioen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - SDIO clock enable" ]
        pub fn sdioen(&self) -> SdioenR {
            SdioenR { bits: self._sdioen() }
        }
        fn _adc3en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - ADC3 clock enable" ]
        pub fn adc3en(&self) -> Adc3enR {
            Adc3enR { bits: self._adc3en() }
        }
        fn _adc2en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - ADC2 clock enable" ]
        pub fn adc2en(&self) -> Adc2enR {
            Adc2enR { bits: self._adc2en() }
        }
        fn _adc1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - ADC1 clock enable" ]
        pub fn adc1en(&self) -> Adc1enR {
            Adc1enR { bits: self._adc1en() }
        }
        fn _usart6en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - USART6 clock enable" ]
        pub fn usart6en(&self) -> Usart6enR {
            Usart6enR { bits: self._usart6en() }
        }
        fn _usart1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - USART1 clock enable" ]
        pub fn usart1en(&self) -> Usart1enR {
            Usart1enR { bits: self._usart1en() }
        }
        fn _tim8en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM8 clock enable" ]
        pub fn tim8en(&self) -> Tim8enR {
            Tim8enR { bits: self._tim8en() }
        }
        fn _tim1en(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM1 clock enable" ]
        pub fn tim1en(&self) -> Tim1enR {
            Tim1enR { bits: self._tim1en() }
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
        # [ doc = "Bit 18 - TIM11 clock enable" ]
        pub fn tim11en(&mut self) -> _Tim11enW {
            _Tim11enW { register: self }
        }
        # [ doc = "Bit 17 - TIM10 clock enable" ]
        pub fn tim10en(&mut self) -> _Tim10enW {
            _Tim10enW { register: self }
        }
        # [ doc = "Bit 16 - TIM9 clock enable" ]
        pub fn tim9en(&mut self) -> _Tim9enW {
            _Tim9enW { register: self }
        }
        # [ doc = "Bit 14 - System configuration controller clock enable" ]
        pub fn syscfgen(&mut self) -> _SyscfgenW {
            _SyscfgenW { register: self }
        }
        # [ doc = "Bit 12 - SPI1 clock enable" ]
        pub fn spi1en(&mut self) -> _Spi1enW {
            _Spi1enW { register: self }
        }
        # [ doc = "Bit 11 - SDIO clock enable" ]
        pub fn sdioen(&mut self) -> _SdioenW {
            _SdioenW { register: self }
        }
        # [ doc = "Bit 10 - ADC3 clock enable" ]
        pub fn adc3en(&mut self) -> _Adc3enW {
            _Adc3enW { register: self }
        }
        # [ doc = "Bit 9 - ADC2 clock enable" ]
        pub fn adc2en(&mut self) -> _Adc2enW {
            _Adc2enW { register: self }
        }
        # [ doc = "Bit 8 - ADC1 clock enable" ]
        pub fn adc1en(&mut self) -> _Adc1enW {
            _Adc1enW { register: self }
        }
        # [ doc = "Bit 5 - USART6 clock enable" ]
        pub fn usart6en(&mut self) -> _Usart6enW {
            _Usart6enW { register: self }
        }
        # [ doc = "Bit 4 - USART1 clock enable" ]
        pub fn usart1en(&mut self) -> _Usart1enW {
            _Usart1enW { register: self }
        }
        # [ doc = "Bit 1 - TIM8 clock enable" ]
        pub fn tim8en(&mut self) -> _Tim8enW {
            _Tim8enW { register: self }
        }
        # [ doc = "Bit 0 - TIM1 clock enable" ]
        pub fn tim1en(&mut self) -> _Tim1enW {
            _Tim1enW { register: self }
        }
    }
}

# [ doc = "AHB1 peripheral clock enable in low power mode register" ]
# [ repr ( C ) ]
pub struct Ahb1lpenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB1 peripheral clock enable in low power mode register" ]
pub mod ahb1lpenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb1lpenr {
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
    # [ doc = "Value of the field OTGHSULPILPEN" ]
    pub struct OtghsulpilpenR {
        bits: u8,
    }
    impl OtghsulpilpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field OTGHSLPEN" ]
    pub struct OtghslpenR {
        bits: u8,
    }
    impl OtghslpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACPTPLPEN" ]
    pub struct EthmacptplpenR {
        bits: u8,
    }
    impl EthmacptplpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACRXLPEN" ]
    pub struct EthmacrxlpenR {
        bits: u8,
    }
    impl EthmacrxlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACTXLPEN" ]
    pub struct EthmactxlpenR {
        bits: u8,
    }
    impl EthmactxlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ETHMACLPEN" ]
    pub struct EthmaclpenR {
        bits: u8,
    }
    impl EthmaclpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA2LPEN" ]
    pub struct Dma2lpenR {
        bits: u8,
    }
    impl Dma2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DMA1LPEN" ]
    pub struct Dma1lpenR {
        bits: u8,
    }
    impl Dma1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BKPSRAMLPEN" ]
    pub struct BkpsramlpenR {
        bits: u8,
    }
    impl BkpsramlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SRAM2LPEN" ]
    pub struct Sram2lpenR {
        bits: u8,
    }
    impl Sram2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SRAM1LPEN" ]
    pub struct Sram1lpenR {
        bits: u8,
    }
    impl Sram1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field FLITFLPEN" ]
    pub struct FlitflpenR {
        bits: u8,
    }
    impl FlitflpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CRCLPEN" ]
    pub struct CrclpenR {
        bits: u8,
    }
    impl CrclpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOILPEN" ]
    pub struct GpioilpenR {
        bits: u8,
    }
    impl GpioilpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOHLPEN" ]
    pub struct GpiohlpenR {
        bits: u8,
    }
    impl GpiohlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOGLPEN" ]
    pub struct GpioglpenR {
        bits: u8,
    }
    impl GpioglpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOFLPEN" ]
    pub struct GpioflpenR {
        bits: u8,
    }
    impl GpioflpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOELPEN" ]
    pub struct GpioelpenR {
        bits: u8,
    }
    impl GpioelpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIODLPEN" ]
    pub struct GpiodlpenR {
        bits: u8,
    }
    impl GpiodlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOCLPEN" ]
    pub struct GpioclpenR {
        bits: u8,
    }
    impl GpioclpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOBLPEN" ]
    pub struct GpioblpenR {
        bits: u8,
    }
    impl GpioblpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field GPIOALPEN" ]
    pub struct GpioalpenR {
        bits: u8,
    }
    impl GpioalpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtghsulpilpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtghsulpilpenW<'a> {
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
    pub struct _OtghslpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtghslpenW<'a> {
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
    pub struct _EthmacptplpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacptplpenW<'a> {
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
    pub struct _EthmacrxlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmacrxlpenW<'a> {
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
    pub struct _EthmactxlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmactxlpenW<'a> {
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
    pub struct _EthmaclpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _EthmaclpenW<'a> {
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
    pub struct _Dma2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma2lpenW<'a> {
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
    pub struct _Dma1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Dma1lpenW<'a> {
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
    pub struct _BkpsramlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BkpsramlpenW<'a> {
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
    pub struct _Sram2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sram2lpenW<'a> {
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
    pub struct _Sram1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Sram1lpenW<'a> {
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
    pub struct _FlitflpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FlitflpenW<'a> {
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
    pub struct _CrclpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _CrclpenW<'a> {
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
    pub struct _GpioilpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioilpenW<'a> {
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
    pub struct _GpiohlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiohlpenW<'a> {
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
    pub struct _GpioglpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioglpenW<'a> {
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
    pub struct _GpioflpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioflpenW<'a> {
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
    pub struct _GpioelpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioelpenW<'a> {
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
    pub struct _GpiodlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpiodlpenW<'a> {
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
    pub struct _GpioclpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioclpenW<'a> {
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
    pub struct _GpioblpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioblpenW<'a> {
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
    pub struct _GpioalpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _GpioalpenW<'a> {
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
        fn _otghsulpilpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode" ]
        pub fn otghsulpilpen(&self) -> OtghsulpilpenR {
            OtghsulpilpenR { bits: self._otghsulpilpen() }
        }
        fn _otghslpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - USB OTG HS clock enable during Sleep mode" ]
        pub fn otghslpen(&self) -> OtghslpenR {
            OtghslpenR { bits: self._otghslpen() }
        }
        fn _ethmacptplpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode" ]
        pub fn ethmacptplpen(&self) -> EthmacptplpenR {
            EthmacptplpenR { bits: self._ethmacptplpen() }
        }
        fn _ethmacrxlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - Ethernet reception clock enable during Sleep mode" ]
        pub fn ethmacrxlpen(&self) -> EthmacrxlpenR {
            EthmacrxlpenR { bits: self._ethmacrxlpen() }
        }
        fn _ethmactxlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode" ]
        pub fn ethmactxlpen(&self) -> EthmactxlpenR {
            EthmactxlpenR { bits: self._ethmactxlpen() }
        }
        fn _ethmaclpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode" ]
        pub fn ethmaclpen(&self) -> EthmaclpenR {
            EthmaclpenR { bits: self._ethmaclpen() }
        }
        fn _dma2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - DMA2 clock enable during Sleep mode" ]
        pub fn dma2lpen(&self) -> Dma2lpenR {
            Dma2lpenR { bits: self._dma2lpen() }
        }
        fn _dma1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - DMA1 clock enable during Sleep mode" ]
        pub fn dma1lpen(&self) -> Dma1lpenR {
            Dma1lpenR { bits: self._dma1lpen() }
        }
        fn _bkpsramlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode" ]
        pub fn bkpsramlpen(&self) -> BkpsramlpenR {
            BkpsramlpenR { bits: self._bkpsramlpen() }
        }
        fn _sram2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode" ]
        pub fn sram2lpen(&self) -> Sram2lpenR {
            Sram2lpenR { bits: self._sram2lpen() }
        }
        fn _sram1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode" ]
        pub fn sram1lpen(&self) -> Sram1lpenR {
            Sram1lpenR { bits: self._sram1lpen() }
        }
        fn _flitflpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - Flash interface clock enable during Sleep mode" ]
        pub fn flitflpen(&self) -> FlitflpenR {
            FlitflpenR { bits: self._flitflpen() }
        }
        fn _crclpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - CRC clock enable during Sleep mode" ]
        pub fn crclpen(&self) -> CrclpenR {
            CrclpenR { bits: self._crclpen() }
        }
        fn _gpioilpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - IO port I clock enable during Sleep mode" ]
        pub fn gpioilpen(&self) -> GpioilpenR {
            GpioilpenR { bits: self._gpioilpen() }
        }
        fn _gpiohlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - IO port H clock enable during Sleep mode" ]
        pub fn gpiohlpen(&self) -> GpiohlpenR {
            GpiohlpenR { bits: self._gpiohlpen() }
        }
        fn _gpioglpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - IO port G clock enable during Sleep mode" ]
        pub fn gpioglpen(&self) -> GpioglpenR {
            GpioglpenR { bits: self._gpioglpen() }
        }
        fn _gpioflpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - IO port F clock enable during Sleep mode" ]
        pub fn gpioflpen(&self) -> GpioflpenR {
            GpioflpenR { bits: self._gpioflpen() }
        }
        fn _gpioelpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - IO port E clock enable during Sleep mode" ]
        pub fn gpioelpen(&self) -> GpioelpenR {
            GpioelpenR { bits: self._gpioelpen() }
        }
        fn _gpiodlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - IO port D clock enable during Sleep mode" ]
        pub fn gpiodlpen(&self) -> GpiodlpenR {
            GpiodlpenR { bits: self._gpiodlpen() }
        }
        fn _gpioclpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - IO port C clock enable during Sleep mode" ]
        pub fn gpioclpen(&self) -> GpioclpenR {
            GpioclpenR { bits: self._gpioclpen() }
        }
        fn _gpioblpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - IO port B clock enable during Sleep mode" ]
        pub fn gpioblpen(&self) -> GpioblpenR {
            GpioblpenR { bits: self._gpioblpen() }
        }
        fn _gpioalpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - IO port A clock enable during sleep mode" ]
        pub fn gpioalpen(&self) -> GpioalpenR {
            GpioalpenR { bits: self._gpioalpen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 2120716799 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 30 - USB OTG HS ULPI clock enable during Sleep mode" ]
        pub fn otghsulpilpen(&mut self) -> _OtghsulpilpenW {
            _OtghsulpilpenW { register: self }
        }
        # [ doc = "Bit 29 - USB OTG HS clock enable during Sleep mode" ]
        pub fn otghslpen(&mut self) -> _OtghslpenW {
            _OtghslpenW { register: self }
        }
        # [ doc = "Bit 28 - Ethernet PTP clock enable during Sleep mode" ]
        pub fn ethmacptplpen(&mut self) -> _EthmacptplpenW {
            _EthmacptplpenW { register: self }
        }
        # [ doc = "Bit 27 - Ethernet reception clock enable during Sleep mode" ]
        pub fn ethmacrxlpen(&mut self) -> _EthmacrxlpenW {
            _EthmacrxlpenW { register: self }
        }
        # [ doc = "Bit 26 - Ethernet transmission clock enable during Sleep mode" ]
        pub fn ethmactxlpen(&mut self) -> _EthmactxlpenW {
            _EthmactxlpenW { register: self }
        }
        # [ doc = "Bit 25 - Ethernet MAC clock enable during Sleep mode" ]
        pub fn ethmaclpen(&mut self) -> _EthmaclpenW {
            _EthmaclpenW { register: self }
        }
        # [ doc = "Bit 22 - DMA2 clock enable during Sleep mode" ]
        pub fn dma2lpen(&mut self) -> _Dma2lpenW {
            _Dma2lpenW { register: self }
        }
        # [ doc = "Bit 21 - DMA1 clock enable during Sleep mode" ]
        pub fn dma1lpen(&mut self) -> _Dma1lpenW {
            _Dma1lpenW { register: self }
        }
        # [ doc = "Bit 18 - Backup SRAM interface clock enable during Sleep mode" ]
        pub fn bkpsramlpen(&mut self) -> _BkpsramlpenW {
            _BkpsramlpenW { register: self }
        }
        # [ doc = "Bit 17 - SRAM 2 interface clock enable during Sleep mode" ]
        pub fn sram2lpen(&mut self) -> _Sram2lpenW {
            _Sram2lpenW { register: self }
        }
        # [ doc = "Bit 16 - SRAM 1interface clock enable during Sleep mode" ]
        pub fn sram1lpen(&mut self) -> _Sram1lpenW {
            _Sram1lpenW { register: self }
        }
        # [ doc = "Bit 15 - Flash interface clock enable during Sleep mode" ]
        pub fn flitflpen(&mut self) -> _FlitflpenW {
            _FlitflpenW { register: self }
        }
        # [ doc = "Bit 12 - CRC clock enable during Sleep mode" ]
        pub fn crclpen(&mut self) -> _CrclpenW {
            _CrclpenW { register: self }
        }
        # [ doc = "Bit 8 - IO port I clock enable during Sleep mode" ]
        pub fn gpioilpen(&mut self) -> _GpioilpenW {
            _GpioilpenW { register: self }
        }
        # [ doc = "Bit 7 - IO port H clock enable during Sleep mode" ]
        pub fn gpiohlpen(&mut self) -> _GpiohlpenW {
            _GpiohlpenW { register: self }
        }
        # [ doc = "Bit 6 - IO port G clock enable during Sleep mode" ]
        pub fn gpioglpen(&mut self) -> _GpioglpenW {
            _GpioglpenW { register: self }
        }
        # [ doc = "Bit 5 - IO port F clock enable during Sleep mode" ]
        pub fn gpioflpen(&mut self) -> _GpioflpenW {
            _GpioflpenW { register: self }
        }
        # [ doc = "Bit 4 - IO port E clock enable during Sleep mode" ]
        pub fn gpioelpen(&mut self) -> _GpioelpenW {
            _GpioelpenW { register: self }
        }
        # [ doc = "Bit 3 - IO port D clock enable during Sleep mode" ]
        pub fn gpiodlpen(&mut self) -> _GpiodlpenW {
            _GpiodlpenW { register: self }
        }
        # [ doc = "Bit 2 - IO port C clock enable during Sleep mode" ]
        pub fn gpioclpen(&mut self) -> _GpioclpenW {
            _GpioclpenW { register: self }
        }
        # [ doc = "Bit 1 - IO port B clock enable during Sleep mode" ]
        pub fn gpioblpen(&mut self) -> _GpioblpenW {
            _GpioblpenW { register: self }
        }
        # [ doc = "Bit 0 - IO port A clock enable during sleep mode" ]
        pub fn gpioalpen(&mut self) -> _GpioalpenW {
            _GpioalpenW { register: self }
        }
    }
}

# [ doc = "AHB2 peripheral clock enable in low power mode register" ]
# [ repr ( C ) ]
pub struct Ahb2lpenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB2 peripheral clock enable in low power mode register" ]
pub mod ahb2lpenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb2lpenr {
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
    # [ doc = "Value of the field OTGFSLPEN" ]
    pub struct OtgfslpenR {
        bits: u8,
    }
    impl OtgfslpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RNGLPEN" ]
    pub struct RnglpenR {
        bits: u8,
    }
    impl RnglpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field DCMILPEN" ]
    pub struct DcmilpenR {
        bits: u8,
    }
    impl DcmilpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _OtgfslpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _OtgfslpenW<'a> {
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
    pub struct _RnglpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RnglpenW<'a> {
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
    pub struct _DcmilpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DcmilpenW<'a> {
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
        fn _otgfslpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - USB OTG FS clock enable during Sleep mode" ]
        pub fn otgfslpen(&self) -> OtgfslpenR {
            OtgfslpenR { bits: self._otgfslpen() }
        }
        fn _rnglpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - Random number generator clock enable during Sleep mode" ]
        pub fn rnglpen(&self) -> RnglpenR {
            RnglpenR { bits: self._rnglpen() }
        }
        fn _dcmilpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Camera interface enable during Sleep mode" ]
        pub fn dcmilpen(&self) -> DcmilpenR {
            DcmilpenR { bits: self._dcmilpen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 241 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 7 - USB OTG FS clock enable during Sleep mode" ]
        pub fn otgfslpen(&mut self) -> _OtgfslpenW {
            _OtgfslpenW { register: self }
        }
        # [ doc = "Bit 6 - Random number generator clock enable during Sleep mode" ]
        pub fn rnglpen(&mut self) -> _RnglpenW {
            _RnglpenW { register: self }
        }
        # [ doc = "Bit 0 - Camera interface enable during Sleep mode" ]
        pub fn dcmilpen(&mut self) -> _DcmilpenW {
            _DcmilpenW { register: self }
        }
    }
}

# [ doc = "AHB3 peripheral clock enable in low power mode register" ]
# [ repr ( C ) ]
pub struct Ahb3lpenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "AHB3 peripheral clock enable in low power mode register" ]
pub mod ahb3lpenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Ahb3lpenr {
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
    # [ doc = "Value of the field FSMCLPEN" ]
    pub struct FsmclpenR {
        bits: u8,
    }
    impl FsmclpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _FsmclpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _FsmclpenW<'a> {
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
        fn _fsmclpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode" ]
        pub fn fsmclpen(&self) -> FsmclpenR {
            FsmclpenR { bits: self._fsmclpen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 1 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 0 - Flexible static memory controller module clock enable during Sleep mode" ]
        pub fn fsmclpen(&mut self) -> _FsmclpenW {
            _FsmclpenW { register: self }
        }
    }
}

# [ doc = "APB1 peripheral clock enable in low power mode register" ]
# [ repr ( C ) ]
pub struct Apb1lpenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB1 peripheral clock enable in low power mode register" ]
pub mod apb1lpenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb1lpenr {
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
    # [ doc = "Value of the field DACLPEN" ]
    pub struct DaclpenR {
        bits: u8,
    }
    impl DaclpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PWRLPEN" ]
    pub struct PwrlpenR {
        bits: u8,
    }
    impl PwrlpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN2LPEN" ]
    pub struct Can2lpenR {
        bits: u8,
    }
    impl Can2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field CAN1LPEN" ]
    pub struct Can1lpenR {
        bits: u8,
    }
    impl Can1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C3LPEN" ]
    pub struct I2c3lpenR {
        bits: u8,
    }
    impl I2c3lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C2LPEN" ]
    pub struct I2c2lpenR {
        bits: u8,
    }
    impl I2c2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field I2C1LPEN" ]
    pub struct I2c1lpenR {
        bits: u8,
    }
    impl I2c1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART5LPEN" ]
    pub struct Uart5lpenR {
        bits: u8,
    }
    impl Uart5lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field UART4LPEN" ]
    pub struct Uart4lpenR {
        bits: u8,
    }
    impl Uart4lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART3LPEN" ]
    pub struct Usart3lpenR {
        bits: u8,
    }
    impl Usart3lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART2LPEN" ]
    pub struct Usart2lpenR {
        bits: u8,
    }
    impl Usart2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI3LPEN" ]
    pub struct Spi3lpenR {
        bits: u8,
    }
    impl Spi3lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI2LPEN" ]
    pub struct Spi2lpenR {
        bits: u8,
    }
    impl Spi2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGLPEN" ]
    pub struct WwdglpenR {
        bits: u8,
    }
    impl WwdglpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM14LPEN" ]
    pub struct Tim14lpenR {
        bits: u8,
    }
    impl Tim14lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM13LPEN" ]
    pub struct Tim13lpenR {
        bits: u8,
    }
    impl Tim13lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM12LPEN" ]
    pub struct Tim12lpenR {
        bits: u8,
    }
    impl Tim12lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM7LPEN" ]
    pub struct Tim7lpenR {
        bits: u8,
    }
    impl Tim7lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM6LPEN" ]
    pub struct Tim6lpenR {
        bits: u8,
    }
    impl Tim6lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM5LPEN" ]
    pub struct Tim5lpenR {
        bits: u8,
    }
    impl Tim5lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM4LPEN" ]
    pub struct Tim4lpenR {
        bits: u8,
    }
    impl Tim4lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM3LPEN" ]
    pub struct Tim3lpenR {
        bits: u8,
    }
    impl Tim3lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM2LPEN" ]
    pub struct Tim2lpenR {
        bits: u8,
    }
    impl Tim2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _DaclpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _DaclpenW<'a> {
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
    pub struct _PwrlpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PwrlpenW<'a> {
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
    pub struct _Can2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can2lpenW<'a> {
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
    pub struct _Can1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Can1lpenW<'a> {
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
    pub struct _I2c3lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c3lpenW<'a> {
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
    pub struct _I2c2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c2lpenW<'a> {
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
    pub struct _I2c1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _I2c1lpenW<'a> {
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
    pub struct _Uart5lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart5lpenW<'a> {
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
    pub struct _Uart4lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Uart4lpenW<'a> {
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
    pub struct _Usart3lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart3lpenW<'a> {
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
    pub struct _Usart2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart2lpenW<'a> {
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
    pub struct _Spi3lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi3lpenW<'a> {
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
    pub struct _Spi2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi2lpenW<'a> {
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
    pub struct _WwdglpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdglpenW<'a> {
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
    pub struct _Tim14lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim14lpenW<'a> {
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
    pub struct _Tim13lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim13lpenW<'a> {
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
    pub struct _Tim12lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim12lpenW<'a> {
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
    pub struct _Tim7lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim7lpenW<'a> {
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
    pub struct _Tim6lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim6lpenW<'a> {
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
    pub struct _Tim5lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim5lpenW<'a> {
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
    pub struct _Tim4lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim4lpenW<'a> {
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
    pub struct _Tim3lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim3lpenW<'a> {
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
    pub struct _Tim2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim2lpenW<'a> {
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
        fn _daclpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - DAC interface clock enable during Sleep mode" ]
        pub fn daclpen(&self) -> DaclpenR {
            DaclpenR { bits: self._daclpen() }
        }
        fn _pwrlpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Power interface clock enable during Sleep mode" ]
        pub fn pwrlpen(&self) -> PwrlpenR {
            PwrlpenR { bits: self._pwrlpen() }
        }
        fn _can2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - CAN 2 clock enable during Sleep mode" ]
        pub fn can2lpen(&self) -> Can2lpenR {
            Can2lpenR { bits: self._can2lpen() }
        }
        fn _can1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - CAN 1 clock enable during Sleep mode" ]
        pub fn can1lpen(&self) -> Can1lpenR {
            Can1lpenR { bits: self._can1lpen() }
        }
        fn _i2c3lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 23 - I2C3 clock enable during Sleep mode" ]
        pub fn i2c3lpen(&self) -> I2c3lpenR {
            I2c3lpenR { bits: self._i2c3lpen() }
        }
        fn _i2c2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 22 - I2C2 clock enable during Sleep mode" ]
        pub fn i2c2lpen(&self) -> I2c2lpenR {
            I2c2lpenR { bits: self._i2c2lpen() }
        }
        fn _i2c1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 21 - I2C1 clock enable during Sleep mode" ]
        pub fn i2c1lpen(&self) -> I2c1lpenR {
            I2c1lpenR { bits: self._i2c1lpen() }
        }
        fn _uart5lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 20 - UART5 clock enable during Sleep mode" ]
        pub fn uart5lpen(&self) -> Uart5lpenR {
            Uart5lpenR { bits: self._uart5lpen() }
        }
        fn _uart4lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 19 - UART4 clock enable during Sleep mode" ]
        pub fn uart4lpen(&self) -> Uart4lpenR {
            Uart4lpenR { bits: self._uart4lpen() }
        }
        fn _usart3lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - USART3 clock enable during Sleep mode" ]
        pub fn usart3lpen(&self) -> Usart3lpenR {
            Usart3lpenR { bits: self._usart3lpen() }
        }
        fn _usart2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - USART2 clock enable during Sleep mode" ]
        pub fn usart2lpen(&self) -> Usart2lpenR {
            Usart2lpenR { bits: self._usart2lpen() }
        }
        fn _spi3lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - SPI3 clock enable during Sleep mode" ]
        pub fn spi3lpen(&self) -> Spi3lpenR {
            Spi3lpenR { bits: self._spi3lpen() }
        }
        fn _spi2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - SPI2 clock enable during Sleep mode" ]
        pub fn spi2lpen(&self) -> Spi2lpenR {
            Spi2lpenR { bits: self._spi2lpen() }
        }
        fn _wwdglpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - Window watchdog clock enable during Sleep mode" ]
        pub fn wwdglpen(&self) -> WwdglpenR {
            WwdglpenR { bits: self._wwdglpen() }
        }
        fn _tim14lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - TIM14 clock enable during Sleep mode" ]
        pub fn tim14lpen(&self) -> Tim14lpenR {
            Tim14lpenR { bits: self._tim14lpen() }
        }
        fn _tim13lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 7 - TIM13 clock enable during Sleep mode" ]
        pub fn tim13lpen(&self) -> Tim13lpenR {
            Tim13lpenR { bits: self._tim13lpen() }
        }
        fn _tim12lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 6 - TIM12 clock enable during Sleep mode" ]
        pub fn tim12lpen(&self) -> Tim12lpenR {
            Tim12lpenR { bits: self._tim12lpen() }
        }
        fn _tim7lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - TIM7 clock enable during Sleep mode" ]
        pub fn tim7lpen(&self) -> Tim7lpenR {
            Tim7lpenR { bits: self._tim7lpen() }
        }
        fn _tim6lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - TIM6 clock enable during Sleep mode" ]
        pub fn tim6lpen(&self) -> Tim6lpenR {
            Tim6lpenR { bits: self._tim6lpen() }
        }
        fn _tim5lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 3 - TIM5 clock enable during Sleep mode" ]
        pub fn tim5lpen(&self) -> Tim5lpenR {
            Tim5lpenR { bits: self._tim5lpen() }
        }
        fn _tim4lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - TIM4 clock enable during Sleep mode" ]
        pub fn tim4lpen(&self) -> Tim4lpenR {
            Tim4lpenR { bits: self._tim4lpen() }
        }
        fn _tim3lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM3 clock enable during Sleep mode" ]
        pub fn tim3lpen(&self) -> Tim3lpenR {
            Tim3lpenR { bits: self._tim3lpen() }
        }
        fn _tim2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM2 clock enable during Sleep mode" ]
        pub fn tim2lpen(&self) -> Tim2lpenR {
            Tim2lpenR { bits: self._tim2lpen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 922667519 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 29 - DAC interface clock enable during Sleep mode" ]
        pub fn daclpen(&mut self) -> _DaclpenW {
            _DaclpenW { register: self }
        }
        # [ doc = "Bit 28 - Power interface clock enable during Sleep mode" ]
        pub fn pwrlpen(&mut self) -> _PwrlpenW {
            _PwrlpenW { register: self }
        }
        # [ doc = "Bit 26 - CAN 2 clock enable during Sleep mode" ]
        pub fn can2lpen(&mut self) -> _Can2lpenW {
            _Can2lpenW { register: self }
        }
        # [ doc = "Bit 25 - CAN 1 clock enable during Sleep mode" ]
        pub fn can1lpen(&mut self) -> _Can1lpenW {
            _Can1lpenW { register: self }
        }
        # [ doc = "Bit 23 - I2C3 clock enable during Sleep mode" ]
        pub fn i2c3lpen(&mut self) -> _I2c3lpenW {
            _I2c3lpenW { register: self }
        }
        # [ doc = "Bit 22 - I2C2 clock enable during Sleep mode" ]
        pub fn i2c2lpen(&mut self) -> _I2c2lpenW {
            _I2c2lpenW { register: self }
        }
        # [ doc = "Bit 21 - I2C1 clock enable during Sleep mode" ]
        pub fn i2c1lpen(&mut self) -> _I2c1lpenW {
            _I2c1lpenW { register: self }
        }
        # [ doc = "Bit 20 - UART5 clock enable during Sleep mode" ]
        pub fn uart5lpen(&mut self) -> _Uart5lpenW {
            _Uart5lpenW { register: self }
        }
        # [ doc = "Bit 19 - UART4 clock enable during Sleep mode" ]
        pub fn uart4lpen(&mut self) -> _Uart4lpenW {
            _Uart4lpenW { register: self }
        }
        # [ doc = "Bit 18 - USART3 clock enable during Sleep mode" ]
        pub fn usart3lpen(&mut self) -> _Usart3lpenW {
            _Usart3lpenW { register: self }
        }
        # [ doc = "Bit 17 - USART2 clock enable during Sleep mode" ]
        pub fn usart2lpen(&mut self) -> _Usart2lpenW {
            _Usart2lpenW { register: self }
        }
        # [ doc = "Bit 15 - SPI3 clock enable during Sleep mode" ]
        pub fn spi3lpen(&mut self) -> _Spi3lpenW {
            _Spi3lpenW { register: self }
        }
        # [ doc = "Bit 14 - SPI2 clock enable during Sleep mode" ]
        pub fn spi2lpen(&mut self) -> _Spi2lpenW {
            _Spi2lpenW { register: self }
        }
        # [ doc = "Bit 11 - Window watchdog clock enable during Sleep mode" ]
        pub fn wwdglpen(&mut self) -> _WwdglpenW {
            _WwdglpenW { register: self }
        }
        # [ doc = "Bit 8 - TIM14 clock enable during Sleep mode" ]
        pub fn tim14lpen(&mut self) -> _Tim14lpenW {
            _Tim14lpenW { register: self }
        }
        # [ doc = "Bit 7 - TIM13 clock enable during Sleep mode" ]
        pub fn tim13lpen(&mut self) -> _Tim13lpenW {
            _Tim13lpenW { register: self }
        }
        # [ doc = "Bit 6 - TIM12 clock enable during Sleep mode" ]
        pub fn tim12lpen(&mut self) -> _Tim12lpenW {
            _Tim12lpenW { register: self }
        }
        # [ doc = "Bit 5 - TIM7 clock enable during Sleep mode" ]
        pub fn tim7lpen(&mut self) -> _Tim7lpenW {
            _Tim7lpenW { register: self }
        }
        # [ doc = "Bit 4 - TIM6 clock enable during Sleep mode" ]
        pub fn tim6lpen(&mut self) -> _Tim6lpenW {
            _Tim6lpenW { register: self }
        }
        # [ doc = "Bit 3 - TIM5 clock enable during Sleep mode" ]
        pub fn tim5lpen(&mut self) -> _Tim5lpenW {
            _Tim5lpenW { register: self }
        }
        # [ doc = "Bit 2 - TIM4 clock enable during Sleep mode" ]
        pub fn tim4lpen(&mut self) -> _Tim4lpenW {
            _Tim4lpenW { register: self }
        }
        # [ doc = "Bit 1 - TIM3 clock enable during Sleep mode" ]
        pub fn tim3lpen(&mut self) -> _Tim3lpenW {
            _Tim3lpenW { register: self }
        }
        # [ doc = "Bit 0 - TIM2 clock enable during Sleep mode" ]
        pub fn tim2lpen(&mut self) -> _Tim2lpenW {
            _Tim2lpenW { register: self }
        }
    }
}

# [ doc = "APB2 peripheral clock enabled in low power mode register" ]
# [ repr ( C ) ]
pub struct Apb2lpenr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "APB2 peripheral clock enabled in low power mode register" ]
pub mod apb2lpenr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Apb2lpenr {
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
    # [ doc = "Value of the field TIM11LPEN" ]
    pub struct Tim11lpenR {
        bits: u8,
    }
    impl Tim11lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM10LPEN" ]
    pub struct Tim10lpenR {
        bits: u8,
    }
    impl Tim10lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM9LPEN" ]
    pub struct Tim9lpenR {
        bits: u8,
    }
    impl Tim9lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SYSCFGLPEN" ]
    pub struct SyscfglpenR {
        bits: u8,
    }
    impl SyscfglpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPI1LPEN" ]
    pub struct Spi1lpenR {
        bits: u8,
    }
    impl Spi1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SDIOLPEN" ]
    pub struct SdiolpenR {
        bits: u8,
    }
    impl SdiolpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC3LPEN" ]
    pub struct Adc3lpenR {
        bits: u8,
    }
    impl Adc3lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC2LPEN" ]
    pub struct Adc2lpenR {
        bits: u8,
    }
    impl Adc2lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field ADC1LPEN" ]
    pub struct Adc1lpenR {
        bits: u8,
    }
    impl Adc1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART6LPEN" ]
    pub struct Usart6lpenR {
        bits: u8,
    }
    impl Usart6lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field USART1LPEN" ]
    pub struct Usart1lpenR {
        bits: u8,
    }
    impl Usart1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM8LPEN" ]
    pub struct Tim8lpenR {
        bits: u8,
    }
    impl Tim8lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field TIM1LPEN" ]
    pub struct Tim1lpenR {
        bits: u8,
    }
    impl Tim1lpenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Tim11lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim11lpenW<'a> {
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
    pub struct _Tim10lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim10lpenW<'a> {
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
    pub struct _Tim9lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim9lpenW<'a> {
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
    pub struct _SyscfglpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SyscfglpenW<'a> {
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
    pub struct _Spi1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Spi1lpenW<'a> {
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
    pub struct _SdiolpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SdiolpenW<'a> {
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
    pub struct _Adc3lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc3lpenW<'a> {
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
    pub struct _Adc2lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc2lpenW<'a> {
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
    pub struct _Adc1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Adc1lpenW<'a> {
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
    pub struct _Usart6lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart6lpenW<'a> {
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
    pub struct _Usart1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Usart1lpenW<'a> {
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
    pub struct _Tim8lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim8lpenW<'a> {
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
    pub struct _Tim1lpenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Tim1lpenW<'a> {
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
        fn _tim11lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 18 - TIM11 clock enable during Sleep mode" ]
        pub fn tim11lpen(&self) -> Tim11lpenR {
            Tim11lpenR { bits: self._tim11lpen() }
        }
        fn _tim10lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 17 - TIM10 clock enable during Sleep mode" ]
        pub fn tim10lpen(&self) -> Tim10lpenR {
            Tim10lpenR { bits: self._tim10lpen() }
        }
        fn _tim9lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - TIM9 clock enable during sleep mode" ]
        pub fn tim9lpen(&self) -> Tim9lpenR {
            Tim9lpenR { bits: self._tim9lpen() }
        }
        fn _syscfglpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 14 - System configuration controller clock enable during Sleep mode" ]
        pub fn syscfglpen(&self) -> SyscfglpenR {
            SyscfglpenR { bits: self._syscfglpen() }
        }
        fn _spi1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 12 - SPI 1 clock enable during Sleep mode" ]
        pub fn spi1lpen(&self) -> Spi1lpenR {
            Spi1lpenR { bits: self._spi1lpen() }
        }
        fn _sdiolpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 11 - SDIO clock enable during Sleep mode" ]
        pub fn sdiolpen(&self) -> SdiolpenR {
            SdiolpenR { bits: self._sdiolpen() }
        }
        fn _adc3lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 10 - ADC 3 clock enable during Sleep mode" ]
        pub fn adc3lpen(&self) -> Adc3lpenR {
            Adc3lpenR { bits: self._adc3lpen() }
        }
        fn _adc2lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - ADC2 clock enable during Sleep mode" ]
        pub fn adc2lpen(&self) -> Adc2lpenR {
            Adc2lpenR { bits: self._adc2lpen() }
        }
        fn _adc1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - ADC1 clock enable during Sleep mode" ]
        pub fn adc1lpen(&self) -> Adc1lpenR {
            Adc1lpenR { bits: self._adc1lpen() }
        }
        fn _usart6lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 5 - USART6 clock enable during Sleep mode" ]
        pub fn usart6lpen(&self) -> Usart6lpenR {
            Usart6lpenR { bits: self._usart6lpen() }
        }
        fn _usart1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 4 - USART1 clock enable during Sleep mode" ]
        pub fn usart1lpen(&self) -> Usart1lpenR {
            Usart1lpenR { bits: self._usart1lpen() }
        }
        fn _tim8lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - TIM8 clock enable during Sleep mode" ]
        pub fn tim8lpen(&self) -> Tim8lpenR {
            Tim8lpenR { bits: self._tim8lpen() }
        }
        fn _tim1lpen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - TIM1 clock enable during Sleep mode" ]
        pub fn tim1lpen(&self) -> Tim1lpenR {
            Tim1lpenR { bits: self._tim1lpen() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 483123 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 18 - TIM11 clock enable during Sleep mode" ]
        pub fn tim11lpen(&mut self) -> _Tim11lpenW {
            _Tim11lpenW { register: self }
        }
        # [ doc = "Bit 17 - TIM10 clock enable during Sleep mode" ]
        pub fn tim10lpen(&mut self) -> _Tim10lpenW {
            _Tim10lpenW { register: self }
        }
        # [ doc = "Bit 16 - TIM9 clock enable during sleep mode" ]
        pub fn tim9lpen(&mut self) -> _Tim9lpenW {
            _Tim9lpenW { register: self }
        }
        # [ doc = "Bit 14 - System configuration controller clock enable during Sleep mode" ]
        pub fn syscfglpen(&mut self) -> _SyscfglpenW {
            _SyscfglpenW { register: self }
        }
        # [ doc = "Bit 12 - SPI 1 clock enable during Sleep mode" ]
        pub fn spi1lpen(&mut self) -> _Spi1lpenW {
            _Spi1lpenW { register: self }
        }
        # [ doc = "Bit 11 - SDIO clock enable during Sleep mode" ]
        pub fn sdiolpen(&mut self) -> _SdiolpenW {
            _SdiolpenW { register: self }
        }
        # [ doc = "Bit 10 - ADC 3 clock enable during Sleep mode" ]
        pub fn adc3lpen(&mut self) -> _Adc3lpenW {
            _Adc3lpenW { register: self }
        }
        # [ doc = "Bit 9 - ADC2 clock enable during Sleep mode" ]
        pub fn adc2lpen(&mut self) -> _Adc2lpenW {
            _Adc2lpenW { register: self }
        }
        # [ doc = "Bit 8 - ADC1 clock enable during Sleep mode" ]
        pub fn adc1lpen(&mut self) -> _Adc1lpenW {
            _Adc1lpenW { register: self }
        }
        # [ doc = "Bit 5 - USART6 clock enable during Sleep mode" ]
        pub fn usart6lpen(&mut self) -> _Usart6lpenW {
            _Usart6lpenW { register: self }
        }
        # [ doc = "Bit 4 - USART1 clock enable during Sleep mode" ]
        pub fn usart1lpen(&mut self) -> _Usart1lpenW {
            _Usart1lpenW { register: self }
        }
        # [ doc = "Bit 1 - TIM8 clock enable during Sleep mode" ]
        pub fn tim8lpen(&mut self) -> _Tim8lpenW {
            _Tim8lpenW { register: self }
        }
        # [ doc = "Bit 0 - TIM1 clock enable during Sleep mode" ]
        pub fn tim1lpen(&mut self) -> _Tim1lpenW {
            _Tim1lpenW { register: self }
        }
    }
}

# [ doc = "Backup domain control register" ]
# [ repr ( C ) ]
pub struct Bdcr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "Backup domain control register" ]
pub mod bdcr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Bdcr {
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
    # [ doc = "Value of the field BDRST" ]
    pub struct BdrstR {
        bits: u8,
    }
    impl BdrstR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCEN" ]
    pub struct RtcenR {
        bits: u8,
    }
    impl RtcenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCSEL1" ]
    pub struct Rtcsel1R {
        bits: u8,
    }
    impl Rtcsel1R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RTCSEL0" ]
    pub struct Rtcsel0R {
        bits: u8,
    }
    impl Rtcsel0R {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSEBYP" ]
    pub struct LsebypR {
        bits: u8,
    }
    impl LsebypR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSERDY" ]
    pub struct LserdyR {
        bits: u8,
    }
    impl LserdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSEON" ]
    pub struct LseonR {
        bits: u8,
    }
    impl LseonR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _BdrstW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BdrstW<'a> {
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
    pub struct _RtcenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RtcenW<'a> {
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
    pub struct _Rtcsel1W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Rtcsel1W<'a> {
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
    pub struct _Rtcsel0W<'a> {
        register: &'a mut W,
    }
    impl<'a> _Rtcsel0W<'a> {
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
    pub struct _LsebypW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsebypW<'a> {
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
    pub struct _LseonW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LseonW<'a> {
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
        fn _bdrst(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 16 - Backup domain software reset" ]
        pub fn bdrst(&self) -> BdrstR {
            BdrstR { bits: self._bdrst() }
        }
        fn _rtcen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        pub fn rtcen(&self) -> RtcenR {
            RtcenR { bits: self._rtcen() }
        }
        fn _rtcsel1(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 9 - RTC clock source selection" ]
        pub fn rtcsel1(&self) -> Rtcsel1R {
            Rtcsel1R { bits: self._rtcsel1() }
        }
        fn _rtcsel0(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 8 - RTC clock source selection" ]
        pub fn rtcsel0(&self) -> Rtcsel0R {
            Rtcsel0R { bits: self._rtcsel0() }
        }
        fn _lsebyp(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 2 - External low-speed oscillator bypass" ]
        pub fn lsebyp(&self) -> LsebypR {
            LsebypR { bits: self._lsebyp() }
        }
        fn _lserdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - External low-speed oscillator ready" ]
        pub fn lserdy(&self) -> LserdyR {
            LserdyR { bits: self._lserdy() }
        }
        fn _lseon(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - External low-speed oscillator enable" ]
        pub fn lseon(&self) -> LseonR {
            LseonR { bits: self._lseon() }
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
        # [ doc = "Bit 16 - Backup domain software reset" ]
        pub fn bdrst(&mut self) -> _BdrstW {
            _BdrstW { register: self }
        }
        # [ doc = "Bit 15 - RTC clock enable" ]
        pub fn rtcen(&mut self) -> _RtcenW {
            _RtcenW { register: self }
        }
        # [ doc = "Bit 9 - RTC clock source selection" ]
        pub fn rtcsel1(&mut self) -> _Rtcsel1W {
            _Rtcsel1W { register: self }
        }
        # [ doc = "Bit 8 - RTC clock source selection" ]
        pub fn rtcsel0(&mut self) -> _Rtcsel0W {
            _Rtcsel0W { register: self }
        }
        # [ doc = "Bit 2 - External low-speed oscillator bypass" ]
        pub fn lsebyp(&mut self) -> _LsebypW {
            _LsebypW { register: self }
        }
        # [ doc = "Bit 0 - External low-speed oscillator enable" ]
        pub fn lseon(&mut self) -> _LseonW {
            _LseonW { register: self }
        }
    }
}

# [ doc = "clock control & status register" ]
# [ repr ( C ) ]
pub struct Csr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "clock control & status register" ]
pub mod csr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Csr {
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
    # [ doc = "Value of the field LPWRRSTF" ]
    pub struct LpwrrstfR {
        bits: u8,
    }
    impl LpwrrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WWDGRSTF" ]
    pub struct WwdgrstfR {
        bits: u8,
    }
    impl WwdgrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field WDGRSTF" ]
    pub struct WdgrstfR {
        bits: u8,
    }
    impl WdgrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SFTRSTF" ]
    pub struct SftrstfR {
        bits: u8,
    }
    impl SftrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PORRSTF" ]
    pub struct PorrstfR {
        bits: u8,
    }
    impl PorrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PADRSTF" ]
    pub struct PadrstfR {
        bits: u8,
    }
    impl PadrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field BORRSTF" ]
    pub struct BorrstfR {
        bits: u8,
    }
    impl BorrstfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field RMVF" ]
    pub struct RmvfR {
        bits: u8,
    }
    impl RmvfR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSIRDY" ]
    pub struct LsirdyR {
        bits: u8,
    }
    impl LsirdyR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field LSION" ]
    pub struct LsionR {
        bits: u8,
    }
    impl LsionR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _LpwrrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LpwrrstfW<'a> {
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
    pub struct _WwdgrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WwdgrstfW<'a> {
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
    pub struct _WdgrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _WdgrstfW<'a> {
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
    pub struct _SftrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SftrstfW<'a> {
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
    pub struct _PorrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PorrstfW<'a> {
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
    pub struct _PadrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _PadrstfW<'a> {
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
    pub struct _BorrstfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _BorrstfW<'a> {
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
    pub struct _RmvfW<'a> {
        register: &'a mut W,
    }
    impl<'a> _RmvfW<'a> {
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
    pub struct _LsionW<'a> {
        register: &'a mut W,
    }
    impl<'a> _LsionW<'a> {
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
        fn _lpwrrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        pub fn lpwrrstf(&self) -> LpwrrstfR {
            LpwrrstfR { bits: self._lpwrrstf() }
        }
        fn _wwdgrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        pub fn wwdgrstf(&self) -> WwdgrstfR {
            WwdgrstfR { bits: self._wwdgrstf() }
        }
        fn _wdgrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        pub fn wdgrstf(&self) -> WdgrstfR {
            WdgrstfR { bits: self._wdgrstf() }
        }
        fn _sftrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        pub fn sftrstf(&self) -> SftrstfR {
            SftrstfR { bits: self._sftrstf() }
        }
        fn _porrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        pub fn porrstf(&self) -> PorrstfR {
            PorrstfR { bits: self._porrstf() }
        }
        fn _padrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        pub fn padrstf(&self) -> PadrstfR {
            PadrstfR { bits: self._padrstf() }
        }
        fn _borrstf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 25 - BOR reset flag" ]
        pub fn borrstf(&self) -> BorrstfR {
            BorrstfR { bits: self._borrstf() }
        }
        fn _rmvf(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        pub fn rmvf(&self) -> RmvfR {
            RmvfR { bits: self._rmvf() }
        }
        fn _lsirdy(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 1 - Internal low-speed oscillator ready" ]
        pub fn lsirdy(&self) -> LsirdyR {
            LsirdyR { bits: self._lsirdy() }
        }
        fn _lsion(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 0 - Internal low-speed oscillator enable" ]
        pub fn lsion(&self) -> LsionR {
            LsionR { bits: self._lsion() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 234881024 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bit 31 - Low-power reset flag" ]
        pub fn lpwrrstf(&mut self) -> _LpwrrstfW {
            _LpwrrstfW { register: self }
        }
        # [ doc = "Bit 30 - Window watchdog reset flag" ]
        pub fn wwdgrstf(&mut self) -> _WwdgrstfW {
            _WwdgrstfW { register: self }
        }
        # [ doc = "Bit 29 - Independent watchdog reset flag" ]
        pub fn wdgrstf(&mut self) -> _WdgrstfW {
            _WdgrstfW { register: self }
        }
        # [ doc = "Bit 28 - Software reset flag" ]
        pub fn sftrstf(&mut self) -> _SftrstfW {
            _SftrstfW { register: self }
        }
        # [ doc = "Bit 27 - POR/PDR reset flag" ]
        pub fn porrstf(&mut self) -> _PorrstfW {
            _PorrstfW { register: self }
        }
        # [ doc = "Bit 26 - PIN reset flag" ]
        pub fn padrstf(&mut self) -> _PadrstfW {
            _PadrstfW { register: self }
        }
        # [ doc = "Bit 25 - BOR reset flag" ]
        pub fn borrstf(&mut self) -> _BorrstfW {
            _BorrstfW { register: self }
        }
        # [ doc = "Bit 24 - Remove reset flag" ]
        pub fn rmvf(&mut self) -> _RmvfW {
            _RmvfW { register: self }
        }
        # [ doc = "Bit 0 - Internal low-speed oscillator enable" ]
        pub fn lsion(&mut self) -> _LsionW {
            _LsionW { register: self }
        }
    }
}

# [ doc = "spread spectrum clock generation register" ]
# [ repr ( C ) ]
pub struct Sscgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "spread spectrum clock generation register" ]
pub mod sscgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Sscgr {
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
    # [ doc = "Value of the field SSCGEN" ]
    pub struct SscgenR {
        bits: u8,
    }
    impl SscgenR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field SPREADSEL" ]
    pub struct SpreadselR {
        bits: u8,
    }
    impl SpreadselR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field INCSTEP" ]
    pub struct IncstepR {
        bits: u16,
    }
    impl IncstepR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = "Value of the field MODPER" ]
    pub struct ModperR {
        bits: u16,
    }
    impl ModperR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _SscgenW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SscgenW<'a> {
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
    pub struct _SpreadselW<'a> {
        register: &'a mut W,
    }
    impl<'a> _SpreadselW<'a> {
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
    pub struct _IncstepW<'a> {
        register: &'a mut W,
    }
    impl<'a> _IncstepW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 13;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _ModperW<'a> {
        register: &'a mut W,
    }
    impl<'a> _ModperW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 8191;
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
        fn _sscgen(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 31 - Spread spectrum modulation enable" ]
        pub fn sscgen(&self) -> SscgenR {
            SscgenR { bits: self._sscgen() }
        }
        fn _spreadsel(&self) -> u8 {
            const MASK: u8 = 1;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bit 30 - Spread Select" ]
        pub fn spreadsel(&self) -> SpreadselR {
            SpreadselR { bits: self._spreadsel() }
        }
        fn _incstep(&self) -> u16 {
            const MASK: u16 = 32767;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 13:27 - Incrementation step" ]
        pub fn incstep(&self) -> IncstepR {
            IncstepR { bits: self._incstep() }
        }
        fn _modper(&self) -> u16 {
            const MASK: u16 = 8191;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 0:12 - Modulation period" ]
        pub fn modper(&self) -> ModperR {
            ModperR { bits: self._modper() }
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
        # [ doc = "Bit 31 - Spread spectrum modulation enable" ]
        pub fn sscgen(&mut self) -> _SscgenW {
            _SscgenW { register: self }
        }
        # [ doc = "Bit 30 - Spread Select" ]
        pub fn spreadsel(&mut self) -> _SpreadselW {
            _SpreadselW { register: self }
        }
        # [ doc = "Bits 13:27 - Incrementation step" ]
        pub fn incstep(&mut self) -> _IncstepW {
            _IncstepW { register: self }
        }
        # [ doc = "Bits 0:12 - Modulation period" ]
        pub fn modper(&mut self) -> _ModperW {
            _ModperW { register: self }
        }
    }
}

# [ doc = "PLLI2S configuration register" ]
# [ repr ( C ) ]
pub struct Plli2scfgr {
    register: ::volatile_register::RW<u32>,
}

# [ doc = "PLLI2S configuration register" ]
pub mod plli2scfgr {
    # [ doc = r" Value read from the register" ]
    pub struct R {
        bits: u32,
    }
    # [ doc = r" Value to write to the register" ]
    pub struct W {
        bits: u32,
    }
    impl super::Plli2scfgr {
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
    # [ doc = "Value of the field PLLI2SRx" ]
    pub struct Plli2srxR {
        bits: u8,
    }
    impl Plli2srxR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u8 {
            self.bits
        }
    }
    # [ doc = "Value of the field PLLI2SNx" ]
    pub struct Plli2snxR {
        bits: u16,
    }
    impl Plli2snxR {
        # [ doc = r" Value of the field as raw bits" ]
        pub fn bits(&self) -> u16 {
            self.bits
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Plli2srxW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plli2srxW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u8) -> &'a mut W {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            self.register.bits &= !((MASK as u32) << OFFSET);
            self.register.bits |= ((bits & MASK) as u32) << OFFSET;
            self.register
        }
    }
    # [ doc = r" Proxy" ]
    pub struct _Plli2snxW<'a> {
        register: &'a mut W,
    }
    impl<'a> _Plli2snxW<'a> {
        # [ doc = r" Writes raw `bits` to the field" ]
        pub unsafe fn bits(self, bits: u16) -> &'a mut W {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
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
        fn _plli2srx(&self) -> u8 {
            const MASK: u8 = 7;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        }
        # [ doc = "Bits 28:30 - PLLI2S division factor for I2S clocks" ]
        pub fn plli2srx(&self) -> Plli2srxR {
            Plli2srxR { bits: self._plli2srx() }
        }
        fn _plli2snx(&self) -> u16 {
            const MASK: u16 = 511;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        }
        # [ doc = "Bits 6:14 - PLLI2S multiplication factor for VCO" ]
        pub fn plli2snx(&self) -> Plli2snxR {
            Plli2snxR { bits: self._plli2snx() }
        }
    }
    impl W {
        # [ doc = r" Reset value of the register" ]
        pub fn reset_value() -> W {
            W { bits: 536883200 }
        }
        # [ doc = r" Writes raw `bits` to the register" ]
        pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
            self.bits = bits;
            self
        }
        # [ doc = "Bits 28:30 - PLLI2S division factor for I2S clocks" ]
        pub fn plli2srx(&mut self) -> _Plli2srxW {
            _Plli2srxW { register: self }
        }
        # [ doc = "Bits 6:14 - PLLI2S multiplication factor for VCO" ]
        pub fn plli2snx(&mut self) -> _Plli2snxW {
            _Plli2snxW { register: self }
        }
    }
}
