#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(allocator_api)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use shivini::boojum::config::CSConfig;
use shivini::boojum::config::CSSetupConfig;
use shivini::boojum::config::CSWitnessEvaluationConfig;
use shivini::boojum::cs::gates::ConstantAllocatableCS;
use shivini::boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use shivini::boojum::cs::gates::FmaGateInBaseWithoutConstantParams;
use shivini::boojum::cs::gates::ReductionGate;
use shivini::boojum::cs::gates::ReductionGateParams;
use shivini::boojum::cs::traits::cs::ConstraintSystem;
use shivini::boojum::cs::Place;
use shivini::boojum::cs::Variable;
use shivini::boojum::field::SmallField;
use shivini::boojum::gadgets::boolean::Boolean;
use shivini::boojum::gadgets::num::Num;
use shivini::boojum::gadgets::traits::allocatable::CSAllocatable;
use shivini::boojum::gadgets::traits::selectable::Selectable;
use shivini::boojum::gadgets::traits::witnessable::CSWitnessable;
use shivini::boojum::gadgets::traits::witnessable::WitnessHookable;
use shivini::boojum::gadgets::u32::UInt32;

use mersenne_field::field::{Field, PrimeField};
use mersenne_field::Mersenne31Field;

use shivini::boojum::gadgets::u8::UInt8;

pub mod extension_trait;
pub mod field;
pub mod fourth_ext;
pub mod second_ext;

pub use field::MersenneField;
pub use fourth_ext::MersenneQuartic;
pub use second_ext::MersenneComplex;

pub const M31_MODULUS: u64 = Mersenne31Field::CHARACTERISTICS; // 2^31 - 1
