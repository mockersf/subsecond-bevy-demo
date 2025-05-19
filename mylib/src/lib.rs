use bevy::prelude::*;

#[derive(Component)]
pub struct UpdatedFromCrate;

pub fn update_text(text_query: Query<&mut Text, With<UpdatedFromCrate>>) {
    dioxus_devtools::subsecond::HotFn::current(wrapped_crate).call((text_query,));
}

fn wrapped_crate(mut text_query: Query<&mut Text, With<UpdatedFromCrate>>) {
    let mut text = text_query.single_mut().unwrap();
    **text = "from crate".to_string();
}
