# La defensa en vivo: 5 minutos, 4 voces

> En español, porque se dice en español ante el jurado de Montevideo. Cada número sale de `pitch/numbers.md`; ninguno
> se cambia ni se redondea distinto en escena. La duración del jurado local no está publicada (ni Space Uruguay ni la
> guía de Space Apps la dicen: la fija cada sede); este guion dura **5:00** y abajo está el **corte de 3:00**. Hay
> que confirmarla con la organización el día antes.

Hablan **Axel, Kevin, Bayron y Santiago**. Kevin maneja la laptop toda la presentación; nadie más la toca.

## La idea de la defensa

Una sola pregunta abre y cierra: **"¿A cuál le creen?"**. Dos números de la NASA para el mismo lugar y el mismo año,
uno doce veces y media el otro. Todo lo que sigue es la respuesta, y el cierre la devuelve. En el medio, una sola
acción que nadie más va a hacer en el escenario: **apagar el wifi y seguir contestando**.

Técnicas que usa, y dónde:

| Técnica | Dónde | Por qué funciona |
|---|---|---|
| Interrupción de patrón | 0:00: silencio, sin saludo, dos números | El jurado viene de escuchar "Hola, somos el equipo X". Lo distinto se recuerda. |
| Lazo abierto | 0:00 a 4:25: "¿A cuál le creen?" queda sin responder | El cerebro quiere cerrar la pregunta; el jurado escucha hasta el final. |
| La frase de ascensor | 0:40: para quién, qué es, qué lo hace distinto | En 20 segundos el jurado sabe qué evalúa. |
| Mostrar, no contar | 1:05: la demo en vivo, no capturas | Un número que aparece en pantalla vale más que uno dicho. |
| Honestidad como prueba | 2:55: el 0,77 que no da exacto | Admitir el límite hace creíble todo lo demás. |
| El golpe | 3:45: "Kevin, apagá el wifi" | Es la acción que se cuenta en el pasillo después. |
| Relevo con pregunta | Cada cambio de voz termina con una pregunta al siguiente | No hay silencios muertos ni "ahora les va a hablar…". |
| Callback y regla de tres | 4:25: vuelve la pregunta; cierre en tres golpes | Cierra el lazo y deja una frase para repetir. |

## El guion

Unas 690 palabras: un ritmo tranquilo para cinco minutos (unas 140 por minuto). Lo que va entre paréntesis se hace,
no se dice.

### 0:00 a 0:40 — Axel: el gancho

**Pantalla:** papel vacío. Se dibujan dos números, uno debajo del otro.

> (Axel entra al centro. No saluda. Tres segundos de silencio.)
>
> Cincuenta mil cuatrocientos noventa y siete.
>
> Seiscientos treinta y un mil setecientos cincuenta y uno.
>
> Mismo lugar: el sudeste de Australia. Mismo año: 2019, el Verano Negro. Dos satélites de la NASA contando focos de
> calor. Uno vio doce veces y media más que el otro.
>
> ¿A cuál le creen?
>
> (Pausa. Mirar al jurado.)
>
> Esa pregunta hoy no la puede responder nadie que tenga que decidir dónde mirar el fuego. MODIS mira desde el 2000;
> VIIRS, desde el 2012. Y sus números no se pueden poner uno al lado del otro.

### 0:40 a 1:05 — Axel: qué es firecal

**Pantalla:** el logo se dibuja; debajo, una línea.

> Somos seis uruguayos y construimos firecal: un calendario de actividad de quema para cualquier zona que dibujes,
> con los dos satélites puestos de acuerdo, y con algo que nadie más te da: cuánto confiar en cada número.
>
> Kevin, mostrales.

### 1:05 a 2:20 — Kevin: la demo

**Pantalla:** la app, en vivo. La región del sudeste de Australia ya está guardada para usar sin conexión (ver
"Antes de subir").

> Dibujo un rectángulo sobre la costa sur de Nueva Gales del Sur. Eso es todo lo que hace el usuario.
>
> (El calendario se dibuja mes por mes.)
>
> Meses de izquierda a derecha, años de arriba abajo. Veinte años en una imagen. Cuanto más oscuro, más focos.
>
> (Señalar diciembre de 2019.)
>
> Miren diciembre de 2019: siete mil ciento doce focos de calor, más o menos ochenta y ocho. La mediana de un
> diciembre acá es siete. Ese rombo dice "inusual", y no lo decide el color: lo decide el núcleo, solo cuando el mes
> supera su banda de incertidumbre y todos los años anteriores. Si fuera un cambio de sensor, no lo marcaría.
>
> (Tocar **Energy**.)
>
> Y no solo contamos: en energía radiativa, muchas quemas chicas y un incendio grande dejan de parecerse.
>
> (Tocar **Count**, después el año **2009** en "Years most like 2019".)
>
> ¿A qué años se parece esta temporada? 2009, 2001 y 2002. Es historia, no un pronóstico. Y todo se descarga en CSV.
>
> Bayron: ¿cómo pusiste de acuerdo a dos satélites que difieren doce veces?

