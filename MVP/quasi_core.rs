//! quasi_core.rs
//! Quantum Superpositional Infrastructure (QUASI)
//! Author: OBINexus / Namdi
//! License: OBINexus Public License (OPL)
//!
//! Each QuasiState represents a quantum duality of matter/antimatter
//! encoded in a topological “iceberg” model — visible surface and hidden depth.

use std::fmt::{Display, Formatter};

/// Represents the fundamental quantum token.
/// Carries both type identity and quantized value.
#[derive(Clone, Debug)]
pub struct QToken {
    pub qtype: String,
    pub value: f64, // symbolic or probabilistic representation
}

/// Represents a dual field (matter ↔ antimatter) in topological space.
#[derive(Clone, Debug)]
pub struct QuasiField {
    pub matter: QToken,
    pub antimatter: QToken,
    pub coherence: f64, // 0.0 - 1.0 range, quantum symmetry measure
}

/// The primary computational entity — a state existing in superposition.
#[derive(Clone, Debug)]
pub struct QuasiState {
    pub id: String,
    pub field: QuasiField,
    pub observed: bool,
}

impl QuasiState {
    /// Initialize a new superposed quantum state.
    pub fn new(id: &str, qtype: &str, matter: f64, antimatter: f64) -> Self {
        let field = QuasiField {
            matter: QToken {
                qtype: qtype.to_string(),
                value: matter,
            },
            antimatter: QToken {
                qtype: qtype.to_string(),
                value: antimatter,
            },
            coherence: 1.0 - ((matter - antimatter).abs() / (matter.abs() + antimatter.abs() + 1.0)),
        };
        Self {
            id: id.to_string(),
            field,
            observed: false,
        }
    }

    /// Collapse the quantum superposition — observation defines truth.
    pub fn observe(&mut self) -> f64 {
        self.observed = true;
        // Compute the "collapsed" reality
        (self.field.matter.value + self.field.antimatter.value) / 2.0
    }

    /// Measure current coherence (how stable the state is)
    pub fn measure_coherence(&self) -> f64 {
        self.field.coherence
    }

    /// Perform a quantum inversion — swap matter ↔ antimatter
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.field.matter, &mut self.field.antimatter);
    }
}

impl Display for QuasiState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let obs_state = if self.observed { "Observed" } else { "Superposed" };
        write!(
            f,
            "🧊 QuasiState [{}]\nType: {}\nMatter: {:.3}\nAntimatter: {:.3}\nCoherence: {:.3}\nState: {}",
            self.id,
            self.field.matter.qtype,
            self.field.matter.value,
            self.field.antimatter.value,
            self.field.coherence,
            obs_state
        )
    }
}

/// Example quantum routine
pub fn demo() {
    let mut q = QuasiState::new("iceberg_01", "energy", 42.0, -41.8);
    println!("{}", q);

    println!("\n→ Measuring coherence: {:.3}", q.measure_coherence());
    println!("→ Observing collapse: {:.3}", q.observe());
    println!("→ State after observation:\n{}", q);

    println!("\n→ Performing inversion...");
    q.invert();
    println!("→ State after inversion:\n{}", q);
}

fn main() {
    println!("=== QUASI :: Quantum Superpositional Infrastructure ===");
    demo();
}
