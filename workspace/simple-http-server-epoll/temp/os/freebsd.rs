

// tcflow()
pub const TCOOFF: c_int = 1;
pub const TCOON:  c_int = 2;
pub const TCIOFF: c_int = 3;
pub const TCION:  c_int = 4;

// tcflush()
pub const TCIFLUSH:  c_int = 1;
pub const TCOFLUSH:  c_int = 2;
pub const TCIOFLUSH: c_int = 3;

// tcsetattr()
pub const TCSANOW:   c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
pub const TCSASOFT:  c_int = 0x10;
