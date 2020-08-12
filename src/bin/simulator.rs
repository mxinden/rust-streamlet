use streamlet::simulator::Builder;

fn main() {
    let mut simulator = Builder::new().build();
    simulator.run();
}
