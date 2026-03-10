# Installation

## Pré-requis

- JDK 17+
- Le SDK utilise JNA pour charger la bibliothèque native (générée via UniFFI).

## Configuration du dépot

Le SDK est publié sur GitHub Packages. Ajoutez le dépot a votre fichier de build :

```kotlin
repositories {
    maven {
        url = uri("https://maven.pkg.github.com/idpass/claim-169")
        credentials {
            username = project.findProperty("gpr.user") as String? ?: System.getenv("GITHUB_ACTOR")
            password = project.findProperty("gpr.key") as String? ?: System.getenv("GITHUB_TOKEN")
        }
    }
}
```

!!! note
    GitHub Packages nécessite une authentification même pour les packages publics. Vous avez besoin d'un token GitHub avec le scope `read:packages`.

## Gradle (Kotlin DSL)

```kotlin
dependencies {
  implementation("org.idpass.claim169:claim169-core:0.2.0-alpha")
}
```

## Gradle (Groovy)

```groovy
dependencies {
  implementation "org.idpass.claim169:claim169-core:0.2.0-alpha"
}
```

## Où placer la lib native

Dans la plupart des cas (usage standard), vous n’avez rien à faire : la lib native est embarquée et chargée par JNA.

Si vous avez un besoin spécifique (tests, packaging, Android, chemins custom), vous devrez peut-être configurer :

- `java.library.path`
- `jna.library.path`

!!! note "Détails"
    Pour l’ordre exact de recherche et les exemples de configuration (Linux/macOS/Windows, Android), basculez sur la version anglaise via le sélecteur de langue (English).
