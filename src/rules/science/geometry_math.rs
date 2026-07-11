//! 几何学规则
//!
//! 几何学研究空间形状、大小、位置及其变化规律。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: GeometryMathRules,
    name: "几何学规则",
    desc: "几何学定律、定理与应用方法",
    origin: "数学",
    tags: ["科学", "数学", "几何"]
}

impl GeometryMathRules {
    /// 平面几何基础
    pub fn plane_geometry(&self) -> Vec<&'static str> {
        vec![
            "三角形内角和: 三角形三个内角之和等于180°",
            "三角形外角定理: 外角等于不相邻两个内角之和",
            "平行线性质: 两直线平行，同位角相等、内错角相等、同旁内角互补",
            "相似三角形: 对应角相等，对应边成比例",
            "全等三角形判定: SSS、SAS、ASA、AAS、HL（直角三角形）",
            "等腰三角形性质: 两底角相等，顶角平分线、底边中线、底边高线三线合一",
            "等边三角形: 三条边相等，三个角都是60°",
            "圆的性质: 圆心角、弧、弦的关系定理",
        ]
    }

    /// 圆的几何
    pub fn circle_geometry(&self) -> Vec<&'static str> {
        vec![
            "圆周率: π ≈ 3.14159...，圆周长与直径的比值",
            "圆的周长: C = 2πr = πd",
            "圆的面积: S = πr² = πd²/4",
            "圆心角定理: 圆心角等于所对弧的度数",
            "圆周角定理: 圆周角等于所对弧度数的一半",
            "弦的性质: 在同圆或等圆中，相等的圆心角所对的弦相等",
            "切线性质: 过半径外端的直线垂直于半径时为切线",
            "切割线定理: 从圆外一点引切线和割线，切线长² = 割线全长×外部份",
            "相交弦定理: 圆内两弦相交，交点分得的线段乘积相等",
        ]
    }

    /// 立体几何
    pub fn solid_geometry(&self) -> Vec<&'static str> {
        vec![
            "正方体体积: V = a³，表面积 S = 6a²",
            "长方体体积: V = abc，表面积 S = 2(ab + bc + ca)",
            "圆柱体积: V = πr²h，侧面积 S = 2πrh",
            "圆锥体积: V = πr²h/3，侧面积 S = πrl（l为母线）",
            "球体体积: V = 4πr³/3，表面积 S = 4πr²",
            "棱锥体积: V = Sh/3（S为底面积，h为高）",
            "棱柱体积: V = Sh（S为底面积，h为高）",
            "欧拉公式: 多面体顶点数V、棱数E、面数F满足 V - E + F = 2",
            "空间直线位置: 相交、平行、异面",
            "空间角度: 线线角、线面角、面面角",
        ]
    }

    /// 解析几何
    pub fn analytic_geometry(&self) -> Vec<&'static str> {
        vec![
            "两点距离: d = √[(x₂-x₁)² + (y₂-y₁)²]",
            "中点坐标: M = ((x₁+x₂)/2, (y₁+y₂)/2)",
            "直线斜率: k = (y₂-y₁)/(x₂-x₁)（两点式）",
            "直线方程: 点斜式 y-y₁ = k(x-x₁)，斜截式 y = kx + b",
            "直线一般式: Ax + By + C = 0",
            "两直线平行: k₁ = k₂（斜率相等）",
            "两直线垂直: k₁·k₂ = -1（斜率乘积为-1）",
            "点到直线距离: d = |Ax₀ + By₀ + C|/√(A² + B²)",
            "圆的标准方程: (x-a)² + (y-b)² = r²",
            "椭圆方程: x²/a² + y²/b² = 1（a > b）",
            "双曲线方程: x²/a² - y²/b² = 1",
            "抛物线方程: y² = 2px（开口向右）",
        ]
    }

    /// 三角几何
    pub fn trigonometry(&self) -> Vec<&'static str> {
        vec![
            "正弦定理: a/sinA = b/sinB = c/sinC = 2R",
            "余弦定理: c² = a² + b² - 2ab·cosC",
            "勾股定理: a² + b² = c²（直角三角形）",
            "三角恒等式: sin²θ + cos²θ = 1",
            "和角公式: sin(A+B) = sinA·cosB + cosA·sinB",
            "差角公式: sin(A-B) = sinA·cosB - cosA·sinB",
            "倍角公式: sin(2θ) = 2sinθ·cosθ",
            "半角公式: sin(θ/2) = ±√[(1-cosθ)/2]",
            "正弦面积公式: S = ½ab·sinC",
            "海伦公式: S = √[p(p-a)(p-b)(p-c)]，p = (a+b+c)/2",
        ]
    }

    /// 几何变换
    pub fn geometric_transformations(&self) -> Vec<&'static str> {
        vec![
            "平移变换: 图形沿某方向移动，形状大小不变",
            "旋转变换: 图形绕某点旋转，保持形状大小",
            "反射变换: 图形关于某直线对称（镜像）",
            "位似变换: 图形按比例放大或缩小",
            "等距变换: 保持距离不变的变换（平移、旋转、反射）",
            "仿射变换: 保持平行性的线性变换",
            "射影变换: 保持共线性的变换",
            "拓扑变换: 允许拉伸弯曲但不允许撕裂粘合",
            "对称变换: 旋转对称、反射对称、平移对称",
            "变换群: 变换的集合构成群，满足封闭性、结合律、单位元、逆元",
        ]
    }

    /// 非欧几何
    pub fn non_euclidean_geometry(&self) -> Vec<&'static str> {
        vec![
            "欧氏第五公设: 过直线外一点有且只有一条平行线",
            "罗巴切夫斯基几何: 过直线外一点至少有两条平行线（双曲几何）",
            "黎曼几何: 过直线外一点没有平行线（椭圆几何）",
            "双曲几何三角内角和: 小于180°",
            "椭圆几何三角内角和: 大于180°",
            "双曲几何圆周率: 大于π",
            "椭圆几何圆周率: 小于π",
            "黎曼曲率: 描述空间弯曲程度的量",
            "测地线: 曲面上两点间最短路径",
            "应用领域: 广义相对论、宇宙学、GPS定位",
        ]
    }

    /// 几何作图
    pub fn geometric_construction(&self) -> Vec<&'static str> {
        vec![
            "尺规作图: 仅用无刻度直尺和圆规作图",
            "作角平分线: 以角顶点为圆心画弧，连接交点",
            "作垂直平分线: 分别以两端点为圆心画弧，连接交点",
            "作等边三角形: 已知边长，用圆规确定顶点",
            "作正六边形: 以圆心为顶点，半径为边长依次作弧",
            "三等分角: 一般角不能用尺规三等分（不可能问题）",
            "倍立方问题: 用尺规作体积为原立方体2倍的立方体（不可能）",
            "化圆为方: 用尺规作与圆面积相等的正方形（不可能）",
            "作平行线: 过直线外一点作已知直线的平行线",
            "黄金分割: 将线段分成比例 (√5-1):2 ≈ 0.618",
        ]
    }

    /// 几何定理
    pub fn geometry_theorems(&self) -> Vec<&'static str> {
        vec![
            "托勒密定理: 圆内接四边形对角线乘积等于两组对边乘积之和",
            "梅涅劳斯定理: 三角形截线三点共线的充要条件",
            "塞瓦定理: 三角形三线共点的充要条件",
            "斯特瓦尔特定理: 三角形一边上的点与另两边的关系",
            "蝴蝶定理: 过弦中点作两弦，连接端点与原弦的交点对称",
            "帕斯卡定理: 圆内接六边形三组对边延长线的交点共线",
            "布里昂雄定理: 圆外切六边形三组对顶点连线共点",
            "费马点: 到三角形三个顶点距离之和最小的点",
            "拿破仑定理: 以三角形三边向外作等边三角形，中心构成等边三角形",
            "西姆松定理: 三角形外接圆上一点到三边的垂足共线",
        ]
    }

    /// 几何应用
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "建筑设计: 几何图形在建筑结构中的应用",
            "工程制图: 三视图、剖视图、轴测图",
            "地理测量: 三角测量、地形图绘制",
            "航海导航: 球面三角形定位",
            "艺术设计: 对称、旋转、缩放在艺术中的应用",
            "计算机图形学: 三维建模、渲染、动画",
            "机器人学: 运动学、路径规划",
            "天文学: 天体轨道计算、星图绘制",
            "医学成像: CT扫描、MRI图像重建",
            "虚拟现实: 三维空间重建与交互",
        ]
    }
}

impl Rule for GeometryMathRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("geometry_math")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "几何学规则",
            &[
                ("平面几何基础", &self.plane_geometry()),
                ("圆的几何", &self.circle_geometry()),
                ("立体几何", &self.solid_geometry()),
                ("解析几何", &self.analytic_geometry()),
                ("三角几何", &self.trigonometry()),
                ("几何变换", &self.geometric_transformations()),
                ("非欧几何", &self.non_euclidean_geometry()),
                ("几何作图", &self.geometric_construction()),
                ("几何定理", &self.geometry_theorems()),
                ("几何应用", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_math_rules() {
        let rules = GeometryMathRules::new();
        assert_eq!(rules.metadata().name, "几何学规则");
        assert!(!rules.explain().is_empty());
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
}
