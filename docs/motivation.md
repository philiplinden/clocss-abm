# Motivation
Recent advancements in space technologies have prompted a surge in lunar missions, both crewed and uncrewed. Such an influx demands scalable, commercially-accessible Positioning, Navigation, and Timing (PNT) frameworks for the development of a cislunar economy. In order to bring PNT infrastructure to the lunar ecosystem and have it be as ubiquitous and as useful as Global Navigation Satellite Systems (GNSS) are in the interoperable Space Service Volume (SSV), there needs to be accurate, traceable and accessible timing and ranging infrastructure that is also resilient, reliable and flexible. NASA’s LunaNet and ESA’s Moonlight are two major initiatives to promote interoperability and connectivity in cislunar space by providing a common communications framework and standards. Lunar constellations equivalent to terrestrial GNSS are one approach to delivering a cislunar PNT but it is not the only solution. Peer-to-peer networks of satellites with precision timekeeping may serve as an alternative method of implementing a PNT service to traditional GNSS constellations.

This project will use agent-based modeling to compare satellite network topologies using metrics known from the current PNT solution such as accuracy, availability, continuity and integrity, in addition to costs, timeline and technology development requirements of implementing each system in a cislunar context. The goal is to develop the proposed solution to a level mature enough to predict the system’s performance relative to the number and distribution of interconnected assets, and quantitatively demonstrate that our approach becomes more robust and performant as it scales to service the anticipated demands of a thriving lunar ecosystem. The study will also consider specific lunar PNT user needs and infrastructure combination opportunities, as well as requirements for Earth / Earth Orbit systems to be usable with minimum changes for lunar applications.

## Diverse Cislunar Ecosystems of PNT and Communications Infrastructures are Inevitable

A fundamental characteristic of the this proposed design is its peer-to-peer topology, ensuring resilience against centralized points of failure. The structure stands robust against interference, adversarial or accidental. The decentralized nature of the design further augments its flexibility, permitting in-flight mission adaptations and potential as a backup for lunar missions, reducing their dependence on individualized PNT structures. Since Wi-Wi is a protocol that works with any radio band, it is likely that several independent PNT services could emerge on different parts of the spectrum. This allows actors to maintain closed PNT utilities or to offer services for a self-sustaining, monetizable, commercially owned-and-operated lunar infrastructure. Critically, public and private PNT utilities may coexist under this paradigm, like how a single transponder can access both terrestrial open-air radio and encrypted radio channels. In essence, this philosophy aims to nurture a resilient PNT ecosystem that accommodates both public and private ventures. Through a credibly neutral protocol for timekeeping, bad actors would not only have difficulty manipulating the service, but they may use this infrastructure themselves and even work to support its canonization.

## Interoperability with Other Missions

In the design of a lunar PNT system one important consideration is the definition of a reference frame to allow for absolute position. This time transfer and relative position concept could be used to define a network of realization points (fixed points for the reference frame) to assist in the realization of a Lunar Reference Frame. These lunar realization points would be located on the near side of the Moon and equipped with e.g., laser retroreflectors for accurate ranging from Earth by the existing Lunar Laser Ranging (LLR) stations. Like GNSS, passive receivers can obtain the time and position in reference to the PNT node’s position by observing the transmitted signal so long as the receiver’s clock is synchronized to the node.