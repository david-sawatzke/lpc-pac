#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
    pub ir: IR,
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
    pub tcr: TCR,
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR."]
    pub tc: TC,
    #[doc = "0x0c - Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC."]
    pub pr: PR,
    #[doc = "0x10 - Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
    pub pc: PC,
    #[doc = "0x14 - Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
    pub mcr: MCR,
    #[doc = "0x18 - Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
    pub mr: [MR; 4],
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
    pub ccr: CCR,
    #[doc = "0x2c - Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
    pub cr: [CR; 4],
    #[doc = "0x3c - External Match Register. The EMR controls the match function and the external match pins."]
    pub emr: EMR,
    _reserved0: [u8; 48usize],
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    pub ctcr: CTCR,
    #[doc = "0x74 - PWM Control Register. The PWMCON enables PWM mode for the external match pins."]
    pub pwmc: PWMC,
    #[doc = "0x78 - Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero."]
    pub msr: [MSR; 4],
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
pub struct IR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Register. The IR can be written to clear interrupts. The IR can be read to identify which of eight possible interrupt sources are pending."]
pub mod ir;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
pub struct TCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR."]
pub mod tcr;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR."]
pub struct TC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of the APB bus clock. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC."]
pub struct PR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Register. When the Prescale Counter (PC) is equal to this value, the next clock increments the TC and clears the PC."]
pub mod pr;
#[doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
pub struct PC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Prescale Counter. The 32 bit PC is a counter which is incremented to the value stored in PR. When the value in PR is reached, the TC is incremented and the PC is cleared. The PC is observable and controllable through the bus interface."]
pub mod pc;
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
pub struct MCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs."]
pub mod mcr;
#[doc = "Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
pub struct MR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Register . MR can be enabled through the MCR to reset the TC, stop both the TC and PC, and/or generate an interrupt every time MR matches the TC."]
pub mod mr;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
pub struct CCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated when a capture takes place."]
pub mod ccr;
#[doc = "Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
pub struct CR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Capture Register . CR is loaded with the value of TC when there is an event on the CAPn. input."]
pub mod cr;
#[doc = "External Match Register. The EMR controls the match function and the external match pins."]
pub struct EMR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "External Match Register. The EMR controls the match function and the external match pins."]
pub mod emr;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub struct CTCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
#[doc = "PWM Control Register. The PWMCON enables PWM mode for the external match pins."]
pub struct PWMC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PWM Control Register. The PWMCON enables PWM mode for the external match pins."]
pub mod pwmc;
#[doc = "Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero."]
pub struct MSR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Match Shadow Register . If enabled, the Match Register will be automatically reloaded with the contents of this register whenever the TC is reset to zero."]
pub mod msr;
