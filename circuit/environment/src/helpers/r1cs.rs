// Copyright (C) 2019-2023 Aleo Systems Inc.
// This file is part of the snarkVM library.

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
// http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{
    helpers::{Constraint, Counter},
    prelude::*,
    LookupConstraint,
};
use snarkvm_algorithms::r1cs::LookupTable;
use snarkvm_fields::PrimeField;

use std::rc::Rc;

pub type Scope = String;

#[derive(Debug)]
pub struct R1CS<F: PrimeField> {
    constants: Vec<Variable<F>>,
    public: Vec<Variable<F>>,
    private: Vec<Variable<F>>,
    constraints: Vec<Constraint<F>>,
    lookup_constraints: Vec<LookupConstraint<F>>,
    counter: Counter<F>,
    pub tables: Vec<LookupTable<F>>,
    nonzeros: (u64, u64, u64),
}

impl<F: PrimeField> R1CS<F> {
    /// Returns a new instance of a constraint system.
    pub fn new() -> Self {
        Self {
            constants: Default::default(),
            public: vec![Variable::Public(0u64, Rc::new(F::one()))],
            private: Default::default(),
            constraints: Default::default(),
            lookup_constraints: Default::default(),
            counter: Default::default(),
            tables: Default::default(),
            nonzeros: (0, 0, 0),
        }
    }

    pub fn add_lookup_table(&mut self, table: LookupTable<F>) {
        self.tables.push(table);
    }

    /// Appends the given scope to the current environment.
    pub fn push_scope<S: Into<String>>(&mut self, name: S) -> Result<(), String> {
        self.counter.push(name)
    }

    /// Removes the given scope from the current environment.
    pub fn pop_scope<S: Into<String>>(&mut self, name: S) -> Result<(), String> {
        self.counter.pop(name)
    }

    /// Returns a new constant with the given value and scope.
    pub fn new_constant(&mut self, value: F) -> Variable<F> {
        let variable = Variable::Constant(Rc::new(value));
        self.constants.push(variable.clone());
        self.counter.increment_constant();
        variable
    }

    /// Returns a new public variable with the given value and scope.
    pub fn new_public(&mut self, value: F) -> Variable<F> {
        let variable = Variable::Public(self.public.len() as u64, Rc::new(value));
        self.public.push(variable.clone());
        self.counter.increment_public();
        variable
    }

    /// Returns a new private variable with the given value and scope.
    pub fn new_private(&mut self, value: F) -> Variable<F> {
        let variable = Variable::Private(self.private.len() as u64, Rc::new(value));
        self.private.push(variable.clone());
        self.counter.increment_private();
        variable
    }

    /// Adds one constraint enforcing that `(A * B) == C`.
    pub fn enforce(&mut self, constraint: Constraint<F>) {
        let (a_nonzeros, b_nonzeros, c_nonzeros) = constraint.num_nonzeros();
        self.nonzeros.0 += a_nonzeros;
        self.nonzeros.1 += b_nonzeros;
        self.nonzeros.2 += c_nonzeros;

        self.constraints.push(constraint.clone());
        self.counter.add_constraint(constraint);
    }

    /// Adds one constraint enforcing that `(A * B) == C`.
    pub fn enforce_lookup(&mut self, constraint: LookupConstraint<F>) {
        let (a_nonzeros, b_nonzeros, c_nonzeros) = constraint.num_nonzeros();
        self.nonzeros.0 += a_nonzeros;
        self.nonzeros.1 += b_nonzeros;
        self.nonzeros.2 += c_nonzeros;

        self.lookup_constraints.push(constraint.clone());
        self.counter.add_lookup_constraint(constraint);
    }

    /// Returns `true` if all constraints in the environment are satisfied.
    pub fn is_satisfied(&self) -> bool {
        self.constraints.iter().all(|constraint| constraint.is_satisfied())
    }

    /// Returns `true` if all constraints in the current scope are satisfied.
    pub fn is_satisfied_in_scope(&self) -> bool {
        self.counter.is_satisfied_in_scope()
    }

    /// Returns the current scope.
    pub fn scope(&self) -> Scope {
        self.counter.scope()
    }

    /// Returns the number of constants in the constraint system.
    pub fn num_constants(&self) -> u64 {
        self.constants.len() as u64
    }

    /// Returns the number of public variables in the constraint system.
    pub fn num_public(&self) -> u64 {
        self.public.len() as u64
    }

    /// Returns the number of private variables in the constraint system.
    pub fn num_private(&self) -> u64 {
        self.private.len() as u64
    }

    /// Returns the number of constraints in the constraint system.
    pub fn num_constraints(&self) -> u64 {
        self.constraints.len() as u64
    }

    /// Returns the number of lookup constraints in the constraint system.
    pub fn num_lookup_constraints(&self) -> u64 {
        self.lookup_constraints.len() as u64
    }

    /// Returns the number of nonzeros in the constraint system.
    pub fn num_nonzeros(&self) -> (u64, u64, u64) {
        self.nonzeros
    }

    /// Returns the number of constants for the current scope.
    pub fn num_constants_in_scope(&self) -> u64 {
        self.counter.num_constants_in_scope()
    }

    /// Returns the number of public variables for the current scope.
    pub fn num_public_in_scope(&self) -> u64 {
        self.counter.num_public_in_scope()
    }

    /// Returns the number of private variables for the current scope.
    pub fn num_private_in_scope(&self) -> u64 {
        self.counter.num_private_in_scope()
    }

    /// Returns the number of constraints for the current scope.
    pub fn num_constraints_in_scope(&self) -> u64 {
        self.counter.num_constraints_in_scope()
    }

    /// Returns the number of lookup constraints for the current scope.
    pub fn num_lookup_constraints_in_scope(&self) -> u64 {
        self.counter.num_lookup_constraints_in_scope()
    }

    /// Returns the number of nonzeros for the current scope.
    pub fn num_nonzeros_in_scope(&self) -> (u64, u64, u64) {
        self.counter.num_nonzeros_in_scope()
    }

    /// Returns the public variables in the constraint system.
    pub fn to_public_variables(&self) -> &Vec<Variable<F>> {
        &self.public
    }

    /// Returns the private variables in the constraint system.
    pub fn to_private_variables(&self) -> &Vec<Variable<F>> {
        &self.private
    }

    /// Returns the constraints in the constraint system.
    pub fn to_constraints(&self) -> &Vec<Constraint<F>> {
        &self.constraints
    }

    /// Returns the lookup constraints in the constraint system.
    pub fn to_lookup_constraints(&self) -> &Vec<LookupConstraint<F>> {
        &self.lookup_constraints
    }
}

impl<F: PrimeField> Display for R1CS<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::default();
        for constraint in self.to_constraints() {
            output += &constraint.to_string();
        }
        for constraint in self.to_lookup_constraints() {
            output += &constraint.to_string();
        }
        output += "\n";

        write!(f, "{output}")
    }
}
