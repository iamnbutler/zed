//! Benchmarks for text input operations.
//!
//! These benchmarks measure the core string manipulation operations used by InputState,
//! without requiring the full GPUI test infrastructure. This lets us measure the
//! performance impact of different implementation strategies.

use criterion::{
    BatchSize, BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main,
};
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::ops::Range;

/// Generate random ASCII text for benchmarking.
fn generate_random_text(rng: &mut StdRng, len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789 \n";
    (0..len)
        .map(|_| CHARS[rng.random_range(0..CHARS.len())] as char)
        .collect()
}

/// Generate text with Unicode characters (CJK, emoji, combining marks).
fn generate_unicode_text(rng: &mut StdRng, len: usize) -> String {
    let chars = [
        'a', 'b', 'c', ' ', '\n', // ASCII
        'é', 'ñ', 'ü', // Latin extended
        '日', '本', '語', // CJK
        '😀', '👋', '🎉', // Emoji
    ];
    (0..len)
        .map(|_| chars[rng.random_range(0..chars.len())])
        .collect()
}

// =============================================================================
// String replacement benchmarks - comparing different mutation strategies
// =============================================================================

/// Old approach: concatenate slices into new String
fn replace_via_concat(content: &str, range: Range<usize>, replacement: &str) -> String {
    content[0..range.start].to_owned() + replacement + &content[range.end..]
}

/// New approach: in-place replacement
fn replace_via_replace_range(content: &mut String, range: Range<usize>, replacement: &str) {
    content.replace_range(range, replacement);
}

