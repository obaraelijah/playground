# TODO

* [x] invoke rust from frontend

* [x] dev documentation in readme

* [x] environment variable for directory to store projects in

* [x] entry type

* [x] use String instead of OsString

* [ ] data layer: 

  - [x] create project (sqlite db on disk)

  - [x] delete project  

  - [x] list projects  

  - [x] better projects abstraction than procedural API with 
    `ProjectsDir` struct

  - [ ] fs layer that makes sure paths are correctly formatted and
    that files exist  

  - [ ] insert entry into project

  - [ ] delete entry from project

  - [ ] read entries from project

* [ ] proper state management

  - [ ] cache db connections

* [ ] graphql interface

* [ ] secure tauri (isolation, csp, ...)