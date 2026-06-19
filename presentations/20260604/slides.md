---
theme: none
colorSchema: light
title: "AI-Assisted Software Engineering: Building Moltis as a Solo Founder"
info: |
  Non Fungible Conference / NFC Summit
  Lisbon, June 4, 2026
class: text-left
transition: slide-left
mdc: true
drawings:
  persist: false
---

<div class="min-h-[455px] rounded-[32px] border border-fuchsia-200 bg-gradient-to-br from-fuchsia-50 via-white to-cyan-50 p-6 shadow-sm">
  <div class="flex items-start justify-between gap-6">
    <div class="inline-flex items-center gap-2 rounded-full border border-fuchsia-200 bg-white px-4 py-2 text-sm font-medium text-gray-700">
      <span class="text-fuchsia-700">NFC Summit Lisbon</span>
      <span class="opacity-50">•</span>
      <span>June 4, 2026</span>
    </div>
    <img src="https://www.moltis.org/favicon.svg" alt="Moltis logo" class="h-16 w-16" />
  </div>

  <div class="mt-9 max-w-5xl text-6xl leading-[1.05] font-semibold tracking-tight text-gray-950">
    AI-Assisted<br />Software Engineering
  </div>

  <div class="mt-5 max-w-4xl text-3xl leading-10 text-gray-700">
    Building Moltis as a solo founder
  </div>

  <div class="mt-9 text-3xl font-semibold text-gray-900">
    Fabien Penso · https://pen.so · @fabienpenso
  </div>

  <div class="mt-3 text-2xl leading-9 text-gray-700">
    https://www.moltis.org
  </div>
</div>

<!--
About 45 seconds.

Open with the audience, not the tool.

Suggested opening:
"I'm not here to convince you that AI can write code. You already know that.
The more useful question for entrepreneurs is: what changes when one person can
operate with the output of a small engineering team, and what still does not
change?"

Optional NFC bridge:
NFC is about digital culture becoming real in physical space. This talk is the
engineering version of that: AI is no longer a demo window, it is changing how
real products are built.
-->
---

# Who I am

<div class="mt-2 text-lg leading-7 text-gray-600">
  I have been building internet products, infrastructure, and developer tools since the late 1990s.
</div>

<div class="mt-5 grid grid-cols-3 gap-4">
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-4 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-fuchsia-700">Founder / CTO</div>
    <div class="mt-2 text-xl font-semibold leading-tight text-gray-950">LinuxFr, Stuart, Kard</div>
    <div class="mt-3 text-sm leading-6 text-gray-700">Open-source communities, logistics, and fintech products used at scale.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Principal engineer</div>
    <div class="mt-2 text-xl font-semibold leading-tight text-gray-950">Beam, Constellations, Dango</div>
    <div class="mt-3 text-sm leading-6 text-gray-700">Rust systems, encrypted sync, Cosmos infrastructure, exchanges, and production indexing for Stargaze.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Solo builder</div>
    <div class="mt-2 text-xl font-semibold leading-tight text-gray-950">Constellations -> Moltis</div>
    <div class="mt-3 text-sm leading-6 text-gray-700">Two serious solo Rust projects: before and after daily AI-assisted engineering.</div>
  </div>
</div>

<div class="mt-5 rounded-2xl border border-gray-200 bg-white px-5 py-3 text-lg leading-7 shadow-sm">
  This talk comes from day-to-day AI-assisted work on software I have to maintain and ship. Later this month I am joining Microsoft AI and moving to California.
</div>

<!--
About 60 seconds.

Sources: https://pen.so/work/ and cv_en.pdf.

Keep it brief: this is credibility, not a biography.

Useful line:
"The important context is that I built production systems before AI was useful,
so I have a baseline for what solo engineering used to feel like."
-->
---

# What Moltis is

<div class="mt-1 text-base leading-6 text-gray-600">
  A <strong>local-first persistent personal agent server</strong>: one Rust binary between you, your tools, your memory, your channels, and multiple LLM providers.
</div>

<div class="mt-3 grid grid-cols-4 gap-3">
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">GitHub stars</div>
    <div class="mt-1 text-4xl font-semibold text-gray-950">2.7K</div>
    <div class="mt-1 text-sm text-gray-600">moltis-org/moltis</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Issues</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">367</div>
    <div class="mt-1 text-sm text-gray-600">49 open · 318 closed</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Pull requests</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">679</div>
    <div class="mt-1 text-sm text-gray-600">12 open · 667 closed</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Merged PRs</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">566</div>
    <div class="mt-1 text-sm text-gray-600">shipping velocity</div>
  </div>
