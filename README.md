# warhead-testrange
Test range to determine which stack will be used for warheads.io

## What will be tested?
They should be able to handle these things fast and well:
* SQL, both SQLite and RDBMSes
* Cryptography, e.g. Hashing, Authorization, etc.
* Filesystem, e.g. serving the Web GUI for non-full-stack apps and hosting Warheads
* Routing and MVC or MVC-alike

Other important things include resource consumption, security and scalability.
## Who are the canidates?
Some might get added or removed later on:
* .NET with ASP.NET
* Elixir with Phoenix
* Rust with Actix-Web (back-end) and Astro (front-end)
