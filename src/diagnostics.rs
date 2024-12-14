// use bevy::prelude::*;
// use bevy_inspector_egui::prelude::*;
// use bevy_inspector_egui::quick::ResourceInspectorPlugin;
//
// pub struct SomeDiagnosticsPlugin;
//
// impl Plugin for SomeDiagnosticsPlugin {
//    fn build(
//       &self,
//       app: &mut App,
//    ) {
//       app.init_resource::<Configuration>()
//          .register_type::<Configuration>()
//          .add_plugins(ResourceInspectorPlugin::<Configuration>::default())
//          .add_plugins(ResourceInspectorPlugin::<Time>::default());
//    }
// }
//
// #[derive(Reflect, Resource, Default, InspectorOptions)]
// #[reflect(Resource, InspectorOptions)]
// pub struct Configuration {
//    name: String,
//    #[inspector(min = 0.0, max = 1.0)]
//    option: f32,
// }