</div>

<div class="mt-3 grid grid-cols-3 gap-3">
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Gateway</div>
    <div class="mt-1 text-xl font-semibold text-gray-950">One place for agents</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Streaming chat, provider routing, coding agents, tools, sessions, and APIs.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Memory</div>
    <div class="mt-1 text-xl font-semibold text-gray-950">Persistent context</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Durable sessions, long-term memory, workspace files, project context, hooks, and cron.</div>
  </div>
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-fuchsia-700">Control</div>
    <div class="mt-1 text-xl font-semibold text-gray-950">Runs on your machine</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Password/passkey auth, encrypted vault, sandboxing, local data, and self-hosted deploys.</div>
  </div>
</div>

<!--
About 75 seconds.

Sources: README feature list, How It Works section, and GitHub live counts via `gh`.

GitHub counts checked during prep:
- 2,719 stars
- 49 open issues, 318 closed issues
- 12 open PRs, 667 closed PRs, 566 merged PRs

Useful spoken version:
"Moltis is not just a chat UI. It is the server I wanted between me and AI
models: persistent sessions, memory, tools, channels, auth, sandboxing, and
deployment. And it is not a private toy anymore: thousands of stars, hundreds of
issues and PRs, and real user pressure. The key idea is continuity."
-->
---

# Before AI: Constellations was not a toy

<div class="mt-2 text-base text-gray-600">
  A production Cosmos indexer I built before the current AI-assisted workflow.
</div>

<div class="mt-4 grid grid-cols-3 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-gray-500">Workload</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">3,500h</div>
    <div class="mt-1 text-sm text-gray-600">about 2 years full-time</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-gray-500">Codebase</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">118K</div>
    <div class="mt-1 text-sm text-gray-600">current tokei LoC</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-gray-500">Traffic</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">15M+</div>
    <div class="mt-1 text-sm text-gray-600">Stargaze requests/day</div>
  </div>
</div>

<div class="mt-4 grid grid-cols-2 gap-4">
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-5 py-3">
    <div class="text-lg font-semibold text-gray-950">What it did</div>
    <div class="mt-2 text-base leading-6 text-gray-700">
      Indexed CosmWasm contracts and on-chain data for Stargaze, Osmosis, Neutron, Noble, dYdX, Kujira, and more.
    </div>
  </div>
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-5 py-3">
    <div class="text-lg font-semibold text-gray-950">Why it mattered</div>
    <div class="mt-2 text-base leading-6 text-gray-700">
      Stargaze pages went from loading in <strong>&gt;30 seconds</strong> to <strong>&lt;500ms</strong> through a real-time API.
    </div>
  </div>
</div>

<div class="mt-4 rounded-2xl border border-gray-200 bg-white px-6 py-3 text-lg leading-6 shadow-sm">
  This is my baseline for "before AI": a serious solo Rust infrastructure project, running real production traffic.
</div>

<!--
About 75 seconds.

Sources: your Constellations talks, devmos_2024.pdf, constellations_nebular.pdf, and current `tokei` for LoC.

Use this slide to make the comparison credible:
- Constellations was not a weekend toy or a small demo.
- It was production infrastructure for Stargaze and other Cosmos chains.
- It had meaningful traffic and hard operational constraints.

Useful line:
"So when I compare Moltis to Constellations, I am not comparing AI work to a
toy. I am comparing two serious solo Rust projects, one mostly before AI and
one built with AI as part of the daily engineering workflow."
-->
---

