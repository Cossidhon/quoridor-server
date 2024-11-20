This project is for learning Rust by developing a webservice to play the board game of Quoridor.

This service will be developed as a cloud native service, this means:
- It will expose a REST API
- It will be horizontally scalable when using an external PostGreSQL DB
- It will have an installation option of using either internally SQLliste or externally PostGreSQL
- It will be made available as a container image and as a kubernetes operator
- It will export an observability endpoint such that automatic horizontal scaling on kubernetes is possible

The service will:
- Provide the functionality to run multiple games at the same time
- Maintain a record of registered players. No guest mode, players need to sign up first
- Each game can have 2-4 players. With the 2 player option one of them can be the webservice itself (play against the computer)
- Provide a leaderboard functionality
- Provide a player matching functionality
- Provide a tournament functionality
- Provide an admin level interface for special maintenance tasks

For the rules of the game of Quoridor itself, see: https://en.wikipedia.org/wiki/Quoridor

This is the backend service part. The following clients are planned (in seperate repositories):
- Linux CLI/TUI version
- Linux Gnome version
- Web version (implemented as a stateless webservice)