use crate::model::NetNode;
use crate::model::NetworkState;
use krabmaga::bevy::prelude::*;
use krabmaga::engine::fields::network::{Edge, Network};
use krabmaga::engine::location::Real2D;
use krabmaga::visualization::fields::network::{EdgeRenderInfo, NetworkRender};

impl NetworkRender<NetNode, String, NetworkState> for NetworkState {
    fn get_network(state: &NetworkState) -> &Network<NetNode, String> {
        &state.network
    }

    fn get_edge_info(edge: &Edge<String>, network: &Network<NetNode, String>) -> EdgeRenderInfo {
        EdgeRenderInfo {
            line_color: Color::BLACK,
            line_width: 1.,
            source_loc: network.get_object(edge.u).unwrap().loc,
            target_loc: network.get_object(edge.v).unwrap().loc,
            is_static: true,
        }
    }

    fn get_loc(network: &Network<NetNode, String>, node: u32) -> Real2D {
        network.get_object(node).unwrap().loc
    }
}
