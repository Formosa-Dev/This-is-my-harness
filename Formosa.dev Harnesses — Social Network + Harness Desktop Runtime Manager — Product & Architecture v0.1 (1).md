# FORMOSA.DEV HARNESSES

## Social Network \+ Harness Desktop Runtime Manager

## Documento Maestro de Producto, Arquitectura, Distribución, Seguridad y Modelo de Negocio v0.1

Este documento desarrolla una segunda capa de producto distinta del registry estático inicial: una red social pública de builders y harnesses conectada a una aplicación de escritorio multiplataforma capaz de descubrir, instalar, adaptar, activar, probar, cambiar y revertir harnesses en distintos runtimes de agentes de código.  
La hipótesis central es que el botón más importante del producto no es “Like”. Es “Use”. Cuando un harness puede pasar de una página pública a un entorno de trabajo real con pocos pasos, el registry deja de ser contenido y empieza a comportarse como infraestructura.  
Objetivo: construir un producto global nacido dentro de Formosa.dev, con una primera implementación rigurosa, segura y portable, capaz de generar comunidad, reputación técnica, distribución de conocimiento y eventualmente ingresos recurrentes. Ninguna parte de este documento presupone que el producto garantice resultados financieros; el modelo se diseña para maximizar utilidad, adopción y capacidad de monetización real.

# 0\. RESUMEN EJECUTIVO

La evolución propuesta tiene cuatro superficies coordinadas:

* Web social: perfiles públicos, feed, harness pages, votos, comentarios, follows, collections y discovery.  
* Registry: metadata versionada, bundles, versiones, compatibilidad, integridad, licencias y señales de confianza.  
* Harness Desktop: aplicación instalable para Windows, macOS y Linux que administra runtimes y harnesses localmente.  
* CLI/Core Engine: motor reutilizable que más adelante puede exponer las mismas operaciones del desktop desde terminal y CI.

La gran innovación práctica es la experiencia de “Use this harness”. Desde la web, una persona elige un harness, elige un runtime compatible y abre Harness Desktop mediante un deep link. La app descarga una versión inmutable del bundle, verifica su integridad, analiza permisos y efectos, muestra un diff, crea un snapshot local, aplica la configuración al proyecto o al usuario, valida la instalación y permite lanzar el runtime seleccionado.  
El usuario puede alternar entre diferentes harnesses sin editar manualmente decenas de archivos. Puede probar el harness de otra persona con Codex, Claude Code, OpenCode, Cursor u otros runtimes compatibles, comparar resultados y volver a su estado anterior.  
La red social y la aplicación de escritorio se alimentan mutuamente. El feed genera discovery; el botón Use genera utilización real; la utilización real genera señales de calidad; esas señales mejoran el ranking; el ranking devuelve visibilidad a los builders; la visibilidad incentiva nuevas publicaciones y mejoras.  
El producto deja de ser “una red social que habla de harnesses” para transformarse en “la capa desde la que descubro y administro cómo trabajan mis agentes”.

# 1\. LA TESIS DE PRODUCTO

## 1.1 El harness como activo portable

Si un harness representa instrucciones, skills, reglas, MCP, hooks, subagentes, comandos, permisos y workflows, entonces puede tratarse como un artefacto versionado. Si puede versionarse, puede instalarse. Si puede instalarse, puede compararse, probarse, actualizarse y revertirse.  
Ese salto —de documentación a instalación reproducible— es lo que convierte al proyecto en producto.

## 1.2 El network effect

Cada nuevo builder aporta harnesses. Cada nuevo harness hace más útil el explorer. Cada nuevo usuario genera feedback, votos, comentarios y señales de uso. Cada señal ayuda a otros a descubrir configuraciones útiles. Cada fork o adaptación vuelve a alimentar el registry.  
El flywheel propuesto:  
Builder publica → alguien descubre → inspecciona → usa → prueba → vota/comenta → aparece en feeds → otros lo usan → alguien mejora/forkea → builder gana reputación → publica otra vez.

## 1.3 El dato más valioso no es el like

Los productos sociales suelen optimizar interacción. Este producto debe optimizar utilidad. Por eso las señales deben distinguir entre:

* View: alguien vio una página.  
* Save: alguien guardó el harness.  
* Vote: alguien expresa valoración.  
* Install: Harness Desktop instaló una versión.  
* Activate: el harness quedó activo sobre un proyecto/runtime.  
* Use: el usuario confirmó utilización real o el cliente registró una sesión de uso con consentimiento.  
* Revert: el usuario desinstaló o revirtió.  
* Fork: alguien creó una variante.  
* Comment: alguien dejó feedback cualitativo.

La métrica que debería convertirse en señal diferenciadora es “verified use”, no porque pruebe calidad absoluta, sino porque es más difícil de falsificar que una vista y representa adopción concreta.

# 2\. POSICIONAMIENTO

## 2.1 Definición corta

Formosa.dev Harnesses es una red social y runtime manager para descubrir, compartir y usar configuraciones de agentes de IA.

## 2.2 Mensaje principal

“Discover a workflow. Run it on your agent.”  
Alternativa: “Share how your agents work. Use how others work.”  
Explicación: “Browse real agent harnesses, install them safely, switch between runtimes and give credit to the builders behind them.”

## 2.3 Qué no es

* No es un marketplace de prompts.  
* No es solamente un awesome-list.  
* No es un wrapper de un único agente.  
* No pretende reemplazar Codex, Claude Code, OpenCode o Cursor.  
* No almacena credenciales de proveedores como requisito de funcionamiento.  
* No ejecuta automáticamente scripts de terceros sin consentimiento.  
* No necesita inventar un estándar universal para entregar valor.

# 3\. EL PRODUCTO COMO SISTEMA

## 3.1 Surface A — Web

La web es el discovery layer y la identidad pública. Sirve para encontrar personas, harnesses, versiones, discusiones y señales de uso.

## 3.2 Surface B — Desktop

Harness Desktop es el execution layer local. Tiene acceso controlado al filesystem, procesos y configuraciones necesarias para activar un harness. Debe trabajar local-first y mostrar al usuario qué cambia antes de cambiarlo.

## 3.3 Surface C — Registry/API

El registry entrega manifests, versiones, hashes, compatibilidad, assets y metadatos. También conecta el sistema social con el cliente de escritorio.

## 3.4 Surface D — Core Engine

El motor de instalación/adaptación no debe quedar acoplado a la UI. Debe existir como librería nativa reutilizable por Desktop y por una futura CLI.

# 4\. RED SOCIAL: PERFIL PÚBLICO

El perfil público deja de ser solamente una página generada por Git. Al introducir votos, comentarios, follows y feed, aparece identidad autenticada y estado mutable. GitHub continúa siendo el proveedor de identidad recomendado, pero el producto necesita una base propia para las relaciones sociales.

## 4.1 Campos

* handle canónico.  
* GitHub login y GitHub ID.  
* display name.  
* avatar.  
* headline.  
* bio.  
* roles y skills.  
* links públicos.  
* harnesses publicados.  
* harnesses mantenidos.  
* forks.  
* collections.  
* followers / following.  
* verified uses recibidos por sus harnesses.  
* contribution badges verificables.

## 4.2 Reputación

Evitar un score único del tipo 87/100. La reputación debe ser explicable mediante señales visibles: harnesses publicados, usos verificados, contributors, antigüedad de mantenimiento, revisiones, compatibilidad probada y actividad comunitaria.

## 4.3 URL

/harnesses/@username o /harnesses/users/username. Para URLs globales compartibles, @username es más social; para compatibilidad estática y routing, /users/username es más simple. La elección puede hacerse al migrar el hosting dinámico.

# 5\. FEED PÚBLICO

## 5.1 Objetivo

El feed no debe parecer un timeline genérico. Cada unidad del feed debería ayudar a descubrir algo utilizable.

## 5.2 Tipos de items

* Harness publicado.  
* Nueva versión.  
* Harness adaptado a un runtime nuevo.  
* Fork relevante.  
* Collection publicada.  
* Benchmark/eval agregado.  
* Builder que seguís publicó algo.  
* Harness que guardaste recibió una versión.  
* Comentario técnico destacado.

## 5.3 Tabs iniciales

* Trending.  
* New.  
* Following.  
* Most used.

Un “For You” algorítmico puede esperar hasta que exista suficiente actividad.

# 6\. VOTOS, COMENTARIOS Y GUARDADOS

## 6.1 Voto

V1: un upvote por usuario por harness. No arrancar con downvotes. Un downvote mezcla calidad, gusto, rivalidades y moderación en una sola señal; no es necesario para discovery inicial.

## 6.2 Comentarios

Comentarios por harness y por versión. La versión importa: una crítica válida para v1.2 puede haber sido resuelta en v1.3.

* Threads simples.  
* Replies de un nivel o árbol acotado.  
* Markdown limitado.  
* Reportar.  
* Edit history.  
* Builder badge en respuestas del autor.

## 6.3 Guardados

Guardar es una señal privada de intención. Permite armar collections personales sin afirmar públicamente que el usuario recomienda el harness.

## 6.4 Reviews

No lanzar estrellas de 1 a 5 al principio. Un comentario \+ voto \+ uso verificado produce más información y evita convertir el producto en un catálogo de ratings vacíos.

# 7\. VERIFIED USE

## 7.1 Qué significa

“Verified use” significa que un cliente oficial de Harness Desktop aplicó una versión concreta de un harness a un runtime/proyecto y reportó el evento con consentimiento del usuario. No significa que el harness sea bueno, seguro o exitoso.

## 7.2 Privacidad

El evento remoto mínimo debería contener:

* user\_id autenticado o identificador anónimo opt-in.  
* harness\_id.  
* version\_id.  
* runtime.  
* event\_type: install/activate/use/revert.  
* app\_version.  
* timestamp del servidor.

No enviar nombre del proyecto, path local, código fuente, prompts, archivos modificados, API keys ni nombres de repos privados.

## 7.3 Contadores

Mostrar “Used by X builders” puede ser más útil que un contador bruto de instalaciones, siempre que se defina públicamente cómo se calcula y se evite contar reinstalaciones infinitas.

# 8\. RANKING Y DISCOVERY

## 8.1 Principios

* Transparente.  
* Resistente a spam.  
* No premiar exclusivamente antigüedad.  
* No premiar exclusivamente tamaño de audiencia.  
* Separar trending de all-time.  
* Usar señales verificables cuando existan.

## 8.2 Fórmula conceptual

Trending Score \= freshness decay \+ unique verified users \+ upvotes \+ saves \+ meaningful comments \+ forks, con límites por señal para evitar que una métrica domine todo.  
No publicar inicialmente los pesos exactos si eso facilita gaming, pero sí publicar las categorías de señales y las reglas anti-spam.

## 8.3 Most used

Ranking independiente basado en usuarios únicos que activaron/utilizaron una versión mediante el cliente oficial, con ventanas temporales y deduplicación.

## 8.4 Editorial

Featured no debe confundirse con trending. Debe llevar una etiqueta editorial explícita.

# 9\. LA EXPERIENCIA “USE THIS HARNESS”

## 9.1 Desde la web

En una harness page:

* Botón primario: Use.  
* Selector de runtime compatible.  
* Selector de versión.  
* Resumen de permisos.  
* Compatibilidad: native / adapted / partial / untested.  
* Open in Harness Desktop.

## 9.2 Deep link

Ejemplo conceptual:  
harnesses://use/\<owner\>/\<slug\>?version=1.4.0\&runtime=codex  
El navegador intenta abrir la app. Si no está instalada, muestra descarga por sistema operativo.

## 9.3 En Desktop

