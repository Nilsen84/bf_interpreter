use std::io::{stdout, Write};
use getch::Getch;
use crate::error::{Result, Error};

fn compute_jumps(code: &[u8]) -> Result<Vec<usize>> {
    let mut jumps = vec![0; code.len()];
    let mut stack = vec![];

    for (i, op) in code.iter().enumerate() {
        match op {
            b'[' => stack.push(i),
            b']' => {
                let start = stack.pop().ok_or(Error::MissingOpeningBracket(i))?;
                jumps[start] = i;
                jumps[i] = start;
            },
            _ => {}
        }
    }

    if let Some(i) = stack.pop() {
        Err(Error::MissingClosingBracket(i))?
    }

    Ok(jumps)
}

pub fn execute(code: &[u8], memory: &mut [u8]) -> Result<()> {
    let jumps = compute_jumps(code)?;
    let mut ip = 0;
    let mut mp = 0;
    let getch = Getch::new();

    while ip < code.len() {
        match code[ip] {
            b'>' => { mp += 1 }
            b'<' => { mp -= 1 },
            b'+' => { memory[mp] += 1 }
            b'-' => { memory[mp] -= 1 },
            b'.' => {
                let mut stdout = stdout().lock();
                stdout.write(&[memory[mp]]).map_err(|e| Error::StdoutError(e))?;
                stdout.flush().map_err(|e| Error::StdoutError(e))?;
            }
            b',' => { memory[mp] = getch.getch().map_err(|e| Error::StdinError(e))?; }
            b'[' => if memory[mp] == 0 { ip = jumps[ip] }
            b']' => { ip = jumps[ip]; continue }
            _ => {}
        }

        ip += 1;
    }

    Ok(())
}