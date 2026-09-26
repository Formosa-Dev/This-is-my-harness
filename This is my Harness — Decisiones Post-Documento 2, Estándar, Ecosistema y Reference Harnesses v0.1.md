# THIS IS MY HARNESS

## Decisiones Post-Documento 2, Estándar, Ecosistema y Reference Harnesses · v0.1

## Documento consolidado de decisiones, hipótesis técnicas, protocolos compatibles, perfiles de harness y estrategia de validación previa al lanzamiento

Propósito de este documento: consolidar en una sola fuente de verdad todas las decisiones, cambios de tesis, ideas técnicas e investigación que aparecieron en la conversación después de la creación del segundo documento maestro de This is my Harness. No reemplaza los documentos anteriores. Los corrige, amplía y reordena conceptualmente donde las decisiones posteriores cambiaron el centro de gravedad del producto.  
Decisión más importante: This is my Harness no debe entenderse primero como una red social de configuraciones. Debe entenderse como el estándar abierto y el toolchain que permite representar, empaquetar, validar, distribuir, instalar y ejecutar harnesses completos de manera consistente entre diferentes runtimes y arquitecturas agentic.  
La red social, el registry, los perfiles públicos, votos, comentarios, feed, búsqueda y Verified Use siguen siendo parte del producto, pero quedan construidos encima del estándar. El estándar debe tener valor técnico aun si la capa social no existiera.

# 1\. Cambio de tesis: de registry social a estándar de sistemas agentic

El repositorio This-is-my-harness debe responder primero una pregunta técnica: ¿qué tiene que cumplir un artefacto para poder llamarse un harness compatible con This is my Harness?  
La tesis resultante:  
This is my Harness es un estándar abierto y un toolchain para empaquetar, distribuir, validar e instalar arquitecturas agentic portables.  
Versión corta de producto:  
One harness. Multiple agents. One standard way to package, distribute and run it.  
North Star técnica:  
Define once. Run anywhere.  
Definición propuesta:  
A harness is a portable, declarative package that describes how models, agents, tools, context, interfaces, policies, workflows and runtime services work together.  
En español: un harness es un paquete declarativo y portable que describe cómo trabajan juntos modelos, agentes, herramientas, contexto, interfaces, políticas, workflows y servicios de runtime.

# 2\. Qué debe estandarizar el proyecto

## 2.1 Harness Core Specification

* Identidad, nombre y namespace.  
* Versión del paquete.  
* Versión del schema.  
* Manifest canónico.  
* Estructura de paquete.  
* Tipos de componentes.  
* Requirements.  
* Permisos.  
* Risk declarations.  
* Dependencias.  
* Distribución.  
* Compatibilidad.  
* Conformance metadata.

## 2.2 Manifest Specification

Debe existir un entrypoint machine-readable único. Working name: harness.yaml.  
El manifest no reemplaza los archivos nativos de un runtime ni todos los estándares integrados. Describe la composición del sistema y permite que Harness Core resuelva cómo reproducirlo.

## 2.3 Package Structure

El estándar debe definir semántica de carpetas/componentes sin obligar a que todo harness sea complejo.  
Estructura conceptual:  
my-harness/  
├─ harness.yaml  
├─ README.md  
├─ instructions/  
├─ skills/  
├─ agents/  
├─ rules/  
├─ tools/  
├─ mcp/  
├─ models/  
├─ services/  
├─ workflows/  
├─ policies/  
├─ memory/  
├─ hooks/  
├─ scripts/  
└─ adapters/

## 2.4 Runtime Adapter Contract

Los runtimes no son iguales y no deben fingirse equivalentes. El estándar necesita un contrato común para adapters que traduzcan el modelo canónico a formatos nativos.

* detect()  
* installPlan()  
* install()  
* authStatus()  
* inspectExisting(project)  
* capabilities()  
* planApply(bundle, scope)  
* apply(plan)  
* verify(project)  
* launch(project)  
* revert(snapshot)

Pérdidas de compatibilidad deben declararse. Nunca silenciosamente.

## 2.5 Install Protocol

Pipeline conceptual:  
Canonical ID/URL → Resolver → Manifest → Version → Artifact → Hash/Signature → Capability Resolution → Adapter → Install Plan → Snapshot → Apply → Verify → Launch.  
El comando es una implementación del protocolo. No debe confundirse con el estándar en sí.

## 2.6 Conformance

La compatibilidad debe ser verificable, no declarativa.  
Comandos conceptuales:  
harness validate .  
harness test \--runtime codex  
harness conformance  
El objetivo es poder decir exactamente qué pasó: schema válido, package válido, permisos declarados, adapter probado, fixture pasado, capability soportada o capability degradada.

