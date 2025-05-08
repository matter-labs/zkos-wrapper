#![feature(unboxed_closures)]
#![feature(fn_traits)]

use boojum::cs::gates::FmaGateInBaseFieldWithoutConstant;
use boojum::cs::gates::ReductionGate;
use boojum::gadgets::traits::allocatable::CSAllocatable;
use boojum::field::SmallField;
use boojum::gadgets::num::Num;
use boojum::cs::traits::cs::ConstraintSystem;
use boojum::cs::Variable;
use boojum::cs::gates::ConstantAllocatableCS;
use boojum::gadgets::traits::witnessable::CSWitnessable;
use boojum::config::CSSetupConfig;
use boojum::gadgets::boolean::Boolean;
use boojum::gadgets::traits::witnessable::WitnessHookable;
use boojum::cs::gates::FmaGateInBaseWithoutConstantParams;
use boojum::cs::gates::ReductionGateParams;
use boojum::gadgets::u32::UInt32;
use boojum::gadgets::traits::selectable::Selectable;
use boojum::cs::Place;
use boojum::config::CSConfig;
use boojum::config::CSWitnessEvaluationConfig;

use mersenne_field::Mersenne31Field;
use mersenne_field::field::{Field, PrimeField};

use boojum::gadgets::u8::UInt8;

pub mod field;
pub mod second_ext;
pub mod fourth_ext;
pub mod extension_trait;

pub use field::MersenneField;
pub use second_ext::MersenneComplex;
pub use fourth_ext::MersenneQuartic;

pub const M31_MODULUS: u64 = Mersenne31Field::CHARACTERISTICS; // 2^31 - 1
