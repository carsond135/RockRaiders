use amethyst::{
    core::{
        nalgebra::{try_convert, Isometry, Isometry3, Translation3},
        GlobalTransform,
    },
    ecs::prelude::{
        Component, DenseVecStorage, Entities, Entity, Join, Read, ReadStorage, System, World,
        Write, WriteStorage,
    },
    renderer::{Material, TextureHandle},
    shrev::EventChannel,
};

use eventhandling::MouseRay;
use ncollide3d::shape::Shape;

pub struct HoverInteractionSystem;

impl<'a> System<'a> for HoverInteractionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, MouseRay>,
        ReadStorage<'a, GlobalTransform>,
        WriteStorage<'a, HoverHandlerComponent>,
        Write<'a, Hovered>,
        Write<'a, EventChannel<HoverEvent>>,
    );

    fn run(
        &mut self,
        (entities, mouse_ray, transforms, mut hover_handlers, mut hovered, mut hover_channel): Self::SystemData,
    ) {
        let mut nearest_dist = None;
        let mut nearest_entity = None;
        for (entity, hover_handler, transform) in
            (&*entities, &mut hover_handlers, &transforms).join()
        {
            if let Some(collision_distance) = {
                let offset: Translation3<f32> = Translation3::new(
                    0.0,
                    hover_handler
                        .bounding_box()
                        .aabb(&Isometry::identity())
                        .half_extents()
                        .y,
                    0.0,
                );
                let mut translation: Isometry3<f32> = try_convert(transform.0).unwrap();
                translation.append_translation_mut(&offset);

                hover_handler
                    .bounding_box()
                    .as_ray_cast()
                    .unwrap()
                    .toi_with_ray(&translation, &mouse_ray.ray, true)
            } {
                if let Some(ref mut dist) = nearest_dist {
                    if collision_distance < *dist {
                        *dist = collision_distance;
                        nearest_entity = Some(entity);
                    }
                } else {
                    nearest_dist = Some(collision_distance);
                    nearest_entity = Some(entity);
                }
            }
        }
        if nearest_entity != **hovered {
            if let Some(e) = **hovered {
                hover_channel.single_write(HoverEvent {
                    start: false,
                    target: e,
                })
            };

            if let Some(e) = nearest_entity {
                hover_channel.single_write(HoverEvent {
                    start: true,
                    target: e,
                })
            };
        }
        **hovered = nearest_entity;
    }
}

#[derive(Clone, Default)]
pub struct Hovered(pub Option<Entity>);

impl std::ops::Deref for Hovered {
    type Target = Option<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Hovered {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub type HoverHandlerComponent = Box<dyn Hoverable>;

pub trait Hoverable: Sync + Send {
    fn on_hover_start(&mut self, _: Entity, _: &World) {}

    fn on_hover_stop(&mut self, _: Entity, _: &World) {}

    fn bounding_box(&self) -> &Box<dyn Shape<f32>>;
}

impl Component for HoverHandlerComponent {
    type Storage = DenseVecStorage<HoverHandlerComponent>;
}

#[allow(dead_code)]
pub struct NoEffectHoverHandler {
    bounding_box: Box<dyn Shape<f32>>,
}

impl NoEffectHoverHandler {
    #[allow(dead_code)]
    pub fn new<T: Shape<f32>>(bounding_box: T) -> Self {
        Self {
            bounding_box: Box::new(bounding_box) as Box<dyn Shape<f32>>,
        }
    }
}

impl Hoverable for NoEffectHoverHandler {
    fn bounding_box(&self) -> &Box<dyn Shape<f32>> {
        &self.bounding_box
    }
}

pub struct SimpleHoverHandler {
    bounding_box: Box<dyn Shape<f32>>,
    texture: TextureHandle,
}

impl SimpleHoverHandler {
    pub fn new<T: Shape<f32>>(bounding_box: T, handle: TextureHandle) -> Self {
        Self {
            bounding_box: Box::new(bounding_box) as Box<dyn Shape<f32>>,
            texture: handle,
        }
    }
}

