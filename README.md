# RDII (Rainfall-Derived Infiltration and Inflow) analysis tool

[RDII](https://www.wef.org/globalassets/assets-wef/direct-download-library/public/03---resources/wsec-2017-fs-001-rdii-modeling-fact-sheet---final.pdf) tool using Rust and WebAssembly.

All analysis is done in the browser. No data is uploaded to any server.


# Instalation

 Clone the repository
 
 [Prepare environment](https://rustwasm.github.io/docs/book/introduction.html)
 
 Build: 
 ```sh
 wasm-pack build
 ```
 
 init npm package
 ```sh
 cd www
 npm install
 ```

 Run local server
 ```sh
 npm run server
 ```


# Redistributing

rdii-tool source code is distributed under the Apache-2.0 license.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.