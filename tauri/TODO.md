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

* [ ] proper state management

  - [ ] cache db connections

* [ ] graphql interface
  - [ ] error type  

  - [ ] implement endpoints

  - [ ] test  
  
* [ ] secure tauri (isolation, csp, ...)

* [ ] basic frontend

* [ ] bundling

* [ ] update mechanism