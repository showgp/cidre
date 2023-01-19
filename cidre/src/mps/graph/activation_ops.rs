use crate::{arc, mps::graph, ns};

impl graph::Graph {
    #[inline]
    pub fn relu(&self, tensor: &graph::Tensor, name: Option<&ns::String>) -> arc::R<graph::Tensor> {
        unsafe { rsel_reLUWithTensor_name(self, tensor, name) }
    }

    #[inline]
    pub fn sigmoid(
        &self,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_sigmoidWithTensor_name(self, tensor, name) }
    }

    #[inline]
    pub fn soft_max(
        &self,
        tensor: &graph::Tensor,
        axis: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor> {
        unsafe { rsel_softMaxWithTensor_axis_name(self, tensor, axis, name) }
    }
}

#[link(name = "mpsg", kind = "static")]
extern "C" {
    fn rsel_reLUWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_sigmoidWithTensor_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

    fn rsel_softMaxWithTensor_axis_name(
        graph: &graph::Graph,
        tensor: &graph::Tensor,
        axis: ns::Integer,
        name: Option<&ns::String>,
    ) -> arc::R<graph::Tensor>;

}