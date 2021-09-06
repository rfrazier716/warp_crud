<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the zero_to_prod and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Thanks again! Now go create something AMAZING! :D
***
***
***
*** To avoid retyping too much info. Do a search and replace for the following:
*** rfrazier716, zero_to_prod, twitter_handle, email, Zero To Production, project_description
-->



<!-- PROJECT SHIELDS -->
<!--
*** I'm using markdown "reference style" links for readability.
*** Reference links are enclosed in brackets [ ] instead of parentheses ( ).
*** See the bottom of this document for the declaration of the reference variables
*** for contributors-url, forks-url, etc. This is an optional, concise syntax you may use.
*** https://www.markdownguide.org/basic-syntax/#reference-style-links
-->
[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]



<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://github.com/rfrazier716/warp_crud">
    <img src="https://www.rustacean.net/assets/rustacean-flat-happy.png" alt="Logo" width="200">
  </a>

<h3 align="center">Warp Todos</h3>

  <p align="center">
    Basic todo list app with session support using Rust, Warp, and MongoDB.
    <br />
    <a href="https://warp-crud-b45vr.ondigitalocean.app/">View Demo</a>
  </p>
</p>



- [About The Project](#about-the-project)
  * [Features](#features)
  * [Built With](#built-with)
- [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
- [Usage](#usage)
  * [Starting the Server](#starting-the-server)
- [License](#license)
- [Contact](#contact)
- [Acknowledgements](#acknowledgements)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>




<!-- ABOUT THE PROJECT -->
## About The Project

This project spawned from my reading Luca Palmeiri's excellent book [Zero To Production in Rust](https://www.zero2prod.com/). While his book focuses on using [Actix](https://github.com/actix/actix-web) and [Postgres](https://www.postgresql.org/), I was more interested in learning [MongoDB](https://www.mongodb.com) and found myself drawn to [Warp](https://github.com/seanmonstar/warp) due to the Filter Traits. 

At the same time I was also reading through a [Real Python Article](https://realpython.com/flask-connexion-rest-api/) on setting up a REST API. I decided to mimic my routes after the article's, which allowed me to copy their front-end code and focus on the back-end.

Fast Forward a couple months and I was ready to try my hand at deploying the webserver to Heroku or similar, but was mortified at the thought of giving random strangers on the internet write access to a database that other visitors would see! The way around this was to implement session cookies, so each user only saw their own sessions. While I was at it I also changed the "People API" to a general todo list, as I have no idea who would want to make lists of people sorted by date.

Most of the code structure comes from Luca's book and blog posts on LogRocket's Blog, I've tried to credit any relavent ones in the [Acknowledgements](#acknowledgements) section.

### Features
* Create up to a whopping 10 todo items which can be updated and deleted with the press of a button!
* synchronous Webserver with runtime configuration through config files and environment variables.
* Cached CI/CD pipeline with CircleCI including the following jobs:
    * Formatting check with `cargo fmt`
    * Linting Check with `cargo clippy`
    * vulnerability check with `cargo audit`
    * unittests run on every commit
    * Integration tests with database queries that run on commits to `main` or `development`
* Session cookies to Isolate users
* Built with the warp framework, which builds on top of hyper.
* All the security benefits of Rust

### Built With

* [Rust](https://www.rust-lang.org/)
* [Warp](https://github.com/seanmonstar/warp)
* [MongoDB](https://www.mongodb.com)

<!-- GETTING STARTED -->
## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites
It's assumed you already have Rust installed on your computer as well as cargo, follow their 
[installation instructions](https://www.rust-lang.org/tools/install) for OS specific installation guides. 

* [Docker](https://docs.docker.com/get-docker/) -- Required to run integration tests and start a local MongoDB instance.

### Installation

1. Clone the Project 
   ```sh
   git clone https://github.com/rfrazier716/warp_crud.git; cd warp_crud
   ```
2. Build the Code
    ```sh
    cargo build
    ```

## Usage
The Webserver relies on an active MongoDB instance which acts as a persistant data store. to start a local instance use the docker compose located in the [tests/docker](tests/docker) directory

```shell
cd tests/docker
docker compose up
```

### Starting the Server
To launch the web server navigate to the base directory and run. 

```shell
cargo run
```

The default configuration will start the server on `localhost:3030`. You can check the health endpoint is running with `curl`:
```shell
$ curl -i localhost:3030/health
HTTP/1.1 200 OK
content-length: 0
date: Tue, 22 Jun 2021 16:57:23 GMT
```

You can customize the startup configuration by editing the files in [config](config) and setting the `RUN_ENV` environment variable Accordingly. e.g. `RUN_ENV="Production" cargo run` will launch the webserver with the production configuration. Config files must be serializable into a `Settings` struct (see [config.rs](src/config)).

Any field in the settings struct can be provided by the command line by using the "EA" previx and using a double underscore for nested fields. e.g. to set `settings.database.uri` use the environment variable `EA_DATABASE__URI`.

<!-- LICENSE -->
## License
This Code is published under the [MIT](LICENSE.txt) license.


<!-- CONTACT -->
## Contact
Ryan Frazier - [@fotonixandgizmo](https://twitter.com/fotonixandgizmo) - Ryan@Fotonixx.com

Project Link: [https://github.com/rfrazier716/warp_crud](https://github.com/rfrazier716/warp_crud)

<!-- ACKNOWLEDGEMENTS -->
## Acknowledgements
* [Zero To Production in Rust](https://www.zero2prod.com/)
* [Warp's Documentation and Examples](https://docs.rs/warp/0.3.1/warp/)
* [MongoDB's Excellently Documented Rust Driver](https://github.com/mongodb/mongo-rust-driver)
* [Python REST APIS](https://realpython.com/flask-connexion-rest-api/)
* [Mario Zupan's](https://blog.logrocket.com/author/mariozupan/) various blogs
    * [Configuration management in Rust web services](https://blog.logrocket.com/configuration-management-in-rust-web-services/)
    * [Using MongoDB in a Rust web service](https://blog.logrocket.com/using-mongodb-in-a-rust-web-service/)
    * [Create an async CRUD web service in Rust with warp](https://blog.logrocket.com/create-an-async-crud-web-service-in-rust-with-warp/)





<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[contributors-shield]: https://img.shields.io/github/contributors/rfrazier716/warp_crud.svg?style=for-the-badge
[contributors-url]: https://github.com/rfrazier716/warp_crud/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/rfrazier716/warp_crud.svg?style=for-the-badge
[forks-url]: https://github.com/rfrazier716/warp_crud/network/members
[stars-shield]: https://img.shields.io/github/stars/rfrazier716/warp_crud.svg?style=for-the-badge
[stars-url]: https://github.com/rfrazier716/warp_crud/stargazers
[issues-shield]: https://img.shields.io/github/issues/rfrazier716/warp_crud.svg?style=for-the-badge
[issues-url]: https://github.com/rfrazier716/warp_crud/issues
[license-shield]: https://img.shields.io/github/license/rfrazier716/warp_crud.svg?style=for-the-badge
[license-url]: https://github.com/rfrazier716/warp_crud/blob/master/LICENSE.txt
