/*
    Основной код программы,
    в котором обрабатываются полученные данные и передаются в вспомогательные библиотеки
    (в библиотеки передаются данные для шифрование,
    а также библиотеки используются для подключения к базе данных)
*/

use init_lib::ckeys::CKeys;
use rsa::PublicKeyPemEncoding;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::prelude::*;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::u8;
use std::{thread, time};

// Константа для ожидания включения модуля (используется для ожидания включения базы данных)
const SLEEP_FIVE_SECS: time::Duration = time::Duration::from_secs(5);
// Константы для определения шага
const FIRST_STEP: u8 = 1;
const SECOND_STEP: u8 = 2;
const THIRD_STEP: u8 = 3;
const FOUR_STEP: u8 = 4;

// Структура для хранения крипто информации пользователя
struct User {
    // username: Option<String>,
    crypt_info: Option<CKeys>,
}

// имплементация - описаны функции, которые используется для структуры
impl User {
    // функция new - добавляет в функцию получаемые данные
    fn new(_username_data: Option<String>, crypt_info_data: Option<CKeys>) -> User {
        User {
            // username: username_data,
            crypt_info: crypt_info_data,
        }
    }
}

// Структура Transit - используется при десериализации получаемого json файла
#[derive(Serialize, Deserialize)]
pub struct Transit {
    step: u8,         // этап авторизации/аутентификации или регистрации
    req_type: String, // request type - используется для определения типа данных (auth/reg)
    user: String,     // имя пользователя
    data: String,     // здесь хранятся данные при обмене (ошибка/ключ/шифрованный пароль)
}

impl Transit {
    // функция error - сюда передаётся только type из поля json,
    // функция используется для составления ошибки в непредвиденных обстоятельствах
    pub fn error(req_type_data: String) -> Transit {
        Transit {
            step: 0,
            req_type: req_type_data,
            user: String::from("ERROR"),
            data: String::from("ERROR"),
        }
    }
}

// На данный момент функция не используется,
// но была сделана для проверки пользователя на предыдущем шаге
fn _check_username(username_struct: &Option<String>, user_json: &String) -> bool {
    if let Some(username) = username_struct {
        return if user_json.eq(username) { true } else { false };
    }
    false
}

fn parse_data(req_data: &str, user_struct: &mut User) -> Result<String> {
    // Проверяем, что мы можем десериализовать полученный json файл
    return match serde_json::from_str(req_data) {
        Ok(parsed) => {
            // инициализируем переменную request_json с типом Transit,
            // в которую кладём данные с json
            let mut request_json: Transit = parsed;
            // Начинаем проверять шаги в полученном json файле
            match request_json.step {
                FIRST_STEP => {
                    // меняем шаг в json файле
                    request_json.step = SECOND_STEP;
                    // Проверяем, что req_type поле json равно auth
                    if request_json.req_type == "auth".to_string() {
                        // Проверяем, что пользователь есть в базе данных
                        if !database::check_user_redis(&request_json.user).eq("ERROR") {
                            // генерируем пару открытый и закрытый ключ
                            let encrypt_keys = init_lib::crypto_module_gen();
                            // кладём в поле data открытый ключ в формате pkcs8
                            request_json.data = encrypt_keys.public_key.to_pem_pkcs8().unwrap();
                            // присваиваем к User структуре данные о ключах
                            user_struct.crypt_info = Some(encrypt_keys);
                        } else {
                            // Если пользователя нет в базе данных, то в поля подставляем ошибку
                            request_json = Transit::error(request_json.req_type);
                        };
                        // Проверяем, что req_type поле json равно reg
                        // и проверяем, что пользователя нет в базе данных
                    } else if request_json.req_type == "reg".to_string()
                        && database::check_user_redis(&request_json.user).eq("ERROR")
                    {
                        // Генерируем пару ключей
                        let encrypt_keys = init_lib::crypto_module_gen();
                        // кладём в поле data открытый ключ в формате pkcs8
                        request_json.data = encrypt_keys.public_key.to_pem_pkcs8().unwrap();
                        // присваиваем к User структуре данные о ключах
                        user_struct.crypt_info = Some(encrypt_keys);
                    } else {
                        // подставляем ошибку если что-то пошло не так
                        request_json = Transit::error(request_json.req_type)
                    }
                }
                THIRD_STEP => {
                    // меняем шаг в json
                    request_json.step = FOUR_STEP;
                    // проверяем, что тип запроса == auth
                    if request_json.req_type == "auth".to_string() {
                        // создаём и присваиваем переменным полученные данные из json
                        let json_data = request_json.data.clone();
                        let username = request_json.user.clone();
                        // проверяем, что на прошлом этапе мы запомнили крипто данные,
                        // чтобы использовать их для дешифрования и проверки
                        if let Some(ref mut crypt_info) = user_struct.crypt_info {
                            // передаём крипто информацию с прошлого шага,
                            // декодированное значение с нынешнего шага и имя пользователя
                            // в функцию, где проверяем все полученные данные
                            if crypto_module::decrypt_and_compare_data_auth(
                                crypt_info,
                                base64::decode(json_data).unwrap(),
                                username,
                            ) {
                                // если данные проверились успешно,
                                // то в поле data в json отдаём строку "OK"
                                request_json.data = "OK".to_string()
                            } else {
                                // если данные не верны
                                // (не смогли расшифровать, неверный пароль и т.д.),
                                // то в поле data в json отдаём "FAIL"
                                request_json.data = "FAIL".to_string();
                            }
                        }
                    // проверяем, тип запроса == reg
                    } else if request_json.req_type == "reg".to_string() {
                        // создаём и присваиваем переменным полученные данные из json
                        let json_data = request_json.data.clone();
                        let username = request_json.user.clone();
                        // проверяем, что на прошлом этапе мы запомнили крипто данные,
                        // чтобы использовать их для дешифрования и проверки
                        if let Some(ref mut crypt_info) = user_struct.crypt_info {
                            // передаём крипто информацию с прошлого шага,
                            // декодированное значение с нынешнего шага и имя пользователя
                            // в функцию, где создаём нового пользователя
                            if crypto_module::decrypt_and_compare_data_reg(
                                crypt_info,
                                base64::decode(json_data).unwrap(),
                                username,
                            ) {
                                // если зарегистрировали успешно,
                                // то в поле data в json отдаём строку "OK"
                                request_json.data = "OK".to_string()
                            } else {
                                // если не смогли расшифровать или что-то пошло не так
                                // то в поле data в json отдаём "FAIL"
                                request_json.data = "FAIL".to_string();
                            }
                        }
                    }
                }
                _ => {}
            }
            // преобразовываем все данные обратно в json (сериализуем)
            let response_json = serde_json::to_string(&request_json);
            response_json
        }
        // если произошла какая-то ошибка во время обработки информации,
        // то присваиваем шаг 0 и назад отсылаем ошибку и проблему
        Err(e) => {
            let error_struct_parse: Transit = Transit {
                step: 0,
                req_type: "".to_string(),
                user: "".to_string(),
                data: format!("Error: {}", e.to_string()),
            };
            // преобразовываем все данные обратно в json (сериализуем)
            let response_json = serde_json::to_string(&error_struct_parse);
            response_json
        }
    };
}

