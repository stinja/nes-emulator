mod instruction;
mod opcodes;
mod tests;

use self::instruction::{Instruction, InstructionMode, InstructionOperation};
use crate::bus::Bus;
use crate::types::{Address, Result, BitRead};

const ADDRESS_NMI: Address = 0xFFFA;
const ADDRESS_RESET: Address = 0xFFFC;
const ADDRESS_IRQ: Address = 0xFFFE;

pub struct Cpu {
    bus: Bus,
    registers: RegisterSet,
    vectors: VectorSet,
}

impl Cpu {
    pub fn new(bus: Bus) -> Result<Self> {
        let vectors = VectorSet {
            nmi: bus.read_u16(ADDRESS_NMI)?,
            reset: bus.read_u16(ADDRESS_RESET)?,
            irq: bus.read_u16(ADDRESS_IRQ)?,
        };

        let mut registers = RegisterSet::new();
        registers.pc = vectors.reset;

        Ok(Self { bus, registers, vectors })
    }

    pub fn start(&mut self) -> Result {
        while let Some(instruction) = self.determine_instruction_next()? {
            self.process_instruction(instruction)?;
        }

        Ok(())
    }

    fn determine_instruction_next(&self) -> Result<Option<Instruction>> {
        let opcode = self.bus.read(self.registers.pc);
        let instruction = Instruction::from_opcode(opcode);

        // TODO: check if this is correct
        if self.registers.pc + (instruction.len() as Address) < ADDRESS_NMI {
            Ok(Some(instruction))
        } else {
            Ok(None)
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result {
        // account for opcode
        self.registers.pc += 1;

        let instruction_len = instruction.len();
        self.run_instruction(instruction)?;
        self.registers.pc += (instruction_len as Address) - 1;

        Ok(())
    }

    // TODO: find clean way to prevent unwrapping
    fn run_instruction(&mut self, instruction: Instruction) -> Result {
        match instruction.operation() {
            InstructionOperation::Adc => {
                self.run_adc(self.determine_input_byte(instruction.mode())?.unwrap());
            },
            InstructionOperation::And => unimplemented!("execute | And"),
            InstructionOperation::Asl => {
                self.run_asl(self.determine_input_location(instruction.mode())?);
            },
            InstructionOperation::Bcc => unimplemented!("execute | Bcc"),
            InstructionOperation::Bcs => unimplemented!("execute | Bcs"),
            InstructionOperation::Beq => unimplemented!("execute | Beq"),
            InstructionOperation::Bit => unimplemented!("execute | Bit"),
            InstructionOperation::Bmi => unimplemented!("execute | Bmi"),
            InstructionOperation::Bne => unimplemented!("execute | Bne"),
            InstructionOperation::Bpl => unimplemented!("execute | Bpl"),
            InstructionOperation::Brk => {
                // TODO
            },
            InstructionOperation::Bvc => unimplemented!("execute | Bvc"),
            InstructionOperation::Bvs => unimplemented!("execute | Bvs"),
            InstructionOperation::Clc => self.run_clc(),
            InstructionOperation::Cld => self.run_cld(),
            InstructionOperation::Cli => self.run_cli(),
            InstructionOperation::Clv => self.run_clv(),
            InstructionOperation::Cmp => unimplemented!("execute | Cmp"),
            InstructionOperation::Cpx => unimplemented!("execute | Cpx"),
            InstructionOperation::Cpy => unimplemented!("execute | Cpy"),
            InstructionOperation::Dec => unimplemented!("execute | Dec"),
            InstructionOperation::Dex => unimplemented!("execute | Dex"),
            InstructionOperation::Dey => unimplemented!("execute | Dey"),
            InstructionOperation::Eor => unimplemented!("execute | Eor"),
            InstructionOperation::Inc => unimplemented!("execute | Inc"),
            InstructionOperation::Inx => self.run_inx(),
            InstructionOperation::Iny => self.run_iny(),
            InstructionOperation::Jmp => unimplemented!("execute | Jmp"),
            InstructionOperation::Jsr => unimplemented!("execute | Jsr"),
            InstructionOperation::Lda => {
                self.run_lda(self.determine_input_byte(instruction.mode())?.unwrap());
            },
            InstructionOperation::Ldx => {
                self.run_ldx(self.determine_input_byte(instruction.mode())?.unwrap());
            },
            InstructionOperation::Ldy => {
                self.run_ldy(self.determine_input_byte(instruction.mode())?.unwrap());
            },
            InstructionOperation::Lsr => unimplemented!("execute | Lsr"),
            InstructionOperation::Nop => {},
            InstructionOperation::Ora => unimplemented!("execute | Ora"),
            InstructionOperation::Pha => unimplemented!("execute | Pha"),
            InstructionOperation::Php => unimplemented!("execute | Php"),
            InstructionOperation::Pla => unimplemented!("execute | Pla"),
            InstructionOperation::Plp => unimplemented!("execute | Plp"),
            InstructionOperation::Rol => unimplemented!("execute | Rol"),
            InstructionOperation::Ror => unimplemented!("execute | Ror"),
            InstructionOperation::Rti => unimplemented!("execute | Rti"),
            InstructionOperation::Rts => unimplemented!("execute | Rts"),
            InstructionOperation::Sbc => unimplemented!("execute | Sbc"),
            InstructionOperation::Sec => self.run_sec(),
            InstructionOperation::Sed => self.run_sed(),
            InstructionOperation::Sei => self.run_sei(),
            InstructionOperation::Sta => unimplemented!("execute | Sta"),
            InstructionOperation::Stx => unimplemented!("execute | Stx"),
            InstructionOperation::Sty => unimplemented!("execute | Sty"),
            InstructionOperation::Tax => self.run_tax(),
            InstructionOperation::Tay => self.run_tay(),
            InstructionOperation::Tsx => unimplemented!("execute | Tsx"),
            InstructionOperation::Txa => self.run_txa(),
            InstructionOperation::Txs => unimplemented!("execute | Txs"),
            InstructionOperation::Tya => self.run_tya(),
        };

        Ok(())
    }

    fn determine_input_byte(&self, instruction_mode: InstructionMode) -> Result<Option<u8>> {
        let input = match instruction_mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => unimplemented!("input byte | Accumulator"),
            InstructionMode::Immediate => {
                Some(self.bus.read(self.registers.pc))
            },
            InstructionMode::Relative => unimplemented!("input byte | Relative"),
            InstructionMode::ZeroPage => unimplemented!("input byte | ZeroPage"),
            InstructionMode::ZeroPageX => unimplemented!("input byte | ZeroPageX"),
            InstructionMode::ZeroPageY => unimplemented!("input byte | ZeroPageY"),
            InstructionMode::Absolute => {
                let address = self.bus.read_u16(self.registers.pc)?;
                Some(self.bus.read(address))
            },
            InstructionMode::AbsoluteX => unimplemented!("input byte | AbsoluteX"),
            InstructionMode::AbsoluteY => unimplemented!("input byte | AbsoluteY"),
            InstructionMode::Indirect => unimplemented!("input byte | Indirect"),
            InstructionMode::IndirectX => unimplemented!("input byte | IndirectX"),
            InstructionMode::IndirectY => unimplemented!("input byte | IndirectY"),
        };

        Ok(input)
    }

    fn determine_input_location(&self, instruction_mode: InstructionMode) -> Result<Location> {
        let input = match instruction_mode {
            InstructionMode::Implied => unimplemented!("input location | Implied"),
            InstructionMode::Accumulator => Location::Accumulator,
            InstructionMode::Immediate => unimplemented!("input location | Immediate"),
            InstructionMode::Relative => {
                // TODO: how to handle overflow?
                let offset = i32::from(self.bus.read(self.registers.pc) as i8);
                let address = (self.registers.pc as i32).wrapping_add(offset) as Address;
                Location::Address(address)
            },
            InstructionMode::ZeroPage => unimplemented!("input location | ZeroPage"),
            InstructionMode::ZeroPageX => {
                // TODO: check if this would actually wrap around
                let address = self.bus.read(self.registers.pc);
                Location::Address(address.wrapping_add(self.registers.x) as Address)
            },
            InstructionMode::ZeroPageY => unimplemented!("input location | ZeroPageY"),
            InstructionMode::Absolute => unimplemented!("input location | Absolute"),
            InstructionMode::AbsoluteX => unimplemented!("input location | AbsoluteX"),
            InstructionMode::AbsoluteY => unimplemented!("input location | AbsoluteY"),
            InstructionMode::Indirect => unimplemented!("input location | Indirect"),
            InstructionMode::IndirectX => unimplemented!("input location | IndirectX"),
            InstructionMode::IndirectY => unimplemented!("input location | IndirectY"),
        };

        Ok(input)
    }

    fn persist_result(&mut self, result: u8, location: Location) {
        match location {
            Location::Accumulator => self.registers.a = result,
            Location::Address(address) => self.bus.write(address, result),
        }
    }

    fn run_adc(&mut self, input: u8) {
        let carry = (self.registers.p & StatusFlags::CARRY).bits();
        let a_old = self.registers.a;
        let a_new = self.registers.a.wrapping_add(input).wrapping_add(carry);
        self.registers.a = a_new;

        self.registers.p.set(StatusFlags::CARRY, is_carry(input, a_new));
        self.registers.p.set(StatusFlags::ZERO, a_new == 0);
        self.registers.p.set(StatusFlags::OVERFLOW, has_overflown(a_old, a_new));
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(a_new));
    }

