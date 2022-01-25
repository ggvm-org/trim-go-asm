use std::{
    borrow::BorrowMut,
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct Instruction {
    pub kind: InstructionKind,
    pub pc: usize,
}

impl Instruction {
    fn replace_abi_fn(self) -> Self {
        Self {
            kind: self.kind.replace_abiinternal(),
            ..self
        }
    }

    fn rename_for_mac(self) -> Self {
        Self {
            kind: self.kind.rename_for_mac(),
            ..self
        }
    }

    fn call_middle_dot(self) -> Self {
        Self {
            kind: self.kind.call_middle_dot(),
            ..self
        }
    }
}

impl Deref for Instruction {
    type Target = InstructionKind;
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

impl DerefMut for Instruction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_for_each(|inst| write!(f, "{}\n", inst))
    }
}

impl Instructions {
    pub fn replace_abi_fn(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|inst| Instruction::replace_abi_fn(inst))
                .collect(),
        )
    }

    pub fn rename_for_mac(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|inst| Instruction::rename_for_mac(inst))
                .collect(),
        )
    }

    pub fn call_middle_dot(self) -> Self {
        Self(
            self.0
                .into_iter()
                .map(|inst| Instruction::call_middle_dot(inst))
                .collect(),
        )
    }

    pub fn optimize_for_me(self) -> Self {
        self.rename_for_mac().replace_abi_fn().call_middle_dot()
    }
}

impl From<String> for Instruction {
    fn from(line: String) -> Self {
        let mut sp = line.splitn(2, '\t');

        let pc = sp
            .next()
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let inst = sp.fold(String::new(), |mut acc, x| {
            acc.push_str(x);
            acc
        });

        Instruction {
            pc,
            kind: InstructionKind(inst),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InstructionKind(pub String);

impl fmt::Display for InstructionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for InstructionKind {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for InstructionKind {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<String> for InstructionKind {
    fn from(v: String) -> Self {
        Self(v)
    }
}

impl From<Vec<Instruction>> for Instructions {
    fn from(v: Vec<Instruction>) -> Self {
        Self(v)
    }
}

impl InstructionKind {
    fn is_jls(&self) -> bool {
        self.starts_with("JLS")
    }

    pub fn replace_abiinternal(&self) -> Self {
        // 4 means NOSPLIT
        self.replace("NOSPLIT|ABIInternal", "4")
            .replace("ABIInternal", "4")
            .into()
    }

    pub fn rename_for_mac(&self) -> Self {
        self.replacen("\"\".", "main·", 1)
            .replace("\"\".", "")
            .replace("~", "")
            .into()
    }

    pub fn call_middle_dot(&self) -> Self {
        if self.contains("CALL") {
            self.replace(".", "·").into()
        } else {
            self.clone()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instructions(Vec<Instruction>);

impl Deref for Instructions {
    type Target = Vec<Instruction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Instructions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Instructions {
    pub fn new(insts: Vec<Instruction>) -> Self {
        Self(insts)
    }

    pub fn trim_goroutine_instructions(&mut self) -> bool {
        let insts: &mut Vec<Instruction> = self.0.borrow_mut();
        let contains_jls = insts.iter().position(|inst| inst.is_jls());
        if let None = contains_jls {
            return false;
        }
        let position = contains_jls.unwrap();
        let jls_inst = insts.get(position).unwrap();
        let kind = &jls_inst.kind;
        let goto = kind.split('\t').nth(1).unwrap().parse().unwrap();
        let goroutine_after = insts
            .iter()
            .position(|inst| inst.pc == goto)
            .expect("no pc");
        self.0 = insts[position + 2..goroutine_after].to_vec().clone();
        true
    }
}
