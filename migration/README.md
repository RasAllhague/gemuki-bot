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

### platform

| KEY | NAME | TYPE | SIZE | NULLABLE | NOTES |
|---|---|---|---|---|---|
| PK | id | integer | | N | AutoIncrement |
| | name | varchar | 50 | N | |
| | store_link | varchar | 500 | N | |