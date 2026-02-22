# Phase 1 Implementation Coaching Guide (No-Code)
## Project: `raytracer-adventure-lab`
## Phase: Phase 1 (Sebastian Lague - "Coding Adventure: Ray Tracing")
## Purpose: Help You Implement Phase 1 Step by Step Without Getting Lost

### 1. Purpose of This Guide

This guide helps you implement Phase 1 from start to finish while keeping the code 100% yours.

It is a coaching guide, not a coding guide.

It gives you:
- a clear implementation order
- milestone checkpoints
- anti-scope-creep rules
- debugging habits
- recovery steps when you feel stuck

It does not give code.

### 2. How To Use This Guide (Important)

Use these three documents together:
- `docs/phase-1/requirements-spec.md` (what must exist)
- `docs/phase-1/design.md` (how the system is organized)
- `docs/phase-1/implementation-guide.md` (how to execute the work safely)

When confused:
1. go back to requirements (what is required)
2. check design (where it belongs)
3. return to current milestone (what to do next)

This prevents random refactoring and feature drift.

### 3. Phase 1 Rules (Keep These Visible)

- Write all code yourself.
- Use LLMs for design review and troubleshooting strategy only, not code.
- Do not add new features unless they are required or they are in the sandbox lane.
- Keep the preview app small and functional.
- Finish the official 3 demo scenes before polishing anything.

### 4. What "Not Getting Lost" Looks Like

You are not lost if, at any moment, you can answer:
- What milestone am I in?
- What is the next visible result I need?
- What file/module area should this work belong to?
- What is the exit criteria for this milestone?

If you cannot answer those four questions, stop and reset (see Section 13).

### 5. Implementation Strategy (Big Picture)

Build from outer shell to inner complexity, while keeping visible progress:
1. app shell and run loop
2. render/update pipeline skeleton
3. simplest visible image
4. progressive accumulation
5. geometry and materials
6. camera features and presets
7. deterministic mode and acceptance scenes
8. proof capture and cleanup

This order protects motivation and reduces debugging chaos.

### 6. Milestone Plan (Step-by-Step)

### Milestone 0: Project Setup and Execution Plan

Goal:
- Prepare the repo so work stays organized.

Do:
- create the Phase 1 support files if missing (`checklist.md`, `retrospective.md`, `notes/dev-log.md`, `notes/backlog.md`)
- copy the Phase 1 DoD checklist into `docs/phase-1/checklist.md`
- write a short Phase 1 working note: chosen defaults, official scenes, sandbox rule
- create a first dev-log entry

Exit criteria:
- you can point to the requirements, design, checklist, and dev-log files
- you have a one-line statement of the current milestone

Common trap:
- spending time on formatting docs instead of starting the app shell

### Milestone 1: Windowed Preview App Shell (No Real Rendering Yet)

Goal:
- Open the preview window and keep it responsive.

Do:
- build the app shell and window loop
- make the fixed-size window behavior match the design
- add a placeholder frame presentation path (temporary image content is fine)
- add always-visible overlay scaffolding (labels can be placeholders)

Exit criteria:
- a window opens reliably
- the app stays responsive
- overlay area/text can be shown

Common trap:
- mixing renderer domain decisions into the app shell too early

### Milestone 2: CLI Launch Config and Startup Summary

Goal:
- Start the app with explicit launch-time configuration.

Do:
- implement launch-time config parsing for scene selection, deterministic mode, and initial quality profile
- support a default scene with override
- support the optional dev resolution fallback setting
- print a clear startup summary to the terminal

Exit criteria:
- the app can start with different launch settings
- invalid inputs fail clearly and early

Common trap:
- adding too many CLI options before they are needed

### Milestone 3: Module Boundaries and Render Session Skeleton

Goal:
- Lock the architectural seams before rendering complexity grows.

Do:
- create the module structure from the design doc (names can vary slightly)
- create the `render_session` responsibility boundary
- define where runtime preset state lives
- define where accumulation reset decisions live
- make the app shell talk to the render session, not directly to core rendering internals

Exit criteria:
- you can explain where each new piece of logic belongs (`app`, `render_session`, `core`, `scenes`, `presets`)
- the app shell can request "next render step" through the render session boundary

Common trap:
- bypassing `render_session` because it feels faster in the moment

### Milestone 4: Core Math, Buffers, and Domain Conventions

Goal:
- Establish the basic language of the renderer core.

Do:
- implement minimal math types needed for Phase 1 (no general-purpose math library design)
- define and document the world/camera convention in code comments or notes
- add image and accumulation buffer concepts in the core
- align everything with `f32` and linear color accumulation policy

Exit criteria:
- you have stable core types that do not need to be redesigned every day
- image and accumulation concepts exist in the core layer

