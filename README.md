# RDII (Rainfall-Derived Infiltration and Inflow) Tool

[RDII](https://www.wef.org/globalassets/assets-wef/direct-download-library/public/03---resources/wsec-2017-fs-001-rdii-modeling-fact-sheet---final.pdf) tool using Rust and WebAssembly.

All analysis is done in the browser. No data is uploaded to a server.


# work with the project

Install Rust and WASM tools:

https://rustwasm.github.io/docs/book/introduction.html
 
Build Rust backend: 
```sh
$ wasm-pack build
```

Init npm package
```sh
$ cd www
(www)$ npm install
```

Run local server
```sh
(www)$ npm run server
```

Deploy application for histing on Github
```sh
(www)$ npm run deploy
```


# Redistributing

rdii-tool source code is distributed under the Apache-2.0 license.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.