* Resolver harness \+ versión.  
* Descargar manifest.  
* Verificar firma/hash.  
* Mostrar quién publica.  
* Mostrar componentes.  
* Mostrar archivos que se crearán/modificarán.  
* Mostrar comandos/scripts potenciales.  
* Elegir Project scope o User scope.  
* Elegir runtime.  
* Elegir proyecto.  
* Preview diff.  
* Create snapshot.  
* Apply.  
* Verify.  
* Launch.  
* Revert disponible en todo momento.

# 10\. HARNESS DESKTOP

## 10.1 Working name

Harness Desktop o Harness Manager. La marca final puede separarse del nombre del registry; el concepto operativo debe entenderse aunque cambie el naming.

## 10.2 Home

* Installed harnesses.  
* Active projects.  
* Detected runtimes.  
* Recently used.  
* Updates available.  
* Discover.

## 10.3 Project view

Cada proyecto muestra runtime activo, harness activo, archivos administrados, snapshot actual, historial de cambios y acción de rollback.

## 10.4 Harness view

Preview de README, manifest, author, versión, compatibilidad, trust signals, cambios proyectados y acciones Use/Update/Remove.

## 10.5 Runtime view

Estado de Codex/Claude/OpenCode/Cursor/etc.: detected, version, auth state detectable sin leer secretos, paths conocidos y update available.

# 11\. CROSS-PLATFORM: QUÉ SIGNIFICA “UN EXE”

En Windows el producto puede distribuirse como instalador .exe y/o .msi. En macOS la distribución típica directa es .dmg con app firmada/notarizada. En Linux conviene ofrecer AppImage y, según demanda, .deb y .rpm. Por eso el concepto correcto es “desktop application multiplataforma”, no un único .exe para todos los sistemas.  
La capa de producto debe esconder esa complejidad: el usuario entra a Download y recibe automáticamente el instalador correcto.

# 12\. ELECCIÓN DE TECNOLOGÍA: TAURI 2

## 12.1 Recomendación

Usar Tauri 2 para Harness Desktop, con UI React/TypeScript y core nativo en Rust.

## 12.2 Motivos

* Distribución oficial para Windows, macOS y Linux.  
* Generación de instaladores Windows, DMG/app bundle macOS y múltiples formatos Linux.  
* Plugin updater multiplataforma.  
* El updater de Tauri exige verificación criptográfica de actualizaciones y no permite desactivar esa firma.  
* Rust es adecuado para operaciones sensibles de filesystem, hashing, procesos y adapters.  
* Permite reutilizar el core como librería para una futura CLI.  
* Reduce la necesidad de empaquetar un runtime Chromium completo como ocurre con Electron.

## 12.3 Por qué no Electron como primera opción

Electron sigue siendo perfectamente viable y tiene un ecosistema maduro, pero su updater integrado documenta soporte directo solo para macOS y Windows; Linux suele delegar actualización al package manager. Para este producto, donde la capa nativa es importante y queremos una arquitectura ligera, Tauri ofrece un encaje mejor.  
La decisión no debe ser dogmática: si una integración crítica exige Node/Electron, se reevalúa con benchmark real.

# 13\. ARQUITECTURA DEL DESKTOP

Propuesta de módulos:

* desktop-ui — React/Vite o equivalente.  
* harness-core — Rust library.  
* registry-client — HTTP client, cache y verificación.  
* runtime-adapters — Codex, Claude, OpenCode, Cursor, Gemini.  
* snapshot-engine — backups, diff, restore.  
* security-engine — manifests, policy checks, signatures, hashes.  
* process-runner — ejecución explícita y auditada.  
* auth-client — identidad del producto, separada de credenciales de runtimes.  
* telemetry-client — eventos mínimos opt-in.  
* updater — actualización firmada de Harness Desktop.

# 14\. PRINCIPIO LOCAL-FIRST

El Desktop debe funcionar aunque el backend social esté caído, siempre que el bundle ya esté cacheado.

* Config local.  
* Registry cache.  
* Snapshots locales.  
* Install history.  
* No subir contenido del proyecto.  
* No depender de una sesión web para revertir.

La nube agrega identidad, discovery, sincronización y métricas; no debe ser necesaria para recuperar el estado local.

# 15\. RUNTIME ADAPTER CONTRACT

Cada runtime debe implementar una interfaz común. Conceptualmente:

* detect() — presencia, versión, paths.  
* installPlan() — cómo instalar según OS.  
* install() — solo después de aprobación.  
* authStatus() — estado superficial, nunca exfiltrar credenciales.  
* inspectExisting(project) — configuración actual.  
* planApply(bundle, scope) — cambios propuestos.  
* apply(plan) — escritura controlada.  
* verify(project) — archivos/versiones/config válida.  
* launch(project) — abrir runtime o terminal.  
* revert(snapshot) — restaurar estado.  
* capabilities() — componentes soportados.

Los adapters son el activo técnico principal del producto. La interfaz común permite que el mismo harness se pueda probar sobre múltiples runtimes sin convertir la UI en lógica específica para cada proveedor.

# 16\. ADAPTER: CODEX

Codex CLI tiene instaladores oficiales para macOS/Linux y Windows, además de npm y Homebrew. La app no debería reempaquetar Codex; debería detectar una instalación existente o, con permiso, ejecutar el método oficial recomendado.  
El adapter debe entender al menos:

* AGENTS.md y jerarquía de instrucciones.  
* \~/.codex/config.toml.  
* MCP configurado en Codex.  
* Skills/plugins compatibles cuando el harness los declare.  
* Project scope vs user scope.  
* Codex CLI version.

La autenticación de Codex sigue perteneciendo a OpenAI. Harness Desktop no debe pedir ni almacenar la contraseña o token de sesión del usuario.

# 17\. ADAPTER: CLAUDE CODE

Claude Code dispone de instalación oficial en macOS/Linux, PowerShell en Windows, Homebrew y WinGet. El adapter puede detectar o instalar mediante esos canales con aprobación del usuario.  
Debe modelar las capas que Claude Code documenta como extensibilidad real:

* CLAUDE.md.  
* Rules.  
* Skills.  
* Subagents.  
* Hooks.  
* MCP.  
* Plugins.

Los hooks son especialmente sensibles porque pueden ejecutar procesos. El adapter debe mostrarlos separados del contenido pasivo y exigir una confirmación reforzada para habilitarlos.

# 18\. ADAPTER: OPENCODE

OpenCode soporta AGENTS.md, skills, agentes configurables, permisos, MCP y configuración propia. Sus docs actuales además recomiendan WSL para la mejor experiencia en Windows en ciertas variantes, aunque también existen distribuciones nativas/escritorio según versión.  
El adapter debe:

* Detectar versión y canal.  
* No asumir que todas las versiones usan el mismo package name o paths.  
* Resolver .opencode/skills, .agents/skills y compatibilidad con .claude/skills.  
* Leer opencode.json/jsonc.  
* Mapear permisos.  
* Mapear MCP.  
* Advertir cuando la versión del runtime no soporta una capacidad declarada.

# 19\. ADAPTER: CURSOR

Cursor actualmente expone Rules, Agent Skills, plugins, MCP, subagents y hooks. Un harness puede mapear varias de estas piezas de forma nativa.  
Especial atención: los hooks pueden ejecutar scripts antes/después de shell, MCP, lectura y edición de archivos. La UI debe mostrar qué hooks se activarán y en qué eventos.

# 20\. ADAPTERS FUTUROS

* Gemini CLI.  
* Windsurf.  
* VS Code / Copilot agent configuration.  
* Aider y otros coding agents si el modelo de harness tiene sentido.  
* Custom runtime adapter SDK.

No intentar soportar todo en V1. La profundidad de tres adapters buenos vale más que diez badges superficiales.

# 21\. PROJECT SCOPE VS USER SCOPE

## 21.1 Project scope

Instala/adapta el harness dentro de un repositorio concreto. Es el modo recomendado por defecto porque los cambios son visibles en Git y más fáciles de revisar.

## 21.2 User scope

Configura reglas/skills globales del runtime. Es más poderoso y más riesgoso porque afecta otros proyectos.

## 21.3 Política

Por defecto: Project scope. User scope requiere una advertencia adicional y un preview exacto de los paths que se modificarán.

# 22\. SNAPSHOT Y ROLLBACK

Antes de modificar cualquier archivo administrado, Harness Desktop crea un snapshot inmutable.

* Lista de paths.  
* Hash anterior.  
* Contenido anterior cuando corresponda.  
* Hash posterior.  
* Harness/version.  
* Runtime.  
* Timestamp local.  
* Operación.

Rollback debe ser una operación de primera clase, visible en la UI y testeada automáticamente.

## 22.1 Nunca “sobrescribir y rezar”

Si el archivo ya existe, la app no lo reemplaza silenciosamente. Debe elegir entre merge seguro, file overlay, adapter-specific transform o conflicto que requiere decisión humana.

# 23\. PREVIEW DIFF

Antes de Apply:

* Created files.  
* Modified files.  
* Removed files — idealmente ninguno en MVP.  
* Global changes.  
* Executable components.  
* Network/MCP dependencies.  
* Environment variables requeridas por nombre, nunca sus valores.

El usuario debe poder expandir cada cambio y ver el diff.

# 24\. INSTALAR RUNTIMES FALTANTES

La app puede ofrecer “Install Codex”, “Install Claude Code” u “Install OpenCode”, pero solamente mediante métodos oficiales conocidos por el adapter.  
Workflow:

* Detect missing.  
* Show official source/provider.  
* Show exact command/action.  
* Request permission.  
* Execute with least privilege.  
* Stream output.  
* Verify binary/version.  
* Hand off authentication to the runtime.

No almacenar credenciales de Anthropic/OpenAI/model providers dentro del producto salvo que en el futuro exista una función específica y auditada que lo justifique.

# 25\. AUTENTICACIÓN DEL PRODUCTO

## 25.1 Web

GitHub OAuth es la identidad inicial más coherente con builders, repos y PRs.

## 25.2 Desktop

Usar GitHub Device Flow o un browser-based OAuth con deep link de retorno. GitHub documenta Device Flow específicamente para aplicaciones sin UI web persistente, CLIs y desktop apps.

## 25.3 Separación de identidades

Login de Harnesses ≠ login de Codex ≠ login de Claude Code ≠ API keys de OpenCode providers. La app debe mantener esos dominios separados para reducir riesgo y responsabilidad.

# 26\. BUNDLE DE HARNESS

Un harness instalable necesita una release inmutable, no solamente la rama main de un repo.

## 26.1 Componentes

* registry.yaml / manifest.  
* README.  
* src/ native files.  
* adapters/ opcionales.  
* checksums.  
* signature/attestation.  
* license.  
* release metadata.

## 26.2 Identidad

owner/slug@version.  
Ejemplo conceptual: adriangmrraa/nextjs-production@1.4.0.

## 26.3 Content-addressing

Cada bundle debe tener SHA-256. El cliente compara el hash descargado con el manifest antes de ofrecer Apply.

## 26.4 Firmas

La evolución recomendada es firmar releases/artefactos. Sigstore/Cosign permite firmar y verificar blobs y artifacts, incluida firma basada en identidad con registro de transparencia. Esto complementa, no reemplaza, la firma de la aplicación de escritorio exigida por los sistemas operativos.

# 27\. MODELO DE CONFIANZA

## 27.1 Trust signals

* Author identity verified.  
* Manifest valid.  
* Secret scan passed.  
* Maintainer reviewed.  
* Bundle hash verified.  
* Bundle signature verified.  
* Runtime tested.  
* Used by unique builders.  
* No executable components.

No usar “100% safe”. Mostrar exactamente qué se verificó.

## 27.2 Risk classes