# 3\. El manifest canónico

Borrador conceptual:  
apiVersion: thisismyharness.dev/v1alpha1  
kind: Harness  
metadata:  
  name: nextjs-production  
  version: 1.2.0  
  author: ...  
  license: Apache-2.0  
spec:  
  profile: developer  
  components: ...  
  requirements: ...  
  permissions: ...  
  distribution: ...  
  compatibility: ...  
No debe fijarse todavía un v1.0. El propósito de v1alpha1 es permitir que los Reference Harnesses rompan el diseño y obliguen a corregirlo antes de estabilizar.

# 4\. Core pequeño, extensiones fuertes

Una decisión crítica es evitar una megaespecificación imposible de implementar. El Core debe ser pequeño y estable; capacidades avanzadas deben entrar como extensions versionadas.  
Capacidades candidatas:

* models.generative  
* models.system-one  
* models.embedding  
* models.reranker  
* models.classifier  
* interfaces.mcp  
* interfaces.webmcp  
* interfaces.a2a  
* interfaces.agui  
* interfaces.a2ui  
* ui.mcp-apps  
* knowledge.agent-skills  
* workflows.arazzo  
* workflows.mcp-tasks  
* observability.opentelemetry  
* distribution.oci  
* runtime.codex  
* runtime.claude-code  
* runtime.opencode

Regla arquitectónica:  
Standardize composition, not every primitive.

# 5\. No reinventar estándares existentes

Si ya existe un estándar razonable para una capacidad, This is my Harness debe adoptarlo o integrarlo en vez de crear otro formato cerrado.  
Ejemplos:

* MCP: tools/resources/context.  
* WebMCP: herramientas expuestas directamente por una web viva.  
* MCP Apps: UI interactiva devuelta por tools MCP.  
* AG-UI: interacción agent ↔ frontend, eventos y state.  
* A2UI: UI declarativa generada por agentes.  
* A2A: descubrimiento y colaboración agent-to-agent.  
* Agent Skills: conocimiento portable basado en SKILL.md.  
* MCP Tasks: trabajos asíncronos/durables.  
* Arazzo: workflows machine-readable sobre APIs.  
* OpenTelemetry: observabilidad con semánticas compartidas.  
* OCI/ORAS: distribución de artefactos content-addressed.  
* Sigstore/Cosign: firmas y attestations.

# 6\. Distinción fundamental: Runtime ≠ Model

Runtime es el entorno/agente donde el usuario trabaja.

* Codex.  
* Claude Code.  
* OpenCode.  
* Cursor.  
* Gemini CLI.

Model es una capacidad cognitiva ejecutada dentro o alrededor del sistema.

* Generative model.  
* System One decision model.  
* Embedding model.  
* Reranker.  
* Classifier.  
* Vision model.  
* Speech model.  
* Reward/evaluator model.

Un harness puede combinar múltiples modelos y un runtime.

# 7\. Models como componentes first-class

La especificación debe permitir declarar modelos de forma explícita.  
Principio: describir primero la capacidad; usar marcas concretas como implementación preferida o alternativa.  
MAL:  
mustUse: laya  
MEJOR:  
requires capability: typed-decision-model  
preferred implementation: laya-local  
alternative implementation: jev-compatible-hosted  
Esto permite cambiar implementación sin romper la arquitectura.

# 8\. Contrato de modelo

Un model component serio no debería ser solamente un string.

* Input Contract.  
* Output Contract.  
* Capabilities.  
* Resource Contract.  
* Execution location: local/remote.  
* Lifecycle.  
* Fallback.  
* Version.  
* License.  
* Artifact source.

Ejemplo conceptual:  
interface:  
  input: state  
  output:  
    route: enum  
    confidence: number

# 9\. System One como capacidad nativa

La conversación elevó los modelos System One a un caso de uso central. Son especialmente útiles para microdecisiones estructuradas, rápidas y repetitivas que no necesitan generación libre de texto.  
Jev se presenta como un System One Model orientado a decisiones estructuradas y probabilísticas. Laya ofrece una implementación open source/local del patrón, con decisiones tipadas como choice, score y noul.  
El estándar no debe depender de uno u otro. Debe definir un capability contract que permita elegir implementaciones compatibles.  
Usos potenciales dentro de un harness:

* Tool routing.  
* ¿Necesito llamar al frontier LLM?  
* Risk gating.  
* Retry/no-retry.  
* Relevance scoring.  
* Chunk/context prioritization.  
* Policy decisions.  
* Security classification.  
* Browser action selection.  
* Agent/subagent routing.  
* Escalation thresholds.

