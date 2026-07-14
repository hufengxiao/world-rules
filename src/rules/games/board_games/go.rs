//! 围棋规则及变体
//!
//! 支持多种围棋规则变体：中国规则、日本规则、韩国规则、应氏规则、新西兰规则、智运会规则

use crate::rules::core::{Rule, RuleCategory, RuleMetadata};

/// 围棋棋子颜色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stone {
    /// 黑棋
    Black,
    /// 白棋
    White,
}

impl Stone {
    /// 获取对手颜色
    pub fn opposite(&self) -> Self {
        match self {
            Stone::Black => Stone::White,
            Stone::White => Stone::Black,
        }
    }
}

/// 围棋规则变体
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GoVariant {
    /// 中国规则
    /// - 数子法计算胜负
    /// - 贴目：黑贴7.5目（19路）
    /// - 禁止单劫，允许循环劫
    Chinese,
    /// 日本规则
    /// - 数目法计算胜负
    /// - 贴目：黑贴6.5目（19路）
    /// - 严格的打劫规则
    Japanese,
    /// 韩国规则
    /// - 类似日本规则
    /// - 贴目：黑贴6.5目（19路）
    Korean,
    /// 应氏规则
    /// - 计点制（接近数子法）
    /// - 贴目：黑贴7点（约等于7目）
    /// - 禁止全局同形再现
    /// - 使用应氏计时器
    Ing,
    /// 新西兰规则
    /// - 自由落子规则（允许自杀）
    /// - 新西兰式数目法
    /// - 贴目：黑贴6.5目（19路）
    /// - 简化的同形限制
    NewZealand,
    /// 智运会规则
    /// - 世界智力运动会使用
    /// - 采用中国规则的数子法
    /// - 贴目：黑贴7.5目（19路）
    /// - 严格的计时规定
    WMSG,
}

/// 围棋计时系统
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeSystem {
    /// 无限时
    None,
    /// 标准计时（固定时间）
    Standard {
        /// 主时间（秒）
        main_time: u32,
        /// 读秒时间（秒，0表示不读秒）
        byoyomi: u32,
        /// 读秒次数（0表示无限）
        byoyomi_periods: u32,
    },
    /// 应氏计时器
    /// 使用应氏规则特有的计时方式
    IngTimer {
        /// 基本时间（秒）
        basic_time: u32,
        /// 延长时间（秒）
        extended_time: u32,
    },
    /// 加时间计时（Fischer模式）
    Fischer {
        /// 初始时间（秒）
        initial_time: u32,
        /// 每步加时（秒）
        increment: u32,
    },
}

/// 计算方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoringMethod {
    /// 数子法（中国规则）
    /// 计算棋子数 + 围空数
    /// 死子填入对方领地
    Area,
    /// 数目法（日本规则）
    /// 只计算围空数 - 死子数
    Territory,
    /// 计点制（应氏规则）
    /// 类似数子法，但使用点数概念
    Point,
}

/// 围棋规则
pub struct GoRules {
    metadata: RuleMetadata,
    variant: GoVariant,
    board_size: u8,
    time_system: TimeSystem,
}

impl GoRules {
    /// 创建新的围棋规则实例
    pub fn new(board_size: u8) -> Self {
        Self {
            metadata: RuleMetadata::new("围棋规则", "围棋标准规则说明").with_origin("中国"),
            variant: GoVariant::Chinese,
            board_size,
            time_system: TimeSystem::None,
        }
    }

    /// 设置规则变体
    pub fn with_variant(mut self, variant: GoVariant) -> Self {
        self.variant = variant;
        self
    }

    /// 设置计时系统
    pub fn with_time_system(mut self, time_system: TimeSystem) -> Self {
        self.time_system = time_system;
        self
    }

    /// 获取棋盘大小
    pub fn board_size(&self) -> u8 {
        self.board_size
    }

    /// 获取规则变体
    pub fn variant(&self) -> GoVariant {
        self.variant
    }

