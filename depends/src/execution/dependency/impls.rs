use crate::{
    error::ResolveResult, Dependency, DependencyEdge, HashValue, IsDirty, Named, NodeRef, Resolve,
    Visitor,
};

macro_rules! generate_dependencies {
    ($count:expr, $($param:expr),*) => {
        paste::paste! {
            pub struct [<Dependencies $count>]<$([<T $param >]),*> (
                $(Dependency<[<T $param >]>,)*
            );

            impl<$([<T $param >]),*> Named for [<Dependencies $count>]<$([<T $param >]),*> {
                fn name() -> &'static str {
                     stringify!([<Dependencies $count>])
                }
            }

            impl<$([<T $param >]),*> [<Dependencies $count>]<$([<T $param >]),*>
            where
                $([<T $param >]: Resolve,)*
                $(for<'a> <[<T $param >] as Resolve>::Output<'a>: HashValue,)*
            {
                #[allow(clippy::too_many_arguments)]
                pub fn new($([<t $param >]: [<T $param >]),*) -> Self {
                    Self (
                        $(Dependency::new([<t $param >])),*
                    )
                }
            }

            pub struct [<DependencyReference $count>]<'a, $([<T $param >]),*> (
                $(pub DependencyEdge<'a, [<T $param >]>,)*
            );

            pub type [<DepRef $count>]<'a, $([<T $param >]),*> = [<DependencyReference $count>]<'a, $(NodeRef<'a, [<T $param >]>),*>;

            impl<$([<T $param >]),*> IsDirty for [<DependencyReference $count>]<'_, $([<T $param >]),*> {
                fn is_dirty(&self) -> bool {
                    $(self.[< $param >].is_dirty() )||*
                }
            }

            impl<$([<T $param >]),*> Resolve for [<Dependencies $count>]<$([<T $param >]),*>
            where
                $([<T $param >]: Resolve,)*
                $(for<'a> <[<T $param >] as Resolve>::Output<'a>: HashValue,)*
            {
                type Output<'a> = [<DependencyReference $count>]<'a, $([<T $param >]::Output<'a>),*>
                where
                    Self: 'a;

                fn resolve(&self, visitor: &mut impl Visitor) -> ResolveResult<Self::Output<'_>> {
                    visitor.touch_dependency_group(Self::name());
                    Ok([<DependencyReference $count>] (
                        $(self.[< $param >].resolve(visitor)?),*
                    ))
                }
            }
        }
    };
}

generate_dependencies!(2, 0, 1);
generate_dependencies!(3, 0, 1, 2);
generate_dependencies!(4, 0, 1, 2, 3);
generate_dependencies!(5, 0, 1, 2, 3, 4);
generate_dependencies!(6, 0, 1, 2, 3, 4, 5);
generate_dependencies!(7, 0, 1, 2, 3, 4, 5, 6);
generate_dependencies!(8, 0, 1, 2, 3, 4, 5, 6, 7);
generate_dependencies!(9, 0, 1, 2, 3, 4, 5, 6, 7, 8);
generate_dependencies!(10, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
generate_dependencies!(11, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
generate_dependencies!(12, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
generate_dependencies!(13, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
generate_dependencies!(14, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
generate_dependencies!(15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
generate_dependencies!(16, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
