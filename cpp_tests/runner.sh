#!/usr/bin/env bash
for f in ./test_*; do
  [[ -f "$f" && -x "$f" ]] && "$f"
done
