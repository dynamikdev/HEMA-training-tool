For professional-grade Bevy development in 2026, the focus has shifted from "making things work" to "maintaining scalability and performance" in large ECS codebases. Following these conventions ensures that our team maintains a clean, modular, and high-performance architecture.


## 1. ECS Structural Integrity
### Component Requirements & Storage
With the maturity of component requirements, manual boilerplate for "bundle" management is deprecated.

* **Explicit Requirements**: Use the `#[require(T)]` attribute to ensure that a component always has its dependencies (e.g., a `Player` component requiring `Transform` and `Visibility`). This prevents runtime logic errors where a system expects a component that wasn't spawned.
* **Storage Strategy**: Default to table storage. Use `#[component(storage = "SparseSet")]` only for components that are added or removed frequently (like `Frozen` or `Stunned` status effects) to avoid archetype fragmentation.

### Entity Lifecycle Management
* **Scoped Cleanup**: Use `StateScoped(MyState::InGame)` for any entity that must be despawned when leaving a specific state. This replaces manual "Cleanup" systems.
* **Naming for Debugging**: All top-level entities must be spawned with a `Name` component: `commands.spawn((Name::new("Player"), PlayerBundle::default()))`. This is non-negotiable for efficient use of the Bevy Editor and Inspector.


## 2. System Organization & Scheduling
As the system count grows, implicit ordering becomes a significant source of bugs.

* **System Sets**: Group systems into logical sets (e.g., `InputSet`, `PhysicsSet`, `AnimationSet`). Configure the ordering of these sets in the `Plugin` build function rather than ordering individual systems.
* **Explicit Chains**: Within a set, use `.chain()` for systems that have a strict "Producer-Consumer" relationship to avoid one-frame delays.
* **Run Conditions**: Apply `.run_if(in_state(GameState::Playing))` at the **Set level** rather than the system level whenever possible to improve the scheduler's ability to optimize the execution graph.




## 3. Communication Patterns
### Events vs. Queries
* **One-to-Many Communication**: Always use `EventWriter`/`EventReader` for decoupled communication (e.g., `ExplosionEvent`). 
* **Direct Mutation**: Use `Query` only when a system "owns" the logic for that specific component.
* **Run Criteria**: Systems that react to events should use the `.run_if(on_event::<MyEvent>())` condition to prevent the system from waking up unnecessarily.


## 4. Code Architecture & Project Layout
To maintain fast compile times in 2026, we must separate data definitions from logic.

* **Crate Splitting**: 
    * `core_data`: Contains only `Component`, `Resource`, and `Event` definitions. This crate should have minimal dependencies.
    * `logic_systems`: Contains the systems and complex logic. 
    * This split allows for faster iterative compilation when only system logic changes.
* **Plugin-First Design**: Every feature must be encapsulated in a `Plugin`. A feature is only "public" if it is added to the `App` via its plugin.
* **Internal Preludes**: Each major module should export a `prelude.rs` containing the essential components and events needed by other modules, minimizing import clutter.


## 5. Optimization & Safety
* **Query Filtering**: Be aggressive with `With<T>` and `Without<T>` filters to narrow down the archetypes the scheduler needs to lock.
* **Read-Only by Default**: Use `&T` in queries unless mutation is strictly required. This allows Bevy to run more systems in parallel.
* **ParamSet**: When a system needs to access the same component mutably through two different queries, use `ParamSet` to resolve the conflict explicitly.

