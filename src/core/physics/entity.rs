use struct_extension::extendable;

#[extendable]
struct PhysicsEntity {
    position: (u32, u32),
    velocity: (u32, u32)
}