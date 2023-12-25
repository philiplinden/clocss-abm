fn main() {
    println!("Hello, world!");
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

*/
