use self::set::WAY;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unspecified"]
    pub set0: SET,
    _reserved1: [u8; 16usize],
    #[doc = "0x20 - Unspecified"]
    pub set1: SET,
    _reserved2: [u8; 16usize],
    #[doc = "0x40 - Unspecified"]
    pub set2: SET,
    _reserved3: [u8; 16usize],
    #[doc = "0x60 - Unspecified"]
    pub set3: SET,
    _reserved4: [u8; 16usize],
    #[doc = "0x80 - Unspecified"]
    pub set4: SET,
    _reserved5: [u8; 16usize],
    #[doc = "0xa0 - Unspecified"]
    pub set5: SET,
    _reserved6: [u8; 16usize],
    #[doc = "0xc0 - Unspecified"]
    pub set6: SET,
    _reserved7: [u8; 16usize],
    #[doc = "0xe0 - Unspecified"]
    pub set7: SET,
    _reserved8: [u8; 16usize],
    #[doc = "0x100 - Unspecified"]
    pub set8: SET,
    _reserved9: [u8; 16usize],
    #[doc = "0x120 - Unspecified"]
    pub set9: SET,
    _reserved10: [u8; 16usize],
    #[doc = "0x140 - Unspecified"]
    pub set10: SET,
    _reserved11: [u8; 16usize],
    #[doc = "0x160 - Unspecified"]
    pub set11: SET,
    _reserved12: [u8; 16usize],
    #[doc = "0x180 - Unspecified"]
    pub set12: SET,
    _reserved13: [u8; 16usize],
    #[doc = "0x1a0 - Unspecified"]
    pub set13: SET,
    _reserved14: [u8; 16usize],
    #[doc = "0x1c0 - Unspecified"]
    pub set14: SET,
    _reserved15: [u8; 16usize],
    #[doc = "0x1e0 - Unspecified"]
    pub set15: SET,
    _reserved16: [u8; 16usize],
    #[doc = "0x200 - Unspecified"]
    pub set16: SET,
    _reserved17: [u8; 16usize],
    #[doc = "0x220 - Unspecified"]
    pub set17: SET,
    _reserved18: [u8; 16usize],
    #[doc = "0x240 - Unspecified"]
    pub set18: SET,
    _reserved19: [u8; 16usize],
    #[doc = "0x260 - Unspecified"]
    pub set19: SET,
    _reserved20: [u8; 16usize],
    #[doc = "0x280 - Unspecified"]
    pub set20: SET,
    _reserved21: [u8; 16usize],
    #[doc = "0x2a0 - Unspecified"]
    pub set21: SET,
    _reserved22: [u8; 16usize],
    #[doc = "0x2c0 - Unspecified"]
    pub set22: SET,
    _reserved23: [u8; 16usize],
    #[doc = "0x2e0 - Unspecified"]
    pub set23: SET,
    _reserved24: [u8; 16usize],
    #[doc = "0x300 - Unspecified"]
    pub set24: SET,
    _reserved25: [u8; 16usize],
    #[doc = "0x320 - Unspecified"]
    pub set25: SET,
    _reserved26: [u8; 16usize],
    #[doc = "0x340 - Unspecified"]
    pub set26: SET,
    _reserved27: [u8; 16usize],
    #[doc = "0x360 - Unspecified"]
    pub set27: SET,
    _reserved28: [u8; 16usize],
    #[doc = "0x380 - Unspecified"]
    pub set28: SET,
    _reserved29: [u8; 16usize],
    #[doc = "0x3a0 - Unspecified"]
    pub set29: SET,
    _reserved30: [u8; 16usize],
    #[doc = "0x3c0 - Unspecified"]
    pub set30: SET,
    _reserved31: [u8; 16usize],
    #[doc = "0x3e0 - Unspecified"]
    pub set31: SET,
    _reserved32: [u8; 16usize],
    #[doc = "0x400 - Unspecified"]
    pub set32: SET,
    _reserved33: [u8; 16usize],
    #[doc = "0x420 - Unspecified"]
    pub set33: SET,
    _reserved34: [u8; 16usize],
    #[doc = "0x440 - Unspecified"]
    pub set34: SET,
    _reserved35: [u8; 16usize],
    #[doc = "0x460 - Unspecified"]
    pub set35: SET,
    _reserved36: [u8; 16usize],
    #[doc = "0x480 - Unspecified"]
    pub set36: SET,
    _reserved37: [u8; 16usize],
    #[doc = "0x4a0 - Unspecified"]
    pub set37: SET,
    _reserved38: [u8; 16usize],
    #[doc = "0x4c0 - Unspecified"]
    pub set38: SET,
    _reserved39: [u8; 16usize],
    #[doc = "0x4e0 - Unspecified"]
    pub set39: SET,
    _reserved40: [u8; 16usize],
    #[doc = "0x500 - Unspecified"]
    pub set40: SET,
    _reserved41: [u8; 16usize],
    #[doc = "0x520 - Unspecified"]
    pub set41: SET,
    _reserved42: [u8; 16usize],
    #[doc = "0x540 - Unspecified"]
    pub set42: SET,
    _reserved43: [u8; 16usize],
    #[doc = "0x560 - Unspecified"]
    pub set43: SET,
    _reserved44: [u8; 16usize],
    #[doc = "0x580 - Unspecified"]
    pub set44: SET,
    _reserved45: [u8; 16usize],
    #[doc = "0x5a0 - Unspecified"]
    pub set45: SET,
    _reserved46: [u8; 16usize],
    #[doc = "0x5c0 - Unspecified"]
    pub set46: SET,
    _reserved47: [u8; 16usize],
    #[doc = "0x5e0 - Unspecified"]
    pub set47: SET,
    _reserved48: [u8; 16usize],
    #[doc = "0x600 - Unspecified"]
    pub set48: SET,
    _reserved49: [u8; 16usize],
    #[doc = "0x620 - Unspecified"]
    pub set49: SET,
    _reserved50: [u8; 16usize],
    #[doc = "0x640 - Unspecified"]
    pub set50: SET,
    _reserved51: [u8; 16usize],
    #[doc = "0x660 - Unspecified"]
    pub set51: SET,
    _reserved52: [u8; 16usize],
    #[doc = "0x680 - Unspecified"]
    pub set52: SET,
    _reserved53: [u8; 16usize],
    #[doc = "0x6a0 - Unspecified"]
    pub set53: SET,
    _reserved54: [u8; 16usize],
    #[doc = "0x6c0 - Unspecified"]
    pub set54: SET,
    _reserved55: [u8; 16usize],
    #[doc = "0x6e0 - Unspecified"]
    pub set55: SET,
    _reserved56: [u8; 16usize],
    #[doc = "0x700 - Unspecified"]
    pub set56: SET,
    _reserved57: [u8; 16usize],
    #[doc = "0x720 - Unspecified"]
    pub set57: SET,
    _reserved58: [u8; 16usize],
    #[doc = "0x740 - Unspecified"]
    pub set58: SET,
    _reserved59: [u8; 16usize],
    #[doc = "0x760 - Unspecified"]
    pub set59: SET,
    _reserved60: [u8; 16usize],
    #[doc = "0x780 - Unspecified"]
    pub set60: SET,
    _reserved61: [u8; 16usize],
    #[doc = "0x7a0 - Unspecified"]
    pub set61: SET,
    _reserved62: [u8; 16usize],
    #[doc = "0x7c0 - Unspecified"]
    pub set62: SET,
    _reserved63: [u8; 16usize],
    #[doc = "0x7e0 - Unspecified"]
    pub set63: SET,
    _reserved64: [u8; 16usize],
    #[doc = "0x800 - Unspecified"]
    pub set64: SET,
    _reserved65: [u8; 16usize],
    #[doc = "0x820 - Unspecified"]
    pub set65: SET,
    _reserved66: [u8; 16usize],
    #[doc = "0x840 - Unspecified"]
    pub set66: SET,
    _reserved67: [u8; 16usize],
    #[doc = "0x860 - Unspecified"]
    pub set67: SET,
    _reserved68: [u8; 16usize],
    #[doc = "0x880 - Unspecified"]
    pub set68: SET,
    _reserved69: [u8; 16usize],
    #[doc = "0x8a0 - Unspecified"]
    pub set69: SET,
    _reserved70: [u8; 16usize],
    #[doc = "0x8c0 - Unspecified"]
    pub set70: SET,
    _reserved71: [u8; 16usize],
    #[doc = "0x8e0 - Unspecified"]
    pub set71: SET,
    _reserved72: [u8; 16usize],
    #[doc = "0x900 - Unspecified"]
    pub set72: SET,
    _reserved73: [u8; 16usize],
    #[doc = "0x920 - Unspecified"]
    pub set73: SET,
    _reserved74: [u8; 16usize],
    #[doc = "0x940 - Unspecified"]
    pub set74: SET,
    _reserved75: [u8; 16usize],
    #[doc = "0x960 - Unspecified"]
    pub set75: SET,
    _reserved76: [u8; 16usize],
    #[doc = "0x980 - Unspecified"]
    pub set76: SET,
    _reserved77: [u8; 16usize],
    #[doc = "0x9a0 - Unspecified"]
    pub set77: SET,
    _reserved78: [u8; 16usize],
    #[doc = "0x9c0 - Unspecified"]
    pub set78: SET,
    _reserved79: [u8; 16usize],
    #[doc = "0x9e0 - Unspecified"]
    pub set79: SET,
    _reserved80: [u8; 16usize],
    #[doc = "0xa00 - Unspecified"]
    pub set80: SET,
    _reserved81: [u8; 16usize],
    #[doc = "0xa20 - Unspecified"]
    pub set81: SET,
    _reserved82: [u8; 16usize],
    #[doc = "0xa40 - Unspecified"]
    pub set82: SET,
    _reserved83: [u8; 16usize],
    #[doc = "0xa60 - Unspecified"]
    pub set83: SET,
    _reserved84: [u8; 16usize],
    #[doc = "0xa80 - Unspecified"]
    pub set84: SET,
    _reserved85: [u8; 16usize],
    #[doc = "0xaa0 - Unspecified"]
    pub set85: SET,
    _reserved86: [u8; 16usize],
    #[doc = "0xac0 - Unspecified"]
    pub set86: SET,
    _reserved87: [u8; 16usize],
    #[doc = "0xae0 - Unspecified"]
    pub set87: SET,
    _reserved88: [u8; 16usize],
    #[doc = "0xb00 - Unspecified"]
    pub set88: SET,
    _reserved89: [u8; 16usize],
    #[doc = "0xb20 - Unspecified"]
    pub set89: SET,
    _reserved90: [u8; 16usize],
    #[doc = "0xb40 - Unspecified"]
    pub set90: SET,
    _reserved91: [u8; 16usize],
    #[doc = "0xb60 - Unspecified"]
    pub set91: SET,
    _reserved92: [u8; 16usize],
    #[doc = "0xb80 - Unspecified"]
    pub set92: SET,
    _reserved93: [u8; 16usize],
    #[doc = "0xba0 - Unspecified"]
    pub set93: SET,
    _reserved94: [u8; 16usize],
    #[doc = "0xbc0 - Unspecified"]
    pub set94: SET,
    _reserved95: [u8; 16usize],
    #[doc = "0xbe0 - Unspecified"]
    pub set95: SET,
    _reserved96: [u8; 16usize],
    #[doc = "0xc00 - Unspecified"]
    pub set96: SET,
    _reserved97: [u8; 16usize],
    #[doc = "0xc20 - Unspecified"]
    pub set97: SET,
    _reserved98: [u8; 16usize],
    #[doc = "0xc40 - Unspecified"]
    pub set98: SET,
    _reserved99: [u8; 16usize],
    #[doc = "0xc60 - Unspecified"]
    pub set99: SET,
    _reserved100: [u8; 16usize],
    #[doc = "0xc80 - Unspecified"]
    pub set100: SET,
    _reserved101: [u8; 16usize],
    #[doc = "0xca0 - Unspecified"]
    pub set101: SET,
    _reserved102: [u8; 16usize],
    #[doc = "0xcc0 - Unspecified"]
    pub set102: SET,
    _reserved103: [u8; 16usize],
    #[doc = "0xce0 - Unspecified"]
    pub set103: SET,
    _reserved104: [u8; 16usize],
    #[doc = "0xd00 - Unspecified"]
    pub set104: SET,
    _reserved105: [u8; 16usize],
    #[doc = "0xd20 - Unspecified"]
    pub set105: SET,
    _reserved106: [u8; 16usize],
    #[doc = "0xd40 - Unspecified"]
    pub set106: SET,
    _reserved107: [u8; 16usize],
    #[doc = "0xd60 - Unspecified"]
    pub set107: SET,
    _reserved108: [u8; 16usize],
    #[doc = "0xd80 - Unspecified"]
    pub set108: SET,
    _reserved109: [u8; 16usize],
    #[doc = "0xda0 - Unspecified"]
    pub set109: SET,
    _reserved110: [u8; 16usize],
    #[doc = "0xdc0 - Unspecified"]
    pub set110: SET,
    _reserved111: [u8; 16usize],
    #[doc = "0xde0 - Unspecified"]
    pub set111: SET,
    _reserved112: [u8; 16usize],
    #[doc = "0xe00 - Unspecified"]
    pub set112: SET,
    _reserved113: [u8; 16usize],
    #[doc = "0xe20 - Unspecified"]
    pub set113: SET,
    _reserved114: [u8; 16usize],
    #[doc = "0xe40 - Unspecified"]
    pub set114: SET,
    _reserved115: [u8; 16usize],
    #[doc = "0xe60 - Unspecified"]
    pub set115: SET,
    _reserved116: [u8; 16usize],
    #[doc = "0xe80 - Unspecified"]
    pub set116: SET,
    _reserved117: [u8; 16usize],
    #[doc = "0xea0 - Unspecified"]
    pub set117: SET,
    _reserved118: [u8; 16usize],
    #[doc = "0xec0 - Unspecified"]
    pub set118: SET,
    _reserved119: [u8; 16usize],
    #[doc = "0xee0 - Unspecified"]
    pub set119: SET,
    _reserved120: [u8; 16usize],
    #[doc = "0xf00 - Unspecified"]
    pub set120: SET,
    _reserved121: [u8; 16usize],
    #[doc = "0xf20 - Unspecified"]
    pub set121: SET,
    _reserved122: [u8; 16usize],
    #[doc = "0xf40 - Unspecified"]
    pub set122: SET,
    _reserved123: [u8; 16usize],
    #[doc = "0xf60 - Unspecified"]
    pub set123: SET,
    _reserved124: [u8; 16usize],
    #[doc = "0xf80 - Unspecified"]
    pub set124: SET,
    _reserved125: [u8; 16usize],
    #[doc = "0xfa0 - Unspecified"]
    pub set125: SET,
    _reserved126: [u8; 16usize],
    #[doc = "0xfc0 - Unspecified"]
    pub set126: SET,
    _reserved127: [u8; 16usize],
    #[doc = "0xfe0 - Unspecified"]
    pub set127: SET,
    _reserved128: [u8; 16usize],
    #[doc = "0x1000 - Unspecified"]
    pub set128: SET,
    _reserved129: [u8; 16usize],
    #[doc = "0x1020 - Unspecified"]
    pub set129: SET,
    _reserved130: [u8; 16usize],
    #[doc = "0x1040 - Unspecified"]
    pub set130: SET,
    _reserved131: [u8; 16usize],
    #[doc = "0x1060 - Unspecified"]
    pub set131: SET,
    _reserved132: [u8; 16usize],
    #[doc = "0x1080 - Unspecified"]
    pub set132: SET,
    _reserved133: [u8; 16usize],
    #[doc = "0x10a0 - Unspecified"]
    pub set133: SET,
    _reserved134: [u8; 16usize],
    #[doc = "0x10c0 - Unspecified"]
    pub set134: SET,
    _reserved135: [u8; 16usize],
    #[doc = "0x10e0 - Unspecified"]
    pub set135: SET,
    _reserved136: [u8; 16usize],
    #[doc = "0x1100 - Unspecified"]
    pub set136: SET,
    _reserved137: [u8; 16usize],
    #[doc = "0x1120 - Unspecified"]
    pub set137: SET,
    _reserved138: [u8; 16usize],
    #[doc = "0x1140 - Unspecified"]
    pub set138: SET,
    _reserved139: [u8; 16usize],
    #[doc = "0x1160 - Unspecified"]
    pub set139: SET,
    _reserved140: [u8; 16usize],
    #[doc = "0x1180 - Unspecified"]
    pub set140: SET,
    _reserved141: [u8; 16usize],
    #[doc = "0x11a0 - Unspecified"]
    pub set141: SET,
    _reserved142: [u8; 16usize],
    #[doc = "0x11c0 - Unspecified"]
    pub set142: SET,
    _reserved143: [u8; 16usize],
    #[doc = "0x11e0 - Unspecified"]
    pub set143: SET,
    _reserved144: [u8; 16usize],
    #[doc = "0x1200 - Unspecified"]
    pub set144: SET,
    _reserved145: [u8; 16usize],
    #[doc = "0x1220 - Unspecified"]
    pub set145: SET,
    _reserved146: [u8; 16usize],
    #[doc = "0x1240 - Unspecified"]
    pub set146: SET,
    _reserved147: [u8; 16usize],
    #[doc = "0x1260 - Unspecified"]
    pub set147: SET,
    _reserved148: [u8; 16usize],
    #[doc = "0x1280 - Unspecified"]
    pub set148: SET,
    _reserved149: [u8; 16usize],
    #[doc = "0x12a0 - Unspecified"]
    pub set149: SET,
    _reserved150: [u8; 16usize],
    #[doc = "0x12c0 - Unspecified"]
    pub set150: SET,
    _reserved151: [u8; 16usize],
    #[doc = "0x12e0 - Unspecified"]
    pub set151: SET,
    _reserved152: [u8; 16usize],
    #[doc = "0x1300 - Unspecified"]
    pub set152: SET,
    _reserved153: [u8; 16usize],
    #[doc = "0x1320 - Unspecified"]
    pub set153: SET,
    _reserved154: [u8; 16usize],
    #[doc = "0x1340 - Unspecified"]
    pub set154: SET,
    _reserved155: [u8; 16usize],
    #[doc = "0x1360 - Unspecified"]
    pub set155: SET,
    _reserved156: [u8; 16usize],
    #[doc = "0x1380 - Unspecified"]
    pub set156: SET,
    _reserved157: [u8; 16usize],
    #[doc = "0x13a0 - Unspecified"]
    pub set157: SET,
    _reserved158: [u8; 16usize],
    #[doc = "0x13c0 - Unspecified"]
    pub set158: SET,
    _reserved159: [u8; 16usize],
    #[doc = "0x13e0 - Unspecified"]
    pub set159: SET,
    _reserved160: [u8; 16usize],
    #[doc = "0x1400 - Unspecified"]
    pub set160: SET,
    _reserved161: [u8; 16usize],
    #[doc = "0x1420 - Unspecified"]
    pub set161: SET,
    _reserved162: [u8; 16usize],
    #[doc = "0x1440 - Unspecified"]
    pub set162: SET,
    _reserved163: [u8; 16usize],
    #[doc = "0x1460 - Unspecified"]
    pub set163: SET,
    _reserved164: [u8; 16usize],
    #[doc = "0x1480 - Unspecified"]
    pub set164: SET,
    _reserved165: [u8; 16usize],
    #[doc = "0x14a0 - Unspecified"]
    pub set165: SET,
    _reserved166: [u8; 16usize],
    #[doc = "0x14c0 - Unspecified"]
    pub set166: SET,
    _reserved167: [u8; 16usize],
    #[doc = "0x14e0 - Unspecified"]
    pub set167: SET,
    _reserved168: [u8; 16usize],
    #[doc = "0x1500 - Unspecified"]
    pub set168: SET,
    _reserved169: [u8; 16usize],
    #[doc = "0x1520 - Unspecified"]
    pub set169: SET,
    _reserved170: [u8; 16usize],
    #[doc = "0x1540 - Unspecified"]
    pub set170: SET,
    _reserved171: [u8; 16usize],
    #[doc = "0x1560 - Unspecified"]
    pub set171: SET,
    _reserved172: [u8; 16usize],
    #[doc = "0x1580 - Unspecified"]
    pub set172: SET,
    _reserved173: [u8; 16usize],
    #[doc = "0x15a0 - Unspecified"]
    pub set173: SET,
    _reserved174: [u8; 16usize],
    #[doc = "0x15c0 - Unspecified"]
    pub set174: SET,
    _reserved175: [u8; 16usize],
    #[doc = "0x15e0 - Unspecified"]
    pub set175: SET,
    _reserved176: [u8; 16usize],
    #[doc = "0x1600 - Unspecified"]
    pub set176: SET,
    _reserved177: [u8; 16usize],
    #[doc = "0x1620 - Unspecified"]
    pub set177: SET,
    _reserved178: [u8; 16usize],
    #[doc = "0x1640 - Unspecified"]
    pub set178: SET,
    _reserved179: [u8; 16usize],
    #[doc = "0x1660 - Unspecified"]
    pub set179: SET,
    _reserved180: [u8; 16usize],
    #[doc = "0x1680 - Unspecified"]
    pub set180: SET,
    _reserved181: [u8; 16usize],
    #[doc = "0x16a0 - Unspecified"]
    pub set181: SET,
    _reserved182: [u8; 16usize],
    #[doc = "0x16c0 - Unspecified"]
    pub set182: SET,
    _reserved183: [u8; 16usize],
    #[doc = "0x16e0 - Unspecified"]
    pub set183: SET,
    _reserved184: [u8; 16usize],
    #[doc = "0x1700 - Unspecified"]
    pub set184: SET,
    _reserved185: [u8; 16usize],
    #[doc = "0x1720 - Unspecified"]
    pub set185: SET,
    _reserved186: [u8; 16usize],
    #[doc = "0x1740 - Unspecified"]
    pub set186: SET,
    _reserved187: [u8; 16usize],
    #[doc = "0x1760 - Unspecified"]
    pub set187: SET,
    _reserved188: [u8; 16usize],
    #[doc = "0x1780 - Unspecified"]
    pub set188: SET,
    _reserved189: [u8; 16usize],
    #[doc = "0x17a0 - Unspecified"]
    pub set189: SET,
    _reserved190: [u8; 16usize],
    #[doc = "0x17c0 - Unspecified"]
    pub set190: SET,
    _reserved191: [u8; 16usize],
    #[doc = "0x17e0 - Unspecified"]
    pub set191: SET,
    _reserved192: [u8; 16usize],
    #[doc = "0x1800 - Unspecified"]
    pub set192: SET,
    _reserved193: [u8; 16usize],
    #[doc = "0x1820 - Unspecified"]
    pub set193: SET,
    _reserved194: [u8; 16usize],
    #[doc = "0x1840 - Unspecified"]
    pub set194: SET,
    _reserved195: [u8; 16usize],
    #[doc = "0x1860 - Unspecified"]
    pub set195: SET,
    _reserved196: [u8; 16usize],
    #[doc = "0x1880 - Unspecified"]
    pub set196: SET,
    _reserved197: [u8; 16usize],
    #[doc = "0x18a0 - Unspecified"]
    pub set197: SET,
    _reserved198: [u8; 16usize],
    #[doc = "0x18c0 - Unspecified"]
    pub set198: SET,
    _reserved199: [u8; 16usize],
    #[doc = "0x18e0 - Unspecified"]
    pub set199: SET,
    _reserved200: [u8; 16usize],
    #[doc = "0x1900 - Unspecified"]
    pub set200: SET,
    _reserved201: [u8; 16usize],
    #[doc = "0x1920 - Unspecified"]
    pub set201: SET,
    _reserved202: [u8; 16usize],
    #[doc = "0x1940 - Unspecified"]
    pub set202: SET,
    _reserved203: [u8; 16usize],
    #[doc = "0x1960 - Unspecified"]
    pub set203: SET,
    _reserved204: [u8; 16usize],
    #[doc = "0x1980 - Unspecified"]
    pub set204: SET,
    _reserved205: [u8; 16usize],
    #[doc = "0x19a0 - Unspecified"]
    pub set205: SET,
    _reserved206: [u8; 16usize],
    #[doc = "0x19c0 - Unspecified"]
    pub set206: SET,
    _reserved207: [u8; 16usize],
    #[doc = "0x19e0 - Unspecified"]
    pub set207: SET,
    _reserved208: [u8; 16usize],
    #[doc = "0x1a00 - Unspecified"]
    pub set208: SET,
    _reserved209: [u8; 16usize],
    #[doc = "0x1a20 - Unspecified"]
    pub set209: SET,
    _reserved210: [u8; 16usize],
    #[doc = "0x1a40 - Unspecified"]
    pub set210: SET,
    _reserved211: [u8; 16usize],
    #[doc = "0x1a60 - Unspecified"]
    pub set211: SET,
    _reserved212: [u8; 16usize],
    #[doc = "0x1a80 - Unspecified"]
    pub set212: SET,
    _reserved213: [u8; 16usize],
    #[doc = "0x1aa0 - Unspecified"]
    pub set213: SET,
    _reserved214: [u8; 16usize],
    #[doc = "0x1ac0 - Unspecified"]
    pub set214: SET,
    _reserved215: [u8; 16usize],
    #[doc = "0x1ae0 - Unspecified"]
    pub set215: SET,
    _reserved216: [u8; 16usize],
    #[doc = "0x1b00 - Unspecified"]
    pub set216: SET,
    _reserved217: [u8; 16usize],
    #[doc = "0x1b20 - Unspecified"]
    pub set217: SET,
    _reserved218: [u8; 16usize],
    #[doc = "0x1b40 - Unspecified"]
    pub set218: SET,
    _reserved219: [u8; 16usize],
    #[doc = "0x1b60 - Unspecified"]
    pub set219: SET,
    _reserved220: [u8; 16usize],
    #[doc = "0x1b80 - Unspecified"]
    pub set220: SET,
    _reserved221: [u8; 16usize],
    #[doc = "0x1ba0 - Unspecified"]
    pub set221: SET,
    _reserved222: [u8; 16usize],
    #[doc = "0x1bc0 - Unspecified"]
    pub set222: SET,
    _reserved223: [u8; 16usize],
    #[doc = "0x1be0 - Unspecified"]
    pub set223: SET,
    _reserved224: [u8; 16usize],
    #[doc = "0x1c00 - Unspecified"]
    pub set224: SET,
    _reserved225: [u8; 16usize],
    #[doc = "0x1c20 - Unspecified"]
    pub set225: SET,
    _reserved226: [u8; 16usize],
    #[doc = "0x1c40 - Unspecified"]
    pub set226: SET,
    _reserved227: [u8; 16usize],
    #[doc = "0x1c60 - Unspecified"]
    pub set227: SET,
    _reserved228: [u8; 16usize],
    #[doc = "0x1c80 - Unspecified"]
    pub set228: SET,
    _reserved229: [u8; 16usize],
    #[doc = "0x1ca0 - Unspecified"]
    pub set229: SET,
    _reserved230: [u8; 16usize],
    #[doc = "0x1cc0 - Unspecified"]
    pub set230: SET,
    _reserved231: [u8; 16usize],
    #[doc = "0x1ce0 - Unspecified"]
    pub set231: SET,
    _reserved232: [u8; 16usize],
    #[doc = "0x1d00 - Unspecified"]
    pub set232: SET,
    _reserved233: [u8; 16usize],
    #[doc = "0x1d20 - Unspecified"]
    pub set233: SET,
    _reserved234: [u8; 16usize],
    #[doc = "0x1d40 - Unspecified"]
    pub set234: SET,
    _reserved235: [u8; 16usize],
    #[doc = "0x1d60 - Unspecified"]
    pub set235: SET,
    _reserved236: [u8; 16usize],
    #[doc = "0x1d80 - Unspecified"]
    pub set236: SET,
    _reserved237: [u8; 16usize],
    #[doc = "0x1da0 - Unspecified"]
    pub set237: SET,
    _reserved238: [u8; 16usize],
    #[doc = "0x1dc0 - Unspecified"]
    pub set238: SET,
    _reserved239: [u8; 16usize],
    #[doc = "0x1de0 - Unspecified"]
    pub set239: SET,
    _reserved240: [u8; 16usize],
    #[doc = "0x1e00 - Unspecified"]
    pub set240: SET,
    _reserved241: [u8; 16usize],
    #[doc = "0x1e20 - Unspecified"]
    pub set241: SET,
    _reserved242: [u8; 16usize],
    #[doc = "0x1e40 - Unspecified"]
    pub set242: SET,
    _reserved243: [u8; 16usize],
    #[doc = "0x1e60 - Unspecified"]
    pub set243: SET,
    _reserved244: [u8; 16usize],
    #[doc = "0x1e80 - Unspecified"]
    pub set244: SET,
    _reserved245: [u8; 16usize],
    #[doc = "0x1ea0 - Unspecified"]
    pub set245: SET,
    _reserved246: [u8; 16usize],
    #[doc = "0x1ec0 - Unspecified"]
    pub set246: SET,
    _reserved247: [u8; 16usize],
    #[doc = "0x1ee0 - Unspecified"]
    pub set247: SET,
    _reserved248: [u8; 16usize],
    #[doc = "0x1f00 - Unspecified"]
    pub set248: SET,
    _reserved249: [u8; 16usize],
    #[doc = "0x1f20 - Unspecified"]
    pub set249: SET,
    _reserved250: [u8; 16usize],
    #[doc = "0x1f40 - Unspecified"]
    pub set250: SET,
    _reserved251: [u8; 16usize],
    #[doc = "0x1f60 - Unspecified"]
    pub set251: SET,
    _reserved252: [u8; 16usize],
    #[doc = "0x1f80 - Unspecified"]
    pub set252: SET,
    _reserved253: [u8; 16usize],
    #[doc = "0x1fa0 - Unspecified"]
    pub set253: SET,
    _reserved254: [u8; 16usize],
    #[doc = "0x1fc0 - Unspecified"]
    pub set254: SET,
    _reserved255: [u8; 16usize],
    #[doc = "0x1fe0 - Unspecified"]
    pub set255: SET,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct SET {
    #[doc = "0x00 - Unspecified"]
    pub way: [WAY; 2],
}
#[doc = r"Register block"]
#[doc = "Unspecified"]
pub mod set;