### 2:20 a 3:20 — Bayron: cómo se ponen de acuerdo

**Pantalla:** la figura "antes y después": los puntos crudos lejos de la diagonal; los armonizados, sobre ella.

> VIIRS ve fuegos más chicos, entonces cuenta muchos más. No alcanza con dividir.
>
> Hicimos mapeo por cuantiles: en cada cuadrado de diez grados del planeta, tomamos los meses que vieron los dos
> satélites a la vez y le enseñamos a VIIRS a hablar en unidades de MODIS. Lo que sobra, el error que queda en esos
> años, no lo escondemos: es la incertidumbre que vieron en cada celda.
>
> La prueba de verdad: sacamos 2019 entero del entrenamiento. El modelo nunca lo vio. Crudo, VIIRS daba doce veces y
> media lo de MODIS. Armonizado: cero coma setenta y siete.
>
> No da exacto, y les decimos por qué: 2019 fue más extremo que cualquier mes con el que aprendió. Preferimos
> mostrarles eso antes que un número perfecto que no podríamos defender.
>
> Santiago.

### 3:20 a 4:25 — Santiago: el motor y el golpe

**Pantalla:** la app otra vez; el ícono del wifi tiene que verse.

> Para esto leímos el archivo público entero de FIRMS para Australia: dieciséis millones seiscientos mil registros,
> en siete segundos, en una laptop.
>
> Todo lo resuelve un solo núcleo en Rust, compilado dos veces: nativo para procesar los datos y WebAssembly para su
> navegador. Probamos ocho mil novecientas sesenta y ocho consultas de las dos formas: dan idéntico, byte a byte.
>
> ¿Por qué importa?
>
> Kevin, apagá el wifi.
>
> (Kevin apaga el wifi a la vista. Silencio de dos segundos.)
>
> Imaginen que están en una ruta del interior, sin señal, con humo en el horizonte.
>
> (Kevin elige el polígono y dibuja un área nueva dentro de la región guardada. El calendario contesta.)
>
> Un área que nunca consultamos. Sin internet. Respondió en menos de un milisegundo. La misma función que armonizó
> veinte años de satélites está corriendo ahí, en esa laptop.

### 4:25 a 5:00 — Axel: el cierre

**Pantalla:** vuelven los dos números del principio; entre ellos se dibuja una banda.

> Volvamos a la pregunta. ¿A cuál le creen, a cincuenta mil o a seiscientos mil?
>
> A ninguno solo. A los dos juntos, con su margen de error.
>
> firecal ya cubre toda Australia, del 2000 al 2019, y está en línea: firecal punto pages punto dev. Armar una región
> entera le lleva menos de medio segundo; lo que sigue es Uruguay y el resto del mundo.
>
> El sensor deja de importar. Cada número dice cuánto confiar en él. Y funciona sin señal.
>
> Gracias.

## Las diapositivas

`pitch/slides/`: seis diapositivas dibujadas a mano con el mismo motor del sitio y del film (`site/js/draw.js`), que
se trazan solas y hierven ocho veces por segundo. Cada clic lleva el dibujo a la próxima parada; al pasar de
diapositiva, la vieja se desdibuja mientras la nueva se dibuja.

```
python3 -m http.server 8630 --directory .
npm --prefix web run preview
```

Desde la raíz del repo: la primera sirve las diapositivas en http://localhost:8630/pitch/slides/, la segunda la app en
http://localhost:4173/. **F** pantalla completa; **→**, espacio o clic, avanzar; **←**, volver; **D**, la app en vivo
encima de las diapositivas (y **D** otra vez para volver); **Inicio**, la primera.

| Diapositiva | Clic en… | Qué aparece |
|---|---|---|
| 1 | Antes de hablar | 50.497 |
| 1 | "Seiscientos treinta y un mil…" | 631.751 |
| 1 | "Mismo lugar…" | Sudeste de Australia, los dos satélites, la flecha del × 12,5 |
| 1 | "¿A cuál le creen?" | La pregunta |
| 2 | "Somos seis uruguayos…" | El logo se dibuja, firecal y sus tres líneas |
| 3 | "Kevin, mostrales." | El calendario de respaldo. **D** abre la app en vivo |
| 3 | (solo si la app falla) "Miren diciembre de 2019" | 7.112 ± 88, rodeado |
| 3 | (solo si la app falla) "¿A qué años se parece…?" | 2009, 2001 y 2002 marcados |
| 4 | "Bayron:" (Kevin cierra la app con **D**) | MODIS contra VIIRS crudo, × 12,5 |
| 4 | "Armonizado: cero coma setenta y siete." | La barra de VIIRS baja a 38.910, × 0,77 |
| 5 | "Santiago." | Archivo, núcleo en Rust, navegador |
| 5 | "¿Por qué importa?" | El ícono del wifi. **D** abre la app para el golpe |
| 5 | "…corriendo ahí, en esa laptop." (Kevin cierra la app) | El wifi tachado, "Respondió en menos de un milisegundo." |
| 6 | "Volvamos a la pregunta." | Los dos números otra vez |
| 6 | "A ninguno solo." | La banda de incertidumbre y las tres líneas |
| 6 | "Y funciona sin señal." | firecal y firecal.pages.dev |

