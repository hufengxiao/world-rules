// 法律规则性能基准测试
// 测试法律规则验证、解释和序列化的性能表现

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use world_rules::rules::core::{Rule, ValidateContext};
use world_rules::rules::law::{
    CivilLawRules, CompanyLawRules, ConstitutionRules, ConsumerLawRules, CriminalLawRules,
    LaborLawRules, TaxLawRules,
};

/// 基准测试：法律规则创建性能
/// 测试不同法律规则的初始化性能
fn bench_law_rules_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_creation");

    group.bench_function("civil_rules", |b| {
        b.iter(|| black_box(CivilLawRules::new()))
    });

    group.bench_function("criminal_rules", |b| {
        b.iter(|| black_box(CriminalLawRules::new()))
    });

    group.bench_function("constitution_rules", |b| {
        b.iter(|| black_box(ConstitutionRules::new()))
    });

    group.bench_function("company_rules", |b| {
        b.iter(|| black_box(CompanyLawRules::new()))
    });

    group.bench_function("consumer_rules", |b| {
        b.iter(|| black_box(ConsumerLawRules::new()))
    });

    group.bench_function("labor_rules", |b| {
        b.iter(|| black_box(LaborLawRules::new()))
    });

    group.bench_function("tax_rules", |b| b.iter(|| black_box(TaxLawRules::new())));

    group.finish();
}

/// 基准测试：规则验证性能
/// 测试 Rule trait 的 validate 方法性能
fn bench_law_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_validate");

    let civil_rules = CivilLawRules::new();
    let criminal_rules = CriminalLawRules::new();
    let constitution_rules = ConstitutionRules::new();

    // 测试简单文本验证
    let simple_context = ValidateContext::Generic("民事合同".to_string());

    group.bench_function("civil_validate_simple", |b| {
        b.iter(|| black_box(civil_rules.validate(&simple_context)))
    });

    group.bench_function("criminal_validate_simple", |b| {
        b.iter(|| black_box(criminal_rules.validate(&simple_context)))
    });

    group.bench_function("constitution_validate_simple", |b| {
        b.iter(|| black_box(constitution_rules.validate(&simple_context)))
    });

    // 测试复杂文本验证
    let complex_context = ValidateContext::Generic(
        "合同双方约定，甲方应在收到乙方交付的货物后30日内支付货款，\
         如甲方逾期付款，应按每日万分之五的标准支付违约金。\
         本合同自双方签字盖章之日起生效，有效期为一年。"
            .to_string(),
    );

    group.bench_function("civil_validate_complex", |b| {
        b.iter(|| black_box(civil_rules.validate(&complex_context)))
    });

    group.bench_function("criminal_validate_complex", |b| {
        b.iter(|| black_box(criminal_rules.validate(&complex_context)))
    });

    group.finish();
}

/// 基准测试：规则解释性能
/// 测试 Rule trait 的 explain 方法性能
fn bench_law_explain(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_explain");

    let civil_rules = CivilLawRules::new();
    let criminal_rules = CriminalLawRules::new();
    let constitution_rules = ConstitutionRules::new();
    let company_rules = CompanyLawRules::new();
    let consumer_rules = ConsumerLawRules::new();
    let labor_rules = LaborLawRules::new();
    let tax_rules = TaxLawRules::new();

    group.bench_function("civil_explain", |b| {
        b.iter(|| black_box(civil_rules.explain()))
    });

    group.bench_function("criminal_explain", |b| {
        b.iter(|| black_box(criminal_rules.explain()))
    });

    group.bench_function("constitution_explain", |b| {
        b.iter(|| black_box(constitution_rules.explain()))
    });

    group.bench_function("company_explain", |b| {
        b.iter(|| black_box(company_rules.explain()))
    });

    group.bench_function("consumer_explain", |b| {
        b.iter(|| black_box(consumer_rules.explain()))
    });

    group.bench_function("labor_explain", |b| {
        b.iter(|| black_box(labor_rules.explain()))
    });

    group.bench_function("tax_explain", |b| b.iter(|| black_box(tax_rules.explain())));

    group.finish();
}

/// 基准测试：规则元数据获取性能
/// 测试 Rule trait 的 metadata 和 category 方法性能
fn bench_law_metadata(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_metadata");

    let civil_rules = CivilLawRules::new();
    let criminal_rules = CriminalLawRules::new();

    group.bench_function("civil_metadata", |b| {
        b.iter(|| black_box(civil_rules.metadata()))
    });

    group.bench_function("civil_category", |b| {
        b.iter(|| black_box(civil_rules.category()))
    });

    group.bench_function("criminal_metadata", |b| {
        b.iter(|| black_box(criminal_rules.metadata()))
    });

    group.bench_function("criminal_category", |b| {
        b.iter(|| black_box(criminal_rules.category()))
    });

    group.finish();
}

/// 基准测试：民法规则方法性能
/// 测试 CivilLawRules 各方法的性能
fn bench_civil_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("civil_methods");

    let civil_rules = CivilLawRules::new();

    group.bench_function("basic_principles", |b| {
        b.iter(|| black_box(civil_rules.basic_principles()))
    });

    group.bench_function("civil_subjects", |b| {
        b.iter(|| black_box(civil_rules.civil_subjects()))
    });

    group.bench_function("capacity_of_person", |b| {
        b.iter(|| black_box(civil_rules.capacity_of_person()))
    });

    group.bench_function("civil_rights", |b| {
        b.iter(|| black_box(civil_rules.civil_rights()))
    });

    group.bench_function("civil_acts", |b| {
        b.iter(|| black_box(civil_rules.civil_acts()))
    });

    group.bench_function("agency_system", |b| {
        b.iter(|| black_box(civil_rules.agency_system()))
    });

    group.bench_function("limitation_of_action", |b| {
        b.iter(|| black_box(civil_rules.limitation_of_action()))
    });

    group.finish();
}

/// 基准测试：规则序列化性能（如果支持）
/// 测试法律规则的序列化和反序列化性能
fn bench_law_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_serialization");

    // 由于大多数法律规则不直接支持序列化，
    // 这里测试 explain() 结果的字符串处理性能
    let civil_rules = CivilLawRules::new();
    let explanation = civil_rules.explain();

    group.bench_function("explain_string_length", |b| {
        b.iter(|| black_box(explanation.len()))
    });

    group.bench_function("explain_contains_check", |b| {
        b.iter(|| black_box(explanation.contains("民法")))
    });

    group.finish();
}

/// 基准测试：多条规则批量验证
/// 测试同时验证多条规则的性能
fn bench_law_batch_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("law_batch_validate");

    let rules: Vec<Box<dyn Rule>> = vec![
        Box::new(CivilLawRules::new()),
        Box::new(CriminalLawRules::new()),
        Box::new(ConstitutionRules::new()),
        Box::new(CompanyLawRules::new()),
        Box::new(ConsumerLawRules::new()),
    ];

    let context = ValidateContext::Generic("测试文本".to_string());

    group.bench_function("validate_5_rules", |b| {
        b.iter(|| {
            for rule in &rules {
                black_box(rule.validate(&context));
            }
        })
    });

    group.bench_function("validate_10_iterations", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let civil = CivilLawRules::new();
                black_box(civil.validate(&context));
            }
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_law_rules_creation,
    bench_law_validate,
    bench_law_explain,
    bench_law_metadata,
    bench_civil_methods,
    bench_law_serialization,
    bench_law_batch_validate,
);

criterion_main!(benches);