    fn run_and(&mut self, input: u8) {
        unimplemented!("run | and");
    }

    fn run_asl(&mut self, target: Location) {
        let input = match target {
            Location::Accumulator => self.registers.a,
            Location::Address(address) => self.bus.read(address),
        };
        let result = input.wrapping_shl(1);
        self.persist_result(result, target);

        self.registers.p.set(StatusFlags::CARRY, is_carry(input, result));
        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(result));
    }

    fn run_bcc(&mut self, target: Address) {
        unimplemented!("run | bcc");
    }

    fn run_bcs(&mut self, target: Address) {
        unimplemented!("run | bcs");
    }

    fn run_beq(&mut self, target: Address) {
        unimplemented!("run | beq");
    }

    fn run_bit(&mut self, target: Address) {
        unimplemented!("run | bit");
    }

    fn run_bmi(&mut self, target: Address) {
        unimplemented!("run | bmi");
    }

    fn run_bne(&mut self, target: Location) {
        unimplemented!("run | bne");
    }

    fn run_bpl(&mut self, target: Location) {
        unimplemented!("run | bpl");
    }

    fn run_brk(&mut self) {
        unimplemented!("run | brk");
    }

    fn run_bvc(&mut self, target: Address) {
        unimplemented!("run | bvc");
    }

    fn run_bvs(&mut self, target: Address) {
        unimplemented!("run | bvs");
    }

    fn run_clc(&mut self) {
        self.registers.p.remove(StatusFlags::CARRY);
    }

    fn run_cld(&mut self) {
        self.registers.p.remove(StatusFlags::DECIMAL);
    }

    fn run_cli(&mut self) {
        self.registers.p.remove(StatusFlags::INTERRUPT_DISABLE);
    }

    fn run_clv(&mut self) {
        self.registers.p.remove(StatusFlags::OVERFLOW);
    }

    fn run_cmp(&mut self, input: u8) {
        unimplemented!("run | cmp");
    }

    fn run_cpx(&mut self, input: u8) {
        unimplemented!("run | cpx");
    }

    fn run_cpy(&mut self, input: u8) {
        unimplemented!("run | cpy");
    }

    fn run_dec(&mut self, target: Address) {
        unimplemented!("run | dec");
    }

    fn run_dex(&mut self) {
        unimplemented!("run | dex");
    }

    fn run_dey(&mut self) {
        unimplemented!("run | dey");
    }

    fn run_eor(&mut self, input: u8) {
        unimplemented!("run | eor");
    }

    fn run_inc(&mut self, target: Address) {
        unimplemented!("run | inc");
    }

    fn run_inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);

        self.registers.p.set(StatusFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.x));
    }

    fn run_iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);

        self.registers.p.set(StatusFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.y));
    }

    fn run_jmp(&mut self, target: Address) {
        unimplemented!("run | jmp");
    }

    fn run_jsr(&mut self, target: Address) {
        unimplemented!("run | jsr");
    }

    fn run_lda(&mut self, input: u8) {
        self.registers.a = input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_ldx(&mut self, input: u8) {
        self.registers.x = input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.x));
    }

    fn run_ldy(&mut self, input: u8) {
        self.registers.y = input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.y));
    }

    fn run_lsr(&mut self, target: Location) {
        unimplemented!("run | lsr");
    }

    fn run_ora(&mut self, input: u8) {
        unimplemented!("run | ora");
    }

    fn run_pha(&mut self) {
        unimplemented!("run | pha");
    }

    fn run_php(&mut self) {
        unimplemented!("run | php");
    }

    fn run_pla(&mut self) {
        unimplemented!("run | pla");
    }

    fn run_plp(&mut self) {
        unimplemented!("run | plp");
    }

    fn run_rol(&mut self, target: Location) {
        unimplemented!("run | rol");
    }

    fn run_ror(&mut self, target: Location) {
        unimplemented!("run | ror");
    }

    fn run_rti(&mut self) {
        unimplemented!("run | rti");
    }

    fn run_rts(&mut self) {
        unimplemented!("run | rts");
    }

    fn run_sbc(&mut self, input: u8) {
        unimplemented!("run | sbc");
    }

    fn run_sec(&mut self) {
        self.registers.p.insert(StatusFlags::CARRY);
    }

    fn run_sed(&mut self) {
        self.registers.p.insert(StatusFlags::DECIMAL);
    }

    fn run_sei(&mut self) {
        self.registers.p.insert(StatusFlags::INTERRUPT_DISABLE);
    }

    fn run_sta(&mut self, target: Address) {
        unimplemented!("run | sta");
    }

    fn run_stx(&mut self, target: Address) {
        unimplemented!("run | stx");
    }

    fn run_sty(&mut self, target: Address) {
        unimplemented!("run | sty");
    }

    fn run_tax(&mut self) {
        self.registers.x = self.registers.a;

        self.registers.p.set(StatusFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.x));
    }

    fn run_tay(&mut self) {
        self.registers.y = self.registers.a;

        self.registers.p.set(StatusFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.y));
    }

    fn run_tsx(&mut self) {
        unimplemented!("run | tsx");
    }

    fn run_txa(&mut self) {
        self.registers.a = self.registers.x;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_txs(&mut self) {
        unimplemented!("run | txs");
    }

    fn run_tya(&mut self) {
        self.registers.a = self.registers.y;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }
}

