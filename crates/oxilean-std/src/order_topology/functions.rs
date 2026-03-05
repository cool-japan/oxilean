//! Auto-generated module
//!
//! 🤖 Generated with [SplitRS](https://github.com/cool-japan/splitrs)

use oxilean_kernel::{BinderInfo, Declaration, Environment, Expr, Level, Name};

use super::types::{
    DirectedSet, LimSupInf, MacNeilleCompletion, MonotoneFnChecker, Net, OrderTopologyBasis,
    OrderedInterval, OrderedMetricSpace, OrdinalArithmetic, RealInterval, ScottOpenSet,
    SmallOrdinal, SorgenfreyLineTopology, TychonoffSpaceData, VectorLatticeOps,
};

pub fn app(f: Expr, a: Expr) -> Expr {
    Expr::App(Box::new(f), Box::new(a))
}
pub fn app2(f: Expr, a: Expr, b: Expr) -> Expr {
    app(app(f, a), b)
}
pub fn app3(f: Expr, a: Expr, b: Expr, c: Expr) -> Expr {
    app(app2(f, a, b), c)
}
pub fn cst(s: &str) -> Expr {
    Expr::Const(Name::str(s), vec![])
}
pub fn prop() -> Expr {
    Expr::Sort(Level::zero())
}
pub fn type0() -> Expr {
    Expr::Sort(Level::succ(Level::zero()))
}
pub fn pi(bi: BinderInfo, name: &str, dom: Expr, body: Expr) -> Expr {
    Expr::Pi(bi, Name::str(name), Box::new(dom), Box::new(body))
}
pub fn arrow(a: Expr, b: Expr) -> Expr {
    pi(BinderInfo::Default, "_", a, b)
}
pub fn impl_pi(name: &str, dom: Expr, body: Expr) -> Expr {
    pi(BinderInfo::Implicit, name, dom, body)
}
pub fn bvar(n: u32) -> Expr {
    Expr::BVar(n)
}
/// `OrderTopology α : TopologicalSpace α`
///
/// The topology on a linearly ordered type α induced by the order, generated
/// by the open rays (a, ∞) and (-∞, b).
pub fn order_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// `Nhds : α → Filter α`
///
/// The neighbourhood filter of a point x: the set of all sets containing
/// an open neighbourhood of x.
pub fn nhds_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), app(cst("Filter"), bvar(1))))
}
/// `Filter.atTop : Filter α`
///
/// The filter of sets that contain a tail `{x | a ≤ x}` for some `a`.
pub fn filter_at_top_ty() -> Expr {
    impl_pi("α", type0(), app(cst("Filter"), bvar(0)))
}
/// `Filter.atBot : Filter α`
///
/// The filter of sets that contain a tail `{x | x ≤ a}` for some `a`.
pub fn filter_at_bot_ty() -> Expr {
    impl_pi("α", type0(), app(cst("Filter"), bvar(0)))
}
/// `Tendsto : (α → β) → Filter α → Filter β → Prop`
///
/// `Tendsto f l m` means f tends to filter m along filter l, i.e.,
/// for every set s ∈ m, the preimage f⁻¹(s) ∈ l.
pub fn tendsto_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(
                    app(cst("Filter"), bvar(2)),
                    arrow(app(cst("Filter"), bvar(2)), prop()),
                ),
            ),
        ),
    )
}
/// `ContinuousAt : (α → β) → α → Prop`
///
/// `ContinuousAt f x` means f is continuous at the point x,
/// i.e., `Tendsto f (nhds x) (nhds (f x))`.
pub fn continuous_at_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(arrow(bvar(1), bvar(1)), arrow(bvar(2), prop())),
        ),
    )
}
/// `ContinuousOn : (α → β) → Set α → Prop`
///
/// `ContinuousOn f s` means f is continuous at every point of s (within s).
pub fn continuous_on_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(app(cst("Set"), bvar(2)), prop()),
            ),
        ),
    )
}
/// `IsOpen_Ioi : ∀ (α : Type) [LinearOrder α] [OrderTopology α] (a : α), IsOpen (Ioi a)`
///
/// The open ray (a, ∞) is open in the order topology.
pub fn is_open_ioi_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), prop()))
}
/// `IsOpen_Iio : ∀ (α : Type) [LinearOrder α] [OrderTopology α] (b : α), IsOpen (Iio b)`
///
/// The open ray (-∞, b) is open in the order topology.
pub fn is_open_iio_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), prop()))
}
/// `OrderTopology.instTopologicalSpace : TopologicalSpace α`
///
/// The canonical `TopologicalSpace` instance produced by `OrderTopology`.
pub fn order_topology_inst_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// `nhds_order_left : ∀ (α : Type) (a : α), nhds a = Filter.atBot ⊓ principal (Iic a)`
///
/// The left-sided neighbourhood filter at a (within the order topology).
pub fn nhds_order_left_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), prop()))
}
/// `nhds_order_right : ∀ (α : Type) (a : α), nhds a = Filter.atTop ⊓ principal (Ici a)`
///
/// The right-sided neighbourhood filter at a (within the order topology).
pub fn nhds_order_right_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), prop()))
}
/// `Monotone : (α → β) → Prop`
///
/// A function f : α → β is monotone if a ≤ b implies f a ≤ f b.
pub fn monotone_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// `Antitone : (α → β) → Prop`
///
/// A function f : α → β is antitone if a ≤ b implies f b ≤ f a.
pub fn antitone_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// `StrictMono : (α → β) → Prop`
///
/// A function f is strictly monotone: a < b implies f a < f b.
pub fn strict_mono_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// `LimSup : (Nat → α) → Filter Nat → α`
///
/// The limit superior of a sequence along a filter.
pub fn limsup_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            arrow(cst("Nat"), bvar(0)),
            arrow(app(cst("Filter"), cst("Nat")), bvar(1)),
        ),
    )
}
/// `LimInf : (Nat → α) → Filter Nat → α`
///
/// The limit inferior of a sequence along a filter.
pub fn liminf_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            arrow(cst("Nat"), bvar(0)),
            arrow(app(cst("Filter"), cst("Nat")), bvar(1)),
        ),
    )
}
/// `UpperSemicontinuous : (α → β) → Prop`
///
/// f is upper semicontinuous at x if limsup_{y→x} f(y) ≤ f(x).
pub fn upper_semicontinuous_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// `LowerSemicontinuous : (α → β) → Prop`
///
/// f is lower semicontinuous at x if f(x) ≤ liminf_{y→x} f(y).
pub fn lower_semicontinuous_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// `IsCompact_Icc : ∀ (α : Type) (a b : α), IsCompact (Icc a b)`
///
/// Closed bounded intervals are compact in a complete linear order (Heine-Borel).
pub fn is_compact_icc_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `IntermediateValueThm : ∀ (f : α → β) (a b : α) (y : β), ContinuousOn f (Icc a b) → Prop`
///
/// The intermediate value theorem: a continuous function on [a,b] attains every
/// value between f(a) and f(b).
pub fn intermediate_value_thm_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(bvar(2), arrow(bvar(3), arrow(bvar(2), prop()))),
            ),
        ),
    )
}
/// `ExtremeValueThm : ∀ (f : α → β) (s : Set α), IsCompact s → ContinuousOn f s → Prop`
///
/// The extreme value theorem: a continuous function on a compact set attains
/// its maximum and minimum.
pub fn extreme_value_thm_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                arrow(bvar(1), bvar(1)),
                arrow(app(cst("Set"), bvar(2)), prop()),
            ),
        ),
    )
}
/// `DiniTheorem : ∀ (f : Nat → α → β), Monotone f → TendstoUniformly f lim → Prop`
///
/// Dini's theorem: a monotone sequence of continuous functions converging
/// pointwise to a continuous limit on a compact space converges uniformly.
pub fn dini_theorem_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(arrow(cst("Nat"), arrow(bvar(1), bvar(1))), prop()),
        ),
    )
}
/// `MonotoneConvergenceThm : ∀ (f : Nat → α), Monotone f → BoundedAbove f → Prop`
///
/// The monotone convergence theorem: a monotone bounded sequence converges.
pub fn monotone_convergence_thm_ty() -> Expr {
    impl_pi("α", type0(), arrow(arrow(cst("Nat"), bvar(0)), prop()))
}
/// `DedekindComplete : Type → Prop`
///
/// A linearly ordered set is Dedekind complete if every non-empty bounded-above
/// subset has a least upper bound.
pub fn dedekind_complete_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrderConnected : Set α → Prop`
///
/// A set S in an ordered space is order-connected if for all a,b ∈ S with a ≤ b,
/// the interval [a,b] ⊆ S.
pub fn order_connected_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("Set"), bvar(0)), prop()))
}
/// `ConnectedSpace : TopologicalSpace α → Prop`
///
/// A topological space is connected if it cannot be written as a union of two
/// disjoint non-empty open sets.
pub fn connected_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrderTopologyConnected : ∀ (α : Type) [LinearOrder α] [DedekindComplete α], Connected α`
///
/// A Dedekind-complete linear order with the order topology is connected.
pub fn order_topology_connected_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrderedTopologicalGroup : Type → Prop`
///
/// A topological group (G, ·, τ) where G also carries a linear order compatible
/// with the group operation and the topology coincides with the order topology.
pub fn ordered_topological_group_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `TopologicalLattice : Type → Prop`
///
/// A topological lattice is a topological space that is also a lattice where
/// meet (∧) and join (∨) are jointly continuous operations.
pub fn topological_lattice_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `ScottOpen : (α → Prop) → Prop`
///
/// A subset U of a dcpo is Scott-open if it is an upper set and for every
/// directed set D with sup(D) ∈ U we have D ∩ U ≠ ∅.
pub fn scott_open_ty() -> Expr {
    impl_pi("α", type0(), arrow(arrow(bvar(0), prop()), prop()))
}
/// `ScottTopology : Type → TopologicalSpace`
///
/// The Scott topology on a dcpo (directed-complete partial order):
/// the open sets are the Scott-open upper sets.
pub fn scott_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// `LawsonTopology : Type → TopologicalSpace`
///
/// The Lawson topology on a continuous lattice: generated by the Scott topology
/// together with complements of principal filters.
pub fn lawson_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// `AlexandroffTopology : (α → α → Prop) → TopologicalSpace α`
///
/// The Alexandroff topology associated to a preorder: the open sets are the
/// upper sets. The specialization preorder recovers the original preorder.
pub fn alexandroff_topology_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            arrow(bvar(0), arrow(bvar(1), prop())),
            app(cst("TopologicalSpace"), bvar(1)),
        ),
    )
}
/// `SpecializationOrder : TopologicalSpace α → (α → α → Prop)`
///
/// The specialization preorder of a topological space: x ≤ y iff x ∈ cl({y}).
pub fn specialization_order_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            app(cst("TopologicalSpace"), bvar(0)),
            arrow(bvar(1), arrow(bvar(2), prop())),
        ),
    )
}
/// `BirkhoffRepresentation : ∀ (L : Type), FiniteDistributiveLattice L → Prop`
///
/// Birkhoff's representation theorem: every finite distributive lattice is
/// isomorphic to the lattice of order ideals of its poset of join-irreducibles.
pub fn birkhoff_representation_ty() -> Expr {
    impl_pi("L", type0(), prop())
}
/// `PriestleyDuality : ∀ (L : Type), DistributiveLattice L → Prop`
///
/// Priestley duality: the category of bounded distributive lattices is dually
/// equivalent to the category of Priestley spaces (compact totally order-disconnected spaces).
pub fn priestley_duality_ty() -> Expr {
    impl_pi("L", type0(), prop())
}
/// `ZornLemma : ∀ (α : Type), (∀ (c : Set α), IsChain c → ∃ ub, UpperBound c ub) → ∃ m, Maximal m`
///
/// Zorn's lemma: a partially ordered set in which every chain has an upper bound
/// contains at least one maximal element.
pub fn zorn_lemma_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrdinalTopology : Type → TopologicalSpace`
///
/// The order topology on an ordinal: open sets generated by open intervals
/// and open rays in the ordinal ordering.
pub fn ordinal_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// `NetConvergence : (DirectedSet → α) → α → Prop`
///
/// A net (x_d)_{d ∈ D} in a topological space converges to a point x if
/// for every neighbourhood U of x there exists d₀ such that x_d ∈ U for all d ≥ d₀.
pub fn net_convergence_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(arrow(cst("DirectedSet"), bvar(0)), arrow(bvar(1), prop())),
    )
}
/// `OrderBasis_Ioo : ∀ (α : Type) (a b : α), IsOpen (Ioo a b)`
///
/// Open intervals (a, b) form a basis for the order topology.
pub fn order_basis_ioo_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `IsOpen_Ioo : ∀ (α : Type) (a b : α), IsOpen (Ioo a b)`
///
/// Open intervals are open in the order topology.
pub fn is_open_ioo_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `IsClosed_Icc : ∀ (α : Type) (a b : α), IsClosed (Icc a b)`
///
/// Closed intervals are closed in the order topology.
pub fn is_closed_icc_ty() -> Expr {
    impl_pi("α", type0(), arrow(bvar(0), arrow(bvar(1), prop())))
}
/// `OrderTopology.t2Space : T2Space α`
///
/// A linearly ordered space with the order topology is Hausdorff (T₂).
pub fn order_topology_t2_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrderTopology.regularSpace : RegularSpace α`
///
/// A linearly ordered space with the order topology is a regular (T₃) space.
pub fn order_topology_regular_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `OrderedFieldTopology : Type → Prop`
///
/// An ordered field carries a natural topology (the order topology) making
/// addition and multiplication continuous, i.e., it is a topological field.
pub fn ordered_field_topology_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// `Filter.limsup : Filter α → (α → β) → β`
///
/// The filter limsup: `limsup f l = Inf_{s ∈ l} Sup_{x ∈ s} f(x)`.
pub fn filter_limsup_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                app(cst("Filter"), bvar(1)),
                arrow(arrow(bvar(2), bvar(1)), bvar(1)),
            ),
        ),
    )
}
/// `Filter.liminf : Filter α → (α → β) → β`
///
/// The filter liminf: `liminf f l = Sup_{s ∈ l} Inf_{x ∈ s} f(x)`.
pub fn filter_liminf_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi(
            "β",
            type0(),
            arrow(
                app(cst("Filter"), bvar(1)),
                arrow(arrow(bvar(2), bvar(1)), bvar(1)),
            ),
        ),
    )
}
/// `OrderIsoHomeomorph : ∀ (α β : Type), OrderIso α β → Homeomorph α β`
///
/// An order isomorphism between linearly ordered spaces (with order topologies)
/// is a homeomorphism.
pub fn order_iso_homeomorph_ty() -> Expr {
    impl_pi("α", type0(), impl_pi("β", type0(), prop()))
}
/// `ContinuousMono_compose : Monotone f → ContinuousAt f x → ContinuousAt (g ∘ f) x`
///
/// Composition of a continuous function with a monotone function is continuous
/// at points where the monotone function is continuous.
pub fn continuous_mono_compose_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), impl_pi("γ", type0(), prop())),
    )
}
/// MacNeille completion: the smallest complete lattice containing a given poset.
/// Type: `Type → Type`.
pub fn macneille_completion_ty() -> Expr {
    impl_pi("α", type0(), type0())
}
/// Dedekind cuts: the classical construction of ℝ from ℚ via Dedekind cuts.
/// Type: `Prop` (axiom of completeness).
pub fn dedekind_cuts_ty() -> Expr {
    prop()
}
/// Supremum-preserving map: a function preserving all existing sups.
/// Type: `(α → β) → Prop`.
pub fn sup_preserving_map_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// Interval topology: the topology on a poset generated by open intervals.
/// Type: `Type → TopologicalSpace`.
pub fn interval_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// Order topology equals interval topology for linear orders.
/// Type: `Prop`.
pub fn order_eq_interval_topology_ty() -> Expr {
    prop()
}
/// Connected ordered space: a linearly ordered space connected under order topology.
/// Type: `Type → Prop`.
pub fn connected_ordered_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Long line: the order space ω₁ × [0,1) with the order topology (not second-countable).
/// Type: `Type`.
pub fn long_line_ty() -> Expr {
    type0()
}
/// Ordinal space ω₁: the first uncountable ordinal with its order topology.
/// Type: `Type`.
pub fn ordinal_omega1_ty() -> Expr {
    type0()
}
/// Alexandroff compactification of the ordinal ω₁ (adds a point at ∞).
/// Type: `Type`.
pub fn alexandroff_omega1_ty() -> Expr {
    type0()
}
/// Monotonically normal space: a stronger form of normality for ordered spaces.
/// Type: `Type → Prop`.
pub fn monotonically_normal_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Generalized ordered space (GO-space): a subspace of a linearly ordered space.
/// Type: `Type → Prop`.
pub fn generalized_ordered_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Linearly ordered topological space (LOTS): a linear order with its order topology.
/// Type: `Type → Prop`.
pub fn lots_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Sorgenfrey line: ℝ with the lower limit topology (half-open intervals [a,b)).
/// Type: `Type`.
pub fn sorgenfrey_line_ty() -> Expr {
    type0()
}
/// Michael line: ℝ with irrationals given the discrete topology.
/// Type: `Type`.
pub fn michael_line_ty() -> Expr {
    type0()
}
/// Ordinal multiplication: the Cartesian product with lexicographic order.
/// Type: `Type → Type → Type`.
pub fn ordinal_multiplication_ty() -> Expr {
    impl_pi("α", type0(), impl_pi("β", type0(), type0()))
}
/// Ordinal exponentiation: α^β as the set of functions β → α with reverse-lex order.
/// Type: `Type → Type → Type`.
pub fn ordinal_exponentiation_ty() -> Expr {
    impl_pi("α", type0(), impl_pi("β", type0(), type0()))
}
/// Cantor normal form: every ordinal is uniquely α = ω^{β₁}·c₁ + … + ω^{βₙ}·cₙ.
/// Type: `Prop`.
pub fn cantor_normal_form_ty() -> Expr {
    prop()
}
/// Helly's theorem (order version): intervals on a line have the Helly property.
/// Type: `Prop`.
pub fn helly_order_ty() -> Expr {
    prop()
}
/// Convex set in an ordered space: a set C such that [a,b] ⊆ C for all a,b ∈ C.
/// Type: `(α : Type) → Set α → Prop`.
pub fn convex_ordered_set_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("Set"), bvar(0)), prop()))
}
/// Compact convex set in a linearly ordered space.
/// Type: `(α : Type) → Set α → Prop`.
pub fn compact_convex_ordered_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("Set"), bvar(0)), prop()))
}
/// Monotone continuous function between ordered topological spaces.
/// Type: `(α → β) → Prop`.
pub fn monotone_continuous_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        impl_pi("β", type0(), arrow(arrow(bvar(1), bvar(1)), prop())),
    )
}
/// Dini uniform convergence: monotone pointwise convergent sequence on compact ordered
/// space converges uniformly. Type: `Prop`.
pub fn dini_uniform_convergence_ty() -> Expr {
    prop()
}
/// Cofinal subnet: a subnet that is cofinal in the directed set.
/// Type: `(DirectedSet → α) → (DirectedSet → α) → Prop`.
pub fn cofinal_subnet_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            arrow(cst("DirectedSet"), bvar(0)),
            arrow(arrow(cst("DirectedSet"), bvar(1)), prop()),
        ),
    )
}
/// Eventual property: a property P holds eventually along a filter F.
/// Type: `Filter α → (α → Prop) → Prop`.
pub fn eventual_property_ty() -> Expr {
    impl_pi(
        "α",
        type0(),
        arrow(
            app(cst("Filter"), bvar(0)),
            arrow(arrow(bvar(1), prop()), prop()),
        ),
    )
}
/// Order topology base: the collection of open intervals forms a base.
/// Type: `Type → Prop`.
pub fn order_topology_base_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Lattice-ordered group (l-group): a group that is also a lattice with
/// compatible operations.  Type: `Type → Prop`.
pub fn l_group_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Riesz space (vector lattice): a partially ordered vector space where
/// the order is a lattice order.  Type: `Type → Prop`.
pub fn riesz_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Archimedean Riesz space: a Riesz space satisfying the Archimedean property.
/// Type: `Type → Prop`.
pub fn archimedean_riesz_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Ordered Banach space: a Banach space with a closed positive cone.
/// Type: `Type → Prop`.
pub fn ordered_banach_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Cone in an ordered vector space: a set C with C + C ⊆ C and λC ⊆ C for λ ≥ 0.
/// Type: `(α : Type) → Set α → Prop`.
pub fn convex_cone_ty() -> Expr {
    impl_pi("α", type0(), arrow(app(cst("Set"), bvar(0)), prop()))
}
/// Cone duality (Riesz–Kantorovich): the dual of an ordered Banach space is ordered.
/// Type: `Prop`.
pub fn cone_duality_ty() -> Expr {
    prop()
}
/// Boolean space (Stone space): a compact totally disconnected Hausdorff space.
/// Type: `Type → Prop`.
pub fn boolean_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Spectral space: a sober space whose compact open sets form a basis closed
/// under finite intersections.  Type: `Type → Prop`.
pub fn spectral_space_ty() -> Expr {
    impl_pi("α", type0(), prop())
}
/// Patchwork topology (constructible topology): the topology on a spectral space
/// generated by both open and closed quasi-compact sets.
/// Type: `Type → TopologicalSpace`.
pub fn patchwork_topology_ty() -> Expr {
    impl_pi("α", type0(), app(cst("TopologicalSpace"), bvar(0)))
}
/// Register all order-topology axioms into the given kernel environment.
pub fn register_order_topology(env: &mut Environment) {
    let axioms: &[(&str, Expr)] = &[
        ("OrderTopology", order_topology_ty()),
        ("Nhds", nhds_ty()),
        ("Filter.atTop", filter_at_top_ty()),
        ("Filter.atBot", filter_at_bot_ty()),
        ("Tendsto", tendsto_ty()),
        ("ContinuousAt", continuous_at_ty()),
        ("ContinuousOn", continuous_on_ty()),
        ("IsOpen_Ioi", is_open_ioi_ty()),
        ("IsOpen_Iio", is_open_iio_ty()),
        (
            "OrderTopology.instTopologicalSpace",
            order_topology_inst_ty(),
        ),
        ("nhds_order_left", nhds_order_left_ty()),
        ("nhds_order_right", nhds_order_right_ty()),
        ("Monotone", monotone_ty()),
        ("Antitone", antitone_ty()),
        ("StrictMono", strict_mono_ty()),
        ("LimSup", limsup_ty()),
        ("LimInf", liminf_ty()),
        ("UpperSemicontinuous", upper_semicontinuous_ty()),
        ("LowerSemicontinuous", lower_semicontinuous_ty()),
        ("IsCompact_Icc", is_compact_icc_ty()),
        ("IntermediateValueThm", intermediate_value_thm_ty()),
        ("ExtremeValueThm", extreme_value_thm_ty()),
        ("DiniTheorem", dini_theorem_ty()),
        ("MonotoneConvergenceThm", monotone_convergence_thm_ty()),
        ("DedekindComplete", dedekind_complete_ty()),
        ("OrderConnected", order_connected_ty()),
        ("ConnectedSpace", connected_space_ty()),
        ("OrderTopologyConnected", order_topology_connected_ty()),
        ("OrderedTopologicalGroup", ordered_topological_group_ty()),
        ("TopologicalLattice", topological_lattice_ty()),
        ("ScottOpen", scott_open_ty()),
        ("ScottTopology", scott_topology_ty()),
        ("LawsonTopology", lawson_topology_ty()),
        ("AlexandroffTopology", alexandroff_topology_ty()),
        ("SpecializationOrder", specialization_order_ty()),
        ("BirkhoffRepresentation", birkhoff_representation_ty()),
        ("PriestleyDuality", priestley_duality_ty()),
        ("ZornLemma", zorn_lemma_ty()),
        ("OrdinalTopology", ordinal_topology_ty()),
        ("NetConvergence", net_convergence_ty()),
        ("OrderBasis_Ioo", order_basis_ioo_ty()),
        ("IsOpen_Ioo", is_open_ioo_ty()),
        ("IsClosed_Icc", is_closed_icc_ty()),
        ("OrderTopology.t2Space", order_topology_t2_ty()),
        ("OrderTopology.regularSpace", order_topology_regular_ty()),
        ("OrderedFieldTopology", ordered_field_topology_ty()),
        ("Filter.limsup", filter_limsup_ty()),
        ("Filter.liminf", filter_liminf_ty()),
        ("OrderIsoHomeomorph", order_iso_homeomorph_ty()),
        ("ContinuousMono_compose", continuous_mono_compose_ty()),
        ("MacNeilleCompletion", macneille_completion_ty()),
        ("DedekindCuts", dedekind_cuts_ty()),
        ("SupPreservingMap", sup_preserving_map_ty()),
        ("IntervalTopology", interval_topology_ty()),
        ("OrderEqIntervalTopology", order_eq_interval_topology_ty()),
        ("ConnectedOrderedSpace", connected_ordered_space_ty()),
        ("LongLine", long_line_ty()),
        ("OrdinalOmega1", ordinal_omega1_ty()),
        ("AlexandroffOmega1", alexandroff_omega1_ty()),
        ("MonotonicallyNormal", monotonically_normal_ty()),
        ("GeneralizedOrderedSpace", generalized_ordered_space_ty()),
        ("LOTS", lots_ty()),
        ("SorgenfreyLine", sorgenfrey_line_ty()),
        ("MichaelLine", michael_line_ty()),
        ("OrdinalMultiplication", ordinal_multiplication_ty()),
        ("OrdinalExponentiation", ordinal_exponentiation_ty()),
        ("CantorNormalForm", cantor_normal_form_ty()),
        ("HellyOrder", helly_order_ty()),
        ("ConvexOrderedSet", convex_ordered_set_ty()),
        ("CompactConvexOrdered", compact_convex_ordered_ty()),
        ("MonotoneContinuous", monotone_continuous_ty()),
        ("DiniUniformConvergence", dini_uniform_convergence_ty()),
        ("CofinalSubnet", cofinal_subnet_ty()),
        ("EventualProperty", eventual_property_ty()),
        ("OrderTopologyBase", order_topology_base_ty()),
        ("LGroup", l_group_ty()),
        ("RieszSpace", riesz_space_ty()),
        ("ArchimedeanRieszSpace", archimedean_riesz_space_ty()),
        ("OrderedBanachSpace", ordered_banach_space_ty()),
        ("ConvexCone", convex_cone_ty()),
        ("ConeDuality", cone_duality_ty()),
        ("BooleanSpace", boolean_space_ty()),
        ("SpectralSpace", spectral_space_ty()),
        ("PatchworkTopology", patchwork_topology_ty()),
    ];
    for (name, ty) in axioms {
        env.add(Declaration::Axiom {
            name: Name::str(*name),
            univ_params: vec![],
            ty: ty.clone(),
        })
        .ok();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn registered_env() -> Environment {
        let mut env = Environment::new();
        register_order_topology(&mut env);
        env
    }
    #[test]
    fn test_order_topology_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("OrderTopology")).is_some());
    }
    #[test]
    fn test_nhds_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Nhds")).is_some());
    }
    #[test]
    fn test_filter_at_top_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Filter.atTop")).is_some());
    }
    #[test]
    fn test_filter_at_bot_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Filter.atBot")).is_some());
    }
    #[test]
    fn test_tendsto_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Tendsto")).is_some());
    }
    #[test]
    fn test_continuous_at_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ContinuousAt")).is_some());
    }
    #[test]
    fn test_is_open_ioi_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IsOpen_Ioi")).is_some());
    }
    #[test]
    fn test_nhds_order_left_right_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("nhds_order_left")).is_some());
        assert!(env.get(&Name::str("nhds_order_right")).is_some());
    }
    #[test]
    fn test_monotone_antitone_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("Monotone")).is_some());
        assert!(env.get(&Name::str("Antitone")).is_some());
        assert!(env.get(&Name::str("StrictMono")).is_some());
    }
    #[test]
    fn test_limsup_liminf_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("LimSup")).is_some());
        assert!(env.get(&Name::str("LimInf")).is_some());
        assert!(env.get(&Name::str("Filter.limsup")).is_some());
        assert!(env.get(&Name::str("Filter.liminf")).is_some());
    }
    #[test]
    fn test_semicontinuity_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("UpperSemicontinuous")).is_some());
        assert!(env.get(&Name::str("LowerSemicontinuous")).is_some());
    }
    #[test]
    fn test_compactness_ivt_evt_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("IsCompact_Icc")).is_some());
        assert!(env.get(&Name::str("IntermediateValueThm")).is_some());
        assert!(env.get(&Name::str("ExtremeValueThm")).is_some());
    }
    #[test]
    fn test_dini_mct_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("DiniTheorem")).is_some());
        assert!(env.get(&Name::str("MonotoneConvergenceThm")).is_some());
    }
    #[test]
    fn test_dedekind_connectedness_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("DedekindComplete")).is_some());
        assert!(env.get(&Name::str("OrderConnected")).is_some());
        assert!(env.get(&Name::str("ConnectedSpace")).is_some());
        assert!(env.get(&Name::str("OrderTopologyConnected")).is_some());
    }
    #[test]
    fn test_topological_structures_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("OrderedTopologicalGroup")).is_some());
        assert!(env.get(&Name::str("TopologicalLattice")).is_some());
        assert!(env.get(&Name::str("OrderedFieldTopology")).is_some());
    }
    #[test]
    fn test_scott_lawson_alexandroff_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("ScottOpen")).is_some());
        assert!(env.get(&Name::str("ScottTopology")).is_some());
        assert!(env.get(&Name::str("LawsonTopology")).is_some());
        assert!(env.get(&Name::str("AlexandroffTopology")).is_some());
        assert!(env.get(&Name::str("SpecializationOrder")).is_some());
    }
    #[test]
    fn test_birkhoff_priestley_zorn_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("BirkhoffRepresentation")).is_some());
        assert!(env.get(&Name::str("PriestleyDuality")).is_some());
        assert!(env.get(&Name::str("ZornLemma")).is_some());
    }
    #[test]
    fn test_ordinal_net_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("OrdinalTopology")).is_some());
        assert!(env.get(&Name::str("NetConvergence")).is_some());
    }
    #[test]
    fn test_interval_axioms_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("OrderBasis_Ioo")).is_some());
        assert!(env.get(&Name::str("IsOpen_Ioo")).is_some());
        assert!(env.get(&Name::str("IsClosed_Icc")).is_some());
        assert!(env.get(&Name::str("OrderTopology.t2Space")).is_some());
        assert!(env.get(&Name::str("OrderTopology.regularSpace")).is_some());
    }
    #[test]
    fn test_iso_compose_registered() {
        let env = registered_env();
        assert!(env.get(&Name::str("OrderIsoHomeomorph")).is_some());
        assert!(env.get(&Name::str("ContinuousMono_compose")).is_some());
    }
    #[test]
    fn test_ordered_interval_contains() {
        let iv = OrderedInterval::new(1.0_f64, 5.0).expect("OrderedInterval::new should succeed");
        assert!(iv.contains(&3.0));
        assert!(iv.contains(&1.0));
        assert!(iv.contains(&5.0));
        assert!(!iv.contains(&0.0));
        assert!(!iv.contains(&6.0));
    }
    #[test]
    fn test_ordered_interval_overlaps() {
        let a = OrderedInterval::new(1.0_f64, 4.0).expect("OrderedInterval::new should succeed");
        let b = OrderedInterval::new(3.0_f64, 7.0).expect("OrderedInterval::new should succeed");
        let c = OrderedInterval::new(5.0_f64, 9.0).expect("OrderedInterval::new should succeed");
        assert!(a.overlaps(&b));
        assert!(!a.overlaps(&c));
    }
    #[test]
    fn test_ordered_interval_intersect() {
        let a = OrderedInterval::new(1.0_f64, 4.0).expect("OrderedInterval::new should succeed");
        let b = OrderedInterval::new(3.0_f64, 7.0).expect("OrderedInterval::new should succeed");
        let inter = a.intersect(&b).expect("intersect should succeed");
        assert!((inter.lo - 3.0).abs() < 1e-12);
        assert!((inter.hi - 4.0).abs() < 1e-12);
    }
    #[test]
    fn test_ordered_interval_none_when_inverted() {
        assert!(OrderedInterval::new(5.0_f64, 1.0).is_none());
    }
    #[test]
    fn test_limsup_inf_basic() {
        let lsi = LimSupInf::new(vec![1.0, 3.0, 2.0, 5.0, 4.0]);
        assert_eq!(lsi.limsup(), Some(5.0));
        assert_eq!(lsi.liminf(), Some(1.0));
        assert!(!lsi.converges());
    }
    #[test]
    fn test_limsup_inf_constant() {
        let lsi = LimSupInf::new(vec![7.0, 7.0, 7.0]);
        assert_eq!(lsi.limsup(), Some(7.0));
        assert_eq!(lsi.liminf(), Some(7.0));
        assert!(lsi.converges());
        assert_eq!(lsi.limit(), Some(7.0));
    }
    #[test]
    fn test_limsup_inf_empty() {
        let lsi = LimSupInf::new(vec![]);
        assert_eq!(lsi.limsup(), None);
        assert_eq!(lsi.liminf(), None);
    }
    #[test]
    fn test_monotone_checker_increasing() {
        let checker = MonotoneFnChecker::new(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 4.0)]);
        assert!(checker.is_monotone());
        assert!(checker.is_strict_mono());
        assert!(!checker.is_antitone());
    }
    #[test]
    fn test_monotone_checker_decreasing() {
        let checker = MonotoneFnChecker::new(vec![(0.0, 9.0), (1.0, 4.0), (2.0, 1.0)]);
        assert!(!checker.is_monotone());
        assert!(checker.is_antitone());
    }
    #[test]
    fn test_monotone_checker_flat() {
        let checker = MonotoneFnChecker::new(vec![(0.0, 3.0), (1.0, 3.0), (2.0, 3.0)]);
        assert!(checker.is_monotone());
        assert!(!checker.is_strict_mono());
        assert!(checker.is_antitone());
    }
    #[test]
    fn test_order_topology_basis() {
        let basis = OrderTopologyBasis::new(vec![1.0, 3.0, 2.0]);
        assert_eq!(basis.basis_size(), 9);
        let intervals = basis.open_intervals();
        assert_eq!(intervals.len(), 3);
        assert!(OrderTopologyBasis::in_open_interval(1.0, 3.0, 2.0));
        assert!(!OrderTopologyBasis::in_open_interval(1.0, 3.0, 1.0));
    }
    #[test]
    fn test_scott_open_set() {
        let s = ScottOpenSet::new(vec![0, 1, 2, 3], |x| x >= 2);
        assert!(s.contains(2));
        assert!(s.contains(3));
        assert!(!s.contains(0));
        assert!(s.is_upper_set());
        assert_eq!(s.members(), vec![2, 3]);
    }
    #[test]
    fn test_scott_open_set_not_upper() {
        let s = ScottOpenSet::new(vec![0, 1, 2, 3], |x| x == 1);
        assert!(!s.is_upper_set());
    }
    #[test]
    fn test_total_axiom_count() {
        let env = registered_env();
        let expected_names = [
            "OrderTopology",
            "Nhds",
            "Filter.atTop",
            "Filter.atBot",
            "Tendsto",
            "ContinuousAt",
            "ContinuousOn",
            "IsOpen_Ioi",
            "IsOpen_Iio",
            "OrderTopology.instTopologicalSpace",
            "nhds_order_left",
            "nhds_order_right",
            "Monotone",
            "Antitone",
            "StrictMono",
            "LimSup",
            "LimInf",
            "UpperSemicontinuous",
            "LowerSemicontinuous",
            "IsCompact_Icc",
            "IntermediateValueThm",
            "ExtremeValueThm",
            "DiniTheorem",
            "MonotoneConvergenceThm",
            "DedekindComplete",
            "OrderConnected",
            "ConnectedSpace",
            "OrderTopologyConnected",
            "OrderedTopologicalGroup",
            "TopologicalLattice",
            "ScottOpen",
            "ScottTopology",
            "LawsonTopology",
            "AlexandroffTopology",
            "SpecializationOrder",
            "BirkhoffRepresentation",
            "PriestleyDuality",
            "ZornLemma",
            "OrdinalTopology",
            "NetConvergence",
            "OrderBasis_Ioo",
            "IsOpen_Ioo",
            "IsClosed_Icc",
            "OrderTopology.t2Space",
            "OrderTopology.regularSpace",
            "OrderedFieldTopology",
            "Filter.limsup",
            "Filter.liminf",
            "OrderIsoHomeomorph",
            "ContinuousMono_compose",
        ];
        for name in expected_names {
            assert!(env.get(&Name::str(name)).is_some(), "Missing axiom: {name}");
        }
    }
    #[test]
    fn test_new_extended_axioms_registered() {
        let env = registered_env();
        let expected = [
            "MacNeilleCompletion",
            "DedekindCuts",
            "SupPreservingMap",
            "IntervalTopology",
            "OrderEqIntervalTopology",
            "ConnectedOrderedSpace",
            "LongLine",
            "OrdinalOmega1",
            "AlexandroffOmega1",
            "MonotonicallyNormal",
            "GeneralizedOrderedSpace",
            "LOTS",
            "SorgenfreyLine",
            "MichaelLine",
            "OrdinalMultiplication",
            "OrdinalExponentiation",
            "CantorNormalForm",
            "HellyOrder",
            "ConvexOrderedSet",
            "CompactConvexOrdered",
            "MonotoneContinuous",
            "DiniUniformConvergence",
            "CofinalSubnet",
            "EventualProperty",
            "OrderTopologyBase",
            "LGroup",
            "RieszSpace",
            "ArchimedeanRieszSpace",
            "OrderedBanachSpace",
            "ConvexCone",
            "ConeDuality",
            "BooleanSpace",
            "SpectralSpace",
            "PatchworkTopology",
        ];
        for name in expected {
            assert!(
                env.get(&Name::str(name)).is_some(),
                "Missing new axiom: {name}"
            );
        }
    }
    #[test]
    fn test_macneille_closed_empty() {
        let order = vec![
            vec![true, true, true],
            vec![false, true, true],
            vec![false, false, true],
        ];
        let mc = MacNeilleCompletion::new(order);
        assert!(mc.is_closed(&[]));
    }
    #[test]
    fn test_macneille_closed_principal_down() {
        let order = vec![
            vec![true, true, true],
            vec![false, true, true],
            vec![false, false, true],
        ];
        let mc = MacNeilleCompletion::new(order);
        assert!(mc.is_closed(&[0, 1]));
    }
    #[test]
    fn test_macneille_all_closed_chain() {
        let order = vec![
            vec![true, true, true],
            vec![false, true, true],
            vec![false, false, true],
        ];
        let mc = MacNeilleCompletion::new(order);
        let closed = mc.all_closed_sets();
        assert_eq!(closed.len(), 4, "3-chain has 4 MacNeille closed sets");
    }
    #[test]
    fn test_sorgenfrey_in_basis_set() {
        assert!(SorgenfreyLineTopology::in_basis_set(1.0, 3.0, 1.0));
        assert!(SorgenfreyLineTopology::in_basis_set(1.0, 3.0, 2.0));
        assert!(!SorgenfreyLineTopology::in_basis_set(1.0, 3.0, 3.0));
        assert!(!SorgenfreyLineTopology::in_basis_set(1.0, 3.0, 0.5));
    }
    #[test]
    fn test_sorgenfrey_basis_count() {
        let sorg = SorgenfreyLineTopology::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(sorg.basis_sets().len(), 3);
    }
    #[test]
    fn test_sorgenfrey_intersect_basis() {
        let result = SorgenfreyLineTopology::intersect_basis(1.0, 4.0, 2.0, 5.0);
        assert!(result.is_some());
        let (lo, hi) = result.expect("result should be valid");
        assert!((lo - 2.0).abs() < 1e-12);
        assert!((hi - 4.0).abs() < 1e-12);
        let empty = SorgenfreyLineTopology::intersect_basis(1.0, 2.0, 3.0, 4.0);
        assert!(empty.is_none());
    }
    #[test]
    fn test_ordinal_zero() {
        let zero = OrdinalArithmetic::zero();
        assert!(zero.is_zero());
    }
    #[test]
    fn test_ordinal_add_finite() {
        let a = OrdinalArithmetic::new(vec![(0, 3)]);
        let b = OrdinalArithmetic::new(vec![(0, 2)]);
        let sum = a.add(&b);
        assert_eq!(sum.cnf.len(), 1);
        assert_eq!(sum.cnf[0], (0, 5));
    }
    #[test]
    fn test_ordinal_add_omega_absorbs() {
        let a = OrdinalArithmetic::new(vec![(1, 1)]);
        let b = OrdinalArithmetic::new(vec![(0, 1)]);
        let sum = a.add(&b);
        assert_eq!(sum.cnf.len(), 2);
    }
    #[test]
    fn test_ordinal_compare() {
        let a = OrdinalArithmetic::new(vec![(1, 1)]);
        let b = OrdinalArithmetic::new(vec![(0, 100)]);
        assert_eq!(a.compare(&b), std::cmp::Ordering::Greater);
    }
    #[test]
    fn test_vector_lattice_join_meet() {
        let vl = VectorLatticeOps::new(3);
        let x = [1.0, -2.0, 3.0];
        let y = [-1.0, 4.0, 2.0];
        let join = vl.join(&x, &y);
        assert_eq!(join, vec![1.0, 4.0, 3.0]);
        let meet = vl.meet(&x, &y);
        assert_eq!(meet, vec![-1.0, -2.0, 2.0]);
    }
    #[test]
    fn test_vector_lattice_pos_neg_abs() {
        let vl = VectorLatticeOps::new(3);
        let x = [1.0, -2.0, 0.0];
        let xp = vl.pos_part(&x);
        assert_eq!(xp, vec![1.0, 0.0, 0.0]);
        let xm = vl.neg_part(&x);
        assert_eq!(xm, vec![0.0, 2.0, 0.0]);
        let xa = vl.abs_val(&x);
        assert_eq!(xa, vec![1.0, 2.0, 0.0]);
    }
    #[test]
    fn test_vector_lattice_riesz_decomp() {
        let vl = VectorLatticeOps::new(3);
        let x = [3.0, -1.5, 0.0];
        assert!(vl.check_riesz_decomposition(&x));
    }
    #[test]
    fn test_vector_lattice_le() {
        let vl = VectorLatticeOps::new(2);
        let x = [1.0, 2.0];
        let y = [1.0, 3.0];
        let z = [0.0, 2.0];
        assert!(vl.le(&x, &y));
        assert!(!vl.le(&y, &x));
        assert!(!vl.le(&x, &z));
    }
}
#[cfg(test)]
mod tests_order_topology_ext {
    use super::*;
    #[test]
    fn test_directed_set() {
        let mut ds = DirectedSet::new();
        let a = ds.add_element("a");
        let b = ds.add_element("b");
        let c = ds.add_element("c");
        ds.add_relation(a, c);
        ds.add_relation(b, c);
        assert!(ds.leq(a, c));
        assert!(ds.leq(b, c));
        assert!(ds.is_directed());
    }
    #[test]
    fn test_net_convergence() {
        let vals: Vec<f64> = (0..100).map(|i| 1.0 + 1.0 / (i as f64 + 1.0)).collect();
        let mut net = Net::new(vals);
        net.detect_convergence(0.05);
        assert!(net.converges);
        let lim = net.limit.expect("limit should be valid");
        assert!(
            (lim - 1.0).abs() < 0.1,
            "Net should converge to ~1, got {lim}"
        );
    }
    #[test]
    fn test_tychonoff_space() {
        let r = TychonoffSpaceData::new("R").with_stone_cech("βR");
        assert!(!r.is_compact);
        assert!(r.stone_cech.is_some());
        let unit_interval = TychonoffSpaceData::new("[0,1]");
        assert!(!unit_interval.is_paracompact());
        assert!(r.embedding_theorem().contains("Stone"));
    }
    #[test]
    fn test_ordered_metric_space() {
        let oms = OrderedMetricSpace::from_sorted_reals(vec![1.0, 3.0, 5.0]);
        assert!(oms.order_leq(0, 2));
        assert!((oms.metric(0, 2).expect("metric should succeed") - 4.0).abs() < 1e-10);
        assert_eq!(oms.infimum(1, 2), Some(3.0));
        assert_eq!(oms.supremum(0, 2), Some(5.0));
        assert!(oms.is_riesz_space());
    }
    #[test]
    fn test_small_ordinal() {
        let w = SmallOrdinal::omega();
        assert!(w.is_limit());
        assert!(!w.is_successor());
        let n3 = SmallOrdinal::finite(3);
        assert!(n3.is_successor());
        assert!(!n3.is_limit());
        let wplus3 = w.add(&n3);
        assert_eq!(wplus3.omega_coeff, 1);
        assert_eq!(wplus3.finite_part, 3);
        let three_plus_w = n3.add(&w);
        assert_eq!(three_plus_w.omega_coeff, 1);
        assert_eq!(three_plus_w.finite_part, 0);
        let n3b = SmallOrdinal::finite(3);
        let w_times_3 = w.mul(&n3b);
        assert_eq!(w_times_3.omega_coeff, 3);
    }
}
#[cfg(test)]
mod tests_order_topology_ext2 {
    use super::*;
    #[test]
    fn test_real_interval_closed() {
        let i = RealInterval::closed(0.0, 1.0);
        assert!(i.contains(0.0));
        assert!(i.contains(1.0));
        assert!(i.contains(0.5));
        assert!(!i.contains(1.5));
        assert!(i.is_compact());
        assert!((i.length().expect("length should succeed") - 1.0).abs() < 1e-10);
    }
    #[test]
    fn test_real_interval_open() {
        let i = RealInterval::open(0.0, 1.0);
        assert!(!i.contains(0.0));
        assert!(!i.contains(1.0));
        assert!(i.contains(0.5));
        assert!(!i.is_compact());
    }
    #[test]
    fn test_small_ordinal_zero() {
        let z = SmallOrdinal::finite(0);
        assert!(z.is_zero());
        assert!(!z.is_limit());
        assert!(!z.is_successor());
    }
}