    /// 获取计算方式
    pub fn scoring_method(&self) -> ScoringMethod {
        match self.variant {
            GoVariant::Chinese | GoVariant::WMSG => ScoringMethod::Area,
            GoVariant::Japanese | GoVariant::Korean | GoVariant::NewZealand => {
                ScoringMethod::Territory
            }
            GoVariant::Ing => ScoringMethod::Point,
        }
    }

    /// 获取贴目规则（白方获得的优势目数）
    pub fn komi(&self) -> f32 {
        match (&self.variant, self.board_size) {
            (GoVariant::Chinese, 19) => 7.5,
            (GoVariant::Chinese, 13) => 5.5,
            (GoVariant::Chinese, 9) => 5.5,
            (GoVariant::Japanese, 19) => 6.5,
            (GoVariant::Japanese, 13) => 5.5,
            (GoVariant::Japanese, 9) => 5.5,
            (GoVariant::Korean, 19) => 6.5,
            (GoVariant::Korean, 13) => 5.5,
            (GoVariant::Korean, 9) => 5.5,
            (GoVariant::Ing, 19) => 7.0, // 应氏规则使用整数贴点
            (GoVariant::Ing, 13) => 5.0,
            (GoVariant::Ing, 9) => 5.0,
            (GoVariant::NewZealand, 19) => 6.5,
            (GoVariant::NewZealand, 13) => 5.5,
            (GoVariant::NewZealand, 9) => 5.5,
            (GoVariant::WMSG, 19) => 7.5,
            (GoVariant::WMSG, 13) => 5.5,
            (GoVariant::WMSG, 9) => 5.5,
            _ => 0.5,
        }
    }

    /// 是否允许自杀（新西兰规则允许）
    pub fn allows_suicide(&self) -> bool {
        self.variant == GoVariant::NewZealand
    }

    /// 是否禁止全局同形（应氏规则特有）
    pub fn prohibits_superko(&self) -> bool {
        matches!(self.variant, GoVariant::Ing | GoVariant::Japanese)
    }

    /// 获取劫的类型限制
    pub fn ko_rule(&self) -> KoRule {
        match self.variant {
            GoVariant::Chinese => KoRule::SimpleKo,
            GoVariant::Japanese => KoRule::SimpleKo,
            GoVariant::Korean => KoRule::SimpleKo,
            GoVariant::Ing => KoRule::Superko,
            GoVariant::NewZealand => KoRule::SimpleKo,
            GoVariant::WMSG => KoRule::SimpleKo,
        }
    }

    /// 计算胜负结果
    pub fn calculate_result(&self, black_territory: u32, white_territory: u32) -> GoResult {
        let adjusted_white = white_territory as f32 + self.komi();
        if black_territory as f32 > adjusted_white {
            GoResult::BlackWins(black_territory as f32 - adjusted_white)
        } else if (black_territory as f32) < adjusted_white {
            GoResult::WhiteWins(adjusted_white - black_territory as f32)
        } else {
            GoResult::Draw
        }
    }

    /// 获取规则变体名称
    pub fn variant_name(&self) -> &'static str {
        match self.variant {
            GoVariant::Chinese => "中国规则",
            GoVariant::Japanese => "日本规则",
            GoVariant::Korean => "韩国规则",
            GoVariant::Ing => "应氏规则",
            GoVariant::NewZealand => "新西兰规则",
            GoVariant::WMSG => "智运会规则",
        }
    }
}

/// 劫的类型限制
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KoRule {
    /// 简单劫规则
    /// 只禁止立即提回的单劫
    SimpleKo,
    /// 超级劫规则
    /// 禁止全局同形再现
    Superko,
}

/// 围棋结果
#[derive(Debug, Clone, PartialEq)]
pub enum GoResult {
    /// 黑方胜（领先目数）
    BlackWins(f32),
    /// 白方胜（领先目数）
    WhiteWins(f32),
    /// 平局
    Draw,
}

