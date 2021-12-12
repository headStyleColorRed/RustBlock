use super::hashable::*;
use std::collections::HashSet;

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}

impl Transaction {
    pub fn new(from: &str, to: &str, value: u64, previous_inputs: Vec<Output>) -> Self {
        Transaction {
            inputs: previous_inputs,
            outputs: vec![
                Output {
                    to_addr: to.to_owned(),
                    from_addr: from.to_owned(),
                    value: value,
                },
            ],
        }
    }

    pub fn input_value(&self) -> u64 {
        self.inputs.iter().map(|input| input.value).sum()
    }

    pub fn output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value).sum()
    }

    pub fn input_hashes(&self) -> HashSet<Vec<u8>> {
        self.inputs.iter().map(|input| input.hash()).collect::<HashSet<Vec<u8>>>()
    }

    pub fn output_hashes(&self) -> HashSet<Vec<u8>> {
        self.outputs.iter().map(|output| output.hash()).collect::<HashSet<Vec<u8>>>()
    }

    pub fn is_coinbase(&self) -> bool {
         self.inputs.len() == 0
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.inputs.iter().flat_map(|input| input.bytes()).collect::<Vec<u8>>());
        bytes.extend(self.outputs.iter().flat_map(|output| output.bytes()).collect::<Vec<u8>>());
        bytes
    }
}

#[derive(Clone)]
pub struct Output {
    pub to_addr: String,
    pub from_addr: String,
    pub value: u64,
}

impl Hashable for Output {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.to_addr.as_bytes());
        bytes.extend(self.from_addr.as_bytes());
        bytes.extend(u64_bytes(&self.value));
        bytes
    }
}

