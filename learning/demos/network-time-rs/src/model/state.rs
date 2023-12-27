use std::any::Any;

use krabmaga::{
    engine::{
        fields::{network::Network, field_2d::Field2D, field::Field},
        schedule::Schedule,
        state::State, location::Real2D,
    },
    rand::{self, Rng},
};

use crate::model::node::{NetNode, NodeStatus};

pub struct NetworkState {
    step: u64,
    num_agents: u32,
    pub network: Network<NetNode, String>,
    pub field: Field2D<NetNode>
}

impl NetworkState {
    pub fn new(dim: (f32, f32), num_agents: u32) -> NetworkState {
        NetworkState {
            step: 0,
            num_agents,
            network: Network::new(false),
            field: Field2D::new(dim.0, dim.1, 0.1, false)
        }
    }
}

impl State for NetworkState {
    fn update(&mut self, step: u64) {
        self.network.update();
        self.field.lazy_update();
        self.step = step;
    }

    fn reset(&mut self) {
        self.step = 0;
        self.network = Network::new(false);
        self.field = Field2D::new(self.field.width, self.field.height, 0.1, false);
    }

    fn init(&mut self, schedule: &mut Schedule) {
        let mut node_set = Vec::new();
        let mut rng = rand::thread_rng();
        self.reset();
        for node_id in 0..self.num_agents {
            let r1: f32 = rng.gen();
            let r2: f32 = rng.gen();

            let init_status: NodeStatus = if rng.gen_bool(0.1) || node_id == 0 {
                NodeStatus::Infected
            } else {
                NodeStatus::Susceptible
            };

            let node = NetNode::new(
                node_id,
                Real2D {
                    x: self.field.width * r1,
                    y: self.field.height * r2,
                },
                init_status,
            );
            self.field.set_object_location(node, node.loc);
            self.network.add_node(node);
            schedule.schedule_repeating(Box::new(node), 0.0, 0);
            node_set.push(node);
        }
        self.network
            .preferential_attachment_BA(&node_set, 2);
    }


    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_state_mut(&mut self) -> &mut dyn State {
        self
    }

    fn as_state(&self) -> &dyn State {
        self
    }
    fn after_step(&mut self, _schedule: &mut Schedule) {
        self.step += 1
    }
}
