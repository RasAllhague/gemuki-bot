# migration database

## db tables

### game

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| UQ | title | varchar | 255 | N |  |
| | description | varchar | 500 | Y | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |
| | modify_date | timestamp | | Y | |
| | modify_user_id | bigint | | Y | |

### gamekey

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| FK | game_id | integer | | N | |
| FK | platform_id | integer | | N | |
| UQ | value | varchar | 255 | N | |
| | keystate | varchar | 10 | N | Enum |
| | page_link | varchar | 500 | Y | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |
| | modify_date | timestamp | | Y | |
| | modify_user_id | bigint | | Y | |

### keylist

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| UQ | name | varchar | 50 | N | Unique with owner_id |
| | description | varchar | 255 | Y | |
| UQ | owner_id | bigint | | N | Unique with name |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |
| | modify_date | timestamp | | Y | |
| | modify_user_id | bigint | | Y | |

### keylist_access

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | keylist_id | int | | N | |
| | target_user_id | bigint | | N | |
| | access_right | ACCESSRIGHT | | N | [READ, WRITE, FULL, ADMIN] |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |
| | modify_date | timestamp | | Y | |
| | modify_user_id | bigint | | Y | |

### keylist_key

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | keylist_id | int | | N | |
| | gamekey_id | int | | N | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |

### platform

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | name | varchar | 50 | N | |
| | store_link | varchar | 500 | N | |

### key_raffle 

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| UQ | name | varchar | 50 | N | Unique with owner_id |
| | description | varchar | 255 | Y | |
| | image_link | varchar  | 255 | Y | |
| UQ | owner_id | bigint | | N | Unique with name |
| | start_at | timestamp | | Y | |
| | end_at | timestamp | | Y | |
| | duration_in_seconds | integer | | Y | |
| | possible_winners | integer | | Y | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |
| | modify_date | timestamp | | Y | |
| | modify_user_id | bigint | | Y | |

### key_raffle_key

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | key_raffle_id | int | | N | |
| | gamekey_id | int | | N | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |

### key_raffle_winner

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | winner_id | bigint | | N | |
| | key_raffle_key_id | int | | N | |
| | create_date | timestamp | | N | |
| | create_user_id | bigint | | N | |