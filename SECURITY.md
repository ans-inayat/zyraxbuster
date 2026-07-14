# Security Policy

## Reporting Vulnerabilities

If you discover a security vulnerability in ZyraxBuster, please report it responsibly.

**Do NOT open a public GitHub issue for security vulnerabilities.**

Instead, please email: **ans-inayat** (via GitHub profile for contact info)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

## Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 1 week
- **Fix or mitigation**: Depends on severity, typically within 2 weeks

## Scope

Security issues include:
- Remote code execution
- Buffer overflows
- Path traversal vulnerabilities
- Command injection
- Authentication bypass
- Denial of service vulnerabilities in the tool itself

## Out of Scope

- Vulnerabilities in target systems (this is a security testing tool)
- Issues related to unauthorized use of the tool

## Responsible Disclosure

We follow responsible disclosure practices. Please:

1. Report privately first
2. Allow reasonable time for a fix
3. Avoid exploiting the vulnerability
4. Don't access or modify data that isn't yours

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Security Best Practices

When using ZyraxBuster:

- Only scan systems you own or have authorization to test
- Use rate limiting (`--delay`) to avoid overwhelming targets
- Be aware of legal implications in your jurisdiction
- Use VPN/proxy for added privacy if needed
