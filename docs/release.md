# Release process

This project follows [semantic versioning](https://semver.org/). The following documentation will
refer to `X.Y.Z` as _major_, _minor_ and _patch_ version.

## Releasing one or more crates

### Prerequisites

- [cargo release](https://github.com/crate-ci/cargo-release/)

### Steps

1. Make sure you have the latest version of the `main` branch:

   ```sh
   $ git checkout main && git pull
   ```

1. Create the new release, replace `0.1.2` with the NEW version number:

   ```sh
   $ cargo release --workspace --sign-tag --tag-prefix "" --execute 0.1.2
   ```

1. Wait for the
   [Release CI/CD Workflow](https://github.com/filecoin-station/zinnia/actions/workflows/release.yml)
   to finish. This usually takes about 25-30 minutes.

1. Find the Draft Release created by the Release workflow in
   [releases](https://github.com/filecoin-station/zinnia/releases)

1. Click on the button `Generate release notes`. Review the list of commits and pick a few notable
   ones. Add a new section `Highlights ✨` at the top of the release notes and describe the selected
   changes.

1. Click on the green button `Publish release`

## Testing the release build pipeline

### Linux binaries

We need to link the Linux binaries with a glibc version that's compatible with the target system, e.g. the Node.js docker image used by Checker Node.

Verification steps (assuming Apple Silicon/arm64 machine):

1. Download the "archives" for your platform (e.g. arm64 for Apple Silicon chips) from the GitHub Actions workflow run, e.g. https://github.com/filecoin-station/zinnia/actions/runs/4687576517

2. Extract the archive, then extract the `zinnia` & `zinniad` archives.

3. Assuming you have the extracted files in `~/Downloads/archives-linux-arm64`, run the following Docker command:

   ```
   ❯ docker run -it --entrypoint /bin/bash -v ~/Downloads/archives-linux-arm64:/archives ubuntu:focal
   ```

   _Note: This command uses the Ubuntu version Focal Fossa (20.04), which was the oldest Ubuntu LTS version with standard support at the time of writing this document._

4. Then, inside the running Docker container.

   Smoke test:

   ```
   root@33a680b31043:/# /archives/zinniad --version
   zinniad 0.22.0
   ```

   Run some JavaScript to test V8 low-level stuff:

   ```
   root@33a680b31043:/# echo 'console.log(await fetch('https://github.com/'))' > test.js
   root@33a680b31043:/# /archives/zinnia run test.js
   Response {
     status: 200,
   (...)
   ```