<div class="px-10 pb-16">
  <div class="text-4xl font-semibold tracking-tight text-gray-950">What changed in output</div>
  <div class="mt-2 max-w-4xl text-base leading-6 text-gray-600">
    Same person. Same solo-founder pattern. Plain <code>tokei</code> code lines in checked-out repositories.
  </div>
  <div class="mt-4 rounded-3xl border border-gray-200 bg-white px-7 py-4 shadow-sm">
    <div class="flex items-center justify-between">
      <div>
        <div class="text-lg font-semibold text-gray-950">Codebase size by elapsed project time</div>
        <div class="mt-1 text-sm text-gray-500">Same 18-week window: <span class="font-semibold text-fuchsia-700">26.1x more LoC</span> with Moltis</div>
      </div>
      <div class="text-xs uppercase tracking-[0.18em] text-gray-500">linear scale</div>
    </div>
    <div class="mt-4 space-y-4">
      <div>
        <div class="mb-2 flex items-baseline justify-between gap-4">
          <div><span class="font-semibold text-fuchsia-700">Moltis</span><span class="ml-2 text-sm text-gray-500">after 18 weeks with AI</span></div>
          <div class="text-2xl font-semibold text-fuchsia-700">474.6K LoC</div>
        </div>
        <div class="h-7 rounded-full bg-fuchsia-100"><div class="h-7 rounded-full bg-fuchsia-600" style="width: 100%"></div></div>
      </div>
      <div>
        <div class="mb-2 flex items-baseline justify-between gap-4">
          <div><span class="font-semibold text-gray-700">Constellations</span><span class="ml-2 text-sm text-gray-500">after the same 18 weeks</span></div>
          <div class="text-2xl font-semibold text-gray-800">18.2K LoC</div>
        </div>
        <div class="h-7 rounded-full bg-gray-100"><div class="h-7 rounded-full bg-gray-400" style="width: 3.8%"></div></div>
      </div>
      <div>
        <div class="mb-2 flex items-baseline justify-between gap-4">
          <div><span class="font-semibold text-gray-700">Constellations</span><span class="ml-2 text-sm text-gray-500">after 126 weeks of solo work</span></div>
          <div class="text-2xl font-semibold text-gray-800">118.2K LoC</div>
        </div>
        <div class="h-7 rounded-full bg-gray-100"><div class="h-7 rounded-full bg-gray-500" style="width: 24.9%"></div></div>
      </div>
    </div>
    <div class="mt-3 grid grid-cols-5 text-xs text-gray-500">
      <div>0</div><div class="text-center">100K</div><div class="text-center">200K</div><div class="text-center">300K</div><div class="text-right">475K</div>
    </div>
  </div>
</div>

<!--
About 90 seconds.

LoC numbers are plain `tokei` code totals.

Data source:
- Moltis current checkout: 474,550 code lines after 127 days / 18 weeks
- Constellations detached worktree at the same elapsed time from 2022-07-01: 18,184 code lines
- Constellations current checkout: 118,225 code lines after 882 days / 126 weeks
- Same-time factor: 474,550 / 18,184 = 26.1x

Be careful with the interpretation:
- LoC is not quality
- tokei counts code-like assets too, which is intentional here because the project includes Swift, TeX, scripts, UI code, etc.
- the real claim is throughput and breadth, not that lines are the goal

Good spoken version:
"This is not a scoreboard for virtue. More lines can be bad. But as a solo
founder, the difference is real: I can now move across frontend, backend, tests,
docs, auth, CI, and release work in a way that used to take much longer."
-->
---

# Rewrites are no longer sacred

<div class="mt-2 text-xl leading-8 text-gray-600">
  The cost of generating code is collapsing. The cost that remains is deciding, verifying, and maintaining it.
</div>

<div class="mt-4 flex items-center gap-3">
  <div class="inline-flex rounded-full border border-gray-200 bg-white px-4 py-2 text-base font-semibold text-gray-700">
    Bun: Zig → Rust
  </div>
  <div class="inline-flex rounded-full border border-cyan-200 bg-cyan-50 px-4 py-2 text-base font-semibold text-cyan-800">
    first → last commit: 9d 18h
  </div>
</div>

<div class="mt-4 grid grid-cols-3 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Bun PR #30412</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">6,755</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">commits merged into main for the Zig → Rust rewrite.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Bun PR #30412</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">1M</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">lines rewritten across 2,188 changed files.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-5 py-3">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Test suite</div>
    <div class="mt-2 text-4xl font-semibold text-gray-950">99.8%</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">of Bun's pre-existing tests reportedly passing.</div>
  </div>
</div>

<div class="mt-4 rounded-2xl border border-cyan-200 bg-cyan-50 px-6 py-3 text-base leading-6 text-gray-700">
  Jarred Sumner wrote that the port fixed memory leaks and flaky tests, and gave Bun compiler-assisted tools for memory bugs that had cost enormous debugging time.
</div>

<div class="mt-4 rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-6 py-3 text-lg leading-7">
  When code is cheap, rewriting becomes a product decision, not an engineering impossibility.
</div>

<!--
About 45 seconds.

Sources:
- https://github.com/oven-sh/bun/pull/30412
- https://www.heise.de/en/news/AI-Porting-Claude-Rewrites-Bun-Codebase-in-Rust-11294318.html

Commit graph checked with `git fetch --filter=blob:none` from oven-sh/bun:
- 6,755 commits in PR diff from base 0d9b296af33f2b851fcbf4df3e9ec89751734ba4 to head ed1a70f81708d7d137de8de057d11668c5f4e220
- first commit: 2026-05-04T13:20:04Z
- last commit: 2026-05-14T00:23:14-07:00 / 2026-05-14T07:23:14Z
- duration: 9 days, 18 hours, 3 minutes

