# TODO

* [x] invoke rust from frontend

* [x] dev documentation in readme

* [x] environment variable for directory to store projects in

* [x] entry type

* [x] use String instead of OsString

* [x] data layer: 

  - [x] create project (sqlite db on disk)

  - [x] delete project  

  - [x] list projects  

  - [x] better projects abstraction than procedural API with 
    `ProjectsDir` struct

  - [x] better fs ops handling

  - [x] insert entry into project

  - [x] delete entry from project

  - [x] read entries from project

* [x] proper state management

  - [x] cache db connections

* [x] graphql interface

  - [x] error handling
 
  - [x] implement endpoints

  - [x] test  

* [ ] generate ts types from SDL 

  - [x] binary for exporting SDL

  - [x] create ts types from SDL

  - [x] write dev documentation on how to create types for ts

  - [ ] integrate into frontend  
 
* [ ] basic frontend

* [ ] secure tauri (isolation, csp, ...)

* [ ] bundling

* [ ] update mechanism