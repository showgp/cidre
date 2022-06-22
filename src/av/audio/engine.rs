use crate::{cf, define_obj_type, ns, os};

use super::{Format, Node, NodeBus};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(transparent)]
pub struct ManualRenderingError(pub os::Status);

impl ManualRenderingError {
    /// The operation cannot be performed because the engine is either not in manual
    ///	rendering mode or the right variant of it.
    pub const INVALID_MODE: Self = Self(os::Status(-80800));

    /// The operation cannot be performed because the engine is initialized (i.e. not stopped).
    pub const INITIALIZED: Self = Self(os::Status(-80801));

    /// The operation cannot be performed because the engine is not running (i.e. not started).
    pub const NOT_RUNNING: Self = Self(os::Status(-80801));
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingStatus {
    /// An error occurred when rendering and no data was returned. See the returned error code
    /// for the description of the error.
    Error = -1,
    /// All of the requested data was returned successfully.
    Succes = 0,
    /// Applicable only to the input node, when it provides input data for rendering
    /// (see `AVAudioInputNode(setManualRenderingInputPCMFormat:inputBlock:)`).
    /// Indicates that not enough input data was returned by the input node to satisfy the
    /// render request at the current time. The output buffer may contain data rendered by other
    /// active sources in the engine's processing graph.
    InsufficientDataFromInputNode = 1,
    /// The operation could not be performed now, but the client could retry later if needed.
    /// This is usually to guard a realtime render operation (e.g. rendering through
    /// `manualRenderingBlock`) when a reconfiguration of the engine's internal state
    /// is in progress.
    CannotDoInCurrentContext = 2,
}

/// By default, the engine is connected to an audio device and automatically renders in realtime.
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client.
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(isize)]
pub enum ManualRenderingMode {
    /// The engine operates in an offline mode without any realtime constraints.
    ManualRenderingModeOffline = 0,
    /// The engine operates under realtime constraints, i.e. it will not make any blocking call
    ///	(e.g. calling libdispatch, blocking on a mutex, allocating memory etc.) while rendering.
    /// Note that only the block based render mechanism can be used in this mode
    /// (see `AVAudioEngine(manualRenderingBlock)`.
    ManualRenderingModeRealtime = 1,
}

define_obj_type!(Engine(ns::Id));

/// An AVAudioEngine contains a group of connected AVAudioNodes ("nodes"), each of which performs
/// an audio signal generation, processing, or input/output task.
///
/// Nodes are created separately and attached to the engine.
///
/// The engine supports dynamic connection, disconnection and removal of nodes while running,
/// with only minor limitations:
/// - all dynamic reconnections must occur upstream of a mixer
/// - while removals of effects will normally result in the automatic connection of the adjacent
///   nodes, removal of a node which has differing input vs. output channel counts, or which
///   is a mixer, is likely to result in a broken graph.
///
/// By default, the engine is connected to an audio device and automatically renders in realtime. 
/// It can also be configured to operate in manual rendering mode, i.e. not connected to an
/// audio device and rendering in response to requests from the client, normally at or
/// faster than realtime rate.
impl Engine {

    /// ```
    /// use cidre::av;
    /// 
    /// let engine = av::audio::Engine::new();
    /// ```
    #[inline]
    pub fn new<'a>() -> cf::Retained<'a, Engine> {
        unsafe { AVAudioEngine_new() }
    }

    #[inline]
    pub fn attach_node(&self, node: &Node) {
        unsafe { wsel_attachNode(self, node) }
    }

    #[inline]
    pub fn detach_node(&self, node: &Node) {
        unsafe { wsel_detachNode(self, node) }
    }

    #[inline]
    pub fn connect_node_to_node_bus_to_bus(
        &self,
        node_from: &Node,
        node_to: &Node,
        from_bus: NodeBus,
        to_bus: NodeBus,
        format: Option<&Format>,
    ) {
        unsafe {
            wsel_connect_to_fromBus_toBus_format(self, node_from, node_to, from_bus, to_bus, format)
        }
    }
    #[inline]
    pub fn connect_node_to_node(
        &self,
        node_from: &Node,
        node_to: &Node,
        format: Option<&Format>,
    ) {
        unsafe {
            wsel_connect_to_format(self, node_from, node_to, format)
        }
    }

    #[inline]
    pub fn prepare(&self) {
        unsafe {
            wsel_prepare(self)
        }
    }

    #[inline]
    pub fn start<'a>(&self) -> Result<(), cf::Retained<'a, cf::Error>> {
        unsafe {
            let mut error = None;
            let res = rsel_startAndReturnError(self, &mut error);
            if res {
                Ok(())
            } else {
                Err(error.unwrap_unchecked())
            }
        }
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    fn AVAudioEngine_new<'a>() -> cf::Retained<'a, Engine>;

    fn wsel_attachNode(id: &ns::Id, node: &Node);
    fn wsel_detachNode(id: &ns::Id, node: &Node);
    fn wsel_connect_to_fromBus_toBus_format(
        id: &ns::Id,
        node_from: &Node,
        node_to: &Node,
        from_bus: NodeBus,
        to_bus: NodeBus,
        format: Option<&Format>,
    );
    fn wsel_connect_to_format(
        id: &ns::Id,
        node_from: &Node,
        node_to: &Node,
        format: Option<&Format>,
    );

    fn wsel_prepare(id: &ns::Id);

    fn rsel_startAndReturnError<'a>(id: &ns::Id, error: &mut Option<cf::Retained<'a, cf::Error>>) -> bool;
}
