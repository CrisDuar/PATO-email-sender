# Email Service

Microservicio en Rust que envía emails con tokens de recuperacion de contrasena. Integrado con una API orquestadora en Go que maneja la logica de generacion y validacion de tokens en Redis.

## Stack Tecnologico

- **Lenguaje:** Rust
- **Framework:** Rocket 0.5.1
- **SMTP:** Lettre 0.11.19
- **Email Provider:** Gmail
- **Containerizacion:** Docker

## Requisitos

- Rust 1.70+ (para desarrollo local)
- Docker (para produccion)
- Gmail con 2FA y app password

## Instalacion y Configuracion

### 1. Variables de Entorno

Crea un archivo `.env` en la raiz del proyecto:

```env
Email=tu_correo@gmail.com
Password=tu_app_password_de_16_caracteres
```

**Generar App Password:**
1. Ve a https://myaccount.google.com/apppasswords
2. Selecciona "Correo" y "Windows Computer" (o tu dispositivo)
3. Copia la contrasena de 16 caracteres sin espacios

### 2. Ejecutar Localmente

```bash
cargo run
```

El servidor levanta en `http://127.0.0.1:8000`

### 3. Docker (Produccion)

```bash
docker build -t email-service .
docker run -d -p 8000:8000 --env-file .env --name email-service email-service
```

## API Endpoints

### POST /api/sendEmailToken

Envia un email con un token de recuperacion de contrasena.

**Request:**
```json
{
  "user": "nombre_usuario",
  "token": "123456",
  "minutos": "15",
  "destinatario": "usuario@ejemplo.com"
}
```

**Response (200 OK):**
```json
{
  "message": "Correo de token enviado correctamente"
}
```

**Error Responses:**

**400 Bad Request - Email Invalido:**
```json
{
  "error_code": "INVALID_RECIPIENT",
  "message": "Direccion de correo invalida",
  "details": "'correo_invalido'"
}
```

**500 Internal Server Error - Variable Faltante:**
```json
{
  "error_code": "ENV_MISSING",
  "message": "Variable de entorno faltante: Email",
  "details": null
}
```

**500 Internal Server Error - Fallo al Enviar:**
```json
{
  "error_code": "SEND_FAILED",
  "message": "Error enviando correo",
  "details": "smtp error details"
}
```

## Codigos de Error

| Codigo | HTTP Status | Descripcion |
|--------|-------------|-------------|
| `ENV_MISSING` | 500 | Falta variable de entorno (Email o Password) |
| `INVALID_SENDER` | 500 | Configuracion del remitente invalida |
| `INVALID_RECIPIENT` | 400 | Email del destinatario invalido |
| `INVALID_MESSAGE` | 500 | Error construyendo el mensaje |
| `SMTP_CONFIG_ERROR` | 500 | Error en la configuracion SMTP |
| `SEND_FAILED` | 500 | Error enviando el correo |

## Integracion con API Go

La API orquestadora en Go debe:

1. Generar un token de 6 digitos
2. Almacenarlo en Redis con TTL
3. Hacer un POST a este servicio

**Ejemplo en Go:**

```go
import (
    "bytes"
    "encoding/json"
    "net/http"
)

payload := map[string]string{
    "user": "Juan",
    "token": "123456",
    "minutos": "15",
    "destinatario": "usuario@ejemplo.com",
}

body, _ := json.Marshal(payload)
resp, err := http.Post(
    "http://localhost:8000/api/sendEmailToken",
    "application/json",
    bytes.NewBuffer(body),
)

if err != nil {
    // Maneja error de conexion
}
defer resp.Body.Close()

// Verifica el status code
if resp.StatusCode != 200 {
    // Maneja error del servicio
}
```

## Flujo Completo

```
1. Usuario solicita recuperar contrasena
         ↓
2. API Go genera token (6 digitos)
         ↓
3. API Go guarda en Redis con TTL (15 min)
         ↓
4. API Go envia POST a Email Service
         ↓
5. Email Service valida y envía por Gmail
         ↓
6. Usuario recibe el token en su correo
         ↓
7. Usuario ingresa el token en la API Go
         ↓
8. API Go valida contra Redis
```

## Estructura del Proyecto

```
PATO-email-sender/
├── src/
│   └── main.rs              # Logica principal del servicio
├── Cargo.toml               # Dependencias de Rust
├── Cargo.lock
├── Dockerfile               # Imagen Docker multi-stage
├── .env                     # Variables de entorno (gitignored)
├── .gitignore
├── README.md               # Este archivo
└── CONTEXTO.txt            # Documentacion adicional
```

## Desarrollo

### Dependencias

- `dotenvy` - Carga variables de entorno
- `lettre` - Cliente SMTP
- `rocket` - Framework web
- `serde` - Serializacion JSON

### Compilar en Release

```bash
cargo build --release
./target/release/email-service
```

### Limpiar Warnings

```bash
cargo fix --bin "email-service" -p email-service
```

## Deployment

### En VPS Linux

```bash
# Clonar repositorio
git clone https://github.com/tu-repo/PATO-email-sender.git
cd PATO-email-sender

# Crear .env
nano .env
# Email=patopems@gmail.com
# Password=tu_app_password

# Build y run con Docker
docker build -t email-service .
docker run -d -p 8000:8000 --env-file .env --name email-service email-service

# Verificar
docker ps
docker logs email-service
```

### Con Reverse Proxy (Nginx)

```nginx
server {
    listen 80;
    server_name tu-dominio.com;

    location /api/ {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## Troubleshooting

**"Variable de entorno 'Password' no encontrada"**
- Reinicia el servidor despues de editar .env
- Verifica que no haya espacios en el valor de Password

**"Direccion de correo invalida"**
- El email en destinatario no tiene formato valido
- Valida el email antes de enviarlo desde Go

**"Error enviando correo"**
- Verifica que Gmail este configurado correctamente
- Checkea que la app password sea correcta
- Verifica conectividad a smtp.gmail.com:587

## Notas Importantes

- Este servicio es **stateless**, solo envía emails
- Los tokens son generados y validados por la API Go, no aqui
- No usar tildes en mensajes de error (configurado)
- El HTML del email si puede tener tildes
- Manejo de errores con codigos HTTP correctos

## Licencia

MIT

## Contacto

Para preguntas o reportar bugs, abre un issue en el repositorio.