impl Rule for GoRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go")
    }

    fn explain(&self) -> String {
        let variant_details = match self.variant {
            GoVariant::Chinese => {
                "中国规则:\n\
                 • 数子法计算胜负\n\
                 • 计算棋子数 + 围空数\n\
                 • 死子填入对方领地\n\
                 • 允许循环劫"
            }
            GoVariant::Japanese => {
                "日本规则:\n\
                 • 数目法计算胜负\n\
                 • 只计算围空数 - 死子数\n\
                 • 严格的打劫规则\n\
                 • 有特殊的死活判定"
            }
            GoVariant::Korean => {
                "韩国规则:\n\
                 • 类似日本规则\n\
                 • 数目法计算胜负\n\
                 • 有特有的判例"
            }
            GoVariant::Ing => {
                "应氏规则:\n\
                 • 计点制（接近数子法）\n\
                 • 禁止全局同形再现\n\
                 • 使用应氏计时器\n\
                 • 贴目7点（整数）\n\
                 • 不允许让子"
            }
            GoVariant::NewZealand => {
                "新西兰规则:\n\
                 • 允许自杀（自由落子）\n\
                 • 新西兰式数目法\n\
                 • 简化的同形限制\n\
                 • 最简单的规则体系"
            }
            GoVariant::WMSG => {
                "智运会规则:\n\
                 • 世界智力运动会使用\n\
                 • 采用中国规则的数子法\n\
                 • 严格的计时规定\n\
                 • 统一的国际规则"
            }
        };

        format!(
            "【围棋规则 - {}】\n\n\
            棋盘大小: {}×{}\n\
            贴目: {}目\n\
            计算方式: {}\n\n\
            基本规则:\n\
            1. 黑先白后，交替落子\n\
            2. 落子后不能移动\n\
            3. 气尽被提（无气的棋子被吃掉）\n\
            4. 禁止全局同形再现（打劫规则）\n\
            5. 终局计算地盘胜负\n\
            6. 白方获得{}目贴目\n\n\
            {}",
            self.variant_name(),
            self.board_size,
            self.board_size,
            self.komi(),
            match self.scoring_method() {
                ScoringMethod::Area => "数子法",
                ScoringMethod::Territory => "数目法",
                ScoringMethod::Point => "计点制",
            },
            self.komi(),
            variant_details
        )
    }
}

// ============================================================================
// 应氏规则详细实现
// ============================================================================

/// 应氏规则详细实现
pub struct IngRules {
    metadata: RuleMetadata,
    board_size: u8,
}

impl IngRules {
    /// 创建应氏规则实例
    pub fn new(board_size: u8) -> Self {
        Self {
            metadata: RuleMetadata::new("应氏围棋规则", "应昌期先生创立的围棋规则体系")
                .with_origin("台湾"),
            board_size,
        }
    }

    /// 获取棋盘大小
    pub fn board_size(&self) -> u8 {
        self.board_size
    }

    /// 获取贴点（应氏规则使用整数贴点）
    pub fn komi(&self) -> u8 {
        match self.board_size {
            19 => 7,
            13 => 5,
            9 => 5,
            _ => 7,
        }
    }

    /// 检查是否违反全局同形
    /// 应氏规则禁止任何形式的同形再现
    pub fn check_superko(&self, board_history: &[u64], current_hash: u64) -> bool {
        board_history.contains(&current_hash)
    }

    /// 计算点数（应氏规则的计点制）
    /// 点数 = 棋子数 + 围空数
    pub fn calculate_points(&self, stones: u32, territory: u32) -> u32 {
        stones + territory
    }

