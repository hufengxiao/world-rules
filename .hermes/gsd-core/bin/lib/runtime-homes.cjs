"use strict";
/**
 * runtime-homes.cts — canonical runtime → global config/skills directory mapping.
 *
 * Single source of truth for resolving the global config base directory and
 * the correct global skills directory for every GSD-supported runtime.
 *
 * ADR-457 build-at-publish: the hand-written bin/lib/runtime-homes.cjs
 * collapsed to a TypeScript source of truth. Behaviour is preserved
 * byte-for-behaviour from the prior hand-written .cjs; only types are added.
 *
 * Runtime-specific notes:
 *   hermes  — GSD skills nest under skills/gsd/<skillName>/ (not the flat
 *             skills/<skillName>/ layout used by all other runtimes).
 *   cline   — Skills-capable since v3.48.0 (#782). SKILL.md files live at
 *             ~/.cline/skills/<skillName>/SKILL.md (same flat layout as cursor/codex).
 *             .clinerules is also emitted (rules-based compatibility layer).
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.resolveAntigravityGlobalDir = resolveAntigravityGlobalDir;
exports.getGlobalConfigDir = getGlobalConfigDir;
exports.getGlobalSkillsBase = getGlobalSkillsBase;
exports.getGlobalSkillDir = getGlobalSkillDir;
exports.getGlobalSkillDisplayPath = getGlobalSkillDisplayPath;
const node_os_1 = __importDefault(require("node:os"));
const node_path_1 = __importDefault(require("node:path"));
const node_fs_1 = __importDefault(require("node:fs"));
/**
 * Expand a leading ~ to os.homedir().
 */
function expandTilde(p) {
    if (!p)
        return p;
    if (p.startsWith('~/') || p === '~')
        return node_path_1.default.join(node_os_1.default.homedir(), p.slice(1));
    return p;
}
/**
 * Resolve Antigravity global config dir across 1.x and 2.x layouts.
 */
function resolveAntigravityGlobalDir(opts = {}) {
    const env = opts.env ?? process.env;
    const home = opts.home ?? node_os_1.default.homedir();
    const existsSyncFn = opts.existsSync ?? node_fs_1.default.existsSync;
    if (env['ANTIGRAVITY_CONFIG_DIR'])
        return expandTilde(env['ANTIGRAVITY_CONFIG_DIR']);
    const base = node_path_1.default.join(home, '.gemini');
    const candidates = [
        node_path_1.default.join(base, 'antigravity'),
        node_path_1.default.join(base, 'antigravity-ide'),
        node_path_1.default.join(base, 'antigravity-cli'),
    ];
    for (const candidate of candidates) {
        if (existsSyncFn(candidate))
            return candidate;
    }
    return node_path_1.default.join(base, 'antigravity');
}
/**
 * Return the global config base directory for the given runtime.
 * Respects the same env-var overrides as bin/install.js getGlobalDir().
 *
 * @param runtime   - The runtime identifier (e.g. 'claude', 'opencode').
 * @param explicitDir - If provided and non-empty, returned immediately after
 *   tilde-expansion, overriding all env-var and default logic. This matches
 *   the behaviour of bin/install.js getGlobalDir(runtime, explicitDir).
 */