Clase A — Passive: instrucciones, markdown, rules y archivos de configuración sin ejecución.  
Clase B — Tooling: MCP remoto/local, plugins o componentes con acceso a herramientas.  
Clase C — Executable: hooks/scripts/commands que pueden ejecutar procesos.  
Clase D — Privileged: solicita escritura global, elevación, Docker/socket, credenciales o permisos amplios.  
La UI usa estas clases para definir fricción y warnings.

# 28\. POLÍTICA DE EJECUCIÓN

Un harness descargado nunca ejecuta scripts automáticamente durante preview.  
Durante Apply:

* Copiar archivos pasivos puede ser una acción única.  
* Configurar MCP requiere preview.  
* Habilitar hooks requiere aprobación reforzada.  
* Ejecutar setup scripts requiere confirmación por script/comando o una política explícita del usuario.  
* Comandos con privilegios elevados nunca se ejecutan sin prompt del sistema.

El desktop debe tratar un harness como código de terceros, no como configuración inocua.

# 29\. FIRMADO DEL CLIENTE Y ACTUALIZACIONES

La aplicación que modifica configuraciones de agentes debe ser tratada como software sensible.

## 29.1 macOS

Firma de código y notarización para distribución directa. Tauri documenta DMG/app bundle y requisitos de firma/notarización.

## 29.2 Windows

Distribuir setup .exe/NSIS y/o MSI. Firmar los binarios/instaladores para minimizar warnings y proteger integridad.

## 29.3 Linux

AppImage para distribución simple y .deb/.rpm donde aporte valor. Ofrecer checksums y firmas.

## 29.4 Updater

Tauri Updater soporta Windows, Linux y macOS y exige firma de update bundles. La primera versión puede usar un latest.json estático publicado junto a GitHub Releases.

# 30\. BACKEND SOCIAL: EL PUNTO DONDE STATIC-ONLY DEJA DE ALCANZAR

El registry inicial podía vivir completamente en GitHub Pages. Votes, comments, follows, feed personalizado, notificaciones y eventos de uso son estado mutable. En este punto un backend deja de ser sobrearquitectura y pasa a ser parte del producto.  
La web principal de Formosa.dev puede seguir siendo mayormente estática, pero la capa Harnesses necesita Auth \+ Postgres \+ APIs/edge functions.

# 31\. BACKEND RECOMENDADO PARA MVP: SUPABASE

Recomendación inicial: Supabase para Auth, Postgres, Row Level Security, Realtime y Edge Functions.

## 31.1 Motivos

* GitHub social login soportado.  
* Postgres relacional para grafos sociales y ranking.  
* RLS para autorización a nivel de fila.  
* Realtime disponible cuando comentarios/notificaciones lo requieran.  
* Edge Functions TypeScript para operaciones privilegiadas, webhooks y reglas de negocio.  
* Menor carga operativa que separar cinco servicios desde el primer día.

Esto es una decisión de velocidad y mantenibilidad, no un lock-in conceptual. El modelo de datos debe ser PostgreSQL estándar y la lógica crítica no debe depender innecesariamente de features irreversibles.

# 32\. HOSTING WEB Y FORMOSA.DEV

## 32.1 Estado actual

Formosa.dev está exportado estáticamente con Next.js y desplegado en GitHub Pages. Eso sigue siendo excelente para la web institucional y para un registry puramente build-time.

## 32.2 Problema

DNS no permite mandar solamente /harnesses a otro host. Un subdominio sí puede apuntar a otro proveedor, pero un path del dominio raíz no se puede resolver mediante DNS.

## 32.3 Opciones

* A. Mantener /harnesses estático y cargar social data client-side desde Supabase. Funciona, pero nuevas URLs dinámicas siguen dependiendo de rebuilds.  
* B. Crear harnesses.formosa.dev.ar para la app social dinámica y dejar formosa.dev.ar/harnesses como landing/enlace. Es técnicamente limpio.  
* C. Migrar todo formosa.dev.ar a un host compatible con Next.js server/edge, preservando las páginas estáticas y habilitando rutas dinámicas. Es la opción más cohesionada cuando el producto social sea central.

## 32.4 Recomendación

Fase inicial: mantener GitHub Pages mientras se valida registry \+ desktop.  
Cuando votes/comments/feed entren en producción: migrar el mismo proyecto Next.js a Vercel u otro host compatible con Next server/edge, o usar subdominio dedicado si se quiere aislar el producto.  
No migrar solamente por anticipación; migrar cuando las features mutables justifiquen el cambio.

# 33\. MODELO DE DATOS SOCIAL

Esquema conceptual:

### profiles

* id UUID.  
* github\_id unique.  
* username unique.  
* display\_name.  
* avatar\_url.  
* bio.  
* headline.  
* created\_at.

### harnesses

* id UUID.  
* owner\_profile\_id.  
* slug.  
* title.  
* summary.  
* visibility.  
* status.  
* default\_version\_id.

### harness\_versions

* id.  
* harness\_id.  
* semver.  
* bundle\_url.  
* sha256.  
* signature metadata.  
* manifest jsonb.  
* published\_at.

### runtime\_compatibility

* version\_id.  
* runtime.  
* support\_level.  
* tested\_by.  
* tested\_at.

### votes

* user\_id \+ harness\_id unique.

### comments

* id.  
* harness\_id.  
* version\_id optional.  
* author\_id.  
* parent\_id.  
* body.  
* status.

### follows

* follower\_id \+ followed\_id unique.

### saves

* user\_id \+ harness\_id unique.

### collections

* owner\_id.  
* title.  
* visibility.

### collection\_items

* collection\_id \+ harness\_id.

### usage\_events

* user\_id nullable/anonymized.  
* harness\_id.  
* version\_id.  
* runtime.  
* event\_type.  
* client\_version.  
* created\_at.

### notifications

* recipient\_id.  
* type.  
* actor\_id.  
* entity\_id.  
* read\_at.

### reports

* reporter\_id.  
* target\_type.  
* target\_id.  
* reason.  
* status.

# 34\. API / EDGE FUNCTIONS

Operaciones sensibles no deberían depender solamente de inserts directos desde el browser.

* publish-version.  
* record-verified-use.  
* resolve-download.  
* create-vote / remove-vote.  
* create-comment.  
* follow/unfollow.  
* generate-feed.  
* report-content.  
* verify-github-identity.  
* release-signing webhook.

Las operaciones simples pueden usar Supabase client \+ RLS. Las que calculan ranking, validan firmas o agregan señales deben pasar por funciones controladas.

# 35\. WEB ↔ DESKTOP

## 35.1 Deep link

Web llama harnesses://use/... y entrega solamente identificadores públicos. La app resuelve metadata desde el API, evitando meter secretos o manifests completos en la URL.

## 35.2 Download fallback

Si el scheme no responde, la web detecta OS y muestra instalador correspondiente.

## 35.3 Login handoff

Desktop puede iniciar Device Flow directamente o abrir el navegador para completar autenticación y volver mediante deep link.

## 35.4 Session transfer

No meter access tokens en query strings. Usar códigos de un solo uso si se implementa browser → desktop handoff.

# 36\. FLUJO COMPLETO DEL USUARIO

Caso: probar el harness de un amigo.

* 1\. Ve un post/harness en el feed.  
* 2\. Abre la ficha.  
* 3\. Revisa README, runtime support y trust signals.  
* 4\. Pulsa Use.  
* 5\. Selecciona Codex.  
* 6\. Harness Desktop abre.  
* 7\. Selecciona proyecto.  
* 8\. App detecta Codex; si falta, ofrece instalación oficial.  
* 9\. App genera plan.  
* 10\. Usuario revisa diff.  
* 11\. App crea snapshot.  
* 12\. App aplica.  
* 13\. App verifica.  
* 14\. App abre Codex en el proyecto.  
* 15\. Después de usarlo, el usuario puede votar/comentar.  
* 16\. Puede cambiar a otro harness.  
* 17\. Puede volver al snapshot anterior.

# 37\. SWITCHING ENTRE HARNESSES

Esta feature puede convertirse en hábito.  
Un project slot mantiene:

* runtime.  
* active harness.  
* version.  
* scope.  
* snapshot parent.  
* local overrides.

Cambiar A → B no significa apilar B encima de A. El engine parte del snapshot/base administrada, calcula estado deseado para B y aplica una transición determinística.

## 37.1 Local overrides

El usuario puede mantener overrides fuera del bundle. La app debe representar tres capas:

* Base project.  
* Harness managed layer.  
* User overrides.

Eso permite actualizar un harness sin perder personalización local.

# 38\. VERSIONES Y UPDATES

Una versión publicada es inmutable.

* SemVer recomendado.  
* Changelog.  
* Compatibility matrix por versión.  
* Update preview.  
* Pin version.  
* Ignore version.  
* Rollback.

Auto-update de harnesses debe ser opt-in en V1. La app puede avisar, pero no reescribir configuraciones de trabajo silenciosamente.

# 39\. PRIVATE HARNESSES Y TEAMS

Esto aparece como línea de monetización fuerte, pero después del public MVP.

* Private harnesses.  
* Organization namespace.  
* Approved harness catalog.  
* Policy enforcement.  
* Team comments.  
* Internal usage metrics.  
* Audit log.  
* Managed runtime configuration.  
* SSO/SCIM en enterprise.

El valor B2B no está en “más likes”; está en gobernanza y distribución segura de workflows agentic.

# 40\. MODELO DE NEGOCIO

## 40.1 Principio

El loop público debe ser gratuito para maximizar supply, discovery y reputación. La monetización debe cobrar por privacidad, sincronización avanzada, governance, equipos y automatización, no por bloquear el acto básico de compartir/usar.

## 40.2 Free

* Perfil público.  
* Public harnesses.  
* Feed, votes, comments.  
* Desktop app.  
* Instalar y cambiar harnesses públicos.  
* Snapshots básicos.  
* Public collections.

## 40.3 Pro — hipótesis

* Private harnesses personales.  
* Sync multi-device.  
* Unlimited private collections.  
* Advanced local profiles.  
* Encrypted metadata sync — nunca secrets crudos por defecto.  
* Advanced version pinning.  
* Cross-runtime conversion assistance.  
* Personal usage history.

Precio a validar con usuarios; rango de test inicial posible: USD 8–15/mes.

## 40.4 Teams — hipótesis

* Private organization registry.  
* Team roles.  
* Approved harnesses.  
* Policies.  
* Audit.  
* Shared collections.  
* Private comments/reviews.  
* Usage analytics.  
* Managed distribution.

Rango de test: USD 15–30 por usuario/mes, sujeto a valor real y mercado.

## 40.5 Enterprise

* SSO.  
* SCIM.  
* Self-hosted/on-prem options.  
* Policy server.  
* Private artifact mirror.  
* Security integrations.  
* Support/SLA.

## 40.6 Marketplace futuro

Pago por harnesses premium es posible, pero no debe ser una prioridad temprana. Crea complejidad de licencias, refunds, malware, calidad, impuestos y expectativas de soporte. Primero construir usage y trust.

# 41\. POR QUÉ PODRÍA TENER DEFENSIBILIDAD

* Social graph de builders y adopción.  
* Usage graph: qué harnesses se usan realmente y en qué runtimes.  
* Runtime adapter layer mantenido con profundidad.  
* Version history y compatibility history.  
* Trust/review history.  
* Collections y curación comunitaria.  
* Switching/rollback engine local.  
* Open bundle format \+ proprietary convenience layer.

El moat no debería depender de encerrar archivos. Los harnesses públicos pueden seguir siendo open source. El producto gana por hacer discovery, instalación, seguridad, switching, reputation y governance mucho mejores.

# 42\. OPEN SOURCE VS PRODUCTO

Recomendación:

