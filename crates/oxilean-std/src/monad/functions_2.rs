//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::functions::*;
use super::types::*;

/// Build the Free monad type axiom.
/// Free f a = Pure a | Wrap (f (Free f a))
fn mnd_ext_free_monad_type(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("FreeMType"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Free monad return axiom.
/// return a = Pure a
fn mnd_ext_free_monad_return(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("f"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("x"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Sort(Level::succ(Level::zero()))),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("FreeM.return"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Free monad interpreter axiom.
/// foldFree :: Monad m => (forall x. f x -> m x) -> Free f a -> m a
fn mnd_ext_free_monad_interpreter(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("FreeM.interpret"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Codensity monad axiom.
/// Codensity m a = forall r. (a -> m r) -> m r
/// This is the CPS-transformed version for improved asymptotic performance.
fn mnd_ext_codensity_type(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Codensity"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Codensity improvement theorem.
/// lowerCodensity . liftCodensity = id (for any monad m)
fn mnd_ext_codensity_improvement(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Codensity.improvement"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the effect system via monad towers axiom.
/// Represents n-layered monad transformer stacks as effect systems.
fn mnd_ext_effect_system(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("EffectStack"),
        univ_params: vec![],
        ty: Expr::Sort(Level::succ(Level::succ(Level::zero()))),
    })
    .map_err(|e| e.to_string())
}
/// Build the indexed monad type axiom.
/// IxMonad : (i -> i -> Type -> Type) -> Type
/// Tracks pre/post-conditions as type-level indices.
fn mnd_ext_indexed_monad(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("i"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(Expr::BVar(0)),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::Pi(
                        BinderInfo::Default,
                        Name::str("_"),
                        Box::new(type1.clone()),
                        Box::new(type1.clone()),
                    )),
                )),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IxMonad"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the indexed monad bind axiom.
/// ibind :: m i j a -> (a -> m j k b) -> m i k b
fn mnd_ext_indexed_monad_bind(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("i"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("IxMonad.ibind"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the parameterized monad axiom.
/// Parameterized monads generalize indexed monads with richer tracking.
fn mnd_ext_parameterized_monad(env: &mut Environment) -> Result<(), String> {
    env.add(Declaration::Axiom {
        name: Name::str("ParamMonad"),
        univ_params: vec![],
        ty: Expr::Sort(Level::succ(Level::succ(Level::zero()))),
    })
    .map_err(|e| e.to_string())
}
/// Build the Arrow type class axiom.
/// arr :: (a -> b) -> f a b
/// Arrows generalize functions and monads to profunctors.
fn mnd_ext_arrow_type(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("arr"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
        )),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Arrow"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Arrow arr axiom.
/// arr :: (b -> c) -> arr b c (lift pure function)
fn mnd_ext_arrow_arr(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("b"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("c"),
            Box::new(type1.clone()),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("f"),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("_"),
                    Box::new(Expr::BVar(1)),
                    Box::new(Expr::BVar(1)),
                )),
                Box::new(Expr::Sort(Level::succ(Level::zero()))),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Arrow.arr"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Arrow composition axiom.
/// (>>>) :: arr b c -> arr c d -> arr b d
fn mnd_ext_arrow_compose(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("b"),
        Box::new(type1.clone()),
        Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
    );
    env.add(Declaration::Axiom {
        name: Name::str("Arrow.compose"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the do-notation monad comprehension axiom.
/// Monad comprehension desugars: [x | x <- xs, p x] = do x <- xs; guard (p x); return x
fn mnd_ext_do_notation(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MonadComprehension"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the ListT transformer axiom.
/// ListT m a = m [a]  (list monad transformer)
fn mnd_ext_list_t(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ListT"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the MaybeT transformer axiom.
/// MaybeT m a = m (Option a)  (maybe monad transformer)
fn mnd_ext_maybe_t(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MaybeT"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the ExceptT transformer axiom.
/// ExceptT e m a = m (Either e a)  (exception monad transformer)
fn mnd_ext_except_t(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("e"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("a"),
                Box::new(type1.clone()),
                Box::new(Expr::Sort(Level::succ(Level::zero()))),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ExceptT"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the MaybeT lift axiom.
/// lift :: m a -> MaybeT m a = fmap Just
fn mnd_ext_maybe_t_lift(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("m"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Sort(Level::zero())),
    );
    env.add(Declaration::Axiom {
        name: Name::str("MaybeT.lift"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the ExceptT throwError axiom.
/// throwError :: e -> ExceptT e m a
fn mnd_ext_except_t_throw(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("e"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Pi(
                BinderInfo::Implicit,
                Name::str("a"),
                Box::new(type1.clone()),
                Box::new(Expr::Pi(
                    BinderInfo::Default,
                    Name::str("err"),
                    Box::new(Expr::BVar(2)),
                    Box::new(Expr::Sort(Level::succ(Level::zero()))),
                )),
            )),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ExceptT.throwError"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the ExceptT catchError axiom.
/// catchError :: ExceptT e m a -> (e -> ExceptT e m a) -> ExceptT e m a
fn mnd_ext_except_t_catch(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Implicit,
        Name::str("e"),
        Box::new(type1.clone()),
        Box::new(Expr::Pi(
            BinderInfo::Implicit,
            Name::str("m"),
            Box::new(Expr::Pi(
                BinderInfo::Default,
                Name::str("_"),
                Box::new(type1.clone()),
                Box::new(type1.clone()),
            )),
            Box::new(Expr::Sort(Level::succ(Level::succ(Level::zero())))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("ExceptT.catchError"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Build the Operational monad axiom.
/// Models computations as sequences of typed instructions.
fn mnd_ext_operational_monad(env: &mut Environment) -> Result<(), String> {
    let type1 = Expr::Sort(Level::succ(Level::zero()));
    let ty = Expr::Pi(
        BinderInfo::Default,
        Name::str("instr"),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("_"),
            Box::new(type1.clone()),
            Box::new(type1.clone()),
        )),
        Box::new(Expr::Pi(
            BinderInfo::Default,
            Name::str("a"),
            Box::new(type1.clone()),
            Box::new(Expr::Sort(Level::succ(Level::zero()))),
        )),
    );
    env.add(Declaration::Axiom {
        name: Name::str("OperationalM"),
        univ_params: vec![],
        ty,
    })
    .map_err(|e| e.to_string())
}
/// Register all extended monad axioms into the environment.
pub fn register_monad_extended_axioms(env: &mut Environment) {
    let builders: &[fn(&mut Environment) -> Result<(), String>] = &[
        mnd_ext_monad_monoid_in_endofunctors,
        mnd_ext_kleisli_identity,
        mnd_ext_kleisli_composition,
        mnd_ext_kleisli_left_identity_law,
        mnd_ext_kleisli_right_identity_law,
        mnd_ext_kleisli_assoc_law,
        mnd_ext_join_left_unit,
        mnd_ext_join_right_unit,
        mnd_ext_join_assoc,
        mnd_ext_join_naturality,
        mnd_ext_t_algebra,
        mnd_ext_eilenberg_moore_unit,
        mnd_ext_eilenberg_moore_mult,
        mnd_ext_adjunction_monad,
        mnd_ext_adjunction_unit,
        mnd_ext_adjunction_counit,
        mnd_ext_comonad,
        mnd_ext_comonad_extract,
        mnd_ext_comonad_duplicate,
        mnd_ext_comonad_left_id,
        mnd_ext_monad_trans_lift,
        mnd_ext_monad_trans_lift_pure,
        mnd_ext_monad_trans_lift_bind,
        mnd_ext_state_get,
        mnd_ext_state_put,
        mnd_ext_state_modify,
        mnd_ext_state_get_put_law,
        mnd_ext_state_put_get_law,
        mnd_ext_writer_tell,
        mnd_ext_writer_listen,
        mnd_ext_writer_pass,
        mnd_ext_writer_tell_unit,
        mnd_ext_reader_ask,
        mnd_ext_reader_local,
        mnd_ext_reader_ask_ask_law,
        mnd_ext_cont_callcc,
        mnd_ext_cont_callcc_abort,
        mnd_ext_io_free_monad,
        mnd_ext_free_monad_type,
        mnd_ext_free_monad_return,
        mnd_ext_free_monad_interpreter,
        mnd_ext_codensity_type,
        mnd_ext_codensity_improvement,
        mnd_ext_effect_system,
        mnd_ext_indexed_monad,
        mnd_ext_indexed_monad_bind,
        mnd_ext_parameterized_monad,
        mnd_ext_arrow_type,
        mnd_ext_arrow_arr,
        mnd_ext_arrow_compose,
        mnd_ext_do_notation,
        mnd_ext_list_t,
        mnd_ext_maybe_t,
        mnd_ext_except_t,
        mnd_ext_maybe_t_lift,
        mnd_ext_except_t_throw,
        mnd_ext_except_t_catch,
        mnd_ext_operational_monad,
    ];
    for builder in builders {
        let _ = builder(env);
    }
}
/// Kleisli arrow composition for Identity monad.
/// Demonstrates f >=> g = \a -> f(a).bind(g)
pub fn kleisli_compose_identity<A, B, C>(
    f: impl FnOnce(A) -> Identity<B>,
    g: impl FnOnce(B) -> Identity<C>,
) -> impl FnOnce(A) -> Identity<C> {
    move |a| f(a).bind(g)
}
/// Join/flatten for the Identity monad.
/// join :: Identity(Identity(a)) -> Identity(a)
pub fn join_identity<A>(m: Identity<Identity<A>>) -> Identity<A> {
    m.value
}
/// Demonstrates the left-identity monad law for Identity:
/// pure(a).bind(f) == f(a)
pub fn identity_left_law<A: Clone + PartialEq, B: PartialEq>(
    a: A,
    f: impl Fn(A) -> Identity<B>,
) -> bool {
    Identity::pure(a.clone()).bind(&f).value == f(a).value
}
/// Demonstrates the right-identity monad law for Identity:
/// m.bind(pure) == m
pub fn identity_right_law<A: Clone + PartialEq>(m: Identity<A>) -> bool {
    let val = m.value.clone();
    m.bind(Identity::pure).value == val
}
/// A codensity transformation: convert Free monad to Codensity for speed.
/// liftCodensity :: Free f a -> Codensity (Free f) a
/// This is the O(n) -> O(1) left-appending trick.
pub fn lift_to_codensity<A: 'static>(free: FreeM<A>) -> ContM<FreeM<A>, A> {
    ContM {
        run_cont: Box::new(move |k: Box<dyn FnOnce(A) -> FreeM<A>>| match free {
            FreeM::Pure(a) => k(a),
            FreeM::Free(inner) => match *inner {
                FreeM::Pure(a) => k(a),
                other => FreeM::Free(Box::new(other)),
            },
        }),
    }
}
/// Run a ContM (continuation) computation with given continuation.
pub fn run_cont<R, A>(c: ContM<R, A>, k: impl FnOnce(A) -> R + 'static) -> R {
    (c.run_cont)(Box::new(k))
}
/// Build a simple IxState computation that reads and transforms index.
pub fn ix_state_run<I, J, A>(s: IxState<I, J, A>, i: I) -> (A, J) {
    (s.run_ix_state)(i)
}
/// Arrow lifting: wrap a pure function as an ArrowF.
pub fn arrow_arr<A: 'static, B: 'static>(f: impl FnOnce(A) -> B + 'static) -> ArrowF<A, B> {
    ArrowF {
        run_arrow: Box::new(f),
    }
}
/// Arrow application: apply the arrow to an input.
pub fn arrow_run<A, B>(arrow: ArrowF<A, B>, a: A) -> B {
    (arrow.run_arrow)(a)
}
/// Monad transformer: lift a Maybe into a Writer<Vec<String>, Option<A>>.
pub fn lift_maybe_to_writer<A>(m: Maybe<A>) -> Writer<Vec<String>, Option<A>> {
    Writer::new(m.into_option(), vec![])
}
/// ListT bind simulation for Vec-based list monad transformer.
pub fn list_t_bind<A, B>(ma: Vec<Maybe<A>>, f: impl Fn(A) -> Vec<Maybe<B>>) -> Vec<Maybe<B>> {
    let mut result = Vec::new();
    for item in ma {
        match item.into_option() {
            Some(a) => result.extend(f(a)),
            None => result.push(Maybe::nothing()),
        }
    }
    result
}
/// ExceptT simulation: run a fallible computation, catching errors.
pub fn except_t_run<E, A>(
    computation: Either<E, A>,
    handler: impl FnOnce(E) -> Either<E, A>,
) -> Either<E, A> {
    match computation.is_left() {
        true => match computation.into_result() {
            Ok(a) => Either::right(a),
            Err(e) => handler(e),
        },
        false => computation,
    }
}
/// Demonstrates monad comprehension: filter and transform with monadic bind.
/// [f(x) | x <- xs, pred(x)] in monad M
pub fn monad_comprehension<A: Clone, B>(
    xs: Vec<A>,
    pred: impl Fn(&A) -> bool,
    f: impl Fn(A) -> Maybe<B>,
) -> Maybe<Vec<B>> {
    let filtered: Vec<A> = xs.into_iter().filter(|x| pred(x)).collect();
    map_maybe(filtered, f)
}
/// Operational monad interpretation: run a sequence of state operations.
pub fn interpret_state_ops<S: Clone>(
    ops: Vec<Box<dyn FnOnce(S) -> (Option<String>, S)>>,
    initial: S,
) -> (Vec<String>, S) {
    let mut state = initial;
    let mut outputs = Vec::new();
    for op in ops {
        let (out, new_state) = op(state);
        if let Some(msg) = out {
            outputs.push(msg);
        }
        state = new_state;
    }
    (outputs, state)
}
#[cfg(test)]
mod extended_monad_tests {
    use super::*;
    #[test]
    fn test_register_monad_extended_axioms() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Comonad")).is_some());
        assert!(env.get(&Name::str("MonadTrans")).is_some());
        assert!(env.get(&Name::str("FreeMType")).is_some());
        assert!(env.get(&Name::str("Codensity")).is_some());
        assert!(env.get(&Name::str("IxMonad")).is_some());
        assert!(env.get(&Name::str("Arrow")).is_some());
        assert!(env.get(&Name::str("ListT")).is_some());
        assert!(env.get(&Name::str("MaybeT")).is_some());
        assert!(env.get(&Name::str("ExceptT")).is_some());
        assert!(env.get(&Name::str("ContM")).is_some());
        assert!(env.get(&Name::str("IO")).is_some());
    }
    #[test]
    fn test_kleisli_compose_identity_monad() {
        let f = |n: i32| Identity::pure(n + 1);
        let g = |n: i32| Identity::pure(n * 2);
        let fg = kleisli_compose_identity(f, g);
        assert_eq!(fg(5).value, 12);
    }
    #[test]
    fn test_join_identity() {
        let nested = Identity::pure(Identity::pure(42));
        let flat = join_identity(nested);
        assert_eq!(flat.value, 42);
    }
    #[test]
    fn test_identity_left_law() {
        let f = |n: i32| Identity::pure(n * 3);
        assert!(identity_left_law(5, f));
    }
    #[test]
    fn test_identity_right_law() {
        let m = Identity::pure(7);
        assert!(identity_right_law(m));
    }
    #[test]
    fn test_arrow_arr_run() {
        let a = arrow_arr(|x: i32| x * x);
        assert_eq!(arrow_run(a, 4), 16);
    }
    #[test]
    fn test_lift_maybe_to_writer() {
        let m = Maybe::just(42);
        let w = lift_maybe_to_writer(m);
        assert_eq!(w.value, Some(42));
        assert!(w.log.is_empty());
        let n: Maybe<i32> = Maybe::nothing();
        let w2 = lift_maybe_to_writer(n);
        assert_eq!(w2.value, None);
    }
    #[test]
    fn test_list_t_bind() {
        let ma = vec![Maybe::just(1), Maybe::just(2), Maybe::nothing()];
        let f = |n: i32| vec![Maybe::just(n * 10), Maybe::just(n * 100)];
        let result = list_t_bind(ma, f);
        assert_eq!(result.len(), 5);
    }
    #[test]
    fn test_except_t_run_success() {
        let comp: Either<String, i32> = Either::right(42);
        let handler = |_: String| Either::right(0);
        let result = except_t_run(comp, handler);
        assert_eq!(
            result.into_result().expect("into_result should succeed"),
            42
        );
    }
    #[test]
    fn test_except_t_run_error() {
        let comp: Either<String, i32> = Either::left("oops".to_string());
        let handler = |_: String| Either::right(99);
        let result = except_t_run(comp, handler);
        assert_eq!(
            result.into_result().expect("into_result should succeed"),
            99
        );
    }
    #[test]
    fn test_monad_comprehension() {
        let xs = vec![1i32, 2, 3, 4, 5, 6];
        let result = monad_comprehension(xs, |x| *x % 2 == 0, |x| Maybe::just(x * x));
        assert_eq!(result.into_option(), Some(vec![4, 16, 36]));
    }
    #[test]
    fn test_free_monad_pure() {
        let free: FreeM<i32> = FreeM::Pure(42);
        match free {
            FreeM::Pure(v) => assert_eq!(v, 42),
            FreeM::Free(_) => panic!("Expected Pure"),
        }
    }
    #[test]
    fn test_free_monad_wrap() {
        let inner: FreeM<i32> = FreeM::Pure(1);
        let wrapped = FreeM::Free(Box::new(inner));
        match wrapped {
            FreeM::Free(_) => {}
            FreeM::Pure(_) => panic!("Expected Free"),
        }
    }
    #[test]
    fn test_interpret_state_ops() {
        let ops: Vec<Box<dyn FnOnce(i32) -> (Option<String>, i32)>> = vec![
            Box::new(|s: i32| (Some(format!("state={}", s)), s + 1)),
            Box::new(|s: i32| (None, s * 2)),
            Box::new(|s: i32| (Some(format!("final={}", s)), s)),
        ];
        let (outputs, final_state) = interpret_state_ops(ops, 5);
        assert_eq!(final_state, 12);
        assert_eq!(outputs, vec!["state=5", "final=12"]);
    }
    #[test]
    fn test_ix_state_run() {
        let s = IxState {
            run_ix_state: Box::new(|i: i32| (i.to_string(), i + 1)),
        };
        let (a, j) = ix_state_run(s, 10);
        assert_eq!(a, "10");
        assert_eq!(j, 11);
    }
    #[test]
    fn test_kleisli_left_identity() {
        let f = |n: i32| Maybe::just(n * 2);
        let a = 5;
        let lhs = Maybe::just(a).bind(f);
        let rhs = f(a);
        assert_eq!(lhs.into_option(), rhs.into_option());
    }
    #[test]
    fn test_kleisli_right_identity() {
        let m = Maybe::just(42);
        let result = m.bind(Maybe::just);
        assert_eq!(result.into_option(), Some(42));
    }
    #[test]
    fn test_kleisli_associativity() {
        let m = Maybe::just(2i32);
        let f = |n: i32| Maybe::just(n + 3);
        let g = |n: i32| Maybe::just(n * 4);
        let lhs = m.clone().bind(f).bind(g);
        let rhs = m.bind(|x| Maybe::just(x + 3).bind(|n| Maybe::just(n * 4)));
        assert_eq!(lhs.into_option(), rhs.into_option());
    }
    #[test]
    fn test_comonad_axiom_names_registered() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Comonad.extract")).is_some());
        assert!(env.get(&Name::str("Comonad.duplicate")).is_some());
        assert!(env.get(&Name::str("Comonad.left_id")).is_some());
    }
    #[test]
    fn test_eilenberg_moore_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("TAlgebra")).is_some());
        assert!(env.get(&Name::str("EilenbergMoore.unit_law")).is_some());
        assert!(env.get(&Name::str("EilenbergMoore.mult_law")).is_some());
    }
    #[test]
    fn test_join_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Join.left_unit")).is_some());
        assert!(env.get(&Name::str("Join.right_unit")).is_some());
        assert!(env.get(&Name::str("Join.assoc")).is_some());
        assert!(env.get(&Name::str("Join.naturality")).is_some());
    }
    #[test]
    fn test_writer_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("WriterTell")).is_some());
        assert!(env.get(&Name::str("WriterListen")).is_some());
        assert!(env.get(&Name::str("WriterPass")).is_some());
    }
    #[test]
    fn test_state_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("StateGet")).is_some());
        assert!(env.get(&Name::str("StatePut")).is_some());
        assert!(env.get(&Name::str("StateModify")).is_some());
        assert!(env.get(&Name::str("State.get_put_law")).is_some());
        assert!(env.get(&Name::str("State.put_get_law")).is_some());
    }
    #[test]
    fn test_arrow_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("Arrow")).is_some());
        assert!(env.get(&Name::str("Arrow.arr")).is_some());
        assert!(env.get(&Name::str("Arrow.compose")).is_some());
    }
    #[test]
    fn test_cont_callcc_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("ContM")).is_some());
        assert!(env.get(&Name::str("callCC")).is_some());
        assert!(env.get(&Name::str("callCC.abort_law")).is_some());
    }
    #[test]
    fn test_transformer_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("MonadTrans.lift_pure")).is_some());
        assert!(env.get(&Name::str("MonadTrans.lift_bind")).is_some());
        assert!(env.get(&Name::str("MaybeT.lift")).is_some());
        assert!(env.get(&Name::str("ExceptT.throwError")).is_some());
        assert!(env.get(&Name::str("ExceptT.catchError")).is_some());
    }
    #[test]
    fn test_operational_monad_registered() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("OperationalM")).is_some());
        assert!(env.get(&Name::str("EffectStack")).is_some());
    }
    #[test]
    fn test_kleisli_category_axiom_names() {
        let mut env = Environment::new();
        build_monad_env(&mut env).expect("build_monad_env should succeed");
        register_monad_extended_axioms(&mut env);
        assert!(env.get(&Name::str("KleisliId")).is_some());
        assert!(env.get(&Name::str("KleisliComp")).is_some());
        assert!(env.get(&Name::str("Kleisli.left_id")).is_some());
        assert!(env.get(&Name::str("Kleisli.right_id")).is_some());
        assert!(env.get(&Name::str("Kleisli.assoc")).is_some());
    }
}
