use depends::derives::Value;

// ANCHOR: default_hashing
// This type implements `Hash`, therefore it can use the default behaviour.
#[derive(Value, Hash)]
struct DefaultBehaviour {
    data: i32,
}
// ANCHOR_END: default_hashing

// ANCHOR: custom_hashing
// This node manually manages its hash value.
#[derive(Value)]
struct CustomHashStruct {
    // You could increment a counter here, for example.
    #[depends(hash)]
    hash_value: usize,
    // ... other fields go here.
}
// ANCHOR_END: custom_hashing

// ANCHOR: no_hashing
// This node will _always_ be considered dirty to its dependents.
#[derive(Value)]
#[depends(unhashable)]
struct UnhashableStruct {
    // ... your fields go here.
}
// ANCHOR_END: no_hashing
