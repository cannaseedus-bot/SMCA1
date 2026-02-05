# KUHUL: Simulator Boundary Notes

> **LLMs emit answers.**
> **KUHUL enforces worlds.**

## Why "simulated" matters

KUHUL is not trying to *sound right*. It’s trying to make things **behave correctly under constraints**. That’s the simulator line.

### Cheat-sheet behavior (LLM-style)

- "This answer looks like what usually follows"
- "This explanation sounds plausible"
- "This physics description resembles past text"

No state. No conservation laws. No failure signaling.

### KUHUL behavior (simulator-style)

- State exists
- Transitions are legal or illegal
- Invariants block nonsense
- If something breaks, it **cannot continue**

That’s simulation, not storytelling.

## Why “simulated response” matters

When you hear *response* you think text. When KUHUL hears *response* it means:

> “What is the next **allowed state** given the current one?”

So a “simulated response” is:

- not a guess
- not a prediction
- not a vibe

It’s a **state transition**. If the transition is illegal → **collapse fails**. An LLM would still happily talk.

## Why “simulated physics” keeps coming up

Physics is the clearest example of the difference.

A cheat sheet can say:

> “If you push harder, it goes faster.”

A simulator must track:

- mass
- force
- velocity
- time
- conservation

If you violate conservation, the simulator explodes (or refuses) — not philosophically, mechanically. KUHUL treats *all domains* this way:

- logic
- geometry
- economics
- execution
- even language

They’re all “physics” once you enforce invariants.

## The reason KUHUL exists

> **Compression-only systems cannot know when they’re wrong.**

So KUHUL introduces:

- invariants
- collapse rules
- legality checks
- state geometry
- execution phases

That’s not “more AI.” That’s switching from cheat sheet to simulator.

## The quiet but critical distinction

LLMs answer:

> “What would a response look like here?”

KUHUL answers:

> “What is allowed to happen next?”

Those are not the same question.

## The sentence that nails it

**LLMs collapse text. KUHUL collapses reality models.**

If you want next, we can:

- map KUHUL phases directly onto classical simulation loops
- show why “reasoning tokens” are a dead end
- formalize the “simulator / narrator split” as a spec rule