PR wording from Jarred Sumner:
"It passes Bun's pre-existing test suite on all platforms (and fixes several memory leaks and flaky tests) ... most importantly, we now have compiler-assisted tools for catching & preventing memory bugs, which have costed the team an enormous amount of development & debugging time over the years."

Use this carefully:
- The astonishing point is not that AI made a perfect rewrite.
- The point is that a rewrite of this size became thinkable, testable, and mergeable on a timescale that would have sounded absurd a few years ago.
- The cost did not disappear; it moved from typing code to validation, migration, and ownership.
-->
---

# My actual workflow

<div class="mt-7 grid grid-cols-2 gap-6">
  <div class="rounded-3xl border border-fuchsia-200 bg-fuchsia-50 px-6 py-5 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-fuchsia-700">Human loop</div>
    <div class="mt-4 space-y-3 text-xl leading-8 text-gray-800">
      <div>1. Define the user outcome</div>
      <div>2. Ask AI to inspect the codebase first</div>
      <div>3. Make the smallest correct change</div>
      <div>4. Review every diff</div>
      <div>5. Run tests, linters, and builds</div>
      <div>6. Ship, then listen to users</div>
    </div>
  </div>
  <div class="space-y-4">
    <div class="rounded-3xl border border-cyan-300 bg-cyan-50 px-6 py-5 shadow-sm">
      <div class="text-sm uppercase tracking-[0.18em] text-cyan-700">Collaboration</div>
      <div class="mt-3 text-2xl font-semibold text-gray-950">Share prompts, not patches</div>
      <div class="mt-3 text-base leading-7 text-gray-700">
        Other solo founders ask similar things. A prompt is faster to share than a PR, and safer because I can rerun it through my guardrails.
      </div>
    </div>
    <div class="rounded-3xl border border-cyan-300 bg-cyan-50 px-6 py-5 shadow-sm">
      <div class="text-sm uppercase tracking-[0.18em] text-cyan-700">AI workflow</div>
      <div class="mt-3 text-2xl font-semibold text-gray-950">I barely touch the IDE</div>
      <div class="mt-3 text-base leading-7 text-gray-700">
        Most changes start from agent sessions now. I open the IDE only a few times a week.
      </div>
    </div>
  </div>
</div>

<!--
About 90 seconds.

This is the practical center of the talk.

For newbies, translate:
- "inspect the codebase" means do not ask it to invent architecture blind
- "smallest correct change" means avoid giant rewrites
- "review every diff" means look at the code it changed before trusting it
- "tests" means executable proof that behavior still works
- sharing prompts is useful because collaborators can rerun the same intent in
  their own checkout, with their own validation, instead of trusting a patch
- not touching the IDE is not laziness; it means the main interface moved from
  typing code to delegating work, reviewing diffs, and running gates

If you need a simple prompt example:
Bad: "Build login."
Better: "Add password reset to the existing auth system. Reuse the current
credential store, add expiry, add tests for invalid and expired tokens, and do
not change existing login routes."
-->
---

# My parallel issue workflow

<div class="mt-2 max-w-5xl text-lg leading-7 text-gray-600">
  The goal is not one giant AI session. It is many isolated, reviewable workstreams.
</div>

<div class="mt-4 grid grid-cols-3 gap-3">
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-fuchsia-700">Queue</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">Issues first</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Bugs first, because I want Moltis to be stable.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Isolation</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">One workspace per PR</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Separate workspace for each issue or pull request.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Agents</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">OpenCode + Claude</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Both 20x memberships, more than 10B tokens/month.</div>
  </div>
</div>

<div class="mt-3 grid grid-cols-3 gap-3">
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Adversarial review</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">Agents check agents</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">One model reviews another for fixes or simpler approaches.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">External reviewer</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">Greptile on GitHub</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Agents work through feedback until it reaches 5/5.</div>
  </div>
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-4 py-2 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-fuchsia-700">Human owner</div>
    <div class="mt-1 text-lg font-semibold text-gray-950">Fast final review</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">I review, merge, then E2E and release gates catch regressions.</div>
  </div>
</div>

<div class="mt-3 rounded-2xl border border-cyan-300 bg-cyan-50 px-5 py-2 text-base leading-6 shadow-sm">
  This keeps multiple issues moving at once. I am building <strong>Polyphony</strong> to automate more of this: <span class="text-cyan-700">https://polyphony.to</span>
</div>

<!--
About 90 seconds.

