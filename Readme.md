
# BlockChainProjectRegistry

Un proyecto basado en **Rust** para gestionar registros de proyectos en una blockchain privada. Este sistema permite crear proyectos y consultar el estado de la cadena de bloques a través de una API REST.

## **Requisitos Previos**

Antes de comenzar, asegúrate de tener instaladas las siguientes herramientas:

1. **Rust** (versión más reciente):
   - Puedes instalar Rust siguiendo las instrucciones en: [https://rustup.rs](https://rustup.rs)
2. **MongoDB**:
   - Descargar desde: [https://www.mongodb.com/try/download/community](https://www.mongodb.com/try/download/community)
   - Configura la autenticación si es necesario. Consulta la documentación del proyecto para más detalles.
3. **Cargo** (incluido con Rust).

## **Instalación**

Sigue los pasos a continuación para clonar e instalar el proyecto:

1. **Clona el repositorio**:
   ```bash
   git clone https://github.com/fufinop/BlockChainProjectRegistry.git
   cd BlockChainProjectRegistry
   ```

2. **Configura MongoDB**:
    - Asegúrate de que MongoDB esté en ejecución en `localhost:27017`.
    - Crea un usuario con permisos adecuados si estás usando autenticación.

3. **Configura las variables de entorno**:
   Crea un archivo `.env` en el directorio raíz del proyecto y agrega lo siguiente:

   ```env
   MONGO_URI=mongodb://<usuario>:<contraseña>@localhost:27017
   DATABASE_NAME=blockchain_db
   ```

   Reemplaza `<usuario>` y `<contraseña>` con las credenciales correctas de tu base de datos.

4. **Instala las dependencias**:
   Rust maneja las dependencias automáticamente. Simplemente ejecuta el comando de compilación para descargarlas:
   ```bash
   cargo build
   ```

## **Ejecución**

Para ejecutar el proyecto en tu máquina local:

1. Inicia el servidor:
   ```bash
   cargo run
   ```

2. El servidor estará disponible en `http://localhost:8000`.

## **Uso**

### Endpoints Disponibles

#### **1. Crear un Proyecto**
- **Método**: `POST`
- **Endpoint**: `/api/projects`
- **Cuerpo de la solicitud**:
  ```json
  {
  "name": "Proyecto Blockchain", 
  "description": "Un proyecto para gestionar una blockchain educativa",
   "start_date": "2024-11-20",
  "end_date": "2025-05-20", 
  "participants": ["Alice", "Bob", "Charlie"],
   "status": "In Progress"
  }
  ```

#### **2. Consultar el Estado de la Blockchain**
- **Método**: `GET`
- **Endpoint**: `/api/blockchain`

#### **3. Endpoint de Bienvenida**
- **Método**: `GET`
- **Endpoint**: `/`