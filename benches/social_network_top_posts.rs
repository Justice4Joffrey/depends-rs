use std::{collections::HashSet, path::Path, rc::Rc};

use benches::{read_csv_file, read_csv_update, Phase, SocialNetworkConfig};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use depends::core::{Dependency, DerivedNode, InputNode, Resolve, SingleDep};
use envconfig::Envconfig;
use examples::{
    models::{Comment, ExpectedResult, Like, Post, Update},
    Comments, CommentsPostsLikes, CommentsPostsLikesDep, CommentsToPosts, Likes, PostScoresQuery,
    Posts, TrackCommentPostIds, UpdatePostScoresQuery,
};

/// A graph of the social network query 1.
///
/// ``` text
/// digraph G {
///   0[label="Comments"];
///   1[label="Posts"];
///   2[label="Likes"];
///   3[label="CommentsToPosts"];
///   0 -> 3[label="TrackCommentPostIds"];
///   4[label="PostScoresQuery"];
///   0 -> 4[label="UpdatePostScoresQuery"];
///   1 -> 4[label="UpdatePostScoresQuery"];
///   2 -> 4[label="UpdatePostScoresQuery"];
///   3 -> 4[label="UpdatePostScoresQuery"];
/// }
/// ```
#[allow(clippy::type_complexity)]
struct Graph {
    comments: Rc<InputNode<Comments>>,
    posts: Rc<InputNode<Posts>>,
    likes: Rc<InputNode<Likes>>,
    /// We could use type erasure and generics to reduce the complexity of
    /// this type, as the only important thing is it `Resolves` to a
    /// `PostScoresQuery`, but it's useful for demonstration what this type
    /// actually looks like.
    post_scores: Rc<
        DerivedNode<
            CommentsPostsLikesDep<
                InputNode<Comments>,
                DerivedNode<SingleDep<InputNode<Comments>>, TrackCommentPostIds, CommentsToPosts>,
                InputNode<Posts>,
                InputNode<Likes>,
            >,
            UpdatePostScoresQuery,
            PostScoresQuery,
        >,
    >,
}

impl Graph {
    pub fn init_comments(&self, comments: Vec<Comment>) {
        for comment in comments {
            self.comments.update(comment).unwrap();
        }
    }

    pub fn init_posts(&self, posts: Vec<Post>) {
        for post in posts {
            self.posts.update(post).unwrap();
        }
    }

    pub fn init_likes(&self, likes: Vec<Like>) {
        for like in likes {
            self.likes.update(like).unwrap();
        }
    }

    pub fn apply_updates(&self, updates: Vec<Update>) {
        for update in updates {
            match update {
                Update::Posts(post) => self.posts.update(post).unwrap(),
                Update::Comments(comment) => self.comments.update(comment).unwrap(),
                _ => {}
            }
        }
    }
}

#[derive(Clone)]
struct InputBatch {
    comments: Option<Vec<Comment>>,
    posts: Option<Vec<Post>>,
    likes: Option<Vec<Like>>,
    updates: Vec<Vec<Update>>,
}

impl InputBatch {
    pub fn new<P: AsRef<Path>>(path: P, model: &str) -> Result<Self, csv::Error> {
        let dir = path.as_ref().join("models").join(model);
        let comments = read_csv_file(dir.join("csv-comments-initial.csv"), '|')?;
        let posts = read_csv_file(dir.join("csv-posts-initial.csv"), '|')?;
        let likes = read_csv_file(dir.join("csv-likes-initial.csv"), '|')?;
        let mut updates = Vec::new();
        for update in 1..=20 {
            updates.push(read_csv_update(
                dir.join(format!("change{:02}.csv", update)),
            )?);
        }
        // Reverse the updates so we can pop them off the end.
        updates.reverse();
        Ok(Self {
            comments: Some(comments),
            posts: Some(posts),
            likes: Some(likes),
            updates,
        })
    }

    pub fn initialise_graph(&mut self, graph: &Graph) {
        graph.init_comments(self.comments.take().unwrap());
        graph.init_posts(self.posts.take().unwrap());
        graph.init_likes(self.likes.take().unwrap());
    }

    pub fn play_update(&mut self, graph: &Graph) {
        let update = self.updates.pop().unwrap();
        graph.apply_updates(update);
    }

    pub fn initialise_to_phase(
        &mut self,
        graph: &Graph,
        visitor: &mut HashSet<usize>,
        phase: Phase,
    ) {
        self.initialise_graph(graph);
        if phase == Phase::Updates {
            graph.post_scores.resolve_root(visitor).unwrap();
        }
    }
}

fn bench_name(expected_result: &ExpectedResult, phase: Phase) -> String {
    format!(
        "{}: {} - {:?}",
        expected_result.view, expected_result.changeset, phase
    )
}

fn load_data(mut input_batch: InputBatch, phase: Phase) -> (HashSet<usize>, Graph, InputBatch) {
    let comments_node = InputNode::new(Comments::default());
    let posts_node = InputNode::new(Posts::default());
    let likes_node = InputNode::new(Likes::default());

    let comments_to_posts = DerivedNode::new(
        Dependency::new(Rc::clone(&comments_node)),
        TrackCommentPostIds,
        CommentsToPosts::default(),
    );
    let post_scores = DerivedNode::new(
        CommentsPostsLikes::init(
            Rc::clone(&comments_node),
            comments_to_posts,
            Rc::clone(&posts_node),
            Rc::clone(&likes_node),
        ),
        UpdatePostScoresQuery,
        PostScoresQuery::default(),
    );
    let graph = Graph {
        comments: comments_node,
        posts: posts_node,
        likes: likes_node,
        post_scores,
    };
    let mut init_visitor = HashSet::new();
    input_batch.initialise_to_phase(&graph, &mut init_visitor, phase);
    let visitor = HashSet::<usize>::with_capacity(4);
    (visitor, graph, input_batch)
}

fn criterion_benchmark(c: &mut Criterion) {
    let config = SocialNetworkConfig::init_from_env().unwrap();
    let exp_results = config.expected_results().unwrap();

    let mut group = c.benchmark_group("top_posts");
    for model in [1_u32, 2, 4, 8, 16, 32, 64, 128, 256, 512] {
        let expected_result = &exp_results["Q1"][&model];
        let input_batch = InputBatch::new(&config.csv_dir, model.to_string().as_str()).unwrap();

        group.bench_function(
            bench_name(expected_result.values().next().unwrap(), Phase::Initial),
            |b| {
                b.iter_batched(
                    || load_data(input_batch.clone(), Phase::Initial),
                    |(mut visitor, graph, input_batch)| {
                        let expected = &expected_result[&0];
                        {
                            let output = graph.post_scores.resolve(&mut visitor).unwrap();
                            assert_eq!(output.top_posts(), expected.metric_value);
                        }
                        // important to return the data so it isn't dropped
                        // as part of the benchmark
                        (visitor, graph, input_batch)
                    },
                    BatchSize::SmallInput,
                );
            },
        );

        group.bench_function(
            bench_name(expected_result.values().next().unwrap(), Phase::Updates),
            |b| {
                b.iter_batched(
                    || load_data(input_batch.clone(), Phase::Updates),
                    |(mut visitor, graph, mut input_batch)| {
                        for iteration in 1..=20 {
                            let expected = &expected_result[&iteration];
                            input_batch.play_update(&graph);
                            let output = graph.post_scores.resolve_root(&mut visitor).unwrap();
                            assert_eq!(output.top_posts(), expected.metric_value);
                        }
                        // important to return the data so it isn't dropped
                        // as part of the benchmark
                        (visitor, graph, input_batch)
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
