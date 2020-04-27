// Copyright 2019-2020
//     by  Centrality Investments Ltd.
//     and Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::tm_std::*;
use crate::Path;
use crate::registry::TypeParameter;
use crate::form::{Form, CompactForm, MetaForm};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum TypeId {
	/// Any type id
	Any(any::TypeId),
	/// Use a type's path as its unique id
	Path(Path),
	/// Generic type parameter Path + Name
	Parameter(TypeParameter),
	/// Generic type instance
	Generic(GenericTypeId)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct GenericTypeId {
	pub path: Path<CompactForm>,
	pub params: Vec<CompactForm::Type>,
}
