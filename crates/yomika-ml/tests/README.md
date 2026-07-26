# Yomika ML tests

To run test locally if you have CUDA enabled GPU, use the following command:

```bash
# llm tests only
bun run cargo test --package yomika-ml --test llm --features cuda
# all
bun run cargo test --package yomika-ml --features cuda
```