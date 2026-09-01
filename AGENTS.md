# femto

## TODO.md

1. When the user completes anything, look for the corresponding feature/todo in the TODO.md, mark the matching `- [ ]` item(s) as done (`- [x]`). Do not add, remove, rewrite, or reorder TODO content unless the user explicitly asks.

## Bug Analysis / Code Review

1. Whenever user asks for a code-level bug-analysis or edge-case detection, always categorise them into HIGH/MEDIUM/LIGHT severity, and mark them in serial (within each section) so it is easy to reference.

## Docs

1. Whenever the user completes a feature, always look for a suitable documentation file in the `docs/` directory (in repo root) where you can update the architecture/flow of the feature. If not present already, create a new file and update there. The docs should be crisp and concise, and can include mermaid diagrams, flowcharts, textual pointers (not very verbose).
2. Always ensure the docs & readme are in sync with the current state of the feature in the code. If needed, update corresponding docs accordingly.
3. Maintain a current_state.md file in docs/ which would contain the latest state of the code (in terms of features, caveats and whatever). Don't add any TODOs here. This file needs to be in sync with the latest changes in the code. So you will have to update it everytime without saying.

## General Notes

1. Always respond concisely; don't be too verbose (unless really important).
2. Never write code unless explicitly specified. Most of the questions to you will be around asking doubts, best-practices, etc.
3. Always check if the repository code is in sync with the documentations, TODOs and the README. Sync them if they're not.
