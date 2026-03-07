use super::traits::*;

pub struct NoOpBridges;

impl MemoryBridge for NoOpBridges {}
impl VisionBridge for NoOpBridges {}
impl IdentityBridge for NoOpBridges {}
impl TimeBridge for NoOpBridges {}
impl ContractBridge for NoOpBridges {}
impl CommBridge for NoOpBridges {}
impl CodebaseBridge for NoOpBridges {}
impl PlanningBridge for NoOpBridges {}
impl CognitionBridge for NoOpBridges {}
impl RealityBridge for NoOpBridges {}
impl HydraAdapter for NoOpBridges {}