Make clear that this is not "merge whatever AI writes".
The unit of work is still an issue/PR, but each one gets an isolated workspace,
agent pass, adversarial review if useful, Greptile review, CI, and human review.
Mention that this is token-heavy: both 20x memberships, more than 10B tokens/month.
After merge, release E2E and provider gates are another safety net before users get a release.

Useful line:
"The parallelism is not in my attention. The parallelism is in setup, execution,
and review loops that are isolated enough to be safe."
-->
---

# Add determinism around the AI

<div class="mt-3 max-w-5xl text-xl leading-8 text-gray-600">
  AI is non-deterministic. Surround it with deterministic systems so bad output cannot land quietly.
</div>

<div class="mt-7 grid grid-cols-3 gap-4">
  <div class="rounded-3xl border border-gray-200 bg-white px-5 py-5 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-gray-500">Old lesson</div>
    <div class="mt-3 text-2xl font-semibold tracking-tight text-gray-950">Stop debating taste</div>
    <div class="mt-4 text-base leading-7 text-gray-700">
      Spaces vs tabs taught me: put RuboCop, formatters, and linters in CI, then move on.
    </div>
  </div>
  <div class="rounded-3xl border border-cyan-200 bg-cyan-50 px-5 py-5 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-cyan-700">New lesson</div>
    <div class="mt-3 text-2xl font-semibold tracking-tight text-gray-950">Bound the agent</div>
    <div class="mt-4 text-base leading-7 text-gray-700">
      In Moltis, CI is the immune system that keeps AI output from drifting away from the project.
    </div>
  </div>
  <div class="rounded-3xl border border-gray-200 bg-white px-5 py-5 shadow-sm">
    <div class="text-sm uppercase tracking-[0.18em] text-gray-500">Language choice</div>
    <div class="mt-3 text-2xl font-semibold tracking-tight text-gray-950">Rust helps AI</div>
    <div class="mt-4 text-base leading-7 text-gray-700">
      Types, ownership, and memory safety turn whole classes of mistakes into compiler errors.
    </div>
  </div>
</div>

<!--
About 75 seconds.

Tell this as a practical story:
- In the Ruby world, RuboCop removed endless subjective arguments.
- CI made the rule impersonal: the build failed, not a teammate's taste.
- AI brings the same problem back at larger scale: it can produce plausible code
  that slowly violates style, architecture, security, or project direction.
- Rust is a great language for AI-assisted coding because the compiler catches
  structural mistakes early. Microsoft has said roughly 70% of security bugs
  addressed annually were memory-safety issues, which Rust is designed to avoid.
  Source: https://www.zdnet.com/article/microsoft-70-percent-of-all-security-bugs-are-memory-safety-issues/

Good spoken line:
"I don't trust the model to remember my standards. I encode my standards where
the model has to pass them."
-->
---

# Moltis CI is the immune system

<div class="mt-3 max-w-5xl text-xl leading-8 text-gray-600">
  These gates turn taste, architecture, and release risk into executable constraints.
</div>

<div class="mt-7 grid grid-cols-3 gap-4">
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Shape</div>
    <div class="mt-1 text-sm leading-6 text-gray-700">rustfmt, Biome, i18n parity, install docs sync, 1,500-line file limit.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Rust behavior</div>
    <div class="mt-1 text-sm leading-6 text-gray-700">Clippy with <code>-D warnings</code>, nextest, coverage, contract tests.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Frontend</div>
    <div class="mt-1 text-sm leading-6 text-gray-700">TypeScript check, Vite build, Playwright E2E, no JS-error assertions.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Integrations</div>
    <div class="mt-1 text-sm leading-6 text-gray-700">Live provider tests, provider E2E scenarios, OpenAI/Ollama/sandbox E2E.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Release gates</div>
    <div class="mt-1 text-sm leading-6 text-gray-700">Provider release checks, package builds, tag validation, changelog guard.</div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-4 py-4">
    <div class="text-base font-semibold text-gray-950">Local parity</div>
    <div class="mt-1 text-sm leading-6 text-gray-700"><code>local-validate.sh</code> publishes PR statuses: fmt, lint, test, E2E, macOS, iOS.</div>
  </div>
</div>

<div class="mt-7 rounded-2xl border border-fuchsia-300 bg-fuchsia-50 px-6 py-4 text-xl leading-8 shadow-sm">
  The AI can generate code quickly. CI decides whether the project accepts it.
</div>

<!--
About 75 seconds.

