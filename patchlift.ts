#!/usr/bin/env -S deno run --allow-write --allow-read --allow-run=bash --allow-net --allow-env --allow-sys --no-lock

import { applyToPublicRustCrateDir } from "jsr:@dengorbachev/patchlift@0.1.5"

await applyToPublicRustCrateDir(import.meta.dirname)
