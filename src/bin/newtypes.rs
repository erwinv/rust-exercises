struct PoundsOfForce(f64);
struct Newtons(f64);

impl From<PoundsOfForce> for Newtons {
    fn from(force: PoundsOfForce) -> Newtons {
        Newtons(force.0)
    }
}

fn compute_thruster_force() -> PoundsOfForce {
    todo!("Ask a rocket scientist at NASA")
}

fn set_thruster_force(_force: Newtons) {
    // ...
}

fn main() {
    let force = compute_thruster_force();
    set_thruster_force(force.into());
}
