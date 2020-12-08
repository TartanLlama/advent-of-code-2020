use dynasmrt::{dynasm, relocations::Relocation, AssemblyOffset, DynasmApi, DynasmLabelApi};
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;

#[cfg(test)]
mod tests {
    use std::ffi::c_void;

    use dynasmrt::x64::X64Relocation;

    use super::*;

    #[test]
    fn part1() {
        let lines: Vec<_> = include_str!("../input/day8.txt").lines().collect();

        let mut ops = dynasmrt::x64::Assembler::new().unwrap();
        let code_start = ops.offset();

        //rax will be our accumulator, so we need to zero it out
        dynasm!(ops; xor rax, rax); 

        //We'll mark the start of every source instruction with a label so we can jump between them
        let mut instr_labels: Vec<_> = (0..lines.len()).map(|_| ops.new_dynamic_label()).collect();

        for (line_num, line) in lines.iter().enumerate() {
            let (opcode, operand_str) = line.split_at(3);
            let operand: i32 = operand_str[1..].parse().unwrap();

            //Output the label for this instruction
            dynasm!(ops; =>instr_labels[line_num]);
            let loc = ops.offset();

            //HERE BE DRAGONS
            //This instruction modifies itself by scribbling over its opcode with 0xc3 (ret)
            dynasm!(ops; mov DWORD [rip - 10], DWORD 0xc3);

            //Now we JIT code for each source instruction
            match opcode {
                "nop" => (), //Nops are free *does dance*
                "acc" => dynasm!(ops; add rax, DWORD operand),
                //Jump to the instruction using the label number calculated from the offset
                "jmp" => dynasm!(ops; jmp =>instr_labels[(line_num as i32 + operand) as usize]),
                _ => panic!("bad opcode"),
            }
        }

        let mut buf = ops.finalize().unwrap();
        
        //We need to mark the memory for our code as readable, writeable and executable
        unsafe {
            let mut old = 0;
            VirtualProtect(
                (buf.ptr(AssemblyOffset(0)) as *mut c_void),
                buf.size(),
                PAGE_EXECUTE_READWRITE,
                &mut old,
            );
        }

        //Cast our buffer into a function we can call
        let find_solution: extern "win64" fn() -> i32 =
            unsafe { std::mem::transmute(buf.ptr(code_start)) };

        println!("Solution: {}", find_solution());
    }
}
