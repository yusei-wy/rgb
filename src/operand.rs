use std::sync::atomic::{AtomicU16, AtomicU8, Ordering::Relaxed};

use crate::{
    cpu::Cpu,
    instructions::{go, step},
    peripherals::Peripherals,
};

// NOTE: オペランドの返り値が Option なのは 0 M-cycle で値の読み書きが完了するとは限らないため

/// 8bit 命令の操作対象
pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

/// 8bit 命令の操作対象
pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Reg8 { A, B, C, D, E, H, L }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Reg16 { AF, BC, DE, HL, SP }

#[derive(Clone, Copy, Debug)]
pub struct Imm8;

#[derive(Clone, Copy, Debug)]
pub struct Imm16;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Indirect { BC, DE, HL, CFF, HLD, HLI }

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Direct8 { D, DFF }

#[derive(Clone, Copy, Debug)]
pub struct Direct16;

#[rustfmt::skip]
#[derive(Clone, Copy, Debug)]
pub enum Cond { NZ, Z, NC, C }

// NOTE:
// メモリに 8 bit 読み書きするごとに 1 M-cycle 消費する
// レジスタの読み書きは M-cycle を消費しない

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.registers.a,
            Reg8::B => self.registers.b,
            Reg8::C => self.registers.c,
            Reg8::D => self.registers.d,
            Reg8::E => self.registers.e,
            Reg8::H => self.registers.h,
            Reg8::L => self.registers.l,
        })
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        match dst {
            Reg8::A => self.registers.a = val,
            Reg8::B => self.registers.b = val,
            Reg8::C => self.registers.c = val,
            Reg8::D => self.registers.d = val,
            Reg8::E => self.registers.e = val,
            Reg8::H => self.registers.h = val,
            Reg8::L => self.registers.l = val,
        }
        Some(())
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.registers.af(),
            Reg16::BC => self.registers.bc(),
            Reg16::DE => self.registers.de(),
            Reg16::HL => self.registers.hl(),
            Reg16::SP => self.registers.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        match dst {
            Reg16::AF => self.registers.write_af(val),
            Reg16::BC => self.registers.write_bc(val),
            Reg16::DE => self.registers.write_de(val),
            Reg16::HL => self.registers.write_hl(val),
            Reg16::SP => self.registers.sp = val,
        }
        Some(())
    }
}

/// プログラムカウンタが指す場所から読み取られる 8 bit
///
/// 1回のメモリ読み出しが必要なので 1 M-cycle  かかる
impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        step!(None, {
            0: {
                VAL8.store(bus.read(self.registers.pc), Relaxed);
                go!(1);
                return None;
            },
            1: {
                go!(0);
                println!("test");
                return Some(VAL8.load(Relaxed));
            },
        });
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}

/// プログラムカウンタが指す場所から読み取られる 16 bit
///
/// 2回のメモリ読み出しが必要なので 2 M-cycle かかる
impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);
                go!(1);
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
            },
            2: {
                go!(0);
                return Some(    VAL16.load(Relaxed));
            },
        });
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) -> Option<()> {
        unreachable!()
    }
}