# 10\. Frontier LLM \+ System One: colaboración, no sustitución

Arquitectura objetivo:  
Frontier LLM se reserva para generación, razonamiento profundo, programación, planificación y lenguaje.  
System One resuelve microdecisiones estructuradas, scoring y routing.  
La tesis es que un harness puede capturar la arquitectura de inteligencia que rodea al LLM, no solamente el prompt que se le entrega.

# 11\. Reference case: Developer Harness con System One

Este perfil deriva del caso discutido a partir del post de desarrollo con Jev/System One.  
Componentes candidatos:

* Codex / Claude Code / OpenCode.  
* Frontier LLM para código/razonamiento.  
* System One para routing, scoring y gates.  
* Repo map.  
* Retrieval por capas.  
* Context scoring.  
* Tool schema progressive disclosure.  
* Policies por scope.  
* Sensitive-data routing.  
* Command gating a nivel AST.  
* Tests.  
* Eval hooks.  
* Observability.

El objetivo no es copiar una implementación exacta, sino usar este caso como stress test del estándar.

# 12\. Hardware-aware capability resolution

Para modelos locales y servicios pesados, Harness Core debe inspeccionar capacidades de la máquina.

* OS.  
* Architecture.  
* CPU.  
* RAM.  
* GPU.  
* VRAM.  
* Disk.  
* Runtime/backend available.

Ejemplo:  
PC gamer con GPU suficiente → implementación System One local.  
Notebook limitada → provider remoto compatible.  
Offline → implementación local si existe.  
El resolver debe tomar decisiones explicables y mostrar qué implementación eligió.

# 13\. Services como componentes

Algunos elementos de un harness deben correr como servicio.

* Local model server.  
* MCP server.  
* Vector DB.  
* Embedding service.  
* Browser service.  
* Proxy/router.

Lifecycle administrable:  
install → start → healthcheck → stop → update → remove.  
Regla de red: servicios locales deben bindear a localhost por defecto. Exposición 0.0.0.0 debe ser explícita y aprobada.

# 14\. Workflow graph

Un harness complejo puede representar un grafo de ejecución, no una simple carpeta.  
Ejemplo:  
Goal → Browser Agent → WebMCP si existe → fallback estructurado si no existe → System One decision → Playwright/CDP → result.  
El Core v1 no necesita un lenguaje gráfico completo, pero el modelo de datos no debe bloquear workflows compuestos.

# 15\. Composición y dependencias

Dirección futura:  
extends:  
  \- @formosadev/base-coding  
  \- @juan/security  
  \- @formosadev/system-one-router  
Tipos de artefacto posibles:

* Full Harness.  
* Component.  
* Preset.

Tipos de component:

* Skill.  
* Model.  
* Router.  
* Policy.  
* Agent.  
* MCP.  
* Workflow.

Harness Core debe resolver dependency graph, versiones, ciclos, conflictos y permisos.

# 16\. Zero Friction como principio de producto

Principio formal:  
A developer who discovers a harness should be able to run it on a compatible agent without manually locating, copying or configuring individual harness files.  
No significa esconder riesgos.  
Significa que el sistema abstrae instalación, paths, adapters, manifests y lifecycle.

# 17\. URL canónica universal

Cada harness necesita una URL estable que funcione para humanos y máquinas.  
Conceptualmente:  
https://\<host\>/h/\<owner\>/\<slug\>  
La misma URL debe servir para:

* Página pública.  
* Compartir.  
* Resolver manifest.  
* Abrir Desktop.  
* CLI.  
* Pegar en Codex/Claude/OpenCode.  
* Resolver versión y artifact.

# 18\. One command from zero to running

Interfaz lógica:  
harness use \<canonical-url-or-owner/slug\>  
La meta real no es instalar la CLI con un comando. Es llevar al usuario desde la URL hasta un harness funcionando.  
Pipeline de usuario:  
copy command → bootstrap Harness → resolve harness → detect project → detect runtime → optional runtime install → Install Plan → snapshot → apply → verify → launch.

# 19\. Bootstrap multiplataforma

La web debería generar un único comando adecuado para Windows/macOS/Linux.  
Windows:

* Preferir canal firmado/package manager oficial cuando exista.  
* WinGet como candidato.

macOS:

* Homebrew/tap/cask \+ app firmada/notarizada.

Linux:

* Paquetes conocidos o bootstrap verificable.  
* AppImage/.deb/.rpm según distribución.

