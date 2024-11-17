/*
 * In the original implementation, they are called requests but I think this is kind of misleading
 * because it literally orders the B15F Board what to do... so they are more logically called
 * commands.
 */

#[macro_export]
macro_rules! command_buffer {
    [$($x:expr),*] => (
        &[$($x as u8),*]
    );
}

#[allow(dead_code)]
pub enum B15FCommand {
    Discard         =  0,
    Test            =  1,
    Info            =  2,
    IntTest         =  3,
    SelfTest        =  4,
    DigitalWrite0   =  5,
    DigitalWrite1   =  6,
    DigitalRead0    =  7,
    DigitalRead1    =  8,
    ReadDipSwitch   =  9,
    AnalogWrite0    = 10,
    AnalogWrite1    = 11,
    AnalogRead      = 12,
    PwmSetFreq      = 14,
    PwmSetValue     = 15,
    SetMem8         = 16,
    GetMem8         = 17,
    SetMem16        = 18,
    GetMem16        = 19,
    CounterOffset   = 20,
    ServoEnable     = 21,
    ServoDisable    = 22,
    ServoSetPos     = 23
}