fn bench_string_replacement(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_replacement");

    for size in [1_000, 5_000, 10_000, 50_000] {
        let mut rng = StdRng::seed_from_u64(9999);
        let base_text = generate_random_text(&mut rng, size);

        // Benchmark: Replace 10 chars in the middle with 10 chars (same size)
        let mid = size / 2;
        let range = mid..mid + 10;
        let replacement = "XXXXXXXXXX";

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("concat", size),
            &(&base_text, range.clone(), replacement),
            |b, (text, range, repl)| {
                b.iter(|| black_box(replace_via_concat(text, range.clone(), repl)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("replace_range", size),
            &(&base_text, range.clone(), replacement),
            |b, (text, range, repl)| {
                b.iter_batched(
                    || (*text).to_string(),
                    |mut s| {
                        replace_via_replace_range(&mut s, range.clone(), repl);
                        black_box(s)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn bench_insertion_at_cursor(c: &mut Criterion) {
    let mut group = c.benchmark_group("insertion_at_cursor");

    for size in [1_000, 5_000, 10_000] {
        let mut rng = StdRng::seed_from_u64(9999);
        let base_text = generate_random_text(&mut rng, size);

        // Simulate typing at the end (common case)
        let _cursor_at_end = size..size;
        let char_to_insert = "x";

        group.throughput(Throughput::Elements(100)); // 100 insertions

        group.bench_with_input(
            BenchmarkId::new("concat_100_chars", size),
            &base_text,
            |b, text| {
                b.iter_batched(
                    || text.clone(),
                    |mut s| {
                        for _ in 0..100 {
                            let len = s.len();
                            s = replace_via_concat(&s, len..len, char_to_insert);
                        }
                        black_box(s)
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("replace_range_100_chars", size),
            &base_text,
            |b, text| {
                b.iter_batched(
                    || text.clone(),
                    |mut s| {
                        for _ in 0..100 {
                            let len = s.len();
                            replace_via_replace_range(&mut s, len..len, char_to_insert);
                        }
                        black_box(s)
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        // Also test String::push which should be fastest for single chars
        group.bench_with_input(
            BenchmarkId::new("push_100_chars", size),
            &base_text,
            |b, text| {
                b.iter_batched(
                    || text.clone(),
                    |mut s| {
                        for _ in 0..100 {
                            s.push('x');
                        }
                        black_box(s)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// =============================================================================
// UTF-16 conversion benchmarks - important for IME support
// =============================================================================

/// Convert UTF-8 byte offset to UTF-16 code unit offset (current implementation)
fn offset_to_utf16(content: &str, offset: usize) -> usize {
    let mut utf16_offset = 0;
    let mut utf8_count = 0;

    for character in content.chars() {
        if utf8_count >= offset {
            break;
        }
        utf8_count += character.len_utf8();
        utf16_offset += character.len_utf16();
    }

    utf16_offset
}

/// Convert UTF-16 code unit offset to UTF-8 byte offset (current implementation)
fn offset_from_utf16(content: &str, offset: usize) -> usize {
    let mut utf8_offset = 0;
    let mut utf16_count = 0;

    for character in content.chars() {
        if utf16_count >= offset {
            break;
        }
        utf16_count += character.len_utf16();
        utf8_offset += character.len_utf8();
    }

    utf8_offset.min(content.len())
}

fn bench_utf16_conversion(c: &mut Criterion) {
    let mut group = c.benchmark_group("utf16_conversion");

    for size in [1_000, 5_000, 10_000] {
        let mut rng = StdRng::seed_from_u64(9999);

        // ASCII text (1:1 UTF-8 to UTF-16)
        let ascii_text = generate_random_text(&mut rng, size);

        // Unicode text (variable length)
        let unicode_text = generate_unicode_text(&mut rng, size);

        // Benchmark converting offset at various positions
        let offsets: Vec<usize> = vec![0, size / 4, size / 2, size * 3 / 4, size - 1];

        group.bench_with_input(
            BenchmarkId::new("ascii_to_utf16", size),
            &(&ascii_text, &offsets),
            |b, (text, offsets)| {
                b.iter(|| {
                    for &offset in *offsets {
                        black_box(offset_to_utf16(text, offset.min(text.len())));
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("unicode_to_utf16", size),
            &(&unicode_text, &offsets),
            |b, (text, offsets)| {
                b.iter(|| {
                    for &offset in *offsets {
                        black_box(offset_to_utf16(text, offset.min(text.len())));
                    }
                })
            },
        );

        // Also benchmark the reverse conversion
        let utf16_len = unicode_text.encode_utf16().count();
        let utf16_offsets: Vec<usize> = vec![
            0,
            utf16_len / 4,
            utf16_len / 2,
            utf16_len * 3 / 4,
            utf16_len,
        ];

        group.bench_with_input(
            BenchmarkId::new("unicode_from_utf16", size),
            &(&unicode_text, &utf16_offsets),
            |b, (text, offsets)| {
                b.iter(|| {
                    for &offset in *offsets {
                        black_box(offset_from_utf16(text, offset));
                    }
                })
            },
        );
    }

    group.finish();
}

// =============================================================================
// Undo stack benchmarks - comparing full copy vs patch-based
// =============================================================================

/// Simulates the current approach: store full content copy
#[derive(Clone)]
struct FullCopyEntry {
    content: String,
    cursor: usize,
}

/// Simulates patch-based approach: store only the change
#[derive(Clone)]
struct PatchEntry {
    range: Range<usize>,
    old_text: String,
    new_text_len: usize,
    cursor_before: usize,
}

fn bench_undo_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("undo_memory");
    group.sample_size(50);

    for content_size in [1_000, 5_000, 10_000, 50_000] {
        let mut rng = StdRng::seed_from_u64(9999);
        let base_text = generate_random_text(&mut rng, content_size);

        // Simulate 100 small edits (like typing)
        let num_edits = 100;

        group.bench_with_input(
            BenchmarkId::new("full_copy", content_size),
            &base_text,
            |b, text| {
                b.iter_batched(
                    || (text.clone(), Vec::<FullCopyEntry>::with_capacity(num_edits)),
                    |(mut content, mut undo_stack)| {
                        for i in 0..num_edits {
                            // Push current state to undo
                            undo_stack.push(FullCopyEntry {
                                content: content.clone(),
                                cursor: i,
                            });
                            // Make a small edit
                            content.push('x');
                        }
                        black_box((content, undo_stack))
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("patch_based", content_size),
            &base_text,
            |b, text| {
                b.iter_batched(
                    || (text.clone(), Vec::<PatchEntry>::with_capacity(num_edits)),
                    |(mut content, mut undo_stack)| {
                        for i in 0..num_edits {
                            let len = content.len();
                            // Push patch to undo (what will be needed to reverse)
                            undo_stack.push(PatchEntry {
                                range: len..len + 1,
                                old_text: String::new(),
                                new_text_len: 1,
                                cursor_before: i,
                            });
                            // Make a small edit
                            content.push('x');
                        }
                        black_box((content, undo_stack))
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

fn bench_undo_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("undo_apply");

    for content_size in [1_000, 5_000, 10_000] {
        let mut rng = StdRng::seed_from_u64(9999);
        let base_text = generate_random_text(&mut rng, content_size);

        // Create a content string and 50 undo entries
        let num_edits = 50;

        // Setup for full copy approach
        let mut full_copy_content = base_text.clone();
        let mut full_copy_stack = Vec::new();
        for i in 0..num_edits {
            full_copy_stack.push(FullCopyEntry {
                content: full_copy_content.clone(),
                cursor: i,
            });
            full_copy_content.push('x');
        }

        // Setup for patch approach
        let mut patch_content = base_text.clone();
        let mut patch_stack = Vec::new();
        for i in 0..num_edits {
            let len = patch_content.len();
            patch_stack.push(PatchEntry {
                range: len..len + 1,
                old_text: String::new(),
                new_text_len: 1,
                cursor_before: i,
            });
            patch_content.push('x');
        }

        group.bench_with_input(
            BenchmarkId::new("full_copy_undo_all", content_size),
            &(&full_copy_content, &full_copy_stack),
            |b, (content, stack)| {
                b.iter_batched(
                    || ((*content).clone(), (*stack).clone()),
                    |(mut content, mut stack)| {
                        // Undo all edits
                        while let Some(entry) = stack.pop() {
                            content = entry.content;
                        }
                        black_box(content)
                    },
                    BatchSize::SmallInput,
                )
            },
        );

        group.bench_with_input(
            BenchmarkId::new("patch_based_undo_all", content_size),
            &(&patch_content, &patch_stack),
            |b, (content, stack)| {
                b.iter_batched(
                    || ((*content).clone(), (*stack).clone()),
                    |(mut content, mut stack)| {
                        // Undo all edits by applying reverse patches
                        while let Some(entry) = stack.pop() {
                            // Apply the reverse: replace new_text with old_text
                            let start = entry.range.start.min(content.len());
                            let end = (entry.range.start + entry.new_text_len).min(content.len());
                            content.replace_range(start..end, &entry.old_text);
                        }
                        black_box(content)
                    },
                    BatchSize::SmallInput,
                )
            },
        );
    }

    group.finish();
}

// =============================================================================
// Line finding benchmarks
// =============================================================================

fn find_line_start(content: &str, offset: usize) -> usize {
    content[..offset.min(content.len())]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0)
}

fn find_line_end(content: &str, offset: usize) -> usize {
    content[offset.min(content.len())..]
        .find('\n')
        .map(|pos| offset + pos)
        .unwrap_or(content.len())
}

fn bench_line_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("line_operations");

    for num_lines in [100, 500, 1000] {
        let mut rng = StdRng::seed_from_u64(9999);

        // Generate text with specified number of lines
        let mut text = String::new();
        for _ in 0..num_lines {
            let line_len = rng.random_range(20..80);
            for _ in 0..line_len {
                text.push(if rng.random_bool(0.95) { 'a' } else { ' ' });
            }
            text.push('\n');
        }

        let text_len = text.len();

        // Test finding line boundaries at various offsets
        let offsets: Vec<usize> = (0..10)
            .map(|i| (text_len * i / 10).min(text_len.saturating_sub(1)))
            .collect();

        group.bench_with_input(
            BenchmarkId::new("find_line_start", num_lines),
            &(&text, &offsets),
            |b, (text, offsets)| {
                b.iter(|| {
                    for &offset in *offsets {
                        black_box(find_line_start(text, offset));
                    }
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("find_line_end", num_lines),
            &(&text, &offsets),
            |b, (text, offsets)| {
                b.iter(|| {
                    for &offset in *offsets {
                        black_box(find_line_end(text, offset));
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_string_replacement,
    bench_insertion_at_cursor,
    bench_utf16_conversion,
    bench_undo_memory,
    bench_undo_apply,
    bench_line_operations,
);

criterion_main!(benches);