No publicar comandos ficticios antes de existir package IDs reales.  
No usar como UX principal un pipe remoto opaco del tipo curl | sh sin verificación.

# 20\. Pending Intent

Si Harness todavía no está instalado, el bootstrap debe conservar la intención original.

* Harness solicitado.  
* Versión.  
* Runtime.  
* Project.  
* Requested action.

Después de instalar Harness, el flujo continúa sin pedir al usuario que repita el link.

# 21\. Install Plan

Contrato intermedio serializable entre Web, Desktop, CLI y agentes.

* Runtime.  
* Runtime version.  
* Project.  
* Scope.  
* Harness/version.  
* Files create/modify.  
* Conflicts.  
* Adaptations.  
* Unsupported capabilities.  
* MCP.  
* Hooks/scripts.  
* Services.  
* Environment variable names.  
* Risk.  
* Snapshot.  
* Verification.

# 22\. AI-agent-native installation

El mismo link debe poder dársele directamente a un agente.  
Ejemplo:  
“Usá este harness acá: \<URL\>.”  
El agente debe descubrir una guía oficial y delegar filesystem mutation a Harness Core.  
Regla:  
El agente orquesta; Harness Core instala, adapta, crea snapshots y revierte.

# 23\. Machine-readable agent install guide

Endpoint conceptual:  
/api/v1/harnesses/\<owner\>/\<slug\>/agent-install  
Respuesta candidata:

* canonical\_url.  
* resolved\_version.  
* recommended\_command.  
* supported\_runtimes.  
* preferred\_scope.  
* risk\_class.  
* requires\_confirmation.  
* install\_protocol\_version.  
* manifest\_url.

Objetivo: los agentes no inventan comandos desde memoria.

# 24\. CLI output JSON

Los agentes no deben parsear texto humano cuando ejecutan Harness.  
Estados tipados conceptuales:

* planned.  
* needs\_confirmation.  
* installed.  
* verification\_failed.  
* reverted.

Ejemplo conceptual:  
{ status: "needs\_confirmation", risk: "executable", confirmationReason: "hook execution" }  
No debe existir un \--yes absoluto capaz de saltar operaciones Elevated.

# 25\. Niveles de autonomía

## 25.1 Preview

Resolver metadata, descargar manifest, calcular plan, analizar diff.

## 25.2 Safe Apply

Cambios pasivos dentro de project scope con confirmación/policy simple.

## 25.3 Trusted Harness

Publisher/harness previamente autorizado puede recibir menor fricción para updates de bajo riesgo.

## 25.4 Elevated

Scripts, hooks, user scope, runtime installation, admin/sudo o acceso sensible requieren confirmación explícita.

# 26\. WebMCP como capability de un Web Agent Harness

WebMCP permite que una aplicación web exponga herramientas JavaScript a agentes. Al momento de esta consolidación, la especificación figura como Draft Community Group Report del Web Machine Learning Community Group y cuenta con Web Platform Tests.  
Esto habilita un perfil de harness especializado para aplicaciones web agent-native.  
Idea:  
Website → WebMCP tools → agent.  
Si WebMCP no está disponible, el harness puede declarar un fallback de browser automation.

# 27\. MCP Apps / MCP-UI

La dirección estable ya no debería modelarse como una feature propietaria llamada MCP-UI. MCP Apps es una extensión oficial de MCP para que un server entregue UI interactiva a un host.  
Casos:

* Forms.  
* Dashboards.  
* Visualizations.  
* Approval workflows.  
* Real-time displays.

Un Web Agent Harness puede combinar WebMCP para herramientas de la web y MCP Apps para interfaces interactivas entregadas por tools.

# 28\. AG-UI

AG-UI representa otra capa: interacción y transporte de eventos/state entre agentes y aplicaciones frontend.  
En This is my Harness debe aparecer como capability integrable, no como sustituto de MCP.

# 29\. A2UI

A2UI define un protocolo de UI declarativa/streaming enviada desde un agente hacia un renderer.  
Puede entrar como capability separada de MCP Apps porque resuelve una arquitectura distinta.  
Un harness web puede combinar:

* AG-UI para comunicación.  
* A2UI para UI declarativa.  
* MCP Apps para UI de herramientas.  
* WebMCP para tools expuestas por la web.

# 30\. A2A

A2A es la capa de interoperabilidad agent-to-agent.  
MCP conecta agentes con tools/resources; A2A conecta agentes con otros agentes.  
Un Multi-Agent Harness puede declarar agentes distintos, sus capacidades, delegation paths y comunicación A2A.