function getGlobalConfigDir(runtime, explicitDir) {
    if (explicitDir)
        return expandTilde(explicitDir);
    const home = node_os_1.default.homedir();
    const env = process.env;
    switch (runtime) {
        // ── Hermes Agent ──────────────────────────────────────────────────────────
        case 'claude':
            return env['CLAUDE_CONFIG_DIR'] ? expandTilde(env['CLAUDE_CONFIG_DIR']) : node_path_1.default.join(home, '.claude');
        // ── Cursor ───────────────────────────────────────────────────────────────
        case 'cursor':
            return env['CURSOR_CONFIG_DIR'] ? expandTilde(env['CURSOR_CONFIG_DIR']) : node_path_1.default.join(home, '.cursor');
        // ── Gemini CLI ───────────────────────────────────────────────────────────
        case 'gemini':
            return env['GEMINI_CONFIG_DIR'] ? expandTilde(env['GEMINI_CONFIG_DIR']) : node_path_1.default.join(home, '.gemini');
        // ── Codex ────────────────────────────────────────────────────────────────
        case 'codex':
            return env['CODEX_HOME'] ? expandTilde(env['CODEX_HOME']) : node_path_1.default.join(home, '.codex');
        // ── Grok Build ───────────────────────────────────────────────────────────
        case 'grok':
            return env['GROK_AGENTS_HOME'] ? expandTilde(env['GROK_AGENTS_HOME']) : node_path_1.default.join(home, '.agents');
        // ── Copilot (VS Code) ────────────────────────────────────────────────────
        case 'copilot':
            if (env['COPILOT_CONFIG_DIR'])
                return expandTilde(env['COPILOT_CONFIG_DIR']);
            if (env['COPILOT_HOME'])
                return expandTilde(env['COPILOT_HOME']);
            return node_path_1.default.join(home, '.copilot');
        // ── Antigravity ──────────────────────────────────────────────────────────
        case 'antigravity':
            return resolveAntigravityGlobalDir({ env, home });
        // ── Windsurf ─────────────────────────────────────────────────────────────
        case 'windsurf':
            return env['WINDSURF_CONFIG_DIR']
                ? expandTilde(env['WINDSURF_CONFIG_DIR'])
                : node_path_1.default.join(home, '.codeium', 'windsurf');
        // ── Augment ──────────────────────────────────────────────────────────────
        case 'augment':
            return env['AUGMENT_CONFIG_DIR'] ? expandTilde(env['AUGMENT_CONFIG_DIR']) : node_path_1.default.join(home, '.augment');
        // ── Trae ─────────────────────────────────────────────────────────────────
        case 'trae':
            return env['TRAE_CONFIG_DIR'] ? expandTilde(env['TRAE_CONFIG_DIR']) : node_path_1.default.join(home, '.trae');
        // ── Qwen Code ────────────────────────────────────────────────────────────
        case 'qwen':
            return env['QWEN_CONFIG_DIR'] ? expandTilde(env['QWEN_CONFIG_DIR']) : node_path_1.default.join(home, '.qwen');
        // ── Hermes Agent ─────────────────────────────────────────────────────────
        case 'hermes':
            return env['HERMES_HOME'] ? expandTilde(env['HERMES_HOME']) : node_path_1.default.join(home, '.hermes');
        // ── CodeBuddy ────────────────────────────────────────────────────────────
        case 'codebuddy':
            return env['CODEBUDDY_CONFIG_DIR'] ? expandTilde(env['CODEBUDDY_CONFIG_DIR']) : node_path_1.default.join(home, '.codebuddy');
        // ── Cline ────────────────────────────────────────────────────────────────
        case 'cline':
            return env['CLINE_CONFIG_DIR'] ? expandTilde(env['CLINE_CONFIG_DIR']) : node_path_1.default.join(home, '.cline');
        // ── OpenCode (XDG) ───────────────────────────────────────────────────────
        case 'opencode': {
            if (env['OPENCODE_CONFIG_DIR'])
                return expandTilde(env['OPENCODE_CONFIG_DIR']);
            if (env['OPENCODE_CONFIG'])
                return node_path_1.default.dirname(expandTilde(env['OPENCODE_CONFIG']));
            if (env['XDG_CONFIG_HOME'])
                return node_path_1.default.join(expandTilde(env['XDG_CONFIG_HOME']), 'opencode');
            return node_path_1.default.join(home, '.config', 'opencode');
        }
        // ── Kilo (XDG) ───────────────────────────────────────────────────────────
        case 'kilo': {
            if (env['KILO_CONFIG_DIR'])
                return expandTilde(env['KILO_CONFIG_DIR']);
            if (env['KILO_CONFIG'])
                return node_path_1.default.dirname(expandTilde(env['KILO_CONFIG']));
            if (env['XDG_CONFIG_HOME'])
                return node_path_1.default.join(expandTilde(env['XDG_CONFIG_HOME']), 'kilo');
            return node_path_1.default.join(home, '.config', 'kilo');
        }
        // ── Default (Claude fallback) ─────────────────────────────────────────────
        default:
            return env['CLAUDE_CONFIG_DIR'] ? expandTilde(env['CLAUDE_CONFIG_DIR']) : node_path_1.default.join(home, '.claude');
    }
}
/**
 * Return the global skills base directory for the given runtime.
 * Most runtimes: <configDir>/skills
 * Hermes: <configDir>/skills/gsd  (nested category layout — #2841)
 * Cline ≥ v3.48.0: <configDir>/skills  (SKILL.md-based global skills — #782)
 */
function getGlobalSkillsBase(runtime) {
    if (runtime === 'hermes') {
        const configDir = getGlobalConfigDir(runtime);
        return node_path_1.default.join(configDir, 'skills', 'gsd');
    }
    // Kilo Code discovers global skills from ~/.kilo/skills/ (HOME-relative),
    // independent of the XDG-based config dir (~/.config/kilo) used for commands.
    // See: https://kilo.ai/docs/customize/skills
    // "Global skills are located in the `.kilo` directory within your Home
    //  directory: ~/.kilo/skills/"
    if (runtime === 'kilo')
        return node_path_1.default.join(node_os_1.default.homedir(), '.kilo', 'skills');
    const configDir = getGlobalConfigDir(runtime);
    return node_path_1.default.join(configDir, 'skills');
}
/**
 * Return the full path to a specific skill's directory for the given runtime.
 */
function getGlobalSkillDir(runtime, skillName) {
    const base = getGlobalSkillsBase(runtime);
    if (base === null)
        return null;
    return node_path_1.default.join(base, skillName);
}
/**
 * Return a human-readable display path for a global skill (for log messages).
 */
function getGlobalSkillDisplayPath(runtime, skillName) {
    const dir = getGlobalSkillDir(runtime, skillName);
    if (!dir)
        return `(${runtime} does not use a skills directory)`;
    // Replace homedir prefix with ~ for readability
    const home = node_os_1.default.homedir();
    return dir.startsWith(home) ? '~' + dir.slice(home.length) : dir;
}
