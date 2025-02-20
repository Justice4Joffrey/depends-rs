use std::{cell::Ref, path::Path};

use benches::{read_csv_file, read_csv_update, Phase, SocialNetworkConfig};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use depends::{
    derives::Graph, error::ResolveResult, Dependencies4, GraphCreate, NodeState, Resolve, Visitor,
};
use envconfig::Envconfig;
use examples::{models::*, *};
use hashbrown::HashSet;

#[derive(Graph)]
#[depends(
    digraph Dag {
        comments [label="Comments"];
        posts [label="Posts"];
        likes [label="Likes"];
        comment_to_posts [label="CommentsToPosts"];
        comments -> comment_to_posts [label="TrackCommentPostIds"];
        query [label="PostScoresQuery"];
        comments -> query [label="UpdatePostScoresQuery", class="Dependencies4"];
        comment_to_posts -> query [label="UpdatePostScoresQuery", class="Dependencies4"];
        posts -> query [label="UpdatePostScoresQuery", class="Dependencies4"];
        likes -> query [label="UpdatePostScoresQuery", class="Dependencies4"];
    }
)]
struct Foo {}

struct GraphOuter<G>(<Foo as GraphCreate>::Graph<G>);

impl<R> GraphOuter<R>
where
    for<'a> R: Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>> + 'a,
{
    pub fn init_comments(&self, comments: Vec<Comment>) {
        for comment in comments {
            self.0.update_comments(comment).unwrap();
        }
    }

    pub fn init_posts(&self, posts: Vec<Post>) {
        for post in posts {
            self.0.update_posts(post).unwrap();
        }
    }

    pub fn init_likes(&self, likes: Vec<Like>) {
        for like in likes {
            self.0.update_likes(like).unwrap();
        }
    }

    pub fn apply_updates(&self, updates: Vec<Update>) {
        for update in updates {
            match update {
                Update::Posts(post) => self.0.update_posts(post).unwrap(),
                Update::Comments(comment) => self.0.update_comments(comment).unwrap(),
                _ => {}
            }
        }
    }
}

impl<R> Resolve for GraphOuter<R>
where
    for<'a> R: Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>> + 'a,
{
    type Output<'a>
        = <R as Resolve>::Output<'a>
    where
        Self: 'a;

    fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
        self.0.resolve(visitor)
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

    pub fn initialise_graph<R>(&mut self, graph: &GraphOuter<R>)
    where
        for<'a> R: Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>> + 'a,
    {
        graph.init_comments(self.comments.take().unwrap());
        graph.init_posts(self.posts.take().unwrap());
        graph.init_likes(self.likes.take().unwrap());
    }

    pub fn play_update<R>(&mut self, graph: &GraphOuter<R>)
    where
        for<'a> R: Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>> + 'a,
    {
        let update = self.updates.pop().unwrap();
        graph.apply_updates(update);
    }

    pub fn initialise_to_phase<R>(
        &mut self,
        graph: &GraphOuter<R>,
        visitor: &mut HashSet<usize>,
        phase: Phase,
    ) where
        for<'a> R: Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>> + 'a,
    {
        self.initialise_graph(graph);
        if phase == Phase::Updates {
            graph.resolve_root(visitor).unwrap();
        }
    }
}

fn bench_name(expected_result: &ExpectedResult, phase: Phase) -> String {
    format!(
        "{}: {} - {:?}",
        expected_result.view, expected_result.changeset, phase
    )
}

fn load_data(
    mut input_batch: InputBatch,
    phase: Phase,
) -> (
    HashSet<usize>,
    GraphOuter<impl for<'a> Resolve<Output<'a> = Ref<'a, NodeState<PostScoresQuery>>>>,
    InputBatch,
) {
    let graph = GraphOuter(Foo::create_dag(
        Comments::new(),
        Likes::new(),
        Posts::new(),
        CommentsToPosts::new(),
        PostScoresQuery::new(),
    ));
    let mut init_visitor = HashSet::new();
    input_batch.initialise_to_phase(&graph, &mut init_visitor, phase);
    let visitor = HashSet::<usize>::with_capacity(5);
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
                            let output = graph.resolve(&mut visitor).unwrap();
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
                            let output = graph.resolve_root(&mut visitor).unwrap();
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

fn configure_criterion() -> Criterion {
    Criterion::default().configure_from_args().sample_size(500)
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets = criterion_benchmark
);
criterion_main!(benches);
