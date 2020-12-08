use dynasmrt::{dynasm, DynasmApi, DynasmLabelApi, AssemblyOffset};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use super::*;

    #[test]
    fn part1() {
        let lines: Vec<_> = include_str!("../input/day8.txt").lines().collect();

        let mut ops = dynasmrt::x64::Assembler::new().unwrap();

        dynasm!(ops
        ; -> jmp_trampoline:
        ; pop rdx
        ; mov WORD [rdx - 6], WORD 0xcc
        ; jmp rax);
        let code_start = ops.offset();

        let mut instr_labels: Vec<_> = (0..lines.len()).map(|_| ops.new_dynamic_label()).collect();
        for (line_num, line) in lines.iter().enumerate() {            
            let (opcode, operand_str) = line.split_at(3);
            let operand: i32 = operand_str[1..].parse().unwrap();
            
            dynasm!(ops
            ; =>instr_labels[line_num]);
            let loc = ops.offset();
            match opcode {
                "nop" => dynasm!(ops
                    ; nop
                    ; mov WORD [rip - 10], WORD 0xcc
                ),
                "acc" => dynasm!(ops
                    ; add r15, DWORD operand
                    ; mov WORD [rip - 16], WORD 0xcc
                ),
                "jmp" => dynasm!(ops
                            ; lea rax, [=>instr_labels[(line_num as i32 + operand) as usize]]
                            ; call ->jmp_trampoline),
                _ => panic!("bad opcode"),
            }
        }

        let mut buf = ops.finalize().unwrap();
        let mut old = 0;
        unsafe {
            VirtualProtect((buf.ptr(AssemblyOffset(0)) as *mut c_void), buf.size(), PAGE_EXECUTE_READWRITE, &mut old);
        }
        let code_fn: extern "win64" fn() -> bool = unsafe { std::mem::transmute(buf.ptr(code_start)) };
        code_fn();
    }
}
