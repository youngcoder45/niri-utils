# Security Policy

## Supported Versions

This project is under active development.
Only the latest version on the `main` branch is considered supported.

---

## Reporting a Vulnerability

If you discover a security issue, please report it responsibly.

### Preferred Method

* Open a GitHub issue with the label `security`

### Include the Following

* Description of the vulnerability
* Steps to reproduce
* Potential impact
* Suggested fix (if available)

---

## Response Policy

* Issues will be reviewed as soon as possible
* Critical vulnerabilities will be prioritized
* Fixes will be released promptly

---

## Scope

Security considerations mainly apply to:

* Shell scripts (`lock.sh`, install scripts)
* Execution of external commands
* File permissions and paths
* Rust binaries (`niri-idle`)

---

## Best Practices for Users

* Review scripts before executing
* Avoid running unknown code with elevated privileges
* Keep system packages up to date

---

## Notes

> [!NOTE]
> This project does not currently handle sensitive user data, but care is taken to avoid unsafe system operations.
