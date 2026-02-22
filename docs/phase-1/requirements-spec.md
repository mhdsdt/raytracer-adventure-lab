# Phase 1 Requirements Specification
## Project: Recreate Sebastian Lague's "Coding Adventure: Ray Tracing" (First Video)
## Type: Requirements Gathering (SDLC Phase 1)
## Platform Context: macOS
## Version: 1.0

### 1. Purpose of This Document

This document defines what must be built in Phase 1 of a multi-month ray tracing learning project.

Phase 1 is focused on recreating the scope and outcomes of Sebastian Lague's first ray tracing video: **"Coding Adventure: Ray Tracing"**.

This is a requirements document only.

It intentionally does not define:
- architecture
- data flow
- file/folder structure
- engine/framework choice
- language choice
- implementation details
- code-level decisions

Those decisions will be handled later in the design phase (with LLM help if desired), while all coding will be written by the developer.

### 2. Project Context

The developer is a solo learner on macOS and wants to:
- learn ray tracing deeply
- build the code personally (100% self-written)
- use LLMs only for design discussions and planning support
- progress in phases over several months
- iterate on the same project over time

Phase 1 is the first step and should produce a working foundational ray/path tracer experience aligned with the first Sebastian Lague video.

### 3. Phase 1 Goal (What We Are Building)

Build a ray tracing project that reproduces the main visible behaviors and learning outcomes shown in Sebastian Lague's first video, including a basic ray/path-traced scene with progressive rendering and core material/light behavior.

The target is **functional and visual parity at a learning level**, not pixel-perfect matching.

### 4. Phase 1 Scope (In Scope)

Phase 1 must include the following capabilities, as shown in the reference video's feature progression.

#### 4.1 Core Rendering Behavior
- The project must render a scene using ray tracing/path tracing principles.
- The rendered output must show light interaction beyond simple flat coloring.
- The renderer must support multiple ray interactions (bounces) sufficient to produce visible reflection and indirect light behavior similar to the reference video.

#### 4.2 Scene Geometry (Minimum Feature Set)
- The project must support sphere rendering.
- The project must support triangle rendering.
- The project must be able to render a scene containing multiple objects at once.
- The project must demonstrate at least one triangle-based object/shape in the final phase demo scenes.

#### 4.3 Material/Surface Behavior
- The project must support a diffuse-like surface response.
- The project must support specular/reflective surface behavior.
- The project must support a glossy/rough reflective behavior (not only perfect mirror reflection).
- The project must support visibly different surface appearances across objects.

#### 4.4 Lighting and Environment
- The project must support emissive/light-contributing surfaces or scene lighting behavior that affects ray-traced results.
- The project must support a simple sky/background contribution (not only a flat black background).
- The project must demonstrate lighting impact on the final image in at least one test scene.

#### 4.5 Sampling and Randomness (Observable Outcome)
- The project must use sampling behavior that introduces stochastic variation consistent with path tracing style rendering.
- The project must show improvement in image quality as sampling accumulates (for example, reduced noise or smoother final result over time).

This requirement is about the visible result, not the algorithm choice.

#### 4.6 Progressive Rendering
- The project must support progressive rendering/accumulation so the image improves over time while the scene remains unchanged.
- The project must visibly reset or restart progressive accumulation when a change occurs that invalidates the current accumulation (such as camera/scene/material change), if the project includes interactive changes.

#### 4.7 Camera and Image Quality Features
- The project must support camera-based ray generation.
- The project must demonstrate anti-aliasing or comparable sampling-based edge smoothing behavior.
- The project must demonstrate depth-of-field behavior with visible focus and blur differences.
- The project must demonstrate a blur-related image effect consistent with the reference video progression (within the ray-tracing render workflow, not a separate unrelated post effect).

#### 4.8 Demonstration Scenes (Required)
Phase 1 must include a small set of scenes/tests that demonstrate the required features.

At minimum, the final Phase 1 submission must include:
- one scene that proves multiple spheres and material variation
- one scene that proves lighting/emissive contribution and sky/background behavior
- one scene that proves triangle support
- one scene that proves glossy/specular differences
- one scene that proves depth of field
- one scene or comparison that shows progressive rendering improvement over time

A single scene may satisfy multiple requirements.

### 5. Phase 1 Scope (Out of Scope)

The following are explicitly out of scope for Phase 1:
- acceleration structures focused on major optimization (for example BVH-heavy work)
- advanced performance optimization as a primary goal
- glass/caustics/rainbow/spectral rendering features from later videos
- complex production scene import pipelines
- broad cross-platform support (Windows/Linux) unless the developer later chooses it
- UI polish beyond what is necessary to demonstrate the required behaviors
- code generation by LLMs

### 6. Reference Standard (How Scope Is Anchored)

Phase 1 scope is anchored to the observable features shown in Sebastian Lague's first video:
- "Coding Adventure: Ray Tracing" (the first ray tracing video in this sequence)

If a feature is clearly introduced in later videos (for example BVH optimization, glass/caustics), it belongs to later phases and is not required in Phase 1.

