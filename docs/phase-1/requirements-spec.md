# Phase 1 Requirements Specification
## Project: Recreate Sebastian Lague's "Coding Adventure: Ray Tracing" (First Video)
## Type: Requirements Gathering (SDLC Phase 1)
## Platform Context: macOS
## Version: 1.1

### 1. Purpose of This Document

This document defines what must be built in Phase 1 of a multi-month ray tracing learning project.

Phase 1 is focused on recreating the scope and outcomes of Sebastian Lague's first ray tracing video: **"Coding Adventure: Ray Tracing"**.

This is a requirements document only.

It intentionally does not define:
- architecture
- data flow
- file/folder structure
- module layout
- dependency choices
- implementation details
- code-level decisions

Those decisions are defined in the Phase 1 design document. All coding will be written by the developer.

### 2. Project Context

The developer is a solo learner on macOS and wants to:
- learn Rust while building the project
- learn ray tracing deeply
- write the code personally (100% self-written)
- use LLMs only for design/planning discussions, not coding
- progress in phases over several months
- iterate on the same project over time

Phase 1 is the first step and should produce a working foundational ray/path tracer experience aligned with the first Sebastian Lague video.

### 3. Phase 1 Goal (What We Are Building)

Build a ray tracing project that reproduces the main visible behaviors and learning outcomes shown in Sebastian Lague's first video, including a progressive ray/path-traced preview with core geometry, materials, lighting, and camera behavior.

The target is **functional and visual parity at a learning level**, not pixel-perfect matching.

### 4. Phase 1 Deliverable Shape (User-Facing)

Phase 1 must deliver a **windowed progressive preview application** that runs on the developer's macOS machine.

The deliverable must support:
- progressive image refinement while the app is running
- pre-launch scene selection
- preset-based runtime controls for visual comparison and testing
- proof capture via macOS screenshots/clips (built-in export is not required)

### 5. Phase 1 Scope (In Scope)

Phase 1 must include the following capabilities.

#### 5.1 Core Rendering Behavior
- The project must render a scene using ray tracing/path tracing principles.
- The rendered output must show light interaction beyond simple flat coloring.
- The renderer must support multiple ray interactions (bounces) sufficient to produce visible reflection and indirect light behavior similar to the reference video.
- The output must improve progressively while accumulation continues.

#### 5.2 Scene Geometry (Minimum Feature Set)
- The project must support sphere rendering.
- The project must support triangle rendering.
- The project must render multiple objects in the same scene.
- Triangle support must be demonstrated in an official Phase 1 demo scene.

#### 5.3 Material and Surface Behavior
- The project must support a diffuse-like surface response.
- The project must support specular/reflective surface behavior.
- The project must support glossy/rough reflective behavior (not only perfect mirror reflection).
- The project must support visibly different surface appearances across objects.

#### 5.4 Lighting and Environment
- The project must support emissive/light-contributing surfaces or equivalent scene lighting behavior that affects ray-traced results.
- The project must support a simple sky/background contribution (not only a flat black background).
- The project must demonstrate visible lighting impact in at least one official Phase 1 demo scene.

#### 5.5 Progressive Rendering and Reset Behavior
- The project must support progressive rendering/accumulation so the image improves over time while the scene and settings remain unchanged.
- The project must reset/restart accumulation when a runtime setting changes that invalidates the current accumulation.
- The app must remain usable while progressive rendering is active.

#### 5.6 Camera and Image Quality Features
- The project must support camera-based ray generation.
- The project must demonstrate anti-aliasing or equivalent sampling-based edge smoothing behavior.
- The project must demonstrate depth-of-field behavior with visible focus/blur differences.
- The project must support preset-based camera and depth-of-field changes for comparison.

#### 5.7 Phase 1 Interaction Model (Functional Requirements)
- Scene selection must happen before launch/run.
- The app must provide preset-based runtime control of:
  - quality level
  - camera preset (within the selected scene)
  - depth-of-field preset
- Runtime controls may be keyboard-based.
- Built-in image export/capture is not required in Phase 1.

#### 5.8 Debugging and Comparison Support
- The project must provide a deterministic/debug comparison mode selectable at launch.
- Deterministic mode must support repeatable comparison captures on the same machine/build for the same scene and presets.
- Deterministic mode is for debugging/verification support and does not replace normal stochastic rendering behavior.

#### 5.9 Official Demo Scenes (Required)
Phase 1 must include **exactly 3 official demo scenes** used for acceptance checks.

The 3 official scenes must collectively prove all required Phase 1 features, including:
- multiple spheres and material variation
- emissive/lighting and sky/background contribution
- triangle support
- glossy/specular differences
- depth of field
- progressive rendering improvement over time
- preset-based camera/quality/DOF comparison behavior

A single scene may satisfy multiple requirements.

### 6. Phase 1 Scope (Out of Scope)

