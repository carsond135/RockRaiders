use amethyst::{
    core::specs::{storage::GenericReadStorage, Entities, Entity, Join, World},
    shred::Resource,
    ui::UiTransform,
};

pub fn rotate_3x3<T: Clone>(input: &[[T; 3]; 3]) -> [[T; 3]; 3] {
    let mut result = input.clone();

    for x in 0..3 {
        for y in 0..3 {
            result[x][y] = input[3 - y - 1][x].clone()
        }
    }
    result
}

pub fn amount_in<T: Join + GenericReadStorage>(storage: T) -> usize {
    storage.join().count()
}

pub fn find_ui_by_name<'a, T: Join<Type = &'a UiTransform>>(
    id: &str,
    entities: &Entities,
    ui_transforms: T,
) -> Option<Entity> {
    (entities, ui_transforms)
        .join()
        .find(|(_, transform)| (transform).id == id)
        .map(|(entity, _)| entity)
}

pub fn add_resource_soft<T: Resource + Default + PartialEq>(world: &mut World, res: T) {
    if !world.res.has_value::<T>() {
        if (*world.read_resource::<T>()) == T::default() {
            *world.write_resource() = res
        }
    } else {
        world.add_resource::<T>(res)
    }
}
