use raylib::math::Vector2;

pub fn vector2_aim_to_vector2(x: Vector2, y: Vector2) -> Vector2 {
    // get the angle vector between two points
    let mut v = Vector2::new(x.x - y.x, x.y - y.y);
    let len = (v.x * v.x + v.y * v.y).sqrt();
    if len != 0.0 {
        v.x /= len;
        v.y /= len;
    }
    v
}

pub fn vector2_to_angle(v: Vector2) -> f32 {
    Vector2::new(0.0,0.0).angle_to(v) * 180.0 / std::f32::consts::PI
}