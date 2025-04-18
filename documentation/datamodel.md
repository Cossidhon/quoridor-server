server:
- name
- motd
- state (operational/readonly/shutdown)

user:
- user_id
- email
- name
- password
- validation_code
- is_admin
- is_valid
- is_active

game:
- game_id
- state: (waiting, playing, finished)
- turn: (player_id)

player:
- player_id
- game_id
- user_id

wall:
- game_id
- player_id
- wall_id
- position
    - x
    - y

leaderboard:
- user_id
- highscore

