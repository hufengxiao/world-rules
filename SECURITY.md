# Security Policy

## Supported Versions

We release patches for security vulnerabilities regularly.

| Version | Supported          |
| ------- | ------------------ |
| 2.x     | :white_check_mark: |
| < 2.0   | :x:                |

## Reporting a Vulnerability

We take the security of World Rules seriously. If you have discovered a security vulnerability, please report it to us.

### How to Report

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via [GitHub Security Advisories](https://github.com/hufengxiao/world-rules/security/advisories/new).

### What to Include

Please include the following information in your report:

1. Description of the vulnerability
2. Steps to reproduce the issue
3. Potential impact
4. Possible solutions (if any)
5. Your contact information for follow-up

### Response Timeline

- **Initial Response**: Within 48 hours
- **Triage**: Within 7 days
- **Fix Timeline**: Depends on severity, typically within 14 days for critical issues

### Disclosure Policy

- We follow responsible disclosure practices
- We request that you give us reasonable time to fix the issue before public disclosure
- We will credit you in the security advisory (unless you prefer to remain anonymous)

## Security Best Practices

When using World Rules in your projects:

1. Always use the latest stable version
2. Keep your dependencies up to date
3. Review any custom rule implementations for potential security implications
4. Be cautious when processing untrusted input through rule validation

## Known Security Considerations

- This library is primarily a data rules and validation library
- User-supplied data should be validated before processing
- No network operations are performed by the core library
- The optional CLI feature should be used with appropriate input validation

Thank you for helping keep World Rules and our users safe!