# 31\. Agent Skills

Agent Skills/SKILL.md debe adoptarse donde tenga sentido en lugar de inventar un nuevo formato de skill.  
This is my Harness distribuye, versiona, instala y compone skills; no necesita redefinir su contenido desde cero.

# 32\. MCP Tasks

MCP Tasks agrega handles durables para trabajos asíncronos y diferidos.  
Capacidad útil para:

* CI jobs.  
* Deployments.  
* Human approvals.  
* Long-running processing.  
* Deferred agent work.

Puede ser parte de Workflow Automation y Multi-Agent Harnesses.

# 33\. Arazzo

Arazzo 1.1 describe workflows machine-readable sobre OpenAPI y AsyncAPI.  
Para harnesses de automatización, puede representar secuencias determinísticas sobre APIs sin inventar otro workflow DSL.

# 34\. OpenTelemetry

OpenTelemetry mantiene convenciones de GenAI y MCP, todavía con áreas en Development.  
El harness puede declarar observability adapters y policies sin fijar irreversiblemente el core a un schema experimental.  
Uso:

* Traces.  
* Metrics.  
* Tool/MCP spans.  
* Workflow/agent spans.  
* Usage.  
* Error analysis.

# 35\. Distribución con OCI/ORAS

Una decisión fuerte es evitar inventar un transport binario propietario.  
OCI 1.1 admite artefactos no-container mediante artifactType y relaciones mediante subject/referrers.  
ORAS permite usar OCI registries para artefactos arbitrarios.  
Esto habilita una dirección como:  
ghcr.io/\<org\>/harnesses/\<owner\>/\<name\>:1.2.0  
artifactType conceptual:  
application/vnd.thisismyharness.bundle.v1  
Beneficios:

* Immutability por digest.  
* Versioned artifacts.  
* Existing registries.  
* Referrers para signatures/attestations.  
* Tooling maduro.

# 36\. Firmas y attestations

Cosign/Sigstore puede firmar blobs y artefactos. El estándar debe permitir verificar integridad/firma antes de Apply.  
Trust label debe ser preciso:

* Manifest valid.  
* Hash verified.  
* Signature verified.  
* Author identity verified.  
* Maintainer reviewed.  
* Runtime tested.

Nunca “100% safe”.

# 37\. Reference Harness \#1 — Web Agent Harness

Objetivo: demostrar que un harness puede representar una aplicación web agent-native completa.  
Componentes candidatos:

* WebMCP.  
* MCP backend.  
* MCP Apps.  
* AG-UI.  
* A2UI.  
* System One local para decisions rápidas.  
* Playwright/CDP fallback.  
* OpenTelemetry.  
* Security policies.

Flujo:  
Goal → Browser Agent → WebMCP si existe → semantic/browser fallback si no → System One decision → action → result.  
Qué prueba del estándar:

* Interfaces web.  
* Multiple protocols.  
* Browser fallback.  
* Models.  
* Services.  
* Security.  
* Observability.

# 38\. Reference Harness \#2 — Developer Harness

Objetivo: reproducir un entorno de desarrollo agentic sofisticado.  
Componentes:

* Codex/Claude/OpenCode.  
* Agent Skills.  
* System One decision layer.  
* Repo retrieval.  
* Context prioritization.  
* Policies.  
* AST command gates.  
* Tests/evals.  
* MCP.  
* Observability.

Qué prueba:

* Runtime adapters.  
* Cross-runtime portability.  
* Model collaboration.  
* Local/remote capability substitution.  
* Project-scope installation.  
* Policy enforcement.

# 39\. Reference Harness \#3 — Local Hybrid Harness

Objetivo: combinar frontier LLM con inteligencia auxiliar local.  
Componentes:

* Remote generative model.  
* Local System One decision model.  
* Hardware detection.  
* ONNX/PyTorch backend according to implementation.  
* Fallback provider.  
* Local model service.  
* Threshold policies.

Qué prueba:

* Hardware-aware resolution.  
* Model contracts.  
* Local service lifecycle.  
* Offline/online substitution.

# 40\. Reference Harness \#4 — Multi-Agent Harness

Objetivo: validar composición de múltiples agentes.

* A2A.  
* MCP.  
* MCP Tasks.  
* Delegation.  
* Approvals.  
* Shared policies.  
* Observability.

Qué prueba:

* Agent identity.  
* Agent cards/capabilities.  
* Task delegation.  
* Durable async work.  
* Cross-framework interoperability.

# 41\. Reference Harness \#5 — Workflow Automation Harness

Objetivo: validar automation determinística \+ agentic.

