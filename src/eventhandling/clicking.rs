use amethyst::ecs::{Component, DenseVecStorage, Entity, World};

pub type ClickHandlerComponent = Box<dyn Clickable>;

pub trait Clickable: Sync + Send {
    fn on_click(&self, _: Entity, _: &World);
}

impl Component for ClickHandlerComponent {
    type Storage = DenseVecStorage<ClickHandlerComponent>;
}