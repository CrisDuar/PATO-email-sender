#[macro_use]
extern crate rocket;
extern crate dotenv;

use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::serde::{json::Json, Deserialize, Serialize};


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmailToken {
    user: String,
    token: String,
    minutos: String,
    destinatario: String,
}


#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse {
    message: String,
}


fn build_smtp() -> Result<SmtpTransport, String> {
    let user = env::var("Email")
        .map_err(|_| "Variable de entorno 'Email' no encontrada".to_string())?;
    let pass = env::var("Password")
        .map_err(|_| "Variable de entorno 'Password' no encontrada".to_string())?;

    SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| format!("Error configurando SMTP: {:?}", e))
        .map(|t| t.credentials(Credentials::new(user, pass)).build())
}


#[post("/sendEmailToken", data = "<email>")]
fn send_email_token(email: Json<EmailToken>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    let body = format!(
    r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #ffffff; margin: 0; padding: 0; }}
            .container {{ max-width: 520px; margin: 0 auto; padding: 40px 30px; }}
            .content {{ color: #1f2933; font-size: 15px; line-height: 1.6; }}
            .token-box {{ text-align: center; background-color: #EAF9F1; border: 1px solid #2FBF83; border-radius: 8px; padding: 18px; margin: 24px 0; }}
            .token {{ font-size: 26px; font-weight: bold; letter-spacing: 4px; color: #1a8f5e; }}
            .footer {{ text-align: center; font-size: 11px; color: #9aa5b1; margin-top: 30px; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="content">
                <p>Hola <strong>{}</strong>,</p>
                <p>Le enviamos el token de restablecimiento de la contraseña que usted solicitó:</p>
            </div>
            <div class="token-box">
                <span class="token">{}</span>
            </div>
            <div class="content">
                <p>Este token fue programado para vencerse en {} minutos. Recuerda que si el tiempo caduca deberás realizar una solicitud nueva para obtener otro token.</p>
            </div>
            <div class="footer">
                <p>Este mensaje fue generado automáticamente.<br>Por favor no respondas a este correo.</p>
            </div>
        </div>
    </body>
    </html>
    "#,
    email.0.user,
    email.0.token,
    email.0.minutos
);

    let sender = env::var("Email")
        .map_err(|_| Json(ApiResponse { message: "Variable de entorno 'Email' no encontrada".to_string() }))?;
    let from = format!("PATO <{}>", sender);

    let message = Message::builder()
        .from(
            from.parse()
                .map_err(|e| Json(ApiResponse { message: format!("Remitente inválido: {:?}", e) }))?,
        )
        .to(email
            .destinatario
            .parse()
            .map_err(|e| Json(ApiResponse { message: format!("Destinatario inválido: {:?}", e) }))?)
        .subject("Token de recuperación")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|e| Json(ApiResponse { message: format!("Error construyendo mensaje: {:?}", e) }))?;

    let smtp = build_smtp()
        .map_err(|e| Json(ApiResponse { message: e }))?;

    smtp.send(&message)
        .map(|_| Json(ApiResponse { message: "Correo de token enviado correctamente".into() }))
        .map_err(|e| Json(ApiResponse { message: format!("Error enviando correo: {:?}", e) }))
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok(); // <------ Prod
    rocket::build().mount("/api", routes![send_email_token])
}
/*
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]

struct Email {
    user: String,
    hora_inicio: String,   // snake_case: renombrado de horaInicio
    hora_final: String,    // snake_case: renombrado de horaFinal
    dia: u8,
    destinatario: String,
    actividad: String,
}
*/




/*
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmailVerification {
    user: String,
    minutos: String,
    destinatario: String,
    link: String,
}
*/


/*
#[post("/sendEmail", data = "<email>")]
fn send_email(email: Json<Email>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    let dia = match email.dia {
        1 => "lunes",
        2 => "martes",
        3 => "miércoles",
        4 => "jueves",
        5 => "viernes",
        6 => "sábado",
        7 => "domingo",
        _ => {
            return Err(Json(ApiResponse {
                message: "Día inválido. Use un valor entre 1 (lunes) y 7 (domingo).".into(),
            }))
        }
    };

    let body = format!(
    r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #ffffff; margin: 0; padding: 0; }}
            .container {{ max-width: 520px; margin: 0 auto; padding: 40px 30px; }}
            .content {{ color: #1f2933; font-size: 15px; line-height: 1.6; }}
            .actividad {{ border-left: 4px solid #2FBF83; background-color: #EAF9F1; padding: 14px 18px; margin: 20px 0; border-radius: 6px; font-size: 14px; color: #1f2933; }}
            .actividad p {{ margin: 4px 0; }}
            .footer {{ text-align: center; font-size: 11px; color: #9aa5b1; margin-top: 30px; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="content">
                <p>Hola <strong>{}</strong>,</p>
                <p>Te informamos que tienes una actividad próxima a vencer.</p>
                <div class="actividad">
                    <p>📌 <strong>Actividad:</strong> {}</p>
                    <p>📅 <strong>Día:</strong> {}</p>
                    <p>⏰ <strong>Hora:</strong> {}</p>
                </div>
                <p>No dejes que se te pase. Revísala en detalle.</p>
            </div>
            <div class="footer">
                <p>Este mensaje fue generado automáticamente.<br>Por favor no respondas a este correo.</p>
            </div>
        </div>
    </body>
    </html>
    "#,
    email.0.user,
    email.0.actividad,
    dia,
    email.0.hora_final
);

    let sender = env::var("Email")
        .map_err(|_| Json(ApiResponse { message: "Variable de entorno 'Email' no encontrada".to_string() }))?;
    let from = format!("PATO <{}>", sender);

    let message = Message::builder()
        .from(
            from.parse()
                .map_err(|e| Json(ApiResponse { message: format!("Remitente inválido: {:?}", e) }))?,
        )
        .to(email
            .destinatario
            .parse()
            .map_err(|e| Json(ApiResponse { message: format!("Destinatario inválido: {:?}", e) }))?)
        .subject("Recordatorio de actividad")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|e| Json(ApiResponse { message: format!("Error construyendo mensaje: {:?}", e) }))?;

    let smtp = build_smtp()
        .map_err(|e| Json(ApiResponse { message: e }))?;

    smtp.send(&message)
        .map(|_| Json(ApiResponse { message: "Correo enviado correctamente".into() }))
        .map_err(|e| Json(ApiResponse { message: format!("Error enviando correo: {:?}", e) }))
}
*/

/*
#[post("/sendEmailVerification", data = "<email>")]
fn send_email_verification(email: Json<EmailVerification>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    let body = format!(
    r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #ffffff; margin: 0; padding: 0; }}
            .container {{ max-width: 520px; margin: 0 auto; padding: 40px 30px; }}
            .content {{ color: #222222; font-size: 15px; line-height: 1.6; }}
            .btn-container {{ text-align: center; margin: 30px 0; }}
            .btn {{ display: inline-block; background: linear-gradient(to right, #cc2d7e, #a020c0); color: white !important; padding: 14px 36px; border-radius: 30px; text-decoration: none; font-size: 15px; font-weight: bold; }}
            .footer {{ text-align: center; font-size: 11px; color: #aaaaaa; margin-top: 30px; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="content">
                <p>Hola <strong>{}</strong>,</p>
                <p>Gracias por registrarte en PATO. Por favor confirma tu correo electrónico haciendo clic en el siguiente botón:</p>
            </div>
            <div class="btn-container">
                <a class="btn" href="{}">Confirmar correo</a>
            </div>
            <div class="content">
                <p>Este enlace expira en {} minutos. Si no solicitaste este registro, ignora este mensaje.</p>
            </div>
            <div class="footer">
                <p>Este mensaje fue generado automáticamente por PATO.<br>Por favor no respondas a este correo.</p>
            </div>
        </div>
    </body>
    </html>
    "#,
    email.0.user,
    email.0.link,
    email.0.minutos
);

    let sender = env::var("Email")
        .map_err(|_| Json(ApiResponse { message: "Variable de entorno 'Email' no encontrada".to_string() }))?;
    let from = format!("PATO <{}>", sender);

    let message = Message::builder()
        .from(
            from.parse()
                .map_err(|e| Json(ApiResponse { message: format!("Remitente inválido: {:?}", e) }))?,
        )
        .to(email
            .destinatario
            .parse()
            .map_err(|e| Json(ApiResponse { message: format!("Destinatario inválido: {:?}", e) }))?)
        .subject("Confirma tu correo electrónico")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|e| Json(ApiResponse { message: format!("Error construyendo mensaje: {:?}", e) }))?;

    let smtp = build_smtp()
        .map_err(|e| Json(ApiResponse { message: e }))?;

    smtp.send(&message)
        .map(|_| Json(ApiResponse { message: "Correo de verificación enviado correctamente".into() }))
        .map_err(|e| Json(ApiResponse { message: format!("Error enviando correo: {:?}", e) }))
}
*/