* OpenAPI.  
* Arazzo.  
* AsyncAPI.  
* MCP.  
* Tasks.  
* System One gates.  
* Human approval.  
* OpenTelemetry.

Qué prueba:

* Declarative workflows.  
* Long-running state.  
* Policy gates.  
* API composition.  
* Auditable automation.

# 42\. Reference Harnesses son torture tests del estándar

No son solamente demos comerciales. Deben romper el schema.  
Regla propuesta:  
No declarar Harness Spec v1.0 hasta que estos perfiles hayan sido implementados y usados en escenarios reales y el core pueda representarlos sin hacks específicos.  
Si una capability obliga a meter lógica especial dentro del UI o del core, revisar la abstracción.

# 43\. Profiles

Profiles no son estándares separados.  
Son especializaciones sobre el mismo core.

* developer.  
* web-agent.  
* local-hybrid.  
* multi-agent.  
* workflow-automation.

Beneficios:

* Validadores específicos.  
* UX específica.  
* Defaults sensatos.  
* Reference implementations.  
* Discoverability.

# 44\. Labs y proceso de innovación

This is my Harness también debe funcionar como foro técnico para experimentar con nuevas ideas agentic sin desestabilizar el core.  
Pipeline:  
Labs → Reference Harness → Real-world use → Extension candidate → Conformance tests → Stable extension.  
Repo conceptual:  
/labs  
/rfcs  
/extensions  
/profiles  
/conformance  
/examples  
Esto permite estar en la frontera sin convertir cada novedad en norma permanente.

# 45\. Registry y social layer, reposicionados

Siguen vigentes:

* Public profiles.  
* Builder search.  
* Harness search.  
* Votes.  
* Comments.  
* Saves.  
* Follows.  
* Collections.  
* Feed.  
* Verified Use.  
* Trending/New/Most Used.

Pero la red social consume artefactos del estándar. El repo técnico no debe depender de la red social para tener sentido.

# 46\. Perfil público como biblioteca de artefactos

El perfil sigue siendo obligatorio para publicación pública.  
Debe mostrar harnesses conformantes, versiones, runtime support y señales de uso.  
La acción primaria sigue siendo Use, pero la identidad social está subordinada a la reproducibilidad técnica.

# 47\. Búsqueda para humanos y agentes

Search debe funcionar para:

* Users/builders.  
* Harnesses.  
* Use cases.  
* Capabilities.  
* Runtime.  
* Components.  
* Collections.

Agent-facing API conceptual:  
GET /api/v1/search  
GET /api/v1/users/\<username\>  
GET /api/v1/harnesses/\<owner\>/\<slug\>  
GET /api/v1/harnesses/\<owner\>/\<slug\>/versions/\<version\>/manifest  
Eventualmente un MCP oficial de discovery puede exponer search\_harnesses/get\_harness/get\_builder/get\_manifest/get\_install\_plan.

# 48\. Verified Use

Sigue siendo una señal valiosa, pero no prueba calidad ni seguridad.  
Significa que un cliente oficial aplicó/activó una versión específica con consentimiento.  
No debe enviar:

* Project name.  
* Local path.  
* Source code.  
* Prompts.  
* API keys.  
* Private repository metadata.

# 49\. Risk classes

* Passive — instrucciones/config sin ejecución.  
* Tooling — MCP/plugins/integrations.  
* Executable — hooks/scripts/commands.  
* Privileged — user scope, admin, secrets, broad system access.

Risk class gobierna UX y autonomía.

# 50\. Seguridad del filesystem

* Project scope por defecto.  
* No borrar archivos no administrados.  
* No sobrescribir conflictos silenciosamente.  
* Snapshot antes de writes.  
* Operation journal.  
* Crash recovery.  
* Rollback.  
* Path traversal protection.  
* Symlink escape protection.  
* Hash verification.  
* No ejecutar código en Preview.  
* No resolver URLs arbitrarias como comandos.

# 51\. Supply chain

Amenazas:

* Malicious harness.  
* Compromised author.  
* Tampered artifact.  
* Compromised updater.  
* Dependency compromise.  
* Fake compatibility.  
* License contamination.

Controles:

* Immutable versions.  
* Content digests.  
* Signatures.  
* Attestations.  
* Schema validation.  
* Secret scanning.  
* Human review where appropriate.  
* Conformance tests.

# 52\. Repository structure revisada