Concrete Moltis examples:
- rustfmt, Biome, TypeScript, i18n parity
- Clippy -D warnings, nextest, coverage, contract tests
- Playwright frontend E2E, provider live/E2E scenarios, sandbox runtime E2E
- 1,500-line Rust/TS/TSX file-size limit
- release provider gates, package/release checks, changelog guard
- local-validate.sh mirrors CI and publishes commit statuses on the PR
-->
---

<div class="text-4xl font-semibold tracking-tight text-gray-950">Release safety: what users install</div>

<div class="mt-2 max-w-5xl text-lg leading-7 text-gray-600">
  Faster code increases the blast radius of a bad release. The artifact needs proof attached to it.
</div>

<div class="mt-5 grid grid-cols-2 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Signatures</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Sigstore + my GPG key</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Keyless CI signatures plus detached maintainer GPG signatures for release assets.
    </div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Integrity</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Checksums + verification</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      SHA256/SHA512 checksums; <code>verify-release.sh</code> pins my GPG fingerprint.
    </div>
  </div>
</div>

<div class="mt-4 grid grid-cols-2 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">SBOM</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">CycloneDX + SPDX</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Release SBOMs are generated, signed, checksumed, uploaded, and attested.
    </div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Attestations</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">GitHub provenance</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Artifacts, container images, and SBOMs link back to the workflow run.
    </div>
  </div>
</div>

<!--
About 75 seconds.

Concrete Moltis details:
- .github/actions/sign-artifacts creates SHA256/SHA512 and Sigstore keyless .sig/.crt files.
- Release workflow grants id-token: write only where needed for Sigstore signing.
- gpg-sign-release.sh downloads release assets, verifies SHA256 where present, and uploads detached .asc signatures.
- verify-release.sh pins the maintainer GPG fingerprint before importing the key from https://pen.so/gpg.asc.
- cargo-sbom generates CycloneDX and SPDX SBOMs; they are signed and attested too.
- actions/attest-build-provenance links release artifacts, containers, and SBOMs back to GitHub Actions.
-->
---

<div class="text-4xl font-semibold tracking-tight text-gray-950">Release safety: where it came from</div>

<div class="mt-2 max-w-5xl text-lg leading-7 text-gray-600">
  Supply-chain safety is not only signatures. It is also protecting the factory and recording source provenance.
</div>

<div class="mt-5 grid grid-cols-2 gap-4">
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-fuchsia-700">CI protection</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Zizmor guards workflows</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      CI, release, docs, Homebrew, and benchmark workflows are scanned for attack paths.
    </div>
  </div>
  <div class="rounded-2xl border border-cyan-200 bg-cyan-50 px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-cyan-700">Release gates</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Tests before packages</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Clippy, tests, E2E, and live provider checks must pass before packaging.
    </div>
  </div>
</div>

<div class="mt-4 grid grid-cols-2 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Source provenance</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Pin GitHub origins</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Skills/plugins pin GitHub commit SHAs; source drift requires re-trust.
    </div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-4 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">Audit trail</div>
    <div class="mt-2 text-xl font-semibold text-gray-950">Leave evidence behind</div>
    <div class="mt-2 text-sm leading-6 text-gray-700">
      Trust, install, source-drift, enable/disable, and dependency events are logged.
    </div>
  </div>
</div>

<!--
About 75 seconds.

Concrete Moltis details:
- Zizmor runs on CI, release, docs, Homebrew, and Codspeed workflows.
- Release gates wait for clippy, test, e2e, and provider integration results.
- Third-party skills/plugins persist pinned source commit SHAs, resolve GitHub metadata, and require re-trust on source drift.
- Security audit JSONL logs trust/source/install changes.
-->
---

# How I talk to the AI

<div class="mt-8 grid grid-cols-2 gap-6">
  <div class="rounded-2xl border border-gray-200 px-6 py-5">
    <div class="text-xl font-semibold text-gray-500">Weak</div>
    <div class="mt-4 rounded-xl bg-gray-100 p-4 text-xl font-mono text-gray-700">
      Build auth
    </div>
  </div>
  <div class="rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-6 py-5">
    <div class="text-xl font-semibold text-fuchsia-700">Useful</div>
    <div class="mt-4 rounded-xl bg-white p-4 text-base leading-7 font-mono text-gray-800">
      Add password reset using the existing auth middleware. Persist tokens in the current credential store. Add tests for expiry and invalid tokens. Keep existing routes compatible.
    </div>
  </div>
</div>

<div class="mt-10 text-2xl font-medium">
  Prompting is delegation. Context is management.
</div>

<!--
About 75 seconds.

This is for the entrepreneurs/newbies. Make it concrete.