Common trap:
- building a large reusable math library instead of a small project-specific one

### Milestone 5: First Visible Render Path (Simplest Possible)

Goal:
- Prove the full pipeline can show a generated image through the app shell.

Do:
- connect the render session to the renderer core
- produce the simplest valid ray-generated image (even if visually basic)
- pass the display image back to the app shell
- show real status values in the overlay (scene, quality, deterministic mode, sample/progress placeholder)

Exit criteria:
- the window shows a renderer-produced image, not only a placeholder
- the full loop (app -> render session -> core -> display) works

Common trap:
- trying to add materials and advanced lighting before the pipeline is stable

### Milestone 6: Progressive Accumulation and Incremental Updates

Goal:
- Make the preview progressively improve over time.

Do:
- implement incremental render steps
- accumulate samples progressively
- update the display image repeatedly while the app stays responsive
- display a meaningful accumulation/sample counter in the overlay

Exit criteria:
- the image visibly refines over time
- the app remains usable while accumulation continues

Common trap:
- doing too much work per update and making the app feel frozen

### Milestone 7: Sphere Geometry + Sky/Background Baseline

Goal:
- Get a stable baseline scene with visible geometry and environment contribution.

Do:
- implement sphere support in the core pipeline
- implement simple sky/background contribution
- create a simple scene in the scene catalog with sphere objects and per-scene camera presets
- verify camera preset switching path exists (even if presets are few at first)

Exit criteria:
- a sphere scene renders in the preview window
- sky/background is visible
- camera preset switching works and resets accumulation

Common trap:
- adding many spheres/materials before one sphere scene is stable

### Milestone 8: Material Models (Diffuse, Specular, Glossy)

Goal:
- Reach the main material behavior targets from Phase 1.

Do:
- add diffuse-like behavior
- add specular/reflective behavior
- add glossy/rough reflective behavior
- create or refine the materials/lighting showcase scene to make differences obvious

Exit criteria:
- the official materials-focused scene clearly shows surface differences
- visual differences are easy to explain

Common trap:
- tuning forever without writing down what changed and why

### Milestone 9: Emissive Lighting and Multi-Bounce Behavior

Goal:
- Show light contribution beyond simple background lighting.

Do:
- add emissive/light-contributing scene behavior
- verify visible indirect/bounce behavior in at least one scene
- confirm progressive refinement is clearly visible in this scene

Exit criteria:
- lighting impact is obvious in the scene
- a short clip or repeated observation proves progressive improvement

Common trap:
- changing too many variables at once while debugging lighting behavior

### Milestone 10: Triangle Geometry and Mixed Geometry Scene

Goal:
- Satisfy the triangle requirement in the same rendering pipeline.

Do:
- add triangle support
- build the official mixed geometry scene (triangles + spheres)
- verify materials behave consistently across geometry types

Exit criteria:
- the mixed geometry scene is stable and clearly proves triangle support
- triangle support is not isolated to a one-off experiment path

Common trap:
- implementing triangles in a way that bypasses shared material/lighting logic

### Milestone 11: DOF Presets and Camera/DOF Study Scene

Goal:
- Complete the camera feature set for Phase 1.

Do:
- implement DOF preset behavior (`Off`, `Subtle`, `Strong`)
- apply DOF as a preset override on top of scene camera presets
- build the official camera/DOF study scene
- verify camera preset and DOF preset changes both trigger full accumulation reset

Exit criteria:
- focus/blur differences are visible and repeatable
- camera and DOF presets are easy to compare in the preview app

Common trap:
- exposing too many freeform camera controls and drifting into editor scope

### Milestone 12: Quality Profiles and Deterministic Mode

Goal:
- Finalize comparison and debugging workflow.

Do:
- define the 3 quality profiles as full render profiles (`Draft`, `Preview`, `Quality`)
- make quality profile switching a runtime preset change
- implement deterministic mode as a launch-time behavior
- verify deterministic comparison behavior for the same scene/presets on the same machine/build

Exit criteria:
- quality profile changes work and reset accumulation
- deterministic mode helps repeat comparisons
- overlay clearly shows current quality profile and deterministic mode

Common trap:
- treating quality profiles as only a label instead of real render profile bundles

### Milestone 13: Official Scene Finalization and Acceptance Proofs

Goal:
- Produce the Phase 1 evidence package.

Do:
- finalize the 3 official scenes:
  - materials and lighting showcase
  - mixed geometry validation
  - camera and DOF study (deterministic benchmark)
- map each scene to requirements coverage in the checklist
- capture screenshots and short clips for official proof (use macOS tools)
- store artifacts under `captures/phase-1/official/` with consistent names

Exit criteria:
- all Phase 1 requirements are covered by the 3 official scenes
- official captures are organized and understandable

Common trap:
- capturing evidence before scene/preset names and checklist mapping are stable