This-is-my-harness/  
├─ spec/  
│  ├─ core/  
│  ├─ manifest/  
│  ├─ package/  
│  ├─ install-protocol/  
│  └─ adapter-contract/  
├─ extensions/  
│  ├─ mcp/  
│  ├─ webmcp/  
│  ├─ mcp-apps/  
│  ├─ system-one/  
│  ├─ a2a/  
│  ├─ ag-ui/  
│  ├─ a2ui/  
│  ├─ agent-skills/  
│  ├─ arazzo/  
│  ├─ opentelemetry/  
│  └─ oci/  
├─ schemas/  
├─ packages/  
│  ├─ core/  
│  ├─ validator/  
│  ├─ sdk/  
│  └─ cli/  
├─ adapters/  
│  ├─ codex/  
│  ├─ claude-code/  
│  └─ opencode/  
├─ profiles/  
├─ labs/  
├─ conformance/  
├─ examples/  
├─ rfcs/  
├─ CONTRIBUTING.md  
├─ SECURITY.md  
├─ LICENSE  
└─ README.md

# 53\. Licencia

Decisión actual: Apache License 2.0 para especificación/tooling open source.  
Motivos:

* Permisiva para adopción comercial y open source.  
* Grant explícito de patentes.  
* Adecuada para infraestructura developer.

Servicios hosted, social layer, team governance o componentes comerciales futuros pueden usar otro modelo sin cerrar el estándar.

# 54\. Orden de implementación revisado

* 1\. Vocabulario y Core Spec v1alpha.  
* 2\. harness.yaml schema.  
* 3\. Package fixtures.  
* 4\. Validator.  
* 5\. Harness Core plan engine.  
* 6\. Codex adapter.  
* 7\. Snapshot/Apply/Verify/Revert.  
* 8\. Canonical identifier.  
* 9\. Immutable artifact distribution.  
* 10\. One-command bootstrap en un OS.  
* 11\. Agent install guide \+ JSON output.  
* 12\. Developer Reference Harness.  
* 13\. Web Agent Reference Harness.  
* 14\. Claude Code adapter.  
* 15\. OpenCode adapter.  
* 16\. System One/local model extension.  
* 17\. OCI distribution experiment.  
* 18\. Remaining reference harnesses.  
* 19\. Conformance suite.  
* 20\. v1.0 candidate.  
* 21\. Social registry scale-up.

# 55\. Criterio de lanzamiento

No lanzar el estándar como estable por documentación o branding.  
Mínimos recomendados:

* Manifest v1alpha probado.  
* Validator real.  
* Core puede planificar e instalar.  
* Rollback probado.  
* Al menos tres runtime adapters en distinto nivel.  
* Cinco Reference Harnesses representables.  
* System One/local model case funcionando.  
* WebMCP/MCP Apps case funcionando.  
* Artifact integrity/signature story definida.  
* Conformance suite.  
* Threat model.  
* One-command path al menos en un OS completamente probado.

El lanzamiento público puede ocurrir antes como alpha/labs; lo que no debe ocurrir antes es declarar v1.0.

# 56\. Diferenciador estratégico

El producto no debería venderse como “otra configuración para agentes”.  
Diferenciador:  
Publish a reproducible AI architecture.  
O:  
Package the intelligence around your agent.  
O:  
An open standard for packaging, distributing and running complete agentic systems.  
Esto eleva el alcance de prompts/config a arquitectura portable.

# 57\. Lo que no debe hacerse

* Inventar otro skill format si Agent Skills sirve.  
* Inventar otro MCP.  
* Hardcodear Laya/Jev en el Core.  
* Confundir runtime con model.  
* Declarar todos los runtimes “compatible” por marketing.  
* Ejecutar scripts sin preview/policy.  
* Publicar comandos de instalación ficticios.  
* Declarar v1.0 antes de reference harnesses.  
* Hacer que la red social defina la semántica del estándar.  
* Convertir cada novedad del ecosistema en core inmediatamente.

# 58\. Preguntas abiertas

* Nombre definitivo del manifest.  
* Media type OCI definitivo.  
* Namespace y canonical identifier.  
* Qué capabilities entran en Core vs extension.  
* SemVer de spec vs SemVer de package.  
* Contrato exacto de model capability.  
* Cómo expresar workflow graphs sin crear un DSL innecesario.  
* Cómo expresar dependencies/composition.  
* Qué significa conformance por runtime.  
* Quién puede emitir badges de conformance.  
* Cómo versionar profiles.  
* Qué policy language usar.  
* Hasta dónde llega auto-install de runtimes.  
* Cómo manejar secrets declarativamente sin almacenarlos.  
* Cómo exponer local services.  
* Cuándo separar spec repo de implementation repo.

