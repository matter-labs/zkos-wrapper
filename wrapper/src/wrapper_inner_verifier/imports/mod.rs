use super::*;

#[cfg(feature = "wrap_final_machine")]
include!("circuit_layout_for_final_machine.rs");
#[cfg(feature = "wrap_final_machine")]
include!("circuit_quotient_for_final_machine.rs");
#[cfg(feature = "wrap_final_machine")]
include!("final_state_constants_for_final_machine.rs");

#[cfg(feature = "wrap_with_reduced_log_23")]
include!("circuit_layout.rs");
#[cfg(feature = "wrap_with_reduced_log_23")]
include!("circuit_quotient.rs");
#[cfg(feature = "wrap_with_reduced_log_23")]
include!("final_state_constants.rs");
