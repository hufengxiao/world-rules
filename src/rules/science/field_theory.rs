//! 域论规则
//!
//! 域论研究域结构，域是最重要的代数结构之一，在数论和代数几何中广泛应用。

use crate::rules::core::{Rule, RuleCategory, RuleMetadata, RuleResult, ValidateContext};
use crate::simple_rule;

simple_rule! {
    struct: FieldTheoryRules,
    name: "域论规则",
    desc: "域的定义、扩张与伽罗瓦理论",
    origin: "数学",
    tags: ["科学", "数学", "代数", "域论"]
}

impl FieldTheoryRules {
    /// 的基本定义
    pub fn field_definition(&self) -> Vec<&'static str> {
        vec![
            "域定义: 每个非零元素都有乘法逆元的交换幺环",
            "域性质: F 是域 iff F/{0} 对乘法构成群",
            "域的元素: 0（零元）、1（幺元）、非零元素",
            "域的运算: 加法、减法、乘法、除法（非零元素）",
            "特征: 使 p·1 = 0 的素数 p，或特征为 0",
            "特征 0 域: 含 Q（有理数域）作为子域",
            "特征 p 域: 含 F_p = Z/pZ 作为子域",
            "有限域: 元素个数有限的域，阶为 pⁿ",
        ]
    }

    /// 域扩张
    pub fn field_extensions(&self) -> Vec<&'static str> {
        vec![
            "扩张定义: 域 F 包含子域 K，记为 F/K",
            "扩张度: [F:K] = F 作为 K 上向量空间的维数",
            "有限扩张: [F:K] 有限的扩张",
            "无限扩张: [F:K] 无限的扩张",
            "扩张塔定理: [F:K] = [F:E][E:K]",
            "生成扩张: F = K(α₁,...,αₙ)",
            "单扩张: F = K(α)，由一个元素生成",
            "扩张类型: 代数扩张和超越扩张",
        ]
    }

    /// 代数元素
    pub fn algebraic_elements(&self) -> Vec<&'static str> {
        vec![
            "代数元素: α 是某多项式 f(x) ∈ K[x] 的根",
            "超越元素: 不是任何多项式的根",
            "最小多项式: 使 m(α) = 0 的次数最小的多项式",
            "最小多项式性质: 不可约、唯一",
            "元素度: 最小多项式的次数 deg(m)",
            "代数扩张: 每个元素都是代数元素的扩张",
            "超越扩张: 包含超越元素的扩张",
            "代数闭包: 包含所有代数元素的扩张",
        ]
    }

    /// 多项式根
    pub fn polynomial_roots(&self) -> Vec<&'static str> {
        vec![
            "根的存在: n 次多项式最多有 n 个根（在域中）",
            "根与因子: α 是根 iff (x-α) 是因子",
            "重根: 根 α 出现多次",
            "单根: 只出现一次的根",
            "判别式: 判断是否有重根",
            "分裂域: 多项式完全分解的扩张",
            "分裂域存在: 每个多项式都有分裂域",
            "分裂域唯一: 分裂域在同构意义下唯一",
        ]
    }

    /// 伽罗瓦理论
    pub fn galois_theory(&self) -> Vec<&'static str> {
        vec![
            "伽罗瓦群: Aut(F/K)，扩张的自同构群",
            "伽罗瓦扩张: 固定域恰为 K 的扩张",
            "基本定理: 子群与中间域一一对应",
            "固定子群: H = Gal(F/E)，E 是中间域",
            "固定域: E = Fix(H)，H 固定的元素",
            "正规扩张: 伽罗瓦扩张等价于正规且可分扩张",
            "可分扩张: 每个元素的最小多项式无重根",
            "本原元素定理: 有限可分扩张是单扩张",
        ]
    }

    /// 伽罗瓦应用
    pub fn galois_applications(&self) -> Vec<&'static str> {
        vec![
            "方程可解性: 伽罗瓦群可解时方程可解",
            "五次方程: 一般五次方程不可解（伽罗瓦群 S₅ 不可解）",
            "根式解: 用根式表示根的条件",
            "阿贝尔扩张: 伽罗瓦群是阿贝尔群",
            "循环扩张: 伽罗瓦群是循环群",
            "尺规作图: 可作图点的伽罗瓦群条件",
            "正 n 边形: 可作图 iff n = 2ᵏ×(不同 Fermat 素数之积)",
            "三等分角: 一般角不可三等分",
        ]
    }

    /// 有限域
    pub fn finite_fields(&self) -> Vec<&'static str> {
        vec![
            "有限域阶: 必为 pⁿ，p 是素数",
            "存在性: 对每个 pⁿ，存在阶为 pⁿ 的域",
            "唯一性: 阶相同的有限域同构",
            "记号: F_{pⁿ} 或 GF(pⁿ)",
            "素域: F_p = Z/pZ",
            "乘法群: F_{pⁿ}* 是阶为 pⁿ-1 的循环群",
            "构造: F_{pⁿ} = F_p[x]/(f)，f 是 n 次不可约多项式",
            "子域: F_{pⁿ} 的子域是 F_{pᵐ}，m 整除 n",
        ]
    }

    /// 代数闭域
    pub fn algebraically_closed_fields(&self) -> Vec<&'static str> {
        vec![
            "定义: 每个多项式都在域中有根",
            "代数闭包: 域 K 的最小代数闭扩张",
            "复数域 C: 代数闭域",
            "代数基本定理: C 上 n 次多项式恰有 n 个根",
            "代数数域: Q 的代数闭包中的有限扩张",
            "实代数闭域: 实数的代数闭包",
            "闭包存在性: 每个域都有代数闭包",
            "闭包唯一性: 代数闭包在同构意义下唯一",
        ]
    }

    /// 应用实例
    pub fn applications(&self) -> Vec<&'static str> {
        vec![
            "数论: 域论是现代数论的核心工具",
            "代数几何: 埫和函数域的研究",
            "编码理论: 有限域上的纠错码",
            "密码学: 有限域上的密码算法",
            "伽罗瓦理论: 方程可解性判断",
            "计算代数: 域上多项式计算",
            "信号处理: 有限域上的算法",
            "量子计算: 域上的量子算法",
        ]
    }
}

impl Rule for FieldTheoryRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::science("field_theory")
    }

    fn validate(&self, _ctx: &ValidateContext) -> RuleResult<bool> {
        Ok(true)
    }

    fn explain(&self) -> String {
        crate::rules::core::format_rule_sections(
            "域论规则",
            &[
                ("域的基本定义", &self.field_definition()),
                ("域扩张", &self.field_extensions()),
                ("代数元素", &self.algebraic_elements()),
                ("多项式根", &self.polynomial_roots()),
                ("伽罗瓦理论", &self.galois_theory()),
                ("伽罗瓦应用", &self.galois_applications()),
                ("有限域", &self.finite_fields()),
                ("代数闭域", &self.algebraically_closed_fields()),
                ("应用实例", &self.applications()),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_theory_rules() {
        let rules = FieldTheoryRules::new();
        assert_eq!(rules.metadata().name, "域论规则");
        assert!(!rules.explain().is_empty());
        assert!(!rules.field_definition().is_empty());
        assert!(!rules.field_extensions().is_empty());
        assert!(!rules.galois_theory().is_empty());
    }
}