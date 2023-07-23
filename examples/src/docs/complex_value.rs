use std::collections::HashMap;

use depends::{derives::Value, Clean, UpdateInput};
use serial_test::serial;

// ANCHOR: custom_clean
struct Post {
    id: i64,
    // ... your fields go here
}

// This node allows dependencies to iterate only values which have changed
// since the graph was last resolved.
#[derive(Value, Default)]
#[depends(custom_clean)]
struct Posts {
    // Keep track of all the posts we've seen.
    all_posts: HashMap<i64, Post>,
    // Track which data has changed since the last time we resolved this
    // node. Don't worry about how we populate this for now.
    changed_post_ids: Vec<i64>,
    #[depends(hash)]
    generation: usize,
}

impl Clean for Posts {
    fn clean(&mut self) {
        // We _must_ clean up any temporary state used to track changes.
        self.changed_post_ids.clear();
    }
}

impl Posts {
    // This public method can be called by any dependency of `Posts` to
    // iterate only the new/changed values.
    pub fn iter_changed(&self) -> impl Iterator<Item = &Post> + '_ {
        self.changed_post_ids.iter().map(|key| &self.all_posts[key])
    }
}
// ANCHOR_END: custom_clean

// ANCHOR: update_input
impl UpdateInput for Posts {
    // The type of data this node receives from _outside_ the graph.
    type Update = Post;

    fn update_mut(&mut self, post: Self::Update) {
        // Add the post ID to the list of changed posts.
        self.changed_post_ids.push(post.id);

        // Add the post to the map of posts.
        self.all_posts.insert(post.id, post);

        // Increment the generation counter.
        self.generation += 1;
    }
}
// ANCHOR_END: update_input

#[serial]
#[test]
#[rustfmt::skip]
fn create_input_node() {
use depends::InputNode;
// ANCHOR: init_input_node
// We can now create an `InputNode` from our `Posts` struct.
let posts = InputNode::new(Posts::default());

// `posts` is an `Rc`, so after cloning it in to a graph, we can still
// get shared access to it.
posts.update(Post { id: 42 }).unwrap();

let data = posts.data().unwrap();
let changed: Vec<_> = data.iter_changed().collect();

assert_eq!(changed.len(), 1);
assert_eq!(changed[0].id, 42);
// ANCHOR_END: init_input_node
}

// Stop clippy caring about the unused function without using `allow` in
// situ.
#[allow(unused)]
fn iter_changed() {
    let posts = Posts::default();
    let _ = posts.iter_changed().collect::<Vec<_>>();
}
