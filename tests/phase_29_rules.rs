//! Phase 29 数学规则测试 - Phase 29-02 几何规则
//!
//! 测试新增的几何规则（Phase 29-02）

use world_rules::rules::core::{Rule, RuleCategory, ValidateContext};
use world_rules::rules::science::GeometryMathRules;

/// 测试几何学规则
#[test]
fn test_geometry_math_rules() {
    let rules = GeometryMathRules::new();
    assert_eq!(rules.metadata().name, "几何学规则");
    assert_eq!(rules.category(), RuleCategory::science("geometry_math"));
    assert!(rules
        .validate(&ValidateContext::Generic("test".to_string()))
        .is_ok());
    assert!(!rules.explain().is_empty());

    // 测试各方法返回非空
    assert!(!rules.plane_geometry().is_empty());
    assert!(!rules.circle_geometry().is_empty());
    assert!(!rules.solid_geometry().is_empty());
    assert!(!rules.analytic_geometry().is_empty());
    assert!(!rules.trigonometry().is_empty());
    assert!(!rules.geometric_transformations().is_empty());
    assert!(!rules.non_euclidean_geometry().is_empty());
    assert!(!rules.geometric_construction().is_empty());
    assert!(!rules.geometry_theorems().is_empty());
    assert!(!rules.applications().is_empty());
}

/// 测试几何学规则的具体内容
#[test]
fn test_geometry_math_content() {
    let rules = GeometryMathRules::new();

    // 验证平面几何包含三角形内角和定理
    let plane_geo = rules.plane_geometry();
    assert!(plane_geo
        .iter()
        .any(|s| s.contains("三角形内角和") && s.contains("180°")));

    // 验证圆的几何包含圆周率
    let circle_geo = rules.circle_geometry();
    assert!(circle_geo
        .iter()
        .any(|s| s.contains("圆周率") && s.contains("π")));

    // 验证立体几何包含欧拉公式
    let solid_geo = rules.solid_geometry();
    assert!(solid_geo
        .iter()
        .any(|s| s.contains("欧拉公式") && s.contains("V - E + F")));

    // 验证解析几何包含两点距离公式
    let analytic_geo = rules.analytic_geometry();
    assert!(analytic_geo
        .iter()
        .any(|s| s.contains("两点距离") && s.contains("√")));

    // 验证三角几何包含正弦定理和余弦定理
    let trig = rules.trigonometry();
    assert!(trig.iter().any(|s| s.contains("正弦定理")));
    assert!(trig.iter().any(|s| s.contains("余弦定理")));
    assert!(trig.iter().any(|s| s.contains("勾股定理")));

    // 验证几何变换包含平移、旋转、反射
    let transformations = rules.geometric_transformations();
    assert!(transformations.iter().any(|s| s.contains("平移变换")));
    assert!(transformations.iter().any(|s| s.contains("旋转变换")));
    assert!(transformations.iter().any(|s| s.contains("反射变换")));

    // 验证非欧几何包含罗巴切夫斯基和黎曼几何
    let non_euclidean = rules.non_euclidean_geometry();
    assert!(non_euclidean.iter().any(|s| s.contains("罗巴切夫斯基")));
    assert!(non_euclidean.iter().any(|s| s.contains("黎曼")));

    // 验证几何作图包含尺规作图和不可能问题
    let construction = rules.geometric_construction();
    assert!(construction.iter().any(|s| s.contains("尺规作图")));
    assert!(construction.iter().any(|s| s.contains("三等分角")));
    assert!(construction.iter().any(|s| s.contains("不可能")));

    // 验证几何定理包含经典定理
    let theorems = rules.geometry_theorems();
    assert!(theorems.iter().any(|s| s.contains("托勒密定理")));
    assert!(theorems.iter().any(|s| s.contains("梅涅劳斯")));
    assert!(theorems.iter().any(|s| s.contains("塞瓦定理")));

    // 验证应用领域包含实际应用
    let apps = rules.applications();
    assert!(apps.iter().any(|s| s.contains("建筑设计")));
    assert!(apps.iter().any(|s| s.contains("计算机图形学")));
    assert!(apps.iter().any(|s| s.contains("机器人学")));
}

/// 测试几何学规则的 metadata
#[test]
fn test_geometry_math_metadata() {
    let rules = GeometryMathRules::new();

    assert_eq!(rules.metadata().name, "几何学规则");
    assert!(!rules.metadata().description.is_empty());
    assert!(rules.metadata().tags.contains(&"科学".to_string()));
    assert!(rules.metadata().tags.contains(&"数学".to_string()));
    assert!(rules.metadata().tags.contains(&"几何".to_string()));
}

/// 测试几何学规则的方法数量
#[test]
fn test_geometry_math_method_count() {
    let rules = GeometryMathRules::new();

    // 每个方法都应该返回至少 8 条规则
    assert!(rules.plane_geometry().len() >= 8);
    assert!(rules.circle_geometry().len() >= 9);
    assert!(rules.solid_geometry().len() >= 10);
    assert!(rules.analytic_geometry().len() >= 12);
    assert!(rules.trigonometry().len() >= 10);
    assert!(rules.geometric_transformations().len() >= 10);
    assert!(rules.non_euclidean_geometry().len() >= 10);
    assert!(rules.geometric_construction().len() >= 10);
    assert!(rules.geometry_theorems().len() >= 10);
    assert!(rules.applications().len() >= 10);

    // 总规则数应该超过 90
    let total = rules.plane_geometry().len()
        + rules.circle_geometry().len()
        + rules.solid_geometry().len()
        + rules.analytic_geometry().len()
        + rules.trigonometry().len()
        + rules.geometric_transformations().len()
        + rules.non_euclidean_geometry().len()
        + rules.geometric_construction().len()
        + rules.geometry_theorems().len()
        + rules.applications().len();

    assert!(total >= 90);
}
