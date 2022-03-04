[![Check](https://github.com/CNDspace/SidZher_crypto/actions/workflows/check.yml/badge.svg?branch=master)](https://github.com/CNDspace/SidZher_crypto/actions/workflows/check.yml)

# SidZher crypto project

### RU Краткое описание
Данный проект используется для генерации ассиметричных ключей и взаимодействией с базой данных во время работы основного проекта SidZher, проект написан на Rust для увеличения скорости генерации ключей шифрования.

Автор: Александр Жердев

### EN Little description
This project is used to generate asymmetric keys and interact with the database while the main SidZher project is running, the project is written in Rust to increase the speed of generating encryption keys.

Author: Alexander Zherdev

## Build and run
### Debug build
```shell
cargo run
```
Or use compile build in `<project_path>/target/debug/sidzher_crypto_bin`
```shell
cargo build
```

Debug build in runtime slow than release, if you not debug Sidzher crypto use release build

### Release build
```shell
cargo run --release
```
Or use compile build in `<project_path>/target/release/sidzher_crypto_bin`:
```shell
cargo build --release
```

## Code flow graph
```mermaid
stateDiagram-v2
    init_database_connection --> create_TcpListener
    create_TcpListener --> handle_connection
    input_data --> handle_connection
    handle_connection --> parse_data
    handle_connection --> generate_keys
    generate_keys --> match_step_and_req_type
    parse_data --> match_step_and_req_type
    parse_data --> error_parse_data
    error_parse_data --> send_error
    match_step_and_req_type --> first_step_and_type_auth
    match_step_and_req_type --> first_step_and_type_reg
    match_step_and_req_type --> third_step_and_type_auth
    match_step_and_req_type --> third_step_and_type_reg
    first_step_and_type_auth --> check_user_in_database
    first_step_and_type_reg --> add_new_user_in_db
    add_new_user_in_db --> create_json_with_public_key
    third_step_and_type_reg --> decrypt_data
    decrypt_data --> hash_pass_bcrypt
    hash_pass_bcrypt --> add_pass_to_db
    add_pass_to_db --> send_ok_or_fail
    check_user_in_database  --> error_check_user
    error_check_user --> send_error
    check_user_in_database --> create_json_with_public_key
    create_json_with_public_key --> send_data
    third_step_and_type_auth --> decrypt_and_compare_data
    decrypt_and_compare_data --> veryfy_pass_bcrypt
    veryfy_pass_bcrypt --> check_user_in_db
    check_user_in_db --> send_ok_or_fail

```

## Transfer type between server and Sidzher Crypto

### We use json, struct here:

```json
{
  "step": 1,
  "req_type": "<type>",
  "user": "<user>",
  "data": ""
}

```

`step` is used for phased synchronization between the server and SidZher crypto module

`req_type` can be `auth` or `reg`, where:

- `auth` - Used for authentificate users
- `reg` - Used for register new users

`user` field used for users login name

`data` field used for transfer public key, encrypted data and answer for login