// функция используется для отправки данных в сокет
fn send_data(mut stream: &TcpStream, request_message: String) {
    let response = format!("{}", request_message);
    stream.write(response.as_bytes()).unwrap();
}

fn handle_connection(mut stream: TcpStream, user_struct: &mut User) {
    // инициализируем переменную buffer для хранения и получения данных с сокета
    let mut buffer = [0 as u8; 2048];

    // обозначаем поток, чтобы отдавать данные не ожидая принятия новых
    stream
        .set_nonblocking(true)
        .expect("Failed to set nonblocking mode");

    // создаём цикл с чтением сокета до тех пор, пока не получим данные
    loop {
        match stream.read(&mut buffer) {
            // если мы получили данные, то прерываем цикл,
            // остальные кейсы в цикле обрабатывают ошибки буфера
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                println!("encountered IO error: {}", e);
                continue;
            }
        }
    }

    // преобразуем буффер, вычищая все нулевые байты в конце
    let string_buffer = String::from_utf8_lossy(&buffer)
        .trim_matches(char::from(0))
        .to_string();

    // лог информация для отслеживания получаемых данных
    println!("Received from front:\n{}", string_buffer);

    // Вызываем функцию, в которой парсим полученный json файл
    let serialized_data = parse_data(string_buffer.as_str(), user_struct);

    // Отправляем данные
    match serialized_data {
        Ok(parsed) => send_data(&stream, parsed),
        Err(e) => send_data(&stream, e.to_string()),
    }

    // отчищаем буфер для получения новых данных
    stream.flush().unwrap();
}

fn main() {
    // вызываем функцию для засыпания программы (используется для ожидания включения базы данных)
    thread::sleep(SLEEP_FIVE_SECS);
    println!("Sidzher_crypto module started");
    // Проверяется коннект к базе данных
    match init_lib::init_redis_db_connection() {
        // Если подключение успешно, то продолжается работа
        Ok(_) => {
            println!("Connected to database!");
            // в переменной listener задаём адрес и порт для обмена данными
            let listener = TcpListener::bind("127.0.0.1:5141");
            // Если подключение успешно, то продолжаем работу
            if let Ok(listener_ok) = listener {
                // инициализируем структуру User для работы с данными
                let mut init_user = User::new(Some("".to_string()), Some(CKeys::flush()));
                // Запускаем прослушивание на порту
                for stream in listener_ok.incoming() {
                    // инициализируем переменную stream для работы с сетевым потоком
                    let stream = stream.unwrap();
                    handle_connection(stream, &mut init_user);
                }
                // Если возникла ошибка, то выдаётся ошибка о том, что данный порт занят
            } else {
                println!("Error bind listener!")
            }
        }
        // Если не удалось подключиться к базе данных, то выдаётся ошибка
        Err(error) => {
            println!(
                "Failed connect to database!\n{}\nSidzher_crypto module stopped",
                error
            );
            main() // Опасно, по возможности избавиться
        }
    }
}