Say:
"The better you can describe constraints, the better the result. Not because
prompting is mystical, but because delegation without context is bad management."

Optional expansion:
Give three context buckets:
- what user outcome you want
- what constraints must not be broken
- how success will be verified
-->
---

# The PR workflow is the next bottleneck

<div class="mt-1 max-w-5xl text-base leading-6 text-gray-600">
  AI did not just make typing faster. It changed the volume, shape, and cadence of software work.
</div>

<div class="mt-4 grid grid-cols-3 gap-4">
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">GitHub Octoverse 2025</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">65M -> 82M</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">monthly code pushes from 2024 to 2025, a <strong>25% jump</strong>.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">GitHub Octoverse 2025</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">39.5M -> 47.5M</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">monthly PRs created from 2024 to 2025, <strong>+20.4% YoY</strong>.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-5 py-3 shadow-sm">
    <div class="text-xs uppercase tracking-[0.18em] text-gray-500">GitHub workflow report</div>
    <div class="mt-1 text-3xl font-semibold text-gray-950">11.5B</div>
    <div class="mt-1 text-sm leading-5 text-gray-700">Actions minutes running tests in 2025, <strong>+35%</strong> year over year.</div>
  </div>
</div>

<div class="mt-2 text-xs leading-4 text-gray-500">
  Source: GitHub Octoverse 2025 and GitHub workflow report. Supplemental GitHub search snapshot: 2026 PR creation was 2.55x the same May week in 2025.
</div>

<div class="mt-3 rounded-3xl border border-fuchsia-200 bg-fuchsia-50 px-6 py-3 shadow-sm">
  <div class="flex items-center justify-between gap-5">
    <div>
      <div class="text-lg font-semibold text-gray-950">The old loop was designed for scarce code</div>
      <div class="mt-1 text-sm leading-5 text-gray-700">Issue → branch → PR → human review → merge works when changes are relatively expensive.</div>
    </div>
    <div class="rounded-2xl bg-gray-950 px-4 py-2 text-right text-white">
      <div class="text-xs uppercase tracking-[0.18em] text-gray-300">new problem</div>
      <div class="mt-1 text-lg font-semibold">attention scarcity</div>
    </div>
  </div>
</div>

<div class="mt-3 rounded-2xl border border-cyan-200 bg-cyan-50 px-5 py-2 text-base leading-6 text-gray-700">
  The tooling of the last 20 years needs to be rethought: review, ownership, validation, and product intent have to become more continuous than a GitHub PR page.
</div>

<!--
About 90 seconds.

Official public sources:
- GitHub Octoverse 2025: https://github.blog/news-insights/octoverse/octoverse-a-new-developer-joins-github-every-second-as-ai-leads-typescript-to-1/
- GitHub workflow report: https://github.blog/news-insights/octoverse/what-986-million-code-pushes-say-about-the-developer-workflow-in-2025/

Official visible numbers:
- Code pushes monthly average: 65M in 2024, 82.19M in 2025, +25.1% YoY.
- Pull requests created monthly average: 39.5M in 2024, 47.5M in 2025, +20.4% YoY.
- Pull requests merged monthly average: 35M in 2024, 43.2M in 2025, +23% YoY.
- Tests used 11.5B GitHub Actions minutes in 2025, +35% YoY.
- GitHub explicitly says these are observational signals, not causal claims.

Supplemental snapshot from GitHub GraphQL search API, queried 2026-06-03:
- type:pr created:2024-05-26..2024-06-01 => 1,100,133
- type:pr created:2025-05-26..2025-06-01 => 1,338,866
- type:pr created:2026-05-26..2026-06-01 => 3,413,998
- 2026 / 2025 = 2.55x

Be precise when speaking:
"This is not proof that AI caused every extra PR. But it is enough to show the
direction: the rate of software change is rising, and the review interface we
use was built for a slower world."
-->
---

# What changed for me as a solo founder

<div class="mt-8 grid grid-cols-3 gap-6">
  <div class="rounded-2xl border border-gray-200 px-5 py-5">
    <div class="text-xl font-semibold text-fuchsia-700">More surface area</div>
    <div class="mt-3 leading-7">I can touch frontend, backend, docs, tests, CI, and releases in one session.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 px-5 py-5">
    <div class="text-xl font-semibold text-fuchsia-700">Faster recovery</div>
    <div class="mt-3 leading-7">When I hit a compiler error or broken build, I lose less time getting unstuck.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 px-5 py-5">
    <div class="text-xl font-semibold text-fuchsia-700">Higher standards</div>
    <div class="mt-3 leading-7">Because code is cheaper, tests, docs, and polish are less optional.</div>
  </div>