### Milestone 14: Stabilization, Documentation, and Phase 1 Closeout

Goal:
- Finish Phase 1 cleanly and prepare for Phase 2.

Do:
- run through the full Definition of Done checklist
- write the short Phase 1 summary/README update
- write the Phase 1 retrospective (what worked, what hurt, what to improve)
- move deferred ideas into backlog for Phase 2+
- clearly mark any sandbox experiments as non-official

Exit criteria:
- DoD checklist is complete
- docs and captures are organized
- future-you can understand what is done and what is deferred

Common trap:
- starting Phase 2 ideas before closing Phase 1 properly

### 7. Recommended Working Rhythm (Part-Time)

Use short, goal-based sessions.

A good session format:
1. Read current milestone goal (2 minutes)
2. State one visible target for the session (1 minute)
3. Implement only that target
4. Record result in `notes/dev-log.md` (3 to 5 minutes)
5. Decide the next session's first step (1 minute)

This reduces restart friction.

### 8. Daily Dev Log Template (Use This Every Session)

Keep entries short.

Suggested fields:
- Date
- Milestone
- Goal for this session
- What changed
- What worked
- What broke
- Next step
- Deferred ideas (if any)

The dev log is your anti-confusion tool.

### 9. Debugging Workflow (When the Image Looks Wrong)

Use this order:
1. Freeze scope (do not add features)
2. Reproduce in one official scene only
3. Reduce variables (one preset, one camera, one quality level)
4. Use deterministic mode for comparisons
5. Verify one requirement at a time
6. Write down the symptom before changing anything
7. Capture a before/after screenshot if useful

Do not debug by changing many things at once.

### 10. How To Use the Sandbox Experiment Lane Safely

Sandbox lane rules (repeat from design, because this matters):
- one experiment at a time
- time-box it
- mark it as non-official
- stop if it delays the current milestone
- log the result in `notes/backlog.md` or dev-log

Examples of good sandbox use:
- trying a visual tweak briefly
- testing a different scene idea
- exploring a non-required rendering option for learning

Examples of bad sandbox use:
- replacing a core architecture decision mid-phase
- turning a side experiment into a silent requirement

### 11. Scope Change Rule (Very Important)

When a new idea appears, ask:
1. Is it required by `requirements-spec.md`?
2. If no, does it unblock the current milestone?
3. If no, can it wait for sandbox or Phase 2?

If the answer is "it can wait," log it and move on.

This rule will save your project.

### 12. Official Capture Workflow (So Evidence Stays Clean)

Before capturing official screenshots/clips:
- confirm the scene is one of the 3 official scenes
- confirm preset names are stable
- confirm overlay shows useful context
- confirm whether deterministic mode should be on for this capture
- confirm final acceptance captures use `1280x720`

Save captures in:
- `captures/phase-1/official/` for final evidence
- `captures/phase-1/work/` for experiments and debugging

Use consistent names that include:
- scene id/name
- camera preset
- DOF preset
- quality profile
- deterministic marker if relevant

### 13. When You Feel Lost (Recovery Protocol)

If you feel stuck or scattered, do this exactly:

1. Stop coding for 10 minutes.
2. Open these files:
- `docs/phase-1/requirements-spec.md`
- `docs/phase-1/design.md`
- `docs/phase-1/checklist.md`
3. Write one sentence:
- "The current milestone is ..."
4. Write one sentence:
- "The next visible proof I need is ..."
5. Delete or defer anything unrelated to that proof.
6. Resume with a 30-60 minute session target.

This is normal. It is not failure. It is how complex projects are managed.

### 14. How To Use LLMs During Implementation (Without Breaking Your Rule)

Allowed (design/support only):
- review your design choices
- discuss tradeoffs
- help define test scenarios
- help interpret symptoms conceptually
- help rewrite docs/checklists

Not allowed (per your project rule):
- asking for implementation code
- asking for bug fixes as code patches
- asking for line-by-line solutions

A useful prompt pattern:
- "No code. Help me reason about the design/debugging strategy only."

### 15. Phase 1 Exit Checklist (Execution Closeout)

Before declaring Phase 1 complete, confirm:
- all 3 official scenes are finalized
- checklist coverage is complete
- captures are organized
- deterministic comparison proof exists
- retrospective is written
- deferred items are listed for later phases
- you can explain the architecture and reset behavior from memory

If all of the above are true, Phase 1 is genuinely complete.

### 16. What To Do Immediately After Phase 1

Do not jump straight into new features.

Do this first:
1. Review the retrospective
2. Identify the top 3 pain points from Phase 1
3. Use those pain points to write Phase 2 requirements
4. Keep Phase 1 official scenes as regression scenes for later phases

This is how the project compounds instead of restarting every phase.