Kevin hace todos los clics. Las diapositivas y la app corren en la laptop, así que siguen andando con el wifi apagado.

## Si Brandon habla

Toma la lectura del calendario (1:05 a 2:20) mientras Kevin maneja la laptop; Kevin se queda con el golpe del wifi,
porque la mano que apaga es la misma que dibuja. Nada más cambia.

## El corte de 3:00

Si el jurado da tres minutos, se saca, en este orden, sin tocar el gancho, el golpe ni el cierre:

1. Energía y años parecidos (Kevin, 1:50 a 2:15): queda "Y todo se descarga en CSV" pegado a lo inusual.
2. La explicación de cuantiles (Bayron): queda solo "La prueba de verdad…" hasta el final.
3. Los dieciséis millones y el byte a byte (Santiago): arranca en "Todo lo resuelve un solo núcleo en Rust…" y salta a
   "¿Por qué importa?".
4. "Armar una región entera…" (Axel, cierre).

## Antes de subir (10 minutos antes)

1. La app servida desde la laptop con las fichas al lado (`cargo xtask build-wasm`, las fichas del release
   `tiles-australia-2019` en `web/public/tiles/`, `npm --prefix web run build`, `npm --prefix web run preview`). No
   la de internet: si la red del lugar falla, falla la demo.
2. Entrar al sudeste de Australia (West 147, South −38, East 151, North −34), **Use this area**, **Pin for offline
   use**, esperar "Pinned 2 tiles". Probar una vez con el wifi apagado: dibujar un polígono adentro y que conteste.
   Volver a prender el wifi.
3. Pantalla completa, zoom 100 %, notificaciones apagadas, el ícono del wifi a la vista.
4. El film de 30 segundos abierto en otra ventana, por si algo se rompe.
5. Cronómetro a la vista de Axel. Si a las 2:20 Kevin no terminó, Bayron arranca igual.

## Si algo se rompe

| Qué | Quién | Qué se dice |
|---|---|---|
| La app no carga | Kevin | "Les muestro lo mismo en el film" y pone el video; el guion sigue igual. |
| Offline no contesta | Santiago | "Sin señal y sin respuesta: ese es el límite que vamos a mejorar", y sigue al cierre. |
| Se pasan de tiempo | Axel | Salta directo a "Volvamos a la pregunta". El cierre nunca se corta. |

## Lo que nunca se dice

- "Incendios detectados" o "área quemada": son **focos de calor**, anomalías térmicas vistas desde órbita.
- "Pronóstico" o "predicción": los años parecidos son historia.
- "Exacto" o "preciso" sin decir "validado solo en los años que vieron los dos".
- Un número que no esté en `pitch/numbers.md`.

## Preguntas del jurado: quién contesta

| Pregunta | Contesta | Respuesta corta |
|---|---|---|
| ¿Cómo saben que la armonización está bien? | Bayron | Se ajusta en los años que vieron los dos; el error que queda en esos años es la incertidumbre que mostramos. Y la probamos en 2019, que el modelo no vio. |
| ¿Por qué no da exacto en 2019? | Bayron | 2019 superó todo lo que el modelo había visto; lo decimos en vez de esconderlo. |
| ¿Por qué Australia y no Uruguay? | Santiago | El archivo público de Uruguay viene cortado en la frontera, y cada cuadrado de diez grados que lo toca también toca Argentina o Brasil. Necesitamos un modo regional; es lo próximo. |
| ¿Por qué no hay servidor? | Santiago | Un sitio estático no se cae y funciona sin señal. El mismo código corre en la laptop y en el navegador. |
| ¿Qué tan rápido es? | Santiago | 0,75 milisegundos para la serie, la comparación y los años parecidos, en una Apple M4. |
| ¿Es un pronóstico? | Axel | No. Dice que este mes superó lo que muestra el pasado, y que no es un cambio de sensor. |
| ¿Quién lo usaría? | Axel | Quien decide dónde mirar el fuego: gestión de emergencias, de campos, de parques; en la oficina o en la ruta. |
| ¿Usaron IA? | Axel | Sí: agentes de programación escribieron gran parte del código bajo nuestro diseño y nuestras reglas. Lo dice el README y el film lo marca. Los números son todos de datos reales de FIRMS. |
| ¿Qué no hace? | Axel | Alertas, cuentas de usuario, pronósticos, ni el historial de cada foco. |

## Ensayos

| Ensayo | Fecha y hora | Duración | Qué no salió |
|---|---|---|---|
| 1 | | | |
| 2 | | | |
