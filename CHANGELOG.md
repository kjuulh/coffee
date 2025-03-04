# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-03-04

### Added
- remove cuddle
- now with the ability to trim out diff
- use remote origin
- add create pr command

### Fixed
- *(deps)* update rust crate serde_json to v1.0.138
- *(deps)* update rust crate serde_json to v1.0.137
- *(deps)* update rust crate serde_json to v1.0.136
- *(deps)* update rust crate serde_json to v1.0.135
- *(deps)* update rust crate serde_json to v1.0.134
- *(deps)* update rust crate webbrowser to v1.0.3
- *(deps)* update rust crate url to v2.5.4
- *(deps)* update rust crate serde_json to v1.0.133
- *(deps)* update rust crate url to v2.5.3
- *(deps)* update rust crate reqwest to v0.12.9
- *(deps)* update rust crate serde_json to v1.0.132
- *(deps)* update rust crate serde_json to v1.0.129
- *(deps)* update rust crate reqwest to v0.12.8
- *(deps)* update rust crate webbrowser to v1.0.2
- *(deps)* update rust crate serde_json to v1.0.127
- *(deps)* update rust crate serde_json to v1.0.126

### Other
- *(deps)* update all dependencies
- *(deps)* update rust crate clap to v4.5.31
- *(deps)* update all dependencies
- *(deps)* update rust crate clap to v4.5.30
- *(deps)* update rust crate clap to v4.5.29
- *(deps)* update rust crate clap to v4.5.28
- *(deps)* update rust crate axum to v0.8.2
- *(deps)* update rust crate clap to v4.5.27
- *(deps)* update rust crate clap to v4.5.26
- *(deps)* update rust crate tokio to v1.43.0
- *(deps)* update rust crate clap to v4.5.24
- *(deps)* update all dependencies
- *(deps)* update rust crate anyhow to v1.0.95
- *(deps)* update rust crate clap to v4.5.23
- *(deps)* update all dependencies
- *(deps)* update rust crate tracing-subscriber to v0.3.19
- *(deps)* update rust crate tracing to v0.1.41
- *(deps)* update rust crate axum to v0.7.9
- *(deps)* update rust crate axum to v0.7.8
- *(deps)* update rust crate clap to v4.5.21
- *(deps)* update rust crate tokio to v1.41.1
- *(deps)* update rust crate anyhow to v1.0.93
- *(deps)* update rust crate anyhow to v1.0.92
- *(deps)* update all dependencies
- *(deps)* update all dependencies
- *(deps)* update rust crate clap to v4.5.20
- *(deps)* update rust crate clap to v4.5.19
- *(deps)* update rust crate axum to v0.7.7
- *(deps)* update all dependencies
- *(deps)* update rust crate anyhow to v1.0.89
- *(deps)* update rust crate anyhow to v1.0.88
- *(deps)* update all dependencies
- *(deps)* update all dependencies
- *(deps)* update rust crate tokio to v1.40.0
- *(deps)* update all dependencies (#14)
  This PR contains the following updates:
  
  | Package | Type | Update | Change |
  |---|---|---|---|
  | [axum](https://github.com/tokio-rs/axum) | workspace.dependencies | minor | `0.6.20` -> `0.7.0` |
  | [inquire](https://github.com/mikaelmello/inquire) | workspace.dependencies | minor | `0.6.2` -> `0.7.0` |
  | [reqwest](https://github.com/seanmonstar/reqwest) | dependencies | minor | `0.11.18` -> `0.12.0` |
  | [webbrowser](https://github.com/amodm/webbrowser-rs) | dependencies | major | `0.8.10` -> `1.0.0` |
  
  ---
  
  ### Release Notes
  
  <details>
  <summary>tokio-rs/axum (axum)</summary>
  
  ### [`v0.7.5`](https://github.com/tokio-rs/axum/releases/tag/axum-v0.7.5): axum - v0.7.5
  
  [Compare Source](https://github.com/tokio-rs/axum/compare/axum-v0.7.4...axum-v0.7.5)
  
  -   **fixed:** Fixed layers being cloned when calling `axum::serve` directly with
  a `Router` or `MethodRouter` ([#&#8203;2586])
  -   **fixed:** `h2` is no longer pulled as a dependency unless the `http2` feature
  is enabled ([#&#8203;2605])

## [0.1.0] - 2024-08-21

### Added
- use actual drone && cuddle setup
- remove unused stuff
- update
- with clone and list
- with proper stdout and whatnot
- with repo clone
- add coffee repo view -w
- add readme
- fix api
- new folder
- with install
- unknown linux
- with volume again
- list
- without volume
- with depends on ci
- with volume
- with shared mount
- add drone
- with support for closed prs
- add install script
- with open browser
- with test branch
- with swagger client
- with beginning of command structure
- with coffee base
- add base rust cli

### Other
- *(deps)* update rust crate anyhow to 1.0.74
- *(deps)* update rust crate clap to 4.3.21
- *(deps)* update rust crate axum to 0.6.20
- *(deps)* update all dependencies
- Merge pull request 'chore(deps): update all dependencies' (#9) from renovate/all into main

- Merge pull request 'chore(deps): update rust crate clap to 4.3.8' (#8) from renovate/all into main

- Merge pull request 'chore(deps): update rust crate clap to 4.3.6' (#7) from renovate/all into main

- Merge pull request 'chore(deps): update rust crate clap to 4.3.5' (#6) from renovate/all into main

- Merge pull request 'fix(deps): update all dependencies' (#4) from renovate/all into main

- Merge pull request 'Configure Renovate' (#2) from renovate/configure into main

Reviewed-on: https://git.front.kjuulh.io/kjuulh/coffee/pulls/2

- Merge branch 'main' into renovate/configure

- debug some more
- fix cargo path
- Add renovate.json

- Merge pull request 'feat/with-test-branch' (#3) from feat/with-test-branch into main

Reviewed-on: https://git.front.kjuulh.io/kjuulh/coffee/pulls/3

- Merge branch 'main' into feat/with-test-branch

- Merge pull request 'feat: with test branch' (#1) from feat/with-test-branch into main

Reviewed-on: https://git.front.kjuulh.io/kjuulh/coffee/pulls/1

