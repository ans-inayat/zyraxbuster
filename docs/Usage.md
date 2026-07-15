# Usage

New Features

- Flags:
  - `--vhost` — run VHost (host header) fuzzing
  - `--auto-filter` — baseline-response filtering to reduce noise

Examples

Basic VHost fuzzing

```bash
zyraxbuster -u http://10.129.13.57 -w subdomains.txt --vhost -H "Host: paperwork.htb"
```

With auto-filter (excludes baseline-matching responses)

```bash
zyraxbuster -u http://10.129.13.57 -w subdomains.txt --vhost --auto-filter
```

Full example with JSON output

```bash
zyraxbuster -u http://target.com -w subdomains.txt --vhost --auto-filter -t 50 --random-agent -o results.json --json
```

How It Works

- Directory mode (default): `GET http://target.com/WORD`
- VHost mode (`--vhost`): `GET http://target.com/` with header `Host: WORD.target.com`
- Auto-filter (`--auto-filter`): makes a baseline request first, then filters out responses matching the baseline (same status + same size)

For full usage and CLI options, see the README:
https://github.com/ans-inayat/zyraxbuster/blob/main/README.md
