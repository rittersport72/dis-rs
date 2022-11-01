use crate::common::{BodyInfo, Interaction};
use crate::{DescriptorRecord, DetonationResult, EntityId, EntityType, EventId, ExplosiveMaterialCategories, Location, MunitionDescriptor, PduType, VariableParameter, VectorF32};
use crate::constants::VARIABLE_PARAMETER_RECORD_LENGTH;

const BASE_DETONATION_BODY_LENGTH : u16 = 104;

pub struct Detonation {
    pub source_entity_id: EntityId,
    pub target_entity_id: EntityId,
    pub exploding_entity_id: EntityId,
    pub event_id: EventId,
    pub velocity: VectorF32,
    pub location_in_world_coordinates: Location,
    pub descriptor: DescriptorRecord,
    pub location_in_entity_coordinates: VectorF32,
    pub detonation_result: DetonationResult,
    pub variable_parameters: Vec<VariableParameter>,
}

impl Detonation {
    // TODO `new` and `with_` methods

    pub fn with_descriptor(mut self, descriptor: DescriptorRecord) -> Self {
        self.descriptor = descriptor;
        self
    }

    pub fn with_munition_descriptor(mut self, entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        self.descriptor = DescriptorRecord::new_munition(entity_type, munition);
        self
    }

    pub fn with_expendable_descriptor(mut self, entity_type: EntityType) -> Self {
        self.descriptor = DescriptorRecord::Expendable { entity_type };
        self
    }

    pub fn with_explosion_descriptor(mut self,
                                     entity_type: EntityType,
                                     explosive_material: ExplosiveMaterialCategories,
                                     explosive_force: f32) -> Self {
        self.descriptor = DescriptorRecord::new_explosion(entity_type, explosive_material, explosive_force);
        self
    }
}

impl BodyInfo for Detonation {
    fn body_length(&self) -> u16 {
        BASE_DETONATION_BODY_LENGTH + (VARIABLE_PARAMETER_RECORD_LENGTH * (*&self.variable_parameters.len() as u16))
    }

    fn body_type(&self) -> PduType {
        PduType::Detonation
    }
}

impl Interaction for Detonation {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.source_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.target_entity_id)
    }
}