pub fn gen_lua_code_for_object(
    center_id: &str,
    columns: u32,
    rows: u32,
    last_frame_pos: (u32, u32),
    anim_speed: f32,
) -> String {
    let out = format!(
        r#"local {center_id}_dt = 0

local g_up = Game.update
function Game:update(dt)
    g_up(self, dt)

    {center_id}_dt = {center_id}_dt + dt

    if G.P_CENTERS and G.P_CENTERS.{center_id} and {center_id}_dt > {anim_speed} then
        {center_id}_dt = 0

        local {center_id}_obj = G.P_CENTERS.{center_id}

        if {center_id}_obj.pos.x == {} and {center_id}_obj.pos.y == {} then
            {center_id}_obj.pos.x = 0
            {center_id}_obj.pos.y = 0
        elseif {center_id}_obj.pos.x < {} then
            {center_id}_obj.pos.x = {center_id}_obj.pos.x + 1
        elseif {center_id}_obj.pos.y < {} then
            {center_id}_obj.pos.x = 0
            {center_id}_obj.pos.y = {center_id}_obj.pos.y + 1
        end
    end
end"#,
        last_frame_pos.0, last_frame_pos.1, columns, rows
    );

    out
}
