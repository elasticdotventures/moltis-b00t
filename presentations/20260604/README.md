# 2026-06-04 NFC Talk Deck

This folder contains a single-file [Slidev](https://sli.dev) deck for the NFC Summit Lisbon talk.

## Talk

- Title: `AI-Assisted Software Engineering: Building Moltis as a Solo Founder`
- Event: Non Fungible Conference / NFC Summit
- Date: June 4, 2026
- Audience: entrepreneurs, artists, builders, and AI/software beginners

## Files

- `slides.md`: main presentation deck with embedded speaker notes
- `git-stats.rb`: reproduces the Moltis vs Constellations first-14-weeks git activity data used in the deck

## Run Locally

```bash
slidev presentations/20260604/slides.md --open
```

## Export

PDF export:

```bash
slidev export presentations/20260604/slides.md --format pdf
```

Build a static SPA:

```bash
slidev build presentations/20260604/slides.md
```

## Timing

- The core deck is designed for roughly 10 minutes.
- Speaker notes include optional expansion points if the slot stretches toward 20 minutes.
- If time is tight, skip the `How I Work With AI` details and go straight from workflow to takeaways.

## Rebuild Git Stats

```bash
ruby presentations/20260604/git-stats.rb
```

The slide uses raw `git log --numstat` output for commits authored by `Fabien Penso`, normalized to the first 14 weeks of each project.
