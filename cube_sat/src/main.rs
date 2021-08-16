mod cube_sat;

use cube_sat::*;

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mailbox = Mailbox::new();
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);

        base.send(&mut mailbox, sat.id, String::from("hello there!"));
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mailbox);

        println!("msg: {:?}", msg);
    }
}