If a feature is ambiguous, the rule is:
- include it only if it is clearly shown in the first video and supports the Phase 1 learning goal

### 7. Success Criteria (Phase 1)

Phase 1 is successful when:
- the project demonstrates the full Phase 1 feature set listed in this document
- the output visually resembles the class of results shown in the first video (not necessarily identical scenes or exact images)
- the developer can explain what each Phase 1 feature does and where it is demonstrated
- the project runs on the developer's macOS machine
- all code is written by the developer

### 8. Constraints and Working Rules

#### 8.1 Platform Constraint
- Phase 1 deliverable must run on macOS (developer's machine).

#### 8.2 Learning Constraint
- The project is learning-first.
- Readability and understanding matter more than performance in Phase 1.

#### 8.3 LLM Usage Rule
Allowed in design/planning:
- architecture discussion
- data flow discussion
- file/folder organization discussion
- requirement clarification
- review checklists
- debugging strategy discussion (conceptual)

Not allowed:
- code generation
- line-by-line implementation
- copy-paste code solutions for Phase 1 features

#### 8.4 Scope Control Rule
- "Nice-to-have" features discovered during coding must be logged for later phases, not added to Phase 1 unless they are required by this document.

### 9. Deliverables (End of Phase 1)

The Phase 1 output must include:
- a working codebase in the project repository
- a short Phase 1 README or progress summary describing what features are implemented
- a feature checklist mapped to this document
- captured images or short clips proving the required demonstration scenes
- a brief reflection note (what worked, what was hard, what is deferred to Phase 2)

These are required because this is a learning project and future phases depend on clear handoff from one phase to the next.

### 10. Non-Functional Requirements (Phase 1)

- Stability: The project should complete required demonstrations without crashing.
- Repeatability: The developer should be able to rerun the required demos on the same macOS machine.
- Clarity: The project should be understandable enough for the developer to continue into later phases.
- Traceability: It should be easy to map visible results to the Phase 1 requirements.

No strict performance target is required in Phase 1.

### 11. Open Questions for the Design Phase (To Resolve Before Coding)

These are intentionally deferred to the design phase:
- Which engine/framework (if any) will be used?
- Which language/shader stack will be used?
- CPU-based or GPU-based rendering approach?
- How scenes will be defined and changed?
- How progressive accumulation reset conditions will be tracked?
- What minimal controls are needed for testing and demonstrations?
- What output format(s) will be used for saved renders/screenshots?
- What test scene set will be used as the official Phase 1 acceptance set?

### 12. Definition of Done (Phase 1)

Phase 1 is done only when all items below are true.

- A ray/path-traced renderer is producing visible results on macOS.
- Spheres are supported and demonstrated.
- Triangles are supported and demonstrated.
- Multiple objects are rendered in the same scene.
- Diffuse surface behavior is demonstrated.
- Specular/reflective surface behavior is demonstrated.
- Glossy/rough reflective behavior is demonstrated.
- Lighting/emissive contribution is demonstrated.
- Sky/background contribution is demonstrated.
- Progressive rendering/accumulation is demonstrated.
- A progressive quality improvement example is documented (image/clip or before/after capture).
- Anti-aliasing (or equivalent sampling-based smoothing) is demonstrated.
- Depth of field is demonstrated with visible focus/blur difference.
- Phase 1 demo scenes are captured and organized.
- A short Phase 1 summary document exists in the repo.
- The developer confirms all code was written personally.
- Deferred items are listed for later phases.

### 13. Roadmap Preview (Later Phases, High Level Only)

This is a planning preview only. Each later phase should get its own requirements document before work starts.

#### Phase 2 (Estimated 4 to 8 weeks)
Focus:
- performance and scalability improvements inspired by the later ray tracing optimization video(s)
- rendering more complex scenes while preserving Phase 1 behavior

Definition of Done (preview):
- the renderer handles more scene complexity than Phase 1
- performance is measurably improved on the same machine
- Phase 1 visual behavior still works (no regressions in core features)

#### Phase 3 (Estimated 4 to 8 weeks)
Focus:
- glass, refraction, absorption, and caustic-focused features inspired by the later Sebastian Lague ray tracing video

Definition of Done (preview):
- glass/refraction features are demonstrated in test scenes
- at least one caustic-style result is demonstrated
- Phase 1 and Phase 2 core features remain usable

#### Phase 4 (Estimated 3 to 6 weeks)
Focus:
- consolidation, cleanup, documentation, test scenes, and usability for continued experimentation

Definition of Done (preview):
- project is easier to extend
- demo scenes are documented and reproducible
- roadmap-based features are organized and clearly separated

#### Phase 5 (Estimated 3 to 6 weeks, optional but recommended)
Focus:
- personal experiments beyond Sebastian's videos (your own ideas)

Definition of Done (preview):
- at least one original extension is built and documented
- the project shows your own direction, not only reproduction

### 14. Phase 1 Exit Decision

At the end of Phase 1, the next step is not coding immediately.

The next step is:
- create the Phase 1 design document (architecture and project structure decisions)
- then begin implementation using self-written code

This preserves your goal: LLM help for design only, coding by you.

