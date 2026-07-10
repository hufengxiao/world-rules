"use strict";
/**
 * Thin adapter — sources schema data from the manifest via the generated
 * Configuration Module. All inline literals have been removed; the manifest
 * at gsd-core/bin/shared/config-schema.manifest.json is the single source of truth.
 *
 * Imported by:
 *   - config.cjs (isValidConfigKey validator)
 *   - core.cjs
 *   - many tests (config-schema.property.test.cjs, bug-*, feat-*, etc.)
 *
 * See Phase 2 Cycle 5 (#3536) — schema manifest migration.
 *
 * ADR-457 build-at-publish: the hand-written bin/lib/config-schema.cjs collapsed
 * to a TypeScript source of truth. Behaviour is preserved byte-for-behaviour from
 * the prior hand-written .cjs; only types are added.
 */
const configuration_cjs_1 = require("./configuration.cjs");
/**
 * Returns true if keyPath is a valid config key (exact, dynamic pattern, or runtime state).
 */
function isValidConfigKey(keyPath) {
    if (configuration_cjs_1.VALID_CONFIG_KEYS.has(keyPath))
        return true;
    if (configuration_cjs_1.RUNTIME_STATE_KEYS.has(keyPath))
        return true;
    return configuration_cjs_1.DYNAMIC_KEY_PATTERNS.some((p) => p.test(keyPath));
}
module.exports = { VALID_CONFIG_KEYS: configuration_cjs_1.VALID_CONFIG_KEYS, RUNTIME_STATE_KEYS: configuration_cjs_1.RUNTIME_STATE_KEYS, DYNAMIC_KEY_PATTERNS: configuration_cjs_1.DYNAMIC_KEY_PATTERNS, isValidConfigKey };
