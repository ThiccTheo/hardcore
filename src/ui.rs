use {super::game_state::GameState, bevy::prelude::*};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(OnEnter(GameState::Playing), spawn_hud);
    }
}

// fn spawn_hud(mut cmds: Commands) {
// 	cmds.spawn(NodeBundle {
// 	    node: Node {
//         stack_index: todo!(),
//         calculated_size: todo!(),
//         outline_width: todo!(),
//         outline_offset: todo!(),
//         unrounded_size: todo!(),
//     },
// 	    style: todo!(),
// 	    background_color: todo!(),
// 	    border_color: todo!(),
// 	    focus_policy: todo!(),
// 	    transform: todo!(),
// 	    global_transform: todo!(),
// 	    visibility: todo!(),
// 	    inherited_visibility: todo!(),
// 	    view_visibility: todo!(),
// 	    z_index: todo!(),
// 	});
// }
