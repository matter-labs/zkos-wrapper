use super::*;

#[cfg(feature = "final_machine")]
include!("circuit_layout_for_final_machine.rs");
#[cfg(feature = "final_machine")]
include!("circuit_quotient_for_final_machine.rs");
#[cfg(feature = "final_machine")]
include!("final_state_constants_for_final_machine.rs");

#[cfg(not(feature = "final_machine"))]
include!("circuit_layout.rs");
#[cfg(not(feature = "final_machine"))]
include!("circuit_quotient.rs");
#[cfg(not(feature = "final_machine"))]
include!("final_state_constants.rs");