    /// 获取应氏规则的完整说明
    pub fn full_explanation(&self) -> String {
        format!(
            "【应氏围棋规则】\n\n\
            应氏规则是由应昌期先生创立的围棋规则体系，具有以下特点：\n\n\
            一、计点制\n\
            • 使用点数而非目数计算胜负\n\
            • 点数 = 棋子数 + 围空数\n\
            • 黑方贴{}点给白方\n\n\
            二、禁止全局同形\n\
            • 严格禁止任何形式的同形再现\n\
            • 包括单劫、循环劫、三劫循环等\n\
            • 是最严格的劫规则\n\n\
            三、计时系统\n\
            • 使用应氏计时器\n\
            • 基本时间 + 延长时间模式\n\
            • 超时判负\n\n\
            四、特殊规定\n\
            • 不允许让子\n\
            • 使用应氏棋具\n\
            • 棋盘线有特殊标记\n\n\
            五、优点\n\
            • 规则简洁明确\n\
            • 无需判定死活\n\
            • 计算方式统一",
            self.komi()
        )
    }
}

impl Rule for IngRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_ing")
    }

    fn explain(&self) -> String {
        self.full_explanation()
    }
}

// ============================================================================
// 新西兰规则详细实现
// ============================================================================

/// 新西兰规则详细实现
pub struct NewZealandRules {
    metadata: RuleMetadata,
    board_size: u8,
}

impl NewZealandRules {
    /// 创建新西兰规则实例
    pub fn new(board_size: u8) -> Self {
        Self {
            metadata: RuleMetadata::new("新西兰围棋规则", "新西兰围棋协会制定的简化规则")
                .with_origin("新西兰"),
            board_size,
        }
    }

    /// 获取棋盘大小
    pub fn board_size(&self) -> u8 {
        self.board_size
    }

    /// 获取贴目
    pub fn komi(&self) -> f32 {
        match self.board_size {
            19 => 6.5,
            13 => 5.5,
            9 => 5.5,
            _ => 6.5,
        }
    }

    /// 检查自杀是否合法
    /// 新西兰规则允许自杀
    pub fn is_suicide_legal(&self, _board_position: u64) -> bool {
        true // 新西兰规则允许自杀
    }

    /// 获取新西兰规则的完整说明
    pub fn full_explanation(&self) -> String {
        format!(
            "【新西兰围棋规则】\n\n\
            新西兰规则是最简化的围棋规则体系，特点如下：\n\n\
            一、自由落子\n\
            • 允许自杀\n\
            • 任何位置都可以落子（除非违反基本规则）\n\
            • 简化了禁止点的判断\n\n\
            二、数目法\n\
            • 使用新西兰式数目法\n\
            • 只计算围空数\n\
            • 贴目{}目\n\n\
            三、简化规则\n\
            • 规则简单明了\n\
            • 易于理解和执行\n\
            • 减少争议判决\n\n\
            四、适用范围\n\
            • 适合初学者\n\
            • 网络对弈常用\n\
            • 国际比赛参考",
            self.komi()
        )
    }
}

impl Rule for NewZealandRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_nz")
    }

    fn explain(&self) -> String {
        self.full_explanation()
    }
}

// ============================================================================
// 智运会规则详细实现
// ============================================================================

/// 智运会规则详细实现
pub struct WMSGRules {
    metadata: RuleMetadata,
    board_size: u8,
    time_control: WMSGTimeControl,
}

/// 智运会时间控制
#[derive(Debug, Clone, Copy)]
pub struct WMSGTimeControl {
    /// 基本时间（秒）
    pub main_time: u32,
    /// 读秒时间（秒）
    pub byoyomi: u32,
    /// 读秒次数
    pub byoyomi_periods: u32,
}

impl Default for WMSGTimeControl {
    fn default() -> Self {
        Self {
            main_time: 3600,    // 60分钟
            byoyomi: 30,        // 30秒读秒
            byoyomi_periods: 3, // 3次读秒
        }
    }
}

impl WMSGRules {
    /// 创建智运会规则实例
    pub fn new(board_size: u8) -> Self {
        Self {
            metadata: RuleMetadata::new("智运会围棋规则", "世界智力运动会围棋规则")
                .with_origin("国际"),
            board_size,
            time_control: WMSGTimeControl::default(),
        }
    }

    /// 设置时间控制
    pub fn with_time_control(mut self, time_control: WMSGTimeControl) -> Self {
        self.time_control = time_control;
        self
    }

