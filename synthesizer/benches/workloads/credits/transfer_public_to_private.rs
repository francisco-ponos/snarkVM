// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use crate::{BenchmarkOperations, Operation, SetupOperations, Workload};

use console::{
    network::Network,
    program::{Address, Literal, Plaintext, Value, Zero, U64},
};
use snarkvm_synthesizer::Program;
use snarkvm_utilities::TestRng;

use std::{marker::PhantomData, str::FromStr};

pub struct TransferPublicToPrivate<N: Network> {
    num_executions: usize,
    phantom: PhantomData<N>,
}

impl<N: Network> TransferPublicToPrivate<N> {
    pub fn new(num_executions: usize) -> Self {
        Self { num_executions, phantom: Default::default() }
    }
}

impl<N: Network> Workload<N> for TransferPublicToPrivate<N> {
    fn name(&self) -> String {
        format!("transfer_public_to_private/{}_executions", self.num_executions)
    }

    fn init(&mut self) -> (SetupOperations<N>, BenchmarkOperations<N>) {
        // Construct the program.
        let program = Program::from_str(&format!(
            r"
program transfer_public_to_private_{}.aleo;
mapping account:
    key left as address.public;
    value right as u64.public;
function transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    finalize r0 r1;
finalize transfer_public_to_private:
    input r0 as address.public;
    input r1 as u64.public;
    get.or_init account[r0] 0u64 into r2;
    sub r2 r1 into r3;
    set r3 into account[r0];
",
            self.num_executions
        ))
        .unwrap();
        let setups = vec![vec![Operation::Deploy(Box::new(program))]];

        // Initialize storage for the benchmark operations.
        let mut benchmarks = Vec::with_capacity(self.num_executions);
        // Initialize an RNG for generating the operations.
        let rng = &mut TestRng::default();
        // Construct the operations.
        for _ in 0..self.num_executions {
            let sender = Address::rand(rng);
            benchmarks.push(Operation::Execute(
                format!("transfer_public_to_private_{}.aleo", self.num_executions),
                "transfer_public_to_private".to_string(),
                vec![
                    Value::Plaintext(Plaintext::from(Literal::Address(sender))),
                    Value::Plaintext(Plaintext::from(Literal::U64(U64::zero()))),
                ],
            ));
        }

        (setups, benchmarks)
    }
}
