// 数独验证性能基准测试
// 测试数独规则验证和求解算法的性能

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world_rules::rules::core::{Rule, ValidateContext};
use world_rules::rules::games::{SudokuRules, SudokuVariantRules};

/// 基准测试：标准数独验证性能
/// 测试对标准9x9数独的验证性能
fn bench_sudoku_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku_validate");

    let rules = SudokuRules::new();

    // 使用正确的 ValidateContext API
    // 一个有效的数独布局（字符串表示）
    let valid_sudoku = ValidateContext::generic(
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
    );

    group.bench_function("validate_standard", |b| {
        b.iter(|| black_box(rules.validate(&valid_sudoku)))
    });

    // 一个已完成的数独
    let completed_sudoku = ValidateContext::generic(
        "534678912672195348198342567859761423426853791713924856961537284287419635345286179",
    );

    group.bench_function("validate_completed", |b| {
        b.iter(|| black_box(rules.validate(&completed_sudoku)))
    });

    // 一个部分填充的数独
    let partial_sudoku = ValidateContext::generic(
        "530070000600195000000000000000000000000000000000000000000000280000419005000080079",
    );

    group.bench_function("validate_partial", |b| {
        b.iter(|| black_box(rules.validate(&partial_sudoku)))
    });

    group.finish();
}

/// 基准测试：数独变体验证性能
/// 测试不同数独变体的验证性能
fn bench_sudoku_variants(c: &mut Criterion) {
    let mut group = c.benchmark_group("sudoku_variants");

    let standard_rules = SudokuRules::new();
    let variant_rules = SudokuVariantRules::new();

    let test_puzzle = ValidateContext::generic(
        "530070000600195000098000060800060003400803001700020006060000280000419005000080079",
    );

    group.bench_function("standard_9x9", |b| {
        b.iter(|| black_box(standard_rules.validate(&test_puzzle)))
    });

    group.bench_function("variant_rules", |b| {
        b.iter(|| black_box(variant_rules.validate(&test_puzzle)))
    });

    group.finish();
}

/// 基准测试：行/列/宫验证性能
/// 测试数独核心验证逻辑的性能
fn bench_sudoku_line_validation(c: &mut Criterion) {
    // 准备测试数据
    let row: [u8; 9] = [5, 3, 0, 0, 7, 0, 0, 0, 0];
    let col: [u8; 9] = [5, 6, 0, 8, 4, 7, 0, 0, 0];
    let block: [u8; 9] = [5, 3, 0, 6, 0, 0, 0, 9, 8];

    let mut group = c.benchmark_group("sudoku_line_validation");

    group.bench_function("validate_row", |b| {
        b.iter(|| {
            // 检查行是否有效（无重复）
            let mut seen = [false; 10];
            let mut valid = true;
            for &num in &row {
                if num != 0 {
                    if seen[num as usize] {
                        valid = false;
                        break;
                    }
                    seen[num as usize] = true;
                }
            }
            black_box(valid)
        })
    });

    group.bench_function("validate_col", |b| {
        b.iter(|| {
            // 检查列是否有效（无重复）
            let mut seen = [false; 10];
            let mut valid = true;
            for &num in &col {
                if num != 0 {
                    if seen[num as usize] {
                        valid = false;
                        break;
                    }
                    seen[num as usize] = true;
                }
            }
            black_box(valid)
        })
    });

    group.bench_function("validate_block", |b| {
        b.iter(|| {
            // 检查宫是否有效（无重复）
            let mut seen = [false; 10];
            let mut valid = true;
            for &num in &block {
                if num != 0 {
                    if seen[num as usize] {
                        valid = false;
                        break;
                    }
                    seen[num as usize] = true;
                }
            }
            black_box(valid)
        })
    });

    group.finish();
}

/// 基准测试：数独规则解释性能
/// 测试生成规则说明的性能
fn bench_sudoku_explain(c: &mut Criterion) {
    let rules = SudokuRules::new();

    c.bench_function("sudoku_explain", |b| b.iter(|| black_box(rules.explain())));
}

criterion_group!(
    benches,
    bench_sudoku_validate,
    bench_sudoku_variants,
    bench_sudoku_line_validation,
    bench_sudoku_explain,
);
criterion_main!(benches);