    /// 获取棋盘大小
    pub fn board_size(&self) -> u8 {
        self.board_size
    }

    /// 获取贴目
    pub fn komi(&self) -> f32 {
        match self.board_size {
            19 => 7.5,
            13 => 5.5,
            9 => 5.5,
            _ => 7.5,
        }
    }

    /// 获取时间控制
    pub fn time_control(&self) -> &WMSGTimeControl {
        &self.time_control
    }

    /// 获取智运会规则的完整说明
    pub fn full_explanation(&self) -> String {
        format!(
            "【智运会围棋规则】\n\n\
            世界智力运动会围棋规则，综合了各规则体系：\n\n\
            一、计算方式\n\
            • 采用中国规则的数子法\n\
            • 计算棋子数 + 围空数\n\
            • 贴目{}目\n\n\
            二、时间规定\n\
            • 基本时间：{}分钟\n\
            • 读秒时间：{}秒\n\
            • 读秒次数：{}次\n\n\
            三、特殊规定\n\
            • 统一的国际规则\n\
            • 严格的计时执行\n\
            • 标准化的裁判流程\n\n\
            四、适用范围\n\
            • 世界智力运动会\n\
            • 国际性比赛\n\
            • 国际围棋联盟认可",
            self.komi(),
            self.time_control.main_time / 60,
            self.time_control.byoyomi,
            self.time_control.byoyomi_periods
        )
    }
}

impl Rule for WMSGRules {
    fn metadata(&self) -> &RuleMetadata {
        &self.metadata
    }

    fn category(&self) -> RuleCategory {
        RuleCategory::games("go_wmsg")
    }

    fn explain(&self) -> String {
        self.full_explanation()
    }
}

