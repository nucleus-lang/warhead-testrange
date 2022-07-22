# warhead-testrange
Test range to determine which stack will be used for warheads.io

## What will be tested?
They should be able to handle these things fast and well:
* SQL, both SQLite and RDBMSes
* Cryptography, e.g. Hashing, Authorization, etc.
* Filesystem, e.g. serving the Web GUI for non-full-stack apps and hosting Warheads
* Routing and MVC or MVC-alike

Other important things include resource consumption, security and scalability.

## Who are the candidates?

Some might get added or removed later on.

| Framework | Programming language | Dependencies
| --------  | -------------------  | ------------
| [Phoenix](https://www.phoenixframework.org/) | [Elixir](https://en.wikipedia.org/wiki/Elixir_(programming_language)) |
| [Vapor](https://vapor.codes/) | [Swift](https://en.wikipedia.org/wiki/Swift_(programming_language)) | [Front-end framework](https://astro.build/)
| [ASP.NET Core](https://en.wikipedia.org/wiki/ASP.NET_Core) | [C#](https://en.wikipedia.org/wiki/C_Sharp_(programming_language)) &bull; [F#](https://en.wikipedia.org/wiki/F_Sharp_(programming_language)) &bull; [Visual Basic (.NET)](https://en.wikipedia.org/wiki/Visual_Basic_.NET) | |
| [Uno](https://platform.uno/) (WASM) | [C#](https://en.wikipedia.org/wiki/C_Sharp_(programming_language)) &bull; [F#](https://en.wikipedia.org/wiki/F_Sharp_(programming_language)) &bull; [Visual Basic (.NET)](https://en.wikipedia.org/wiki/Visual_Basic_.NET) | |
| [Play](https://www.playframework.com/) | [Scala](https://en.wikipedia.org/wiki/Scala_(programming_language)) &bull; [Kotlin](https://en.wikipedia.org/wiki/Kotlin_(programming_language)) | |
| [Spring](https://spring.io/) | [Scala](https://en.wikipedia.org/wiki/Scala_(programming_language)) &bull; [Kotlin](https://en.wikipedia.org/wiki/Kotlin_(programming_language)) | |
| [Ktor](https://ktor.io/) | [Kotlin](https://en.wikipedia.org/wiki/Kotlin_(programming_language)) &bull; [Scala](https://en.wikipedia.org/wiki/Scala_(programming_language)) |
| [Leafo](https://leafo.net/lapis/) | [MoonScript](https://moonscript.org/) &bull; [Lua](https://en.wikipedia.org/wiki/Lua_(programming_language)) | |
| [Actix-Web](https://actix.rs/) | [Rust](https://en.wikipedia.org/wiki/Rust_(programming_language)) | [Astro](https://astro.build/)
<!-- Rust with Actix-Web (back-end) and Astro (front-end) -->