* Registry schema: open.  
* Bundle spec: open.  
* Core adapter interfaces: considerar open.  
* Public registry data: open/exportable.  
* Desktop app: puede ser source-available u open core según estrategia.  
* Hosted social graph, ranking, sync, team governance y managed distribution: servicio comercial.

Un producto para developers gana credibilidad si el formato y los datos públicos no quedan atrapados.

# 43\. GO-TO-MARKET

## 43.1 Wedge

Coding agents. Developers que ya usan Codex, Claude Code y OpenCode y tienen configuraciones que podrían compartir hoy.

## 43.2 Primeros 100 builders

* Invitación manual a personas con setups públicos.  
* Founding Builder badge.  
* Help to export: el equipo convierte configuraciones existentes.  
* Public “show your harness” challenge.  
* Comparativas de un mismo proyecto con varios harnesses.  
* Meetup/hackathon Formosa.dev dedicado a agent workflows.

## 43.3 Contenido

* “I tried X’s harness on Codex.”  
* “Same repo, three harnesses.”  
* “How this builder structures Claude Code.”  
* “Most used this week.”  
* “Fork this workflow.”

El contenido nace del producto; no depende de inventar campañas externas.

# 44\. MVP REAL

No intentar construir todo el documento como primera release.

## 44.1 Web MVP

* GitHub login.  
* Public profiles.  
* Harness pages.  
* Feed: New \+ Trending.  
* Upvote.  
* Comments.  
* Use button.  
* Download Desktop.

## 44.2 Desktop MVP

* Windows \+ macOS primero; Linux puede entrar en la misma fase si release pipeline está estable.  
* Runtime detection.  
* Codex adapter.  
* Claude Code adapter.  
* OpenCode adapter.  
* Choose project.  
* Preview diff.  
* Snapshot.  
* Apply.  
* Revert.  
* Deep link.  
* Signed app updates.

## 44.3 Registry MVP

* Immutable version bundles.  
* SHA-256.  
* Manifest validation.  
* Runtime compatibility.  
* Security metadata.  
* Release API.

Esto ya constituye un producto completo y demostrable.

# 45\. ROADMAP DE DESARROLLO

## FASE 0 — Product contract

* Cerrar naming.  
* Cerrar bundle spec.  
* Cerrar adapter contract.  
* Cerrar trust model.  
* Cerrar usage event privacy contract.  
* Cerrar data schema.  
* Crear 10 harness fixtures.  
* Escribir threat model.

### Definition of Done

Tres adapters pueden representar los mismos conceptos sin hacks específicos en UI y existe un bundle v1 capaz de instalarse en un fixture local.

## FASE 1 — harness-core

* Rust workspace.  
* Manifest parser.  
* Hash verification.  
* Filesystem plan.  
* Diff.  
* Snapshot.  
* Rollback.  
* Adapter interface.  
* Fixture test harness.  
* Codex adapter.

### Definition of Done

Desde tests/CLI interna se puede aplicar y revertir un harness Codex sin UI.

## FASE 2 — Desktop alpha

* Tauri shell.  
* Project picker.  
* Runtime detection.  
* Harness local import.  
* Preview.  
* Apply.  
* History.  
* Rollback.  
* Logging.  
* No backend todavía.

### Definition of Done

Un usuario técnico instala la app, importa un bundle local y alterna entre dos harnesses sin perder su configuración.

## FASE 3 — Registry API \+ Auth

* Supabase project.  
* GitHub Auth.  
* Profiles.  
* Harness metadata.  
* Versions.  
* Storage/releases.  
* RLS.  
* Release resolver.  
* Device Flow / desktop auth.

### Definition of Done

Desktop autenticado puede resolver y descargar una versión pública desde el registry.

## FASE 4 — Web social

* Profile pages.  
* Harness pages dinámicas.  
* Feed.  
* Votes.  
* Comments.  
* Saves.  
* Follows.  
* Moderation basics.  
* Use deep links.

### Definition of Done

La experiencia completa discovery → Use → feedback funciona con usuarios reales.

## FASE 5 — Multi-runtime beta

* Claude adapter completo.  
* OpenCode adapter completo.  
* Compatibility reports.  
* Runtime install flows.  
* Activation verification.  
* Verified use.  
* Trending ranking.  
* Desktop update pipeline.

### Definition of Done

El mismo harness o sus adapters pueden probarse de forma segura en al menos tres runtimes.

## FASE 6 — Monetization

* Pro billing.  
* Private harnesses.  
* Sync.  
* Teams alpha.  
* Org namespaces.  
* Audit events.  
* Usage analytics.

No habilitar billing hasta que el producto gratuito tenga usuarios que vuelvan a usar Desktop.

# 46\. TAREAS Y SUBTAREAS DEL CORE

## 46.1 Manifest

* JSON Schema.  
* SemVer.  
* Paths allowlist.  
* Runtime targets.  
* Permissions.  
* Executable declarations.  
* Environment requirements.

## 46.2 Plan engine

* Normalize source.  
* Resolve adapter.  
* Inspect target.  
* Detect conflicts.  
* Compute desired state.  
* Generate diff.  
* Classify risk.

## 46.3 Apply engine

* Acquire project lock.  
* Snapshot.  
* Atomic writes where possible.  
* Write journal.  
* Verify.  
* Commit history.  
* Recover after crash.

## 46.4 Restore

* Validate snapshot.  
* Detect external changes after snapshot.  
* Warn on conflicts.  
* Restore only managed paths.  
* Verify.

# 47\. TEST STRATEGY

## 47.1 Unit

* Manifest parsing.  
* Path safety.  
* Merge rules.  
* Hash verification.  
* Adapter capability mapping.

## 47.2 Golden fixtures

Repos de prueba con configuraciones conocidas para cada runtime y expected diff.

## 47.3 Cross-platform CI

* Windows x64.  
* macOS arm64.  
* macOS x64 si se soporta.  
* Linux x64.  
* Linux arm64 según release policy.

## 47.4 Destructive tests

* Crash durante Apply.  
* Disk full.  
* Permission denied.  
* File changed externally.  
* Malformed bundle.  
* Symlink escape.  
* Path traversal.  
* Invalid signature.  
* Hook malicious pattern.

## 47.5 E2E

Web Use → Desktop deep link → download → preview → apply → launch → revert.

# 48\. THREAT MODEL

* Malicious harness author.  
* Compromised builder account.  
* Tampered registry bundle.  
* Compromised update server.  
* Path traversal.  
* Symlink attack.  
* Command injection.  
* Secret exfiltration through MCP/hook.  
* Privilege escalation.  
* OAuth token theft.  
* Fake verified-use events.  
* Vote/comment spam.  
* Dependency compromise.

El threat model debe estar en el repo y actualizarse cada vez que Desktop gane permisos.

# 49\. RELEASE PIPELINE DEL DESKTOP

* Tag release.  
* Build matrix.  
* Tests.  
* Generate SBOM.  
* Create installers.  
* OS code signing/notarization.  
* Generate updater artifacts.  
* Tauri updater signing.  
* Generate checksums.  
* Optional Sigstore attestation.  
* Publish GitHub Release.  
* Publish latest.json.  
* Smoke test install.  
* Progressive rollout en el futuro.

Las signing keys críticas deben vivir en secretos CI o infraestructura de signing, no en repositorios.

# 50\. ANALYTICS

## 50.1 Web

* Profile views.  
* Harness views.  
* Use clicks.  
* Comments.  
* Votes.  
* Follows.  
* Conversion web → desktop.

## 50.2 Desktop

* App install.  
* Runtime detected.  
* Harness install.  
* Activate.  
* Revert.  
* Failure reason categórico.  
* Update.

No capturar comandos del usuario, contenido de archivos o nombres de proyectos como analytics.

# 51\. NORTH STAR Y KPI

North Star candidata: Weekly Active Harness Users — usuarios únicos que activan o usan al menos un harness mediante Desktop en una semana.  
Métricas secundarias:

* Weekly active builders.  
* Published harnesses with verified use.  
* Median uses per published harness.  
* Repeat use rate.  
* Switch rate entre harnesses.  
* Install → successful apply conversion.  
* Apply → revert within short window.  
* Comment/vote after verified use.  
* Builder retention.

# 52\. RIESGOS DE NEGOCIO

## 52.1 Runtimes agregan sus propios marketplaces

Mitigación: ser neutral y cross-runtime. Un marketplace de un proveedor no resuelve switching ni identidad portable.

## 52.2 Configuraciones se estandarizan demasiado

Mitigación: incluso con standards, discovery, versioning, reputation, evals, team governance y UX de instalación siguen siendo producto.

## 52.3 Seguridad frena adopción

Mitigación: preview, snapshots, trust classes, firmas y cero ejecución automática.

## 52.4 Pocos builders publican

Mitigación: extractor \+ Desktop Export \+ founding program.

## 52.5 Feed se vuelve ruido

Mitigación: unidades centradas en artefactos y uso, no posts libres en V1.

## 52.6 Costos

Mitigación: texto/metadata barato; bundles versionados en object storage; limitar realtime a features que lo necesitan.

# 53\. DECISIONES QUE RECOMIENDO TOMAR YA

* Tratar Web Social y Desktop Manager como un único producto, no como features desconectadas.  
* Tauri 2 \+ Rust core.  
* React/TypeScript UI.  
* GitHub como identidad inicial.  
* Supabase como backend MVP.  
* Tres runtimes profundos: Codex, Claude Code, OpenCode.  
* Project scope como default.  
* Preview \+ snapshot \+ rollback obligatorios.  
* No auto-run de scripts.  
* Verified use opt-in y privacy-minimal.  
* Upvotes, comments y saves; sin downvote/rating en V1.  
* Trending \+ New \+ Most Used; Following cuando haya volumen.  
* Bundle releases inmutables.  
* Desktop firmado y updater firmado.  
* Monetización después de retención, empezando por private/team workflows.

# 54\. NOMBRES POSIBLES

Mantener “Formosa.dev Harnesses” como iniciativa/registry es coherente en la etapa inicial.  
Para el desktop:

* Harness Desktop.  
* Harness Manager.  
* Harness Runtime.  
* Harness Hub.

Si la adopción global justifica una marca independiente, la separación debe ocurrir después de validar uso, no antes.

# 55\. VISION DE LARGO PLAZO

La visión grande no es una app para copiar AGENTS.md. Es una capa de distribución y reputación para configuraciones agentic.  
Un futuro usuario podría:

* seguir builders;  
* descubrir un harness;  
* instalarlo;  
* compararlo con otro;  
* correr evals;  
* publicar una variante;  
* llevarlo de Codex a Claude;  
* sincronizarlo entre equipos;  
* aplicar políticas de una organización;  
* medir adopción;  
* comprar soporte o workflows premium;  
* y mantener todo versionado y reversible.

En esa versión del producto, Harness Desktop se comporta como un package manager \+ environment manager \+ social client de la capa agentic.

# 56\. COPY DE REFERENCIA

## Hero

Your next agent workflow is already out there.  
Discover harnesses built by other developers. Try them on Codex, Claude Code or OpenCode. Switch safely. Keep what works.  
CTA: Explore harnesses · Download Harness Desktop.

## Harness card

Used by builders · Compatible with Codex \+ Claude Code · 6 skills · 2 MCPs.

## Use modal

Try this harness on your machine.  
We’ll show every change before applying it and create a restore point first.

## Builder CTA

Your setup is part of your craft. Publish it and let others build on it.

## Territorial signature

Built from Formosa. Open to builders everywhere.

# 57\. CONCLUSIÓN