// ============================================================================
// 测试模块
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_go_rules_basic() {
        let rules = GoRules::new(19);
        assert_eq!(rules.board_size(), 19);
        assert_eq!(rules.komi(), 7.5);
        assert_eq!(rules.variant(), GoVariant::Chinese);
    }

    #[test]
    fn test_chinese_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::Chinese);
        assert_eq!(rules.komi(), 7.5);
        assert_eq!(rules.scoring_method(), ScoringMethod::Area);
        assert!(!rules.allows_suicide());
        assert!(!rules.prohibits_superko());
    }

    #[test]
    fn test_japanese_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::Japanese);
        assert_eq!(rules.komi(), 6.5);
        assert_eq!(rules.scoring_method(), ScoringMethod::Territory);
        assert!(!rules.allows_suicide());
        assert!(rules.prohibits_superko());
    }

    #[test]
    fn test_korean_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::Korean);
        assert_eq!(rules.komi(), 6.5);
        assert_eq!(rules.scoring_method(), ScoringMethod::Territory);
    }

    #[test]
    fn test_ing_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::Ing);
        assert_eq!(rules.komi(), 7.0);
        assert_eq!(rules.scoring_method(), ScoringMethod::Point);
        assert!(rules.prohibits_superko());
    }

    #[test]
    fn test_new_zealand_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::NewZealand);
        assert_eq!(rules.komi(), 6.5);
        assert_eq!(rules.scoring_method(), ScoringMethod::Territory);
        assert!(rules.allows_suicide());
    }

    #[test]
    fn test_wmsg_rules() {
        let rules = GoRules::new(19).with_variant(GoVariant::WMSG);
        assert_eq!(rules.komi(), 7.5);
        assert_eq!(rules.scoring_method(), ScoringMethod::Area);
    }

    #[test]
    fn test_small_board_sizes() {
        let rules_9 = GoRules::new(9).with_variant(GoVariant::Chinese);
        assert_eq!(rules_9.komi(), 5.5);

        let rules_13 = GoRules::new(13).with_variant(GoVariant::Japanese);
        assert_eq!(rules_13.komi(), 5.5);
    }

    #[test]
    fn test_stone_opposite() {
        assert_eq!(Stone::Black.opposite(), Stone::White);
        assert_eq!(Stone::White.opposite(), Stone::Black);
    }

    #[test]
    fn test_calculate_result() {
        let rules = GoRules::new(19).with_variant(GoVariant::Chinese);

        // 黑方胜
        let result = rules.calculate_result(180, 170);
        match result {
            GoResult::BlackWins(diff) => assert!((diff - 2.5).abs() < 0.01),
            _ => panic!("Expected BlackWins"),
        }

        // 白方胜
        let result = rules.calculate_result(170, 180);
        match result {
            GoResult::WhiteWins(diff) => assert!((diff - 17.5).abs() < 0.01),
            _ => panic!("Expected WhiteWins"),
        }
    }

    // ========================================================================
    // 应氏规则测试
    // ========================================================================

    #[test]
    fn test_ing_rules_detailed() {
        let ing = IngRules::new(19);
        assert_eq!(ing.board_size(), 19);
        assert_eq!(ing.komi(), 7);
    }

    #[test]
    fn test_ing_calculate_points() {
        let ing = IngRules::new(19);
        let points = ing.calculate_points(100, 80);
        assert_eq!(points, 180);
    }

    #[test]
    fn test_ing_superko_check() {
        let ing = IngRules::new(19);
        let history = vec![1, 2, 3, 4, 5];
        assert!(ing.check_superko(&history, 3));
        assert!(!ing.check_superko(&history, 10));
    }

    // ========================================================================
    // 新西兰规则测试
    // ========================================================================

    #[test]
    fn test_new_zealand_rules_detailed() {
        let nz = NewZealandRules::new(19);
        assert_eq!(nz.board_size(), 19);
        assert_eq!(nz.komi(), 6.5);
    }

    #[test]
    fn test_new_zealand_suicide_legal() {
        let nz = NewZealandRules::new(19);
        // 新西兰规则允许自杀
        assert!(nz.is_suicide_legal(0));
    }

    // ========================================================================
    // 智运会规则测试
    // ========================================================================

    #[test]
    fn test_wmsg_rules_detailed() {
        let wmsg = WMSGRules::new(19);
        assert_eq!(wmsg.board_size(), 19);
        assert_eq!(wmsg.komi(), 7.5);
    }

    #[test]
    fn test_wmsg_default_time_control() {
        let wmsg = WMSGRules::new(19);
        let tc = wmsg.time_control();
        assert_eq!(tc.main_time, 3600);
        assert_eq!(tc.byoyomi, 30);
        assert_eq!(tc.byoyomi_periods, 3);
    }

    #[test]
    fn test_wmsg_custom_time_control() {
        let wmsg = WMSGRules::new(19).with_time_control(WMSGTimeControl {
            main_time: 1800,
            byoyomi: 60,
            byoyomi_periods: 5,
        });
        let tc = wmsg.time_control();
        assert_eq!(tc.main_time, 1800);
        assert_eq!(tc.byoyomi, 60);
        assert_eq!(tc.byoyomi_periods, 5);
    }

    #[test]
    fn test_variant_names() {
        let rules = GoRules::new(19);
        assert_eq!(rules.variant_name(), "中国规则");

        let rules = GoRules::new(19).with_variant(GoVariant::Ing);
        assert_eq!(rules.variant_name(), "应氏规则");

        let rules = GoRules::new(19).with_variant(GoVariant::WMSG);
        assert_eq!(rules.variant_name(), "智运会规则");
    }

    #[test]
    fn test_ko_rules() {
        let chinese = GoRules::new(19).with_variant(GoVariant::Chinese);
        assert_eq!(chinese.ko_rule(), KoRule::SimpleKo);

        let ing = GoRules::new(19).with_variant(GoVariant::Ing);
        assert_eq!(ing.ko_rule(), KoRule::Superko);
    }

    #[test]
    fn test_explain_method() {
        let rules = GoRules::new(19).with_variant(GoVariant::Ing);
        let explanation = rules.explain();
        assert!(explanation.contains("应氏规则"));
        assert!(explanation.contains("计点制"));
        assert!(explanation.contains("禁止全局同形"));
    }
}
