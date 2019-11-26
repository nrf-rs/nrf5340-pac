#[doc = r"Register block"]
#[repr(C)]
pub struct WAY {
    #[doc = "0x00 - Description cluster: Cache data bits \\[31:0\\] of SET\\[n\\], WAY\\[o\\]."]
    pub data0: self::way::DATA0,
    #[doc = "0x04 - Description cluster: Cache data bits \\[63:32\\] of SET\\[n\\], WAY\\[o\\]."]
    pub data1: self::way::DATA1,
    #[doc = "0x08 - Description cluster: Cache data bits \\[95:64\\] of SET\\[n\\], WAY\\[o\\]."]
    pub data2: self::way::DATA2,
    #[doc = "0x0c - Description cluster: Cache data bits \\[127:96\\] of SET\\[n\\], WAY\\[o\\]."]
    pub data3: self::way::DATA3,
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod way;