</div>

<div class="mt-10 rounded-2xl border border-fuchsia-300 bg-fuchsia-50 px-6 py-5 text-xl leading-8">
  The job becomes less "can I build this?" and more "is this worth building, and can I keep it coherent?"
</div>

<!--
About 85 seconds.

This is a founder lesson, not a coding lesson.

Useful phrasing:
"AI gave me more hands, but not more strategy. It increased my throughput, and
that made prioritization more important, not less."

If expanding to 20 minutes, talk about how this changes hiring timing: you may
delay some engineering hires, but you still need taste, sales, support, and
eventually people who own areas deeply.
-->
---

# Beginner rules that actually help

<div class="mt-7 grid grid-cols-2 gap-x-10 gap-y-4 text-xl leading-8">
  <div>1. Learn enough code to review the output</div>
  <div>2. Keep tasks small</div>
  <div>3. Use version control from day one</div>
  <div>4. Ask AI to explain the change</div>
  <div>5. Run the app and test the unhappy paths</div>
  <div>6. Never paste secrets casually</div>
  <div>7. Prefer boring architecture</div>
  <div>8. Let users, not demos, judge progress</div>
</div>

<div class="mt-8 rounded-2xl border border-cyan-300 bg-cyan-50 px-6 py-4 text-xl leading-8">
  If you cannot tell whether the result is good, slow down. The tool is not the adult in the room.
</div>

<div class="mt-4 rounded-2xl border border-fuchsia-200 bg-fuchsia-50 px-6 py-3 text-base leading-6 text-gray-700">
  AI is great for builders, but it also exposes the next bottlenecks: good ideas, distribution, taste, and stamina. Coding was never the only hard part.
</div>

<!--
About 90 seconds.

This is the most actionable slide for non-engineers.

Expand any two:
- version control lets you undo and compare
- unhappy paths are where real products break
- boring architecture is good because AI can maintain common patterns better
- secrets include API keys, wallet keys, database passwords, tokens

For the NFC audience, mention that if your product touches wallets, identity,
payments, or private communities, the cost of careless AI use is much higher.

Optional aside:
"For years, non-coders said coders were lucky because we could turn ideas into
software. Now more people can do that, and many will discover the bottleneck was
not only code. It is distribution, judgment, product taste, and whether you can
keep going without burning out. AI is amazing for builders, but it also changes
the emotional relationship with coding for people who loved doing it by hand."
-->
---

# Thank you

<div class="mt-16 text-center">
  <div class="text-5xl font-semibold text-gray-950">Moltis</div>
  <div class="mt-6 text-3xl text-fuchsia-700">https://www.moltis.org</div>
  <div class="mt-4 text-2xl text-gray-700">https://github.com/moltis-org/moltis</div>
  <div class="mt-10 text-2xl font-medium text-gray-900">Fabien Penso · https://pen.so · @fabienpenso</div>
</div>

<!--
Backup / Q&A slide.

If people ask what Moltis is, use the short answer:
"Moltis is a local-first persistent agent server: one place where your AI agent
can keep sessions, memory, tools, and channels under your control."
-->
---

# Appendix: if the slot becomes 20 minutes

<div class="mt-8 grid grid-cols-2 gap-6">
  <div class="rounded-2xl border border-gray-200 bg-white px-6 py-5 shadow-sm">
    <div class="text-xl font-semibold text-fuchsia-700">Show a real workflow</div>
    <div class="mt-3 text-base leading-7 text-gray-700">Issue, isolated workspace, agent pass, review feedback, tests, merge.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-6 py-5 shadow-sm">
    <div class="text-xl font-semibold text-fuchsia-700">Open Moltis briefly</div>
    <div class="mt-3 text-base leading-7 text-gray-700">Use it only to make the talk concrete, not as a product tour.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-6 py-5 shadow-sm">
    <div class="text-xl font-semibold text-fuchsia-700">Security guardrails</div>
    <div class="mt-3 text-base leading-7 text-gray-700">Explain why CI, signatures, SBOMs, and provenance matter more when code moves faster.</div>
  </div>
  <div class="rounded-2xl border border-gray-200 bg-white px-6 py-5 shadow-sm">
    <div class="text-xl font-semibold text-fuchsia-700">Audience Q&A</div>
    <div class="mt-3 text-base leading-7 text-gray-700">Ask what they are building and where AI currently breaks their workflow.</div>
  </div>
</div>

<!--
Backup only. Skip this in a 15-minute slot.
-->
