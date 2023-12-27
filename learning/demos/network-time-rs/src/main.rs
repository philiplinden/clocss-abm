mod model;
mod visualization;
use model::NetworkState;

use {
    crate::visualization::vis_state::VisState, krabmaga::bevy::prelude::Color,
    krabmaga::visualization::fields::network::NetworkRender,
    krabmaga::visualization::visualization::Visualization,
};

fn main() {
    let num_agents = 20;
    let dim = (100., 100.);

    let state = NetworkState::new(dim, num_agents);

    let mut app = Visualization::default()
        .with_window_dimensions(1000., 700.)
        .with_simulation_dimensions(dim.0, dim.1)
        .with_background_color(Color::rgb(255., 255., 255.))
        .setup::<VisState, NetworkState>(VisState, state);
    app.add_system(NetworkState::render);
    app.run();
}

/*
                             ╔═══════════════════════╗
                             ║       Stratum 1       ║
                             ║    (Atomic Clocks)    ║
                             ╚═══════════════════════╝
                                  /             \
                                 /               \
                                V                 V
            ╔═══════════════════════╗         ╔═══════════════════════╗
            ║      Stratum 2        ║         ║      Stratum 2        ║
            ║ (Primary Sources)     ║         ║ (Primary Sources)     ║
            ╚═══════════════════════╝         ╚═══════════════════════╝
                 /             \                  /               \
                /               \                /                 \
               V                 V              V                   V
╔═══════════════════════╗     ╔═══════════════════════╗     ╔═══════════════════════╗
║      Stratum 3        ║     ║      Stratum 3        ║<--->║      Stratum 3        ║
║  (Secondary Sources)  ║     ║  (Secondary Sources)  ║     ║  (Secondary Sources)  ║
╚═══════════════════════╝     ╚═══════════════════════╝     ╚═══════════════════════╝
            │                            │                              │
            │                            │                              │
            V                            V                              V
╔═══════════════════════╗     ╔═══════════════════════╗     ╔═══════════════════════╗
║  Client Computers     ║<--->║  Client Computers     ║     ║  Client Computers     ║
╚═══════════════════════╝     ╚═══════════════════════╝     ╚═══════════════════════╝
*/

/*
from https://datatracker.ietf.org/doc/html/rfc5905

    theta_r + +---------\        +----------------+
NTP --------->|  Phase   \  V_d  |                | V_s
    theta_c - | Detector  ------>|  Clock Filter  |----+
    +-------->|          /       |                |    |
    |         +---------/        +----------------+    |
    |                                                  |
  -----                                                |
 /     \                                               |
 | VFO |                                               |
 \     /                                               |
  -----    .......................................     |
    ^      .            Loop Filter              .     |
    |      . +---------+   x  +-------------+    .     |
    | V_c  . |         |<-----|             |    .     |
    +------.-|  Clock  |   y  | Phase/Freq  |<---------+
           . | Adjust  |<-----| Prediction  |    .
           . |         |      |             |    .
           . +---------+      +-------------+    .
           .......................................

              Clock Discipline Feedback Loop

see also: https://stackoverflow.com/questions/19352740/how-does-ntp-clock-discipline-work
*/
