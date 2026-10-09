# Third-party notices

The MIT license in [`LICENSE`](LICENSE) covers this project's own code. The
components below keep their own licenses.

## AVR8js — derived work (MIT)

The converted behavioral suites (`src/suites/`), their generator
(`tools/convert.cjs`) and `conversion-manifest.json` are derived from AVR8js
(<https://github.com/wokwi/avr8js>), pinned as a submodule at revision
`bee6f0a94e0e27786f6bc21aee3c775849fb50fd`.

Generated files carry an SPDX header identifying the upstream copyright. The
upstream notice is retained below.

```
The MIT License (MIT)

Copyright (c) 2019-2025 Uri Shaked

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
```

## Node and TypeScript — development tooling, not distributed

Node and TypeScript are used only to regenerate `src/suites/`, pinned through
`tools/package-lock.json`. They are not part of the contract that is compiled or
of any produced artifact.

## Other third-party components

The engine, the `arduino-cli` end-to-end test and the Microchip/Atmel datasheet
references live in [`avr8rs`](https://github.com/michaelnavazhylau/avr8rs) and are
noticed there, together with the raylib front-end in
[`arduino-sim-rs`](https://github.com/michaelnavazhylau/arduino-sim-rs).