The following are explicitly out of scope for Phase 1:
- acceleration structures focused on major optimization (for example BVH-heavy work)
- advanced performance optimization as a primary goal
- multithreaded rendering implementation
- GPU-first rendering architecture
- glass/caustics/rainbow/spectral features from later videos
- complex production scene import pipelines
- runtime scene editing tools or a scene editor UI
- live free-movement camera controls
- built-in image export pipeline (macOS screenshots/clips are sufficient)
- broad cross-platform support (Windows/Linux) unless chosen later
- code generation by LLMs

### 7. Reference Standard (How Scope Is Anchored)

Phase 1 scope is anchored to the observable features shown in Sebastian Lague's first ray tracing video:
- "Coding Adventure: Ray Tracing" (the first ray tracing video in this sequence)

If a feature is clearly introduced in later videos (for example BVH optimization, glass/caustics), it belongs to later phases and is not required in Phase 1.

If a feature is ambiguous, the rule is:
- include it only if it is clearly shown in the first video and supports the Phase 1 learning goal

### 8. Constraints and Working Rules

#### 8.1 Platform Constraint
- Phase 1 deliverable must run on macOS (developer's machine).

#### 8.2 Learning Constraint
- The project is learning-first.
- Readability and understanding matter more than performance in Phase 1.
- Rust learning is a primary goal of the phase.

#### 8.3 LLM Usage Rule
Allowed in design/planning:
- architecture discussion
- data flow discussion
- file/folder organization discussion
- requirements clarification
- review checklists
- debugging strategy discussion (conceptual)

Not allowed:
- code generation
- line-by-line implementation
- copy-paste code solutions for Phase 1 features

#### 8.4 Scope Control Rule
- Nice-to-have features discovered during coding must be logged for later phases unless they are required by this document.
- Phase 1 may use one limited sandbox experiment lane, but it must not block the official Phase 1 deliverable.

### 9. Deliverables (End of Phase 1)

The Phase 1 output must include:
- a working codebase in the project repository
- a Phase 1 design document (architecture/design decisions only)
- a short Phase 1 README or progress summary describing what features are implemented
- a feature checklist mapped to this document
- screenshots and short clips proving the 3 official demo scenes
- a brief reflection note (what worked, what was hard, what is deferred to Phase 2)
- a list of deferred ideas/backlog items for later phases

These are required because this is a learning project and future phases depend on a clean handoff.

### 10. Non-Functional Requirements (Phase 1)

- Stability: The project should complete official demo scene runs without crashing.
- Repeatability: The developer should be able to rerun the official demos on the same macOS machine.
- Responsiveness: The preview app should remain usable while progressive rendering is active.
- Clarity: The project should remain understandable enough to continue into later phases.
- Traceability: It should be easy to map visible results to the Phase 1 requirements.

No strict performance target is required in Phase 1.

### 11. Success Criteria (Phase 1)

Phase 1 is successful when:
- the project demonstrates the full Phase 1 feature set listed in this document
- the output visually resembles the class of results shown in the first video (not necessarily identical scenes or exact images)
- the 3 official demo scenes collectively cover the required features
- the developer can explain what each Phase 1 feature does and where it is demonstrated
- the project runs on the developer's macOS machine
- all code is written by the developer

### 12. Definition of Done (Phase 1)

Phase 1 is done only when all items below are true.

- A windowed progressive preview app is running on macOS.
- A ray/path-traced renderer is producing visible results in the preview app.
- Spheres are supported and demonstrated.
- Triangles are supported and demonstrated.
- Multiple objects are rendered in the same scene.
- Diffuse surface behavior is demonstrated.
- Specular/reflective surface behavior is demonstrated.
- Glossy/rough reflective behavior is demonstrated.
- Lighting/emissive contribution is demonstrated.
- Sky/background contribution is demonstrated.
- Progressive rendering/accumulation is demonstrated.
- Accumulation reset on runtime preset change is demonstrated.
- A progressive quality improvement example is documented (clip or comparison capture).
- Anti-aliasing (or equivalent sampling-based smoothing) is demonstrated.
- Depth of field is demonstrated with visible focus/blur difference.
- Preset-based runtime control of quality/camera/DOF is demonstrated.
- Deterministic mode is demonstrated for at least one comparison capture.
- The 3 official Phase 1 demo scenes are captured and organized.
- A short Phase 1 summary document exists in the repo.
- The developer confirms all code was written personally.
- Deferred items are listed for later phases.

### 13. Roadmap Preview (Later Phases, High Level Only)

This is a planning preview only. Each later phase should get its own requirements document before work starts.

#### Phase 2 (Estimated 4 to 8 weeks)
Focus:
- performance and scalability improvements (including threading and optimization work)
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

### 14. Handoff to Design and Implementation

This requirements document defines **what** Phase 1 must deliver.

The Phase 1 design document defines **how the system is organized** (architecture, module boundaries, data flow, dependency choices, and runtime behavior rules).

Implementation must follow both documents while preserving the project rule:
- LLMs may help with design decisions
- all implementation code is written by the developer
