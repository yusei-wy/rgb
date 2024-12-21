// WARNING: CPU の命令は 1 M-cycle で完了するとは限らない

use crate::{peripherals::Peripherals, registers::Registers};

/// 実行中の命令を持つ
/// 複数回の emulate_cycle の呼び出しを跨いで 1 命令を実行するための必要
#[derive(Default)]
struct Context {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    pub registers: Registers,
    context: Context,
}

impl Cpu {
    /// プログラムカウンタが示すアドレスに格納された命令 (8 bit) を bus から読み出し
    /// プログラムカウンタをインクリメントする
    pub fn fetch(&mut self, bus: &Peripherals) {
        self.context.opcode = bus.read(self.registers.pc);
        self.registers.pc = self.registers.pc.wrapping_add(1);
        self.context.cb = false;
    }

    /// fetch で読み出した命令の値 `Context::opcode` から何の命令を実行すべきかを解釈する
    pub fn decode(&mut self, bus: &mut Peripherals) {
        match self.context.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02x}", self.context.opcode),
        }
    }

    /// 何もしない NOP 命令, 次の命令を fetch するだけ
    pub fn nop(&mut self, bus: &mut Peripherals) {
        self.fetch(bus);
    }

    /// fetch, decode, execute を繰り返す
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }
}
