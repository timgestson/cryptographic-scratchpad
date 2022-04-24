// Grocery list example from https://medium.com/starkware/arithmetization-i-15c046390862
// Adapted from using decimals to using Field Elements
// Super simple, just add grocery item prices to reach a total
// First line total must be 0 and Last line both values must be the total

use air::{Air, AirContext, Assertion, EvaluationFrame, TraceInfo, TransitionConstraintDegree};
use math::{fields::f64::BaseElement as Felt, FieldElement};
use prover::{FieldExtension, HashFunction, ProofOptions, Prover, Trace, TraceTable};
use verifier::verify;

const TRACE_WIDTH: usize = 2;

pub struct GroceryAir {
    context: AirContext<Felt>,
    result: Felt,
}

impl Air for GroceryAir {
    type BaseField = Felt;
    type PublicInputs = Felt;

    fn new(trace_info: TraceInfo, pub_inputs: Self::BaseField, options: ProofOptions) -> Self {
        let degrees = vec![TransitionConstraintDegree::new(1)];
        assert_eq!(TRACE_WIDTH, trace_info.width());
        GroceryAir {
            context: AirContext::new(trace_info, degrees, options),
            result: pub_inputs,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();
        // expected state width is 2 field elements
        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        // constraints of Grocery List
        // s_{1, i+1} = s_{0, i} + s_{1, i}
        result[0] = next[1] - (current[0] + current[1]);
    }

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            Assertion::single(1, 0, Self::BaseField::ZERO),
            Assertion::single(0, last_step, self.result),
        ]
    }
}

pub struct GroceryProver {
    options: ProofOptions,
}

impl GroceryProver {
    pub fn new(options: ProofOptions) -> Self {
        Self { options }
    }

    pub fn build_trace(&self, item_cost_list: &[Felt]) -> TraceTable<Felt> {
        let mut trace = TraceTable::new(TRACE_WIDTH, 8);
        trace.fill(
            |state| {
                state[0] = item_cost_list[0];
                state[1] = Felt::ZERO;
            },
            |i, state| {
                state[1] += state[0];
                state[0] = item_cost_list[i];
            },
        );
        trace
    }
}

impl Prover for GroceryProver {
    type BaseField = Felt;
    type Air = GroceryAir;
    type Trace = TraceTable<Felt>;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> Felt {
        let last_step = trace.length() - 1;
        trace.get(1, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}

#[test]
fn proof() {
    let options = ProofOptions::new(
        28,
        8,
        0,
        HashFunction::Blake3_256,
        FieldExtension::None,
        4,
        256,
    );
    let prover = GroceryProver::new(options);
    let trace = prover.build_trace(&[
        Felt::new(495),
        Felt::new(798),
        Felt::new(645),
        Felt::new(265),
        Felt::new(354),
        Felt::new(402),
        Felt::new(3454),
    ]);

    let proof = prover.prove(trace).unwrap();
    let proof_with_wrong_output = proof.clone();

    let verification = verify::<GroceryAir>(proof, Felt::new(3454));
    assert!(verification.is_ok());

    let verification = verify::<GroceryAir>(proof_with_wrong_output, Felt::new(100));
    assert!(verification.is_err());
}