fn is_carry(input: u8, value_new: u8) -> bool {
    value_new < input
}

fn has_overflown(value_old: u8, value_new: u8) -> bool {
    value_old.read_bit(7) != value_new.read_bit(7)
}

fn is_negative(value: u8) -> bool {
    value.is_bit_set(7)
}

#[derive(Debug, Eq, PartialEq)]
struct RegisterSet {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: StatusFlags,
    pc: Address,
}

impl RegisterSet {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFF,
            p: StatusFlags::empty(),
            pc: 0,
        }
    }
}

struct VectorSet {
    nmi: Address,
    reset: Address,
    irq: Address,
}

bitflags! {
    struct StatusFlags: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const BREAK_LEFT = 0b0010_0000;
        const BREAK_RIGHT = 0b0001_0000;
        const DECIMAL = 0b0000_1000;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const ZERO = 0b0000_0010;
        const CARRY = 0b0000_0001;
    }
}

impl StatusFlags {
    fn set_break(&mut self, break_type: BreakType) {
        match break_type {
            BreakType::Internal => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.insert(StatusFlags::BREAK_RIGHT);
            },
            BreakType::Instruction => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.remove(StatusFlags::BREAK_RIGHT);
            },
        }
    }

    fn clear_break(&mut self) {
        self.remove(StatusFlags::BREAK_LEFT);
        self.remove(StatusFlags::BREAK_RIGHT);
    }
}

enum BreakType {
    Internal,
    Instruction,
}

#[derive(Debug, Eq, PartialEq)]
enum Location {
    Accumulator,
    Address(Address),
}