El registry por sí solo puede generar comunidad; el Desktop puede convertir esa comunidad en uso diario. La combinación es mucho más fuerte que cualquiera de las dos piezas aisladas.  
La red social aporta identidad, discovery y reputación. El runtime manager aporta utilidad inmediata. El botón Use conecta ambas: convierte el trabajo de un builder en una experiencia reproducible para otra persona.  
La prioridad técnica debe ser ganarse el derecho a tocar la configuración local del usuario. Eso exige un producto extremadamente cuidadoso: cambios explicables, snapshots, rollback, firmas, adapters mantenidos y separación estricta de credenciales.  
La prioridad de producto debe ser que usar un harness de otra persona resulte suficientemente fácil como para convertirse en hábito. Si se logra esa experiencia, votos, comentarios, follows, feed, teams y monetización dejan de ser features aisladas y pasan a girar alrededor de una acción real: adoptar una forma de trabajar.

# 58\. REFERENCIAS TÉCNICAS CONSULTADAS

[Tauri — Distributin](https://v2.tauri.app/distribute/)g  
[Tauri — Update](https://v2.tauri.app/plugin/updater/)r  
[Electron — autoUpdate](https://www.electronjs.org/docs/latest/api/auto-updater)r  
[Electron — Code Signin](https://www.electronjs.org/docs/latest/tutorial/code-signing)g  
[OpenAI Codex — repository / installatio](https://github.com/openai/codex)n  
[OpenAI Developers — Skill](https://developers.openai.com/api/docs/guides/tools-skills)s  
[OpenAI Developers — MCP configuration exampl](https://developers.openai.com/learn/docs-mcp)e  
[Anthropic — Claude Code FAQ / installatio](https://support.claude.com/en/articles/14554922-claude-code-user-faq)n  
[Claude Code — extensibility overvie](https://code.claude.com/docs/en/features-overview)w  
[OpenCode — documentatio](https://opencode.ai/docs)n  
[OpenCode — Agent Skill](https://opencode.ai/docs/skills)s  
[OpenCode — Windows/WS](https://opencode.ai/docs/windows-wsl)L  
[Cursor — Plugin](https://cursor.com/docs/plugins)s  
[Cursor — Skill](https://cursor.com/docs/skills)s  
[Cursor — Hook](https://cursor.com/docs/hooks)s  
[Supabase — Aut](https://supabase.com/docs/guides/auth)h  
[Supabase — GitHub Aut](https://supabase.com/docs/guides/auth/social-login/auth-github)h  
[Supabase — Row Level Securit](https://supabase.com/docs/guides/database/postgres/row-level-security)y  
[Supabase — Edge Function](https://supabase.com/docs/guides/functions)s  
[Supabase — Realtime Authorizatio](https://supabase.com/docs/guides/realtime/authorization)n  
[GitHub — OAuth device flo](https://docs.github.com/en/apps/oauth-apps/building-oauth-apps/authorizing-oauth-apps)w  
[Sigstore — Cosign quickstar](https://docs.sigstore.dev/quickstart/quickstart-cosign/)t  
[Sigstore — Keyless signing overvie](https://docs.sigstore.dev/cosign/signing/overview/)w

# 59\. RELACIÓN CON EL DOCUMENTO MAESTRO ANTERIOR

Este documento no reemplaza “Formosa.dev Harnesses — Documento Maestro de Producto, Posicionamiento, Arquitectura y Roadmap v0.1”. Lo amplía desde otra perspectiva.  
Documento anterior: define registry, lenguaje, perfil público, participación, landing estática, publicación por PR y estrategia de community registry.  
Este documento: define la capa social mutable, el Desktop Runtime Manager, instalación/use, cross-runtime adapters, backend, trust/supply-chain, distribución multiplataforma y monetización.  
La arquitectura final debe leer ambos documentos como dos capas del mismo producto

# 60\. ADENDA DE DECISIONES — ZERO-FRICTION DISCOVERY & INSTALL

Esta sección incorpora como decisiones de producto explícitas las definiciones posteriores al documento original. No reemplaza ninguna sección anterior. Las amplía y eleva a requisitos centrales de producto.  
Decisión madre: una persona debe poder descubrir el harness de otra persona y llevarlo a un agente compatible con la menor cantidad posible de pasos, sin tener que localizar, interpretar, copiar y configurar manualmente cada archivo.  
La experiencia objetivo deja de ser solamente “descubrir un harness” y pasa a ser “descubrirlo y usarlo”. El producto debe diseñarse alrededor de esa continuidad.

# 61\. DECISIÓN: BUSCADOR UNIVERSAL DE BUILDERS \+ HARNESSES

La búsqueda debe ser una función primaria del producto, visible desde la home del ecosistema Harnesses, el feed, el Desktop y eventualmente desde agentes mediante API.

## 61.1 Qué busca

* Usuarios/builders por nombre, handle o GitHub username.  
* Harnesses por nombre, problema, descripción, tags o categorías.  
* Runtimes compatibles: Codex, Claude Code, OpenCode, Cursor y futuros.  
* Skills y capacidades declaradas.  
* Use cases: frontend, backend, testing, security, research, SDD, DevOps, data, etc.  
* Collections.  
* Organizaciones/equipos en fases posteriores.

## 61.2 Sintaxis de búsqueda

El buscador debe reconocer patrones útiles sin exigir filtros avanzados:

* @adriangmrraa → prioriza perfiles.  
* nextjs → prioriza harnesses/tags relacionados.  
* security claude → harnesses de seguridad compatibles con Claude Code.  
* from:@juan codex → harnesses de un builder para Codex.  
* mcp postgres → harnesses que declaran MCP/Postgres.

## 61.3 Resultados agrupados

La UI agrupa resultados en Builders, Harnesses y Collections. El usuario debe distinguir inmediatamente si está encontrando una persona o un artefacto.

## 61.4 Búsqueda dentro de un perfil

Cada perfil público debe ofrecer búsqueda y filtros sobre sus harnesses. Un usuario que llega al perfil de un amigo o referente no debe navegar una lista plana si esa persona tiene muchas publicaciones.

# 62\. DECISIÓN: PERFIL PÚBLICO ORIENTADO A “USAR LO QUE ESTA PERSONA CONSTRUYE”

El perfil público no es solamente una tarjeta social. Debe funcionar como biblioteca personal de workflows reutilizables.  
Ejemplo conceptual:  
@juanpepito

* Next.js Production Harness — Use.  
* Secure Backend Review — Use.  
* SDD Workflow — Use.  
* Frontend Motion System — Use.

Cada item muestra runtime support, versión, uso verificado y acción Use sin obligar a abrir primero una ficha completa.

# 63\. DECISIÓN: “USE” ES EL CTA PRINCIPAL

Cada harness publicado debe tener una acción primaria Use. View source, Save, Vote, Fork y Comment son acciones secundarias.  
La razón es estratégica: el producto debe medir su valor por la capacidad de transformar discovery en utilización real.

## 63.1 Resultado esperado

Use no significa descargar un ZIP. Significa iniciar un flujo controlado que termina con el harness aplicado a un runtime/proyecto o con un plan de instalación listo para ejecutar.

# 64\. DECISIÓN: SELECTOR DE RUNTIME EN CADA HARNESS

Al presionar Use, el usuario elige dónde quiere usar el harness.

* Codex.  
* Claude Code.  
* OpenCode.  
* Cursor.  
* Otros adapters soportados.

## 64.1 Estado de compatibilidad

Cada opción debe mostrar un estado claro:

* Native — el bundle posee representación nativa para ese runtime.  
* Adapted — Harness Core puede generar una adaptación soportada.  
* Partial — algunas capacidades no tienen equivalente.  
* Untested — técnicamente resoluble pero no verificado.  
* Unsupported — no se ofrece Apply.

## 64.2 Nunca ocultar pérdidas

Si pasar de Claude Code a Codex elimina hooks, subagents o una capacidad sin equivalente, el selector debe avisarlo antes de abrir Desktop.

# 65\. DECISIÓN: URL CANÓNICA UNIVERSAL DEL HARNESS

Cada harness debe tener una URL pública permanente que funcione como identidad del artefacto y como punto de entrada universal.  
Forma conceptual:  
https://formosa.dev.ar/h/adriangmrraa/nextjs-production  
La misma URL debe ser útil para:

* Abrir la página pública.  
* Compartir por WhatsApp, X, LinkedIn, Discord o GitHub.  
* Pegar en una conversación con un agente.  
* Resolver metadata machine-readable.  
* Abrir Harness Desktop.  
* Resolver la versión predeterminada.  
* Iniciar un install/use flow.

Una persona no debería necesitar conocer la estructura interna del bundle para compartirlo.

# 66\. DECISIÓN: UN LINK DEBE SER SUFICIENTE PARA UN AGENTE

Caso de uso obligatorio: el usuario copia la URL de un harness y se la entrega a Codex, Claude Code, OpenCode u otro agente compatible con una instrucción mínima.  
Ejemplo:  
“Instalá y usá este harness en este proyecto: https://formosa.dev.ar/h/adriangmrraa/nextjs-production”  
El agente no debe depender de interpretar visualmente la landing. La URL debe exponer mecanismos machine-readable para descubrir el artefacto y delegar la instalación al Harness Core oficial.

# 67\. HARNESS INSTALL PROTOCOL — CONCEPTO

Se define como requisito de producto un protocolo de resolución e instalación de harnesses. Working name: Harness Install Protocol.  
Objetivo: que humanos, Desktop, CLI y agentes resuelvan el mismo artefacto a través de contratos previsibles.

## 67.1 Pipeline

Canonical URL → Resolver → Manifest → Version → Bundle → Integrity Check → Runtime Adapter → Install Plan → User Policy → Snapshot → Apply → Verify → Launch.

## 67.2 Principio

Los agentes no implementan su propio instalador ad hoc. Delegan la operación al Harness Core o a una interfaz oficial compatible.

# 68\. MANIFEST MACHINE-READABLE

Toda URL canónica debe permitir descubrir un manifest machine-readable.

## 68.1 Endpoint conceptual

https://formosa.dev.ar/h/adriangmrraa/nextjs-production/manifest.json  
También puede publicarse mediante content negotiation o un endpoint de API equivalente. La URL humana sigue siendo la identidad primaria.

## 68.2 Información mínima

* schemaVersion.  
* owner.  
* slug.  
* displayName.  
* latestVersion.  
* versions disponibles.  
* bundle URL.  
* SHA-256.  
* firma/attestation cuando exista.  
* runtimes y support level.  
* component inventory.  
* permissions.  
* risk class.  
* requirements.  
* environment variable names sin valores.  
* license.  
* source repository.  
* release date.

## 68.3 Regla

El manifest describe qué instalar y cómo resolverlo. Nunca contiene secretos del autor ni del usuario.

# 69\. INSTALL PLAN COMO CONTRATO INTERMEDIO

Antes de modificar un proyecto, Harness Core genera un Install Plan determinístico.  
El plan debe ser serializable y legible por UI, CLI y agentes.

## 69.1 Contenido

* Runtime elegido.  
* Versión del runtime detectada.  
* Project path local.  
* Scope: project/user.  
* Harness \+ versión.  
* Archivos a crear.  
* Archivos a modificar.  
* Archivos potencialmente conflictivos.  
* Adaptaciones necesarias.  
* Componentes descartados por incompatibilidad.  
* MCP a configurar.  
* Hooks/scripts a habilitar.  
* Comandos potenciales.  
* Environment variables requeridas.  
* Risk class.  
* Snapshot strategy.  
* Verification steps.

## 69.2 Beneficio

La misma instalación puede ser explicada de forma idéntica en Desktop, CLI o conversación con un agente.

# 70\. WEB → DESKTOP: ONE-CLICK USE

Desde la web, después de seleccionar runtime, el flujo recomendado es abrir Harness Desktop mediante un deep link.

## 70.1 Deep link conceptual

harnesses://use/adriangmrraa/nextjs-production?runtime=codex\&version=1.4.0  
El deep link no debe transportar credenciales, código fuente ni información sensible. Solo identifica el artefacto, runtime y versión.

## 70.2 Si Desktop está instalado

* Abrir Harness Desktop.  
* Resolver harness.  
* Detectar proyectos recientes.  
* Detectar runtime.  
* Mostrar destino.  
* Generar Install Plan.  
* Aplicar según política.

## 70.3 Si Desktop no está instalado

* Mostrar landing de descarga.  
* Detectar sistema operativo.  
* Ofrecer instalador correcto.  
* Después de instalar, recuperar la intención Use original cuando sea técnicamente viable.

El objetivo es evitar que instalar Desktop rompa el contexto y obligue a buscar el harness otra vez.

# 71\. DESKTOP: PROJECT PICKER CERO FRICCIÓN

Harness Desktop debe recordar proyectos utilizados previamente sin enviar sus rutas a la nube.  
Cuando llega un Use deep link:

* Mostrar proyectos recientes.  
* Detectar repositorios abiertos recientemente por la app.  
* Permitir Browse.  
* Recordar la última combinación project \+ runtime con permiso del usuario.

Ejemplo de UX:  
Use Next.js Production by @adriangmrraa  
Project: demobondis  
Runtime: Codex  
\[Use harness\]  
Para usuarios recurrentes, el flujo puede reducirse a dos decisiones visibles.

# 72\. CLI: MISMO CORE, MISMO PROTOCOLO

La CLI debe ser otra interfaz de Harness Core, no un segundo sistema.

## 72.1 Comandos objetivo

harness use adriangmrraa/nextjs-production  
harness use @adriangmrraa/nextjs-production  
harness use https://formosa.dev.ar/h/adriangmrraa/nextjs-production  
harness use @adriangmrraa/nextjs-production \--runtime codex  
harness inspect @adriangmrraa/nextjs-production  
harness diff @adriangmrraa/nextjs-production \--runtime claude  
harness revert

## 72.2 Interacción

La CLI detecta runtime(s), muestra un selector si hay más de uno, detecta el repositorio actual como proyecto predeterminado y genera el mismo Install Plan que Desktop.

# 73\. AI-AGENT-NATIVE INSTALLATION

El producto debe soportar explícitamente agentes como clientes de instalación.

## 73.1 Flujo

* Usuario pega URL.  
* Agente reconoce que es un harness.  
* Agente resuelve el manifest/API.  
* Agente detecta si Harness CLI/Core está disponible.  
* Si está disponible, solicita un Install Plan.  
* Explica el plan al usuario cuando la política lo exige.  
* Ejecuta Apply mediante Harness Core.  
* Core crea snapshot.  
* Core verifica.  
* Agente continúa trabajando con el harness activo.

## 73.2 No duplicar lógica

Codex, Claude Code y OpenCode no deberían copiar archivos ellos mismos si existe Harness Core. El agente actúa como orquestador y conversacional layer; Harness Core es la autoridad sobre filesystem, adaptación, snapshots y rollback.

# 74\. BOOTSTRAP: QUÉ PASA SI HARNESS NO ESTÁ INSTALADO

Este es el único punto inevitablemente delicado del flujo agent-native.

## 74.1 Primera estrategia

La página/manifest publica instrucciones bootstrap oficiales y mínimas para cada plataforma. El agente puede reconocerlas, pero debe mostrar al usuario qué instalador/comando oficial va a ejecutar.

## 74.2 Desktop-first

Para usuarios no técnicos, el flujo recomendado sigue siendo instalar Harness Desktop desde el sitio.

## 74.3 CLI-first

Para developers, puede ofrecerse un instalador de CLI firmado o package manager apropiado.

## 74.4 Regla de seguridad

Nunca diseñar un copy-paste del tipo curl URL | sh como experiencia principal. Cero fricción no significa ejecutar código remoto opaco.

# 75\. INSTALACIÓN AUTÓNOMA: NIVELES DE AUTONOMÍA

La instalación puede ser altamente autónoma sin eliminar la seguridad. Se definen niveles.

## 75.1 Level 0 — Preview only

El agente/desktop analiza y muestra el plan; no escribe.

## 75.2 Level 1 — Safe apply

Puede aplicar automáticamente componentes pasivos dentro del project scope después de una confirmación única.

## 75.3 Level 2 — Trusted harness

El usuario puede autorizar previamente a un publisher/harness firmado para aplicar updates de bajo riesgo con fricción reducida.

## 75.4 Level 3 — Elevated

Cambios globales, scripts ejecutables, hooks, instalación de runtimes, uso de sudo/admin o acceso sensible siempre requieren aprobación explícita.  
La autonomía se gobierna por risk class y políticas del usuario, no por una promesa genérica de “auto install”.

# 76\. ZERO FRICTION PRINCIPLE

Se incorpora formalmente como principio de producto:  
“A developer who discovers a harness should be able to run it on a compatible agent without manually locating, copying or configuring individual harness files.”

## 76.1 Qué significa

* Una URL.  
* Un resolver.  
* Un manifest.  
* Un selector de runtime.  
* Un selector de proyecto.  
* Un Install Plan.  
* Un Apply.  
* Un rollback.

## 76.2 Qué no significa

* Ocultar qué cambia.  
* Ejecutar scripts sin informar.  
* Tomar credenciales.  
* Instalar runtimes silenciosamente.  
* Modificar user scope cuando project scope alcanza.  
* Sacrificar reversibilidad.

# 77\. MÉTRICAS DE FRICCIÓN

El principio Zero Friction debe medirse.

## 77.1 Usuario recurrente

* Discovery → Desktop open.  
* Desktop open → Install Plan.  
* Install Plan → successful Apply.  
* Successful Apply → agent launched.

Objetivo de diseño inicial: un usuario con Desktop y runtime ya instalados debería poder pasar de la página de un harness a tenerlo activo en alrededor de un minuto para un harness pasivo y compatible.

## 77.2 Usuario nuevo

Medir por separado download → install → auth → first harness apply. El objetivo no es ocultar pasos inevitables sino evitar repetición, pérdida de contexto y configuración manual.

# 78\. “COPY LINK” Y “COPY FOR AGENT”

Después de publicar un harness, el builder debe recibir herramientas claras para distribuirlo.

## 78.1 Copy link

Copia la URL canónica.

## 78.2 Copy for agent

Copia una instrucción breve y estable:  
Install and use this harness for the current repository using the official Harness installer: https://formosa.dev.ar/h/adriangmrraa/my-harness  
Review the installation plan before applying any elevated or executable changes.

## 78.3 Use with Codex / Claude / OpenCode

Botones específicos pueden generar deep links con runtime preseleccionado.  
No deben generar prompts gigantes. El conocimiento de instalación vive en el manifest \+ Harness Core.

# 79\. PUBLICACIÓN: RESULTADO INMEDIATO PARA EL BUILDER

Después de mergear/publicar:

* Se crea/actualiza su perfil público.  
* Se genera la página del harness.  
* Se genera la versión instalable.  
* Se calcula hash.  
* Se publica manifest.  
* Se actualiza search index.  
* Se generan deep links.  
* Se muestran botones Use.  
* Se habilita Copy link.  
* Se habilita Copy for agent.

El resultado de publicar debe sentirse como lanzar un paquete, no como agregar una fila a un directorio.

# 80\. SEARCH DENTRO DE HARNESS DESKTOP

Desktop también debe permitir discovery.

## 80.1 Search bar

Search builders, harnesses, workflows...

## 80.2 Result card

* Builder.  
* Harness.  
* Summary.  
* Compatibility con runtimes locales detectados.  
* Risk class.  
* Verified uses.  
* Version.  
* Use.

## 80.3 Ventaja

Desktop puede ordenar resultados según lo que realmente está instalado en la máquina. Si solo está Codex, puede priorizar harnesses compatibles con Codex sin ocultar los demás.

# 81\. AGENT SEARCH API

Los agentes deben poder consultar el registry sin scrapeo.

## 81.1 Endpoints conceptuales

GET /api/v1/search?q=frontend+premium\&runtime=codex  
GET /api/v1/users/adriangmrraa  
GET /api/v1/users/adriangmrraa/harnesses  
GET /api/v1/harnesses/adriangmrraa/nextjs-production  
GET /api/v1/harnesses/adriangmrraa/nextjs-production/versions/1.4.0  
GET /api/v1/harnesses/adriangmrraa/nextjs-production/versions/1.4.0/manifest

## 81.2 Requisitos

* JSON estable y versionado.  
* Rate limits.  
* ETag/cache.  
* No auth para metadata pública básica.  
* Auth para acciones del usuario.  
* Nunca devolver secrets.  
* Documentación explícita para agentes y developers.

# 82\. MCP COMO INTERFAZ FUTURA DE DISCOVERY

Además de HTTP API, puede existir un MCP server oficial del registry para que agentes compatibles busquen y consulten harnesses mediante tools.

## 82.1 Tools conceptuales

* search\_harnesses.  
* get\_harness.  
* get\_builder.  
* get\_manifest.  
* get\_install\_plan.  
* get\_compatibility.

## 82.2 Límite

El MCP del registry descubre y planifica. La escritura local debe seguir delegándose a Harness Core con las políticas de seguridad del dispositivo.

# 83\. COMANDO NATURAL COMO EXPERIENCIA

Objetivo de producto:  
“Buscame el harness de @juanpepito para Next.js y usalo acá.”  
El agente debería poder:

* Buscar al usuario.  
* Encontrar sus harnesses.  
* Filtrar por Next.js.  
* Comparar compatibilidad con su runtime.  
* Presentar la opción.  
* Resolver manifest.  
* Generar Install Plan.  
* Aplicar mediante Core.

Esto convierte el registry en infraestructura de discovery para agentes, no solamente en un sitio para humanos.

# 84\. SELECCIÓN INTELIGENTE DE RUNTIME

Si el usuario no especifica runtime, Harness Core debe detectar los disponibles.

## 84.1 Caso uno

Solo Codex instalado → preseleccionar Codex.

## 84.2 Caso dos

Codex \+ Claude instalados → mostrar selector, ordenado por compatibilidad del harness.

## 84.3 Caso tres

Ninguno instalado → explicar opciones y ofrecer instalación oficial.

## 84.4 Caso cuatro

Runtime solicitado pero no instalado → ofrecer instalar ese runtime; no reemplazarlo silenciosamente por otro.

# 85\. INSTALACIÓN DEL RUNTIME DESDE EL MISMO FLUJO

Si falta Codex/Claude/OpenCode, el usuario no debería abandonar el proceso.  
Use → runtime missing → Install runtime → verify → return to pending harness → Install Plan → Apply.  
La app conserva una Pending Intent local para continuar automáticamente después del bootstrap.  
La autenticación del runtime sigue siendo responsabilidad del proveedor y se realiza a través de su flujo oficial.

# 86\. LINK QUE EL MISMO BUILDER PUEDE USAR

El builder no es solamente quien publica. También es usuario.  
Después de publicar desde una computadora, puede copiar su link y en otra máquina:

* Abrirlo en web.  
* Pegarlo en Codex.  
* Ejecutar harness use URL.  
* Abrirlo con Desktop.

Esto funciona como distribución personal cross-machine incluso antes de agregar sync privado.

# 87\. SHAREABILITY COMO MOTOR DE CRECIMIENTO

Cada harness debe ser altamente compartible.

* Preview social con nombre, builder y runtimes.  
* Open Graph.  
* Copy link.  
* QR opcional para eventos.  
* Use with Codex.  
* Use with Claude.  
* Use with OpenCode.  
* Copy for agent.

Un link compartido debe convertir directamente en discovery o Use, no en una landing genérica.

# 88\. FEED \+ USE: CIERRE DEL LOOP SOCIAL

El feed debe aprovechar las acciones reales.

* @juan published X.  
* X now supports Codex.  
* X reached 100 verified users.  
* @maria forked X into Y.  
* A new version of a harness you use is available.

Nunca exponer públicamente qué proyecto privado está usando una persona.  
La acción Use puede elevar un harness en Most Used/Trending, pero no crear un post personal automático sin consentimiento.

# 89\. VOTOS Y COMENTARIOS POST-USE

Después de una instalación exitosa y cierto tiempo de uso, Desktop puede sugerir —sin bloquear—:  
“You tried @juan/nextjs-production. Leave feedback?”

* Upvote.  
* Comment.  
* Report issue.  
* Fork/improve.

Este momento produce feedback mucho más valioso que pedir una valoración inmediatamente después de una vista.

# 90\. LOCAL LIBRARY

Harness Desktop debe tener una Library.

* Installed.  
* Active.  
* Saved.  
* Created by me.  
* Recently used.  
* Updates.  
* Snapshots.

Esto transforma el producto en un gestor cotidiano incluso cuando el usuario no está navegando el feed.

# 91\. SWITCH CON UN CLICK

Una vez que el proyecto está administrado por Harness Core:  
Harness A → \[Switch\] → Harness B.  
Core calcula la transición desde el estado administrado conocido, preserva local overrides, crea nuevo snapshot y aplica.  
El botón Switch debe mostrar incompatibilidades antes de la operación.

# 92\. AUTONOMÍA Y CONFIRMACIÓN: MATRIZ DE DECISIÓN

## 92.1 Sin confirmación adicional

* Leer metadata pública.  
* Buscar builders.  
* Buscar harnesses.  
* Descargar manifest.  
* Calcular Install Plan.  
* Analizar diff.

## 92.2 Confirmación simple

* Aplicar cambios pasivos en project scope.  
* Configurar archivos del runtime dentro del repo.

## 92.3 Confirmación reforzada

* Scripts.  
* Hooks.  
* MCP que accede a red/servicios.  
* User scope.  
* Instalación de runtimes.  
* Cambios fuera del repo.

## 92.4 Confirmación del sistema operativo

* Elevación admin/sudo.  
* Code signing prompts.  
* Acceso a carpetas protegidas.

# 93\. FAIL-SAFE CERO FRICCIÓN

La experiencia debe optimizar velocidad sin convertir errores en pérdida de trabajo.

* Nunca borrar archivos no administrados.  
* Nunca sobrescribir un archivo conflictivo sin plan.  
* Snapshot antes de Apply.  
* Journal de operación.  
* Crash recovery.  
* Rollback visible.  
* Hash verification.  
* No ejecutar código en preview.  
* No resolver URLs arbitrarias como comandos.

# 94\. DEFINITION OF DONE — ZERO-FRICTION FLOW

La feature no está terminada hasta que se cumplan estos casos.

* Buscar una persona por handle y usar uno de sus harnesses.  
* Buscar un harness por función y usarlo.  
* Abrir un link compartido y usarlo.  
* Pegar el link en un agente y completar el flujo.  
* Usar CLI con URL.  
* Cambiar runtime.  
* Detectar runtime faltante y volver al flujo después de instalarlo.  
* Aplicar y revertir sin pérdida.  
* Mostrar pérdidas de compatibilidad.  
* Mantener secrets fuera del registry.  
* Registrar Verified Use sin subir información del proyecto.

# 95\. ROADMAP ESPECÍFICO DE ESTA ADENDA

## 95.1 ZF-0 — Canonical identity

* Definir canonical URL.  
* Definir manifest discovery.  
* Definir resource IDs.  
* Definir version resolution.

## 95.2 ZF-1 — Resolver \+ CLI prototype

* harness inspect URL.  
* harness use URL.  
* Runtime detection.  
* Install Plan.  
* Snapshot/apply/revert.

## 95.3 ZF-2 — Search

* Search API.  
* Users index.  
* Harness index.  
* Runtime filters.  
* Desktop search.

## 95.4 ZF-3 — Web Use

* Use CTA.  
* Runtime selector.  
* Deep link.  
* Download fallback.  
* Pending Intent.

## 95.5 ZF-4 — Agent-native

* Agent-facing API docs.  
* Copy for agent.  
* CLI bootstrap docs.  
* MCP discovery server opcional.  
* Codex/Claude/OpenCode integration tests.

## 95.6 ZF-5 — Social loop

* Verified Use.  
* Post-use feedback.  
* Most Used.  
* Trending.  
* Feed events.

# 96\. DECISIONES CONSOLIDADAS DE ESTA ITERACIÓN

* El buscador debe encontrar builders y harnesses.  
* Cada perfil público funciona como biblioteca de harnesses.  
* Use es CTA primario.  
* El usuario elige runtime en cada Use.  
* Cada harness posee una URL canónica universal.  
* La URL debe poder entregarse directamente a un agente.  
* La URL resuelve un manifest machine-readable.  
* Desktop, CLI y agentes usan el mismo Harness Core.  
* La instalación se expresa primero como Install Plan.  
* Web abre Desktop por deep link.  
* Desktop recuerda proyectos localmente.  
* La CLI acepta owner/slug, @user/slug o URL completa.  
* El agente orquesta; Harness Core modifica el sistema.  
* Cero fricción no elimina confirmaciones de riesgo.  
* Project scope sigue siendo default.  
* Runtime missing se resuelve dentro del mismo flujo.  
* Search API pública forma parte del producto.  
* Un MCP de discovery es una extensión válida futura.  
* Verified Use se vincula al flujo de instalación real.  
* Después del uso se incentiva voto/comentario/fork.  
* El mismo link sirve al autor para reinstalar su harness en otra máquina.

# 97\. PROPUESTA DE VALOR REFORMULADA

La forma más simple de explicar el producto pasa a ser:  
“Encontrá cómo trabaja otra persona con IA y usalo en tu máquina con un click.”  
Versión global:  
“Discover how other builders work with AI. Run their harness on your agent in one click.”  
La complejidad técnica —manifests, adapters, signatures, snapshots, policies y compatibility— existe para hacer verdadera esa promesa, no para convertirse en el mensaje principal.

# 98\. VISIÓN RESULTANTE

La combinación completa deja de parecer un directorio y empieza a parecer una nueva capa de infraestructura developer:  
Social network \+ registry \+ search engine \+ package manager \+ environment manager \+ runtime adapter layer.  
Para humanos: buscar una persona, descubrir su workflow y usarlo.  
Para agentes: resolver una URL, entender el artefacto y delegar una instalación segura.  
Para builders: publicar una vez y obtener perfil, versión, link, distribución y feedback.  
Para equipos: estandarizar workflows más adelante.  
El principio rector es consistente en todos los casos: una buena forma de trabajar con agentes debe poder convertirse en un artefacto encontrable, atribuible, portable, instalable y reversible

# 99\. DECISIÓN: ONE COMMAND FROM ZERO TO RUNNING

Se incorpora una decisión adicional de producto: la experiencia principal debe poder resumirse en un único comando específico para el sistema operativo del usuario.  
El objetivo no es solamente instalar Harness CLI. El objetivo es que ese comando pueda llevar al usuario desde una máquina preparada para desarrollo hasta un harness activo sobre el agente elegido con la menor intervención manual posible.  
Promesa operacional:  
“Copy one command. Run it. Choose your project/runtime if needed. Harness does the rest.”

# 100\. EL COMANDO COMO PUNTO DE ENTRADA UNIVERSAL

Toda página de harness debe mostrar una sección Use in one command.  
La web detecta el sistema operativo del visitante cuando sea posible y prioriza el comando correspondiente, manteniendo tabs Windows / macOS / Linux y una opción Copy for AI agent.  
Conceptualmente, el comando debe combinar dos fases:

* Bootstrap: asegurar que Harness Core/CLI está disponible.  
* Use: resolver e instalar el harness solicitado.

El usuario no necesita ejecutar primero un instalador y después buscar manualmente el harness.

# 101\. FORMATO OBJETIVO DEL COMANDO

La interfaz estable debe converger en una orden lógica única:  
harness use \<canonical-url-or-owner/slug\> \--runtime \<runtime\> \--project \<path\>  
El mecanismo de bootstrap puede variar por sistema operativo, pero una vez instalado Harness, la semántica es idéntica.

## 101.1 Formas válidas de identificar un harness

harness use @adriangmrraa/nextjs-production  
harness use adriangmrraa/nextjs-production  
harness use https://formosa.dev.ar/h/adriangmrraa/nextjs-production

## 101.2 Runtime opcional

Si se omite \--runtime, Harness detecta los runtimes presentes y selecciona automáticamente cuando solo existe una opción claramente compatible; si existen varias, muestra un selector.

## 101.3 Project opcional

Si se ejecuta dentro de un repositorio, el directorio actual es el proyecto predeterminado. Si se ejecuta fuera de un proyecto, Harness solicita selección o abre el project picker.

# 102\. ONE COMMAND POR SISTEMA OPERATIVO

La landing no debe enseñar una secuencia larga de instalación. Debe presentar un comando principal por plataforma, generado específicamente para el harness y runtime seleccionados.

## 102.1 Windows

Objetivo de distribución: publicar Harness en WinGet y disponer de un bootstrap firmado para los casos donde WinGet no esté disponible.  
Forma conceptual cuando Harness ya está disponible mediante WinGet:  
winget install \<HarnessPackageId\> && harness use https://formosa.dev.ar/h/owner/slug \--runtime codex  
En PowerShell se debe proporcionar una variante sintácticamente correcta y probada para encadenar instalación \+ uso. El comando final se fija únicamente después de publicar el package ID real.

## 102.2 macOS

Objetivo de distribución: Homebrew tap/cask y aplicación firmada/notarizada.  
Forma conceptual:  
brew install \<harness-package\> && harness use https://formosa.dev.ar/h/owner/slug \--runtime codex  
El identificador real se publica solamente cuando exista el tap/package oficial.

## 102.3 Linux

Objetivo: ofrecer paquete o bootstrap nativo según distribución y arquitectura.  
Idealmente, soportar canales conocidos (.deb/.rpm/AppImage o repositorios) y una herramienta bootstrap firmada.  
Si se ofrece un instalador shell de una línea, debe descargar una versión identificable, verificar integridad/firma antes de instalar y exponer el script públicamente para auditoría. No debe convertirse en un pipe remoto opaco que ejecute cualquier contenido cambiante sin verificación.

## 102.4 Regla documental

Hasta que los package IDs y dominios finales existan, los comandos del producto permanecen conceptuales. Nunca publicar en la landing un comando que apunte a un identificador ficticio.

# 103\. EL BOOTSTRAPPER

Para cumplir realmente la promesa de un comando, se necesita un bootstrapper muy pequeño, separado del Desktop completo.

## 103.1 Responsabilidades

* Detectar OS.  
* Detectar arquitectura.  
* Resolver la última versión estable compatible de Harness Core/CLI.  
* Descargar artifact desde origen oficial.  
* Verificar SHA-256 y firma.  
* Instalar en user scope cuando sea posible.  
* Agregar el ejecutable al PATH de manera controlada.  
* Continuar la intención original: use \<harness\>.  
* Eliminar archivos temporales.

## 103.2 Lo que no hace

* No interpreta arbitrariamente el README.  
* No ejecuta scripts del harness.  
* No recibe secretos.  
* No instala runtimes sin pasar por el policy engine.  
* No modifica proyectos directamente; delega a Harness Core.

# 104\. PENDING INTENT — NO PERDER LA ACCIÓN ORIGINAL

El bootstrap debe conservar la intención completa que originó el comando.  
Ejemplo conceptual de Pending Intent:

* harness: @adriangmrraa/nextjs-production.  
* version: latest o versión fija.  
* runtime: codex.  
* project: directorio actual.  
* requestedAction: use.

Después de instalar Harness, el bootstrap ejecuta esa intención automáticamente. El usuario no debe volver a copiar el link ni repetir la selección.

# 105\. DEL COMANDO A LA INSTALACIÓN COMPLETA

Cuando se ejecuta el comando, el flujo interno objetivo es:

* 1\. Bootstrapping de Harness si falta.  
* 2\. Resolver URL/owner/slug.  
* 3\. Descargar manifest.  
* 4\. Verificar versión, hash y firma.  
* 5\. Detectar proyecto.  
* 6\. Detectar runtimes.  
* 7\. Resolver adapter.  
* 8\. Si el runtime falta, ofrecer instalación oficial dentro del mismo flujo.  
* 9\. Comprobar autenticación superficial del runtime.  
* 10\. Construir Install Plan.  
* 11\. Clasificar riesgo.  
* 12\. Mostrar solamente las confirmaciones que exige la política.  
* 13\. Crear snapshot.  
* 14\. Aplicar.  
* 15\. Verificar.  
* 16\. Registrar estado local.  
* 17\. Abrir/lanzar el runtime.

El comando puede ser uno; internamente sigue existiendo un proceso auditable y reversible.

# 106\. FAST PATH: CUANDO REALMENTE PUEDE SENTIRSE COMO UN CLICK

Para un harness de riesgo bajo, project scope, runtime ya instalado y usuario con una política previa de confianza, la experiencia puede acercarse a:  
Paste command → Enter → snapshot → apply → launch.  
La salida debe seguir informar qué ocurrió, incluso si no solicita interacción.

## 106.1 Ejemplo de salida

Harness: @juan/nextjs-production@1.4.0  
Runtime: Codex  
Project: current repository  
Risk: Passive  
Snapshot: created  
Applied: 6 files  
Verification: passed  
Launching Codex…

# 107\. AGENT GUIDE: EL COMANDO TAMBIÉN ESTÁ DISEÑADO PARA IA

Cada harness debe exponer una guía concisa específica para agentes de IA. La guía no reemplaza el manifest; explica cómo un agente debe utilizar el instalador oficial sin reinventar el proceso.

## 107.1 Copy for AI agent

La web genera una instrucción preparada para copiar:  
“Use the official Harness installer to install and activate this harness in the current repository: \<canonical URL\>. Detect the current operating system and available runtime. Prefer project scope. Ask before any elevated, executable, global or credential-sensitive action. Do not manually copy harness files when Harness Core is available. After installation, verify the result and continue using the selected runtime.”

## 107.2 Por qué importa

El usuario no necesita explicarle a cada agente dónde vive AGENTS.md, dónde van las skills o cómo mapear hooks. La guía obliga al agente a delegar esas decisiones al Core.

# 108\. MACHINE-READABLE AGENT INSTALL GUIDE

Además del texto Copy for AI agent, cada versión puede publicar una guía machine-readable.

## 108.1 Endpoint conceptual

/api/v1/harnesses/\<owner\>/\<slug\>/agent-install

## 108.2 Respuesta

* canonical\_url.  
* resolved\_version.  
* recommended\_command.  
* supported\_runtimes.  
* preferred\_scope.  
* risk\_class.  
* requires\_confirmation.  
* install\_protocol\_version.  
* manifest\_url.  
* documentation\_url.

## 108.3 Beneficio

Un agente puede preguntar al producto cuál es la instrucción correcta para esa versión en vez de generar comandos por memoria.

# 109\. COMANDOS GENERADOS SEGÚN CONTEXTO

El comando visible no tiene que ser idéntico para todos. La web puede generarlo a partir del contexto seleccionado.

## 109.1 Ejemplo

Usuario seleccionó Windows \+ Codex \+ latest.  
La UI genera el comando bootstrap Windows que termina ejecutando harness use URL \--runtime codex.

## 109.2 Link compartido

Si un amigo abre la misma URL desde macOS, la página muestra automáticamente la variante macOS.

## 109.3 Copiar para agente

No depende del OS del navegador. La instrucción le dice al agente que detecte OS en la máquina donde está ejecutándose y solicite al endpoint oficial el bootstrap correspondiente.

# 110\. API DE BOOTSTRAP

Para permitir agentes y automatización sin hardcodear comandos que cambian, el backend debe poder resolver el bootstrap vigente.

## 110.1 Endpoint conceptual

GET /api/v1/bootstrap?os=windows\&arch=x64\&action=use\&harness=@owner/slug\&runtime=codex

## 110.2 Retorno conceptual

* protocolVersion.  
* installerVersion.  
* artifact URL.  
* sha256.  
* signature.  
* displayCommand.  
* postInstallArgs.  
* minimumOS.  
* releaseNotes URL.

El displayCommand es generado por el servicio y puede actualizarse cuando cambia la distribución sin modificar cada README.

# 111\. INSTALLER DISCOVERY PARA AGENTES

La URL del harness debe anunciar dónde encontrar su instalación oficial mediante metadata fácil de descubrir.

* Link/metadata en HTML.  
* Manifest JSON.  
* API documentada.  
* Opcional llms.txt/agents documentation a nivel del producto.  
* MCP de discovery en fase posterior.

El objetivo es que un agente no necesite hacer scraping creativo ni inferir comandos.

# 112\. INSTALAR TAMBIÉN EL RUNTIME DESDE EL MISMO COMANDO

La ambición zero-friction incluye la posibilidad de que el mismo flujo instale Codex, Claude Code u OpenCode cuando falten.  
Eso no significa empaquetar ni redistribuir esos productos sin autorización. El adapter utiliza su canal oficial de instalación.

## 112.1 Flujo

harness use URL \--runtime codex  
→ Codex not found.  
→ “Codex is required. Install using the official provider method?”  
→ user approves.  
→ adapter invokes verified official installation.  
→ verifies version.  
→ provider authentication if required.  
→ resumes Pending Intent.  
→ harness applies.  
→ launches Codex.

## 112.2 Resultado

Desde el punto de vista conceptual del usuario sigue siendo un solo comando, aunque una instalación nueva de runtime pueda requerir autenticación o confirmaciones del sistema operativo.

# 113\. FLAGS PARA AUTOMATIZACIÓN CONTROLADA

La CLI debe contemplar automatización para usuarios avanzados y agentes, pero sin un \--yes absoluto que saltee cualquier riesgo.

## 113.1 Flags conceptuales

* \--runtime codex.  
* \--project .  
* \--version 1.4.0.  
* \--scope project.  
* \--non-interactive.  
* \--approve-passive.  
* \--json.  
* \--plan-only.  
* \--no-launch.

## 113.2 Policy

\--non-interactive no autoriza operaciones Elevated. Si una operación excede la política declarada, la CLI devuelve un estado machine-readable que exige intervención humana.  
Esto permite que un agente sea autónomo cuando corresponde y se detenga precisamente cuando corresponde.

# 114\. SALIDA JSON PARA AGENTES

La CLI debe poder responder en JSON para evitar que un agente tenga que parsear texto humano.  
Ejemplo conceptual:  
{ status: "needs\_confirmation", risk: "executable", planId: "...", changes: 7, confirmationReason: "hook execution" }  
Después de autorización:  
{ status: "installed", snapshotId: "...", runtime: "codex", version: "1.4.0", verification: "passed" }

# 115\. COMANDO DE UN SOLO USO SIN INSTALACIÓN PERMANENTE

Como opción complementaria para developers, puede existir una modalidad de ejecución efímera si el canal de distribución elegido lo permite.  
Ejemplo conceptual: un launcher descarga Harness Core verificado a un directorio temporal, ejecuta use y luego ofrece instalarlo permanentemente.  
Esta modalidad puede reducir la barrera de primera prueba, pero no debe sacrificar firma, checksum ni auditabilidad.

# 116\. PRIMERA EXPERIENCIA DEL USUARIO

El onboarding ideal no debería comenzar con una pantalla vacía.  
Escenario:

* 1\. Un amigo comparte un harness.  
* 2\. Usuario abre el link.  
* 3\. Ve quién lo hizo, para qué sirve y trust signals.  
* 4\. Selecciona Codex.  
* 5\. Copia el único comando mostrado.  
* 6\. Lo pega en terminal.  
* 7\. Harness se instala si falta.  
* 8\. Detecta el repo actual.  
* 9\. Genera plan.  
* 10\. Usuario confirma únicamente si existe una operación que lo requiere.  
* 11\. Se crea snapshot.  
* 12\. Se aplica.  
* 13\. Codex se abre preparado.

Ese flujo debe convertirse en la demo principal del producto.

# 117\. EXPERIENCIA “DÁSELO A TU AGENTE”

Segundo demo principal:

* 1\. Copiar URL del harness.  
* 2\. Abrir Codex/Claude/OpenCode en un proyecto.  
* 3\. Escribir: “Usá este harness acá: \<URL\>”.  
* 4\. El agente descubre la guía oficial.  
* 5\. Invoca Harness Core/bootstrap.  
* 6\. Recibe Install Plan.  
* 7\. Pide confirmación solo cuando la policy lo exige.  
* 8\. Core instala y verifica.  
* 9\. El agente continúa ya condicionado por el nuevo harness.

Esta experiencia debe funcionar sin pedir al usuario que le explique rutas, formato, skills o reglas del runtime.

# 118\. OBJETIVO DE UX REFORMULADO

La meta no es “tenemos instaladores para Windows/macOS/Linux”.  
La meta es:  
“Every harness has one obvious way to run it.”  
La interfaz puede ofrecer varias superficies —web, Desktop, CLI, AI agent— pero todas convergen en el mismo protocolo y en el mismo Core.

# 119\. DEFINITION OF DONE — ONE COMMAND

* Existe un comando válido y probado para cada OS soportado.  
* El comando puede llevar una URL canónica como argumento.  
* Instala Harness si falta.  
* Conserva Pending Intent.  
* Detecta arquitectura.  
* Detecta runtime.  
* Puede ofrecer instalación oficial de runtime faltante.  
* Detecta proyecto actual.  
* Genera Install Plan.  
* No salta confirmaciones elevadas.  
* Crea snapshot.  
* Aplica.  
* Verifica.  
* Puede lanzar el runtime.  
* Tiene output JSON para agentes.  
* Tiene rollback.  
* La web genera el comando exacto según harness/runtime/OS.  
* Copy for AI Agent usa el mismo protocolo.

# 120\. PRIORIDAD DE IMPLEMENTACIÓN ACTUALIZADA

Esta decisión cambia el orden del MVP. Antes de construir una red social compleja, conviene demostrar la magia principal:  
URL compartible → one command → harness activo → rollback.  
Orden recomendado:

* 1\. Bundle \+ manifest v1.  
* 2\. Harness Core.  
* 3\. Adapter Codex.  
* 4\. One-command bootstrap en un OS.  
* 5\. Snapshot/apply/revert.  
* 6\. Canonical harness URL.  
* 7\. Copy command.  
* 8\. Copy for AI agent.  
* 9\. Segundo y tercer runtime.  
* 10\. Cross-platform packaging.  
* 11\. Search/users.  
* 12\. Feed/votes/comments.

Esta secuencia valida primero la utilidad nuclear. La capa social amplifica algo que ya funciona.

# 121\. DECISIÓN FINAL DE ESTA ITERACIÓN

El producto se diseña desde ahora con una premisa adicional y explícita:  
“Un harness publicado debe poder pasar de una URL a un agente funcionando mediante un único comando o una única instrucción a un agente de IA, usando Harness Core como autoridad de instalación.”  
Todo componente futuro —web, Desktop, CLI, API, feeds y perfiles— debe preservar esa propiedad.