# 59\. Decisiones consolidadas

* This is my Harness es primero un estándar y toolchain.  
* El harness representa un sistema agentic completo, no solo archivos de prompt.  
* El Core debe ser pequeño.  
* Capacidades avanzadas entran como extensions.  
* Se adoptan estándares existentes cuando corresponda.  
* Runtime y model son conceptos separados.  
* Models y services son first-class components.  
* System One es una capability estratégica.  
* La arquitectura favorece capability contracts sobre vendors.  
* Hardware-aware resolution forma parte de la visión.  
* One-command y agent-native install siguen siendo requisitos centrales.  
* Harness Core es autoridad de mutation local.  
* OCI/ORAS es candidato fuerte para distribución.  
* Firmas y attestations son parte del trust model.  
* Reference Harnesses preceden a v1.0.  
* Profiles especializan el estándar sin fragmentarlo.  
* Labs/RFCs permiten innovar sin desestabilizar Core.  
* Social layer se construye encima del estándar.  
* Apache-2.0 es la licencia actual para la capa abierta.

# 60\. Fuente de verdad entre documentos

Documento 1: producto/registry inicial, perfil público, static-first y publicación.  
Documento 2: social network \+ Harness Desktop \+ zero friction \+ one-command, runtime manager, monetización y backend.  
Documento 3 / README v0.2: explicación pública/técnica condensada del repositorio y del estándar.  
Este Documento 4: consolidado exhaustivo de las decisiones posteriores al Documento 2, incluyendo la redefinición como estándar de sistemas agentic, System One/local models, protocolos web/agent, OCI distribution y Reference Harnesses.

# 61\. Fuentes técnicas verificadas

[WebMCP — W3C Community Group draf](https://webmachinelearning.github.io/webmcp/)t  
[MCP Apps — official extension announcemen](https://blog.modelcontextprotocol.io/posts/2026-01-26-mcp-apps/)t  
[MCP Apps — specification overvie](https://apps.extensions.modelcontextprotocol.io/api/documents/overview.html)w  
[MCP Tasks — draft extensio](https://tasks.extensions.modelcontextprotocol.io/specification/draft/tasks)n  
[A2UI Protocol v1.](https://github.com/a2ui-project/a2ui/blob/main/specification/v1_0/docs/a2ui_protocol.md)0  
[A2A Protoco](https://a2a-protocol.org/v1.0.0/)l  
[A2A joins Agentic AI Foundatio](https://a2a-protocol.org/latest/blog/2026/08/27/a-new-chapter-for-a2a-joining-the-agentic-ai-foundation/)n  
[Arazzo Specification 1\.](https://www.openapis.org/arazzo-specification)1  
[OpenTelemetry GenAI / MCP semantic convention](https://github.com/open-telemetry/semantic-conventions-genai/blob/main/docs/gen-ai/mcp.md)s  
[OCI Image and Distribution Specs 1.1 — artifacts/referrer](https://opencontainers.org/posts/blog/2024-03-13-image-and-distribution-1-1/)s  
[ORAS — OCI artifact](https://oras.land/docs/1.1/)s  
[Sigstore Cosign — signing blob](https://docs.sigstore.dev/cosign/signing/signing_with_blobs/)s  
[Laya — System One decision engin](https://github.com/NandhaKishorM/laya)e  
[Laya Node/TypeScript ONNX implementatio](https://github.com/receptron/laya)n  
[Laya server — Jev-compatible local AP](https://github.com/nvkudva/laya-server)I  
[TypeSafe AI — Introducing System One Models & Je](https://typesafe.ai/blog/introducing-system-one-models-and-jev)v  
[One System — local/hosted decision models through one AP](https://github.com/rawwerks/one-system)I

# 62\. Conclusión

La oportunidad de This is my Harness no está en inventar cada pieza del ecosistema agentic. Está en hacer que piezas que hoy están fragmentadas puedan describirse como un sistema reproducible, verificable y portable.  
El estándar debe poder empaquetar desde un harness mínimo de desarrollo hasta una arquitectura web con WebMCP, MCP Apps, UI protocols, modelos System One locales, servicios, workflows y agentes múltiples.  
La prueba de calidad del estándar no será cuántas features enumere el README, sino si un builder puede publicar una arquitectura real y otra persona puede reproducirla en otra máquina con una instalación explicable, segura y reversible.  
La estrategia recomendada es construir primero Reference Harnesses extremos, dejar que rompan el diseño y estabilizar solamente aquello que sobreviva. Esa es la vía para que This is my Harness sea un estándar usado y no solamente una buena idea.  
