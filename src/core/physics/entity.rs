use struct_extension::extendable;

#[extendable]
struct PhysicsEntity {
    position: (f64, f64),
    velocity: (f64, f64)